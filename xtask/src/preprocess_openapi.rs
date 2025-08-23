use serde_yaml::{Mapping, Value};
use std::fs;

// --- Main function to orchestrate the file reading, processing, and writing ---
pub fn preprocess_openapi_file(
    input_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // --- Step 1: Read the OpenAPI spec from the input file ---
    println!("Reading OpenAPI spec from {}...", input_path);
    let input_content = fs::read_to_string(input_path)
        .map_err(|e| format!("Failed to read {}: {}", input_path, e))?;
    let mut doc: Value = serde_yaml::from_str(&input_content)?;

    // Ensure the root is a mutable mapping
    let doc_root = doc
        .as_mapping_mut()
        .ok_or("OpenAPI spec root is not a YAML map")?;

    // --- Step 2: Apply all the required transformations ---
    println!("Preprocessing the spec...");
    unwrap_search_parameters(doc_root)?;
    unwrap_multi_search_parameters(doc_root)?;
    unwrap_parameters_by_path(
        doc_root,
        "/collections/{collectionName}/documents/import",
        "post",
        "importDocumentsParameters",
        Some("ImportDocumentsParameters"), // Copy schema to components
    )?;
    unwrap_parameters_by_path(
        doc_root,
        "/collections/{collectionName}/documents/export",
        "get",
        "exportDocumentsParameters",
        Some("ExportDocumentsParameters"), // Copy schema to components
    )?;
    unwrap_parameters_by_path(
        doc_root,
        "/collections/{collectionName}/documents",
        "patch",
        "updateDocumentsParameters",
        Some("UpdateDocumentsParameters"), // Copy schema to components
    )?;
    unwrap_parameters_by_path(
        doc_root,
        "/collections/{collectionName}/documents",
        "delete",
        "deleteDocumentsParameters",
        Some("DeleteDocumentsParameters"), // Copy schema to components
    )?;
    remove_additional_properties_from_search_hit(doc_root)?;
    println!("Preprocessing complete.");

    // --- Step 3: Serialize the modified spec and write to the output file ---
    println!("Writing processed spec to {}...", output_path);
    let output_yaml = serde_yaml::to_string(&doc)?;
    fs::write(output_path, output_yaml)
        .map_err(|e| format!("Failed to write {}: {}", output_path, e))?;

    println!("Successfully created {}.", output_path);
    Ok(())
}

/// A generic function to:
/// 1. (Optional) Copy an inline parameter schema to `components/schemas`.
/// 2. Unwrap that parameter object into individual query parameters within the `paths` definition.
fn unwrap_parameters_by_path(
    doc: &mut Mapping,
    path: &str,
    method: &str,
    param_name_to_unwrap: &str,
    new_component_name: Option<&str>,
) -> Result<(), String> {
    // --- Step 1 (Optional): Copy the inline schema to components ---
    if let Some(component_name) = new_component_name {
        println!(
            "- Copying inline schema for '{}' to components.schemas.{}...",
            param_name_to_unwrap, component_name
        );

        // Find the parameter with the inline schema to copy using a read-only borrow
        let params_for_copy = doc
            .get("paths")
            .and_then(|p| p.get(path))
            .and_then(|p| p.get(method))
            .and_then(|op| op.get("parameters"))
            .and_then(|params| params.as_sequence())
            .ok_or_else(|| format!("Could not find parameters for {} {}", method, path))?;

        let param_to_copy = params_for_copy
            .iter()
            .find(|p| p.get("name").and_then(|n| n.as_str()) == Some(param_name_to_unwrap))
            .ok_or_else(|| format!("Parameter '{}' not found for copying", param_name_to_unwrap))?;

        let inline_schema = param_to_copy
            .get("schema")
            .cloned() // Clone the schema to avoid borrowing issues
            .ok_or_else(|| format!("No schema found for '{}'", param_name_to_unwrap))?;

        // Get a mutable borrow to insert the cloned schema into components
        let schemas = doc
            .get_mut("components")
            .and_then(|c| c.get_mut("schemas"))
            .and_then(|s| s.as_mapping_mut())
            .ok_or_else(|| "Could not find components/schemas section".to_string())?;

        schemas.insert(component_name.into(), inline_schema);
    }

    // --- Step 2: Unwrap the parameter object into individual parameters ---
    println!(
        "- Unwrapping parameter object '{}'...",
        param_name_to_unwrap
    );

    // Navigate down to the operation's parameters list (mutable)
    let params_for_unwrap = doc
        .get_mut("paths")
        .and_then(|p| p.get_mut(path))
        .and_then(|p| p.get_mut(method))
        .and_then(|op| op.get_mut("parameters"))
        .and_then(|params| params.as_sequence_mut())
        .ok_or_else(|| format!("Could not find parameters for {} {}", method, path))?;

    let param_index = params_for_unwrap
        .iter()
        .position(|p| p.get("name").and_then(|n| n.as_str()) == Some(param_name_to_unwrap))
        .ok_or_else(|| format!("Parameter '{}' not found in {}", param_name_to_unwrap, path))?;

    let param_object = params_for_unwrap.remove(param_index);
    let properties = param_object
        .get("schema")
        .and_then(|s| s.get("properties"))
        .and_then(|p| p.as_mapping())
        .ok_or_else(|| {
            format!(
                "Could not extract properties from '{}'",
                param_name_to_unwrap
            )
        })?;

    for (key, value) in properties {
        let mut new_param = Mapping::new();
        new_param.insert("name".into(), key.clone());
        new_param.insert("in".into(), "query".into());
        new_param.insert("schema".into(), value.clone());
        params_for_unwrap.push(new_param.into());
    }

    Ok(())
}

/// Special handler for unwrapping search parameters from `components/schemas`.
fn unwrap_search_parameters(doc: &mut Mapping) -> Result<(), String> {
    println!("- Unwrapping searchParameters...");
    // Get the definition of SearchParameters from components
    let search_params_props = doc
        .get("components")
        .and_then(|c| c.get("schemas"))
        .and_then(|s| s.get("SearchParameters"))
        .and_then(|sp| sp.get("properties"))
        .and_then(|p| p.as_mapping())
        .cloned() // Clone to avoid borrowing issues
        .ok_or_else(|| "Could not find schema for SearchParameters".to_string())?;

    // Navigate to the operation's parameters list
    let params = doc
        .get_mut("paths")
        .and_then(|p| p.get_mut("/collections/{collectionName}/documents/search"))
        .and_then(|p| p.get_mut("get"))
        .and_then(|op| op.get_mut("parameters"))
        .and_then(|params| params.as_sequence_mut())
        .ok_or_else(|| {
            "Could not find parameters for /collections/{collectionName}/documents/search"
                .to_string()
        })?;

    // Find and remove the old parameter object.
    let param_index = params
        .iter()
        .position(|p| p.get("name").and_then(|n| n.as_str()) == Some("searchParameters"))
        .ok_or_else(|| "searchParameters object not found".to_string())?;
    params.remove(param_index);

    // Add the new individual parameters.
    for (key, value) in search_params_props {
        let mut new_param = Mapping::new();
        new_param.insert("name".into(), key.clone());
        new_param.insert("in".into(), "query".into());
        new_param.insert("schema".into(), value.clone());
        params.push(new_param.into());
    }

    Ok(())
}

/// Special handler for unwrapping multi-search parameters from `components/schemas`.
fn unwrap_multi_search_parameters(doc: &mut Mapping) -> Result<(), String> {
    println!("- Unwrapping multiSearchParameters...");
    // Get the definition of MultiSearchParameters from components
    let search_params_props: Mapping = doc
        .get("components")
        .and_then(|c| c.get("schemas"))
        .and_then(|s| s.get("MultiSearchParameters"))
        .and_then(|sp| sp.get("properties"))
        .and_then(|p| p.as_mapping())
        .cloned()
        .ok_or_else(|| "Could not find schema for MultiSearchParameters".to_string())?;

    // Navigate to the operation's parameters list
    let params = doc
        .get_mut("paths")
        .and_then(|p| p.get_mut("/multi_search"))
        .and_then(|p| p.get_mut("post"))
        .and_then(|op| op.get_mut("parameters"))
        .and_then(|params| params.as_sequence_mut())
        .ok_or_else(|| "Could not find parameters for /multi_search".to_string())?;

    // Find and remove the old parameter object.
    let param_index = params
        .iter()
        .position(|p| p.get("name").and_then(|n| n.as_str()) == Some("multiSearchParameters"))
        .ok_or_else(|| "multiSearchParameters object not found".to_string())?;
    params.remove(param_index);

    // Add the new individual parameters.
    for (key, value) in search_params_props {
        let mut new_param = Mapping::new();
        new_param.insert("name".into(), key.clone());
        new_param.insert("in".into(), "query".into());
        new_param.insert("schema".into(), value.clone());
        params.push(new_param.into());
    }

    Ok(())
}

/// Modifies the SearchResultHit schema to remove `additionalProperties` from the `document` field.
fn remove_additional_properties_from_search_hit(doc: &mut Mapping) -> Result<(), String> {
    let document_prop = doc
        .get_mut("components")
        .and_then(|c| c.get_mut("schemas"))
        .and_then(|s| s.get_mut("SearchResultHit"))
        .and_then(|srh| srh.get_mut("properties"))
        .and_then(|props| props.get_mut("document"))
        .and_then(|doc_val| doc_val.as_mapping_mut())
        .ok_or_else(|| "Could not find document property in SearchResultHit schema".to_string())?;

    // Remove the 'additionalProperties' key
    if document_prop.remove("additionalProperties").is_some() {
        println!("- Removed additionalProperties from SearchResultHit.document");
    }

    Ok(())
}
