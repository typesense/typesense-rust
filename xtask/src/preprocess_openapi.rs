use ::std::{collections::HashSet, fs};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::add_vendor_attributes::add_vendor_attributes;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OpenAPI {
    #[serde(skip_serializing_if = "Option::is_none")]
    openapi: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    info: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    servers: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_docs: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Value>,
    pub(crate) paths: IndexMap<String, OpenAPIPath>,
    pub(crate) components: OpenAPIComponents,

    // Everything else not explicitly listed above
    #[serde(flatten)]
    pub(crate) extra: IndexMap<String, Value>,
}

pub(crate) type OpenAPIPath = IndexMap<String, OpenAPIMethod>;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OpenAPIMethod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tags: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) summary: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) operation_id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parameters: Option<Vec<OpenAPIParameter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) request_body: Option<OpenAPIBody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) responses: Option<IndexMap<String, OpenAPIBody>>,

    // Everything else not explicitly listed above
    #[serde(flatten)]
    pub(crate) extra: IndexMap<String, Value>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OpenAPIBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) content: Option<IndexMap<String, OpenAPIBodyContent>>,

    // Everything else not explicitly listed above
    #[serde(flatten)]
    pub(crate) extra: IndexMap<String, Value>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OpenAPIBodyContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) schema: Option<OpenAPIProperty>,

    // Everything else not explicitly listed above
    #[serde(flatten)]
    pub(crate) extra: IndexMap<String, Value>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OpenAPIParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) required: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) r#in: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) schema: Option<OpenAPIProperty>,

    // Everything else not explicitly listed above
    #[serde(flatten)]
    pub(crate) extra: IndexMap<String, Value>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OpenAPIComponents {
    pub(crate) schemas: IndexMap<String, OpenAPIProperty>,

    // Everything else not explicitly listed above
    #[serde(flatten)]
    pub(crate) extra: IndexMap<String, Value>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OpenAPIProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) default: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) required: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) example: Option<Value>,
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub(crate) r#ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) properties: Option<IndexMap<String, OpenAPIProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) items: Option<Box<OpenAPIProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) one_of: Option<Vec<OpenAPIProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) any_of: Option<Vec<OpenAPIProperty>>,

    // Everything else not explicitly listed above
    #[serde(flatten)]
    pub(crate) extra: IndexMap<String, Value>,
}

// --- Main function to orchestrate the file reading, processing, and writing ---
pub fn preprocess_openapi_file(
    input_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Preprocessing the Open API spec file...");
    // --- Step 1: Read the OpenAPI spec from the input file ---
    println!("Reading OpenAPI spec from {}...", input_path);
    let input_content = fs::read_to_string(input_path)
        .map_err(|e| format!("Failed to read {}: {}", input_path, e))?;
    let mut doc: OpenAPI = serde_yaml::from_str(&input_content)?;

    // --- Step 2: Apply all the required transformations ---
    println!("Preprocessing the spec...");

    println!("Adding custom x-* vendor attributes...");
    add_vendor_attributes(&mut doc)?;

    println!("Unwrapping parameters...");
    doc.unwrap_search_parameters()?;
    doc.unwrap_multi_search_parameters()?;
    doc.unwrap_parameters_by_path(
        "/collections/{collectionName}/documents/import",
        "post",
        "importDocumentsParameters",
        Some("ImportDocumentsParameters"), // Copy schema to components
    )?;
    doc.unwrap_parameters_by_path(
        "/collections/{collectionName}/documents/export",
        "get",
        "exportDocumentsParameters",
        Some("ExportDocumentsParameters"),
    )?;
    doc.unwrap_parameters_by_path(
        "/collections/{collectionName}/documents",
        "patch",
        "updateDocumentsParameters",
        Some("UpdateDocumentsParameters"),
    )?;
    doc.unwrap_parameters_by_path(
        "/collections/{collectionName}/documents",
        "delete",
        "deleteDocumentsParameters",
        Some("DeleteDocumentsParameters"),
    )?;
    doc.unwrap_parameters_by_path(
        "/collections",
        "get",
        "getCollectionsParameters",
        Some("GetCollectionsParameters"),
    )?;
    doc.mark_borrowed_data();
    println!("Preprocessing complete.");

    // --- Step 3: Serialize the modified spec and write to the output file ---
    println!("Writing processed spec to {}...", output_path);
    let output_yaml = serde_yaml::to_string(&doc)?;
    fs::write(output_path, output_yaml)
        .map_err(|e| format!("Failed to write {}: {}", output_path, e))?;

    println!("Successfully created {}.", output_path);
    Ok(())
}

impl OpenAPIProperty {
    fn collect_properties(&self) -> Vec<String> {
        let mut data = Vec::new();
        if let Some(schema) = &self.r#ref {
            data.push(
                schema
                    .trim_start_matches("#/components/schemas/")
                    .to_owned(),
            );
        }
        if let Some(p) = &self.items {
            data.extend(p.collect_properties());
        }
        if let Some(v) = &self.any_of {
            v.iter().for_each(|p| data.extend(p.collect_properties()));
        }
        if let Some(v) = &self.one_of {
            v.iter().for_each(|p| data.extend(p.collect_properties()));
        }
        data
    }

    fn mark_borrowed_property(&mut self) {
        if self.r#type.as_deref() == Some("object") || self.one_of.is_some() {
            self.extra
                .insert("x-rust-has-borrowed-data".to_owned(), Value::Bool(true));
        }
    }
}

impl OpenAPI {
    fn mark_borrowed_data(&mut self) {
        println!("Marking borrowed data...");

        let mut request_schemas = HashSet::new();
        self.paths.iter_mut().for_each(|(_, pms)| {
            pms.iter_mut().for_each(|(_, pm)| {
                if let Some(ps) = &mut pm.parameters {
                    ps.iter_mut().for_each(|p| {
                        if let Some(s) = &mut p.schema {
                            s.mark_borrowed_property();
                            request_schemas.extend(s.collect_properties());
                        }
                    })
                }

                if let Some(reqb) = &mut pm.request_body
                    && let Some(cs) = &mut reqb.content
                {
                    cs.iter_mut().for_each(|(_, c)| {
                        if let Some(s) = &mut c.schema {
                            s.mark_borrowed_property();
                            request_schemas.extend(s.collect_properties());
                        }
                    })
                }
            })
        });

        let schemas = self
            .components
            .schemas
            .iter()
            .filter(|(n, _)| n.ends_with("Parameters") || request_schemas.contains(n.as_str()))
            .map(|(n, _)| n.to_owned())
            .collect::<Vec<String>>();
        drop(request_schemas);

        for schema_name in schemas {
            let Some(schema) = self.components.schemas.get_mut(&schema_name) else {
                continue;
            };

            schema
                .extra
                .insert("x-rust-has-borrowed-data".to_owned(), Value::Bool(true));

            for (_, prop) in schema.properties.iter_mut().flat_map(|v| v.iter_mut()) {
                for inner in prop.one_of.iter_mut().flat_map(|v| v.iter_mut()) {
                    inner.mark_borrowed_property();
                }
                for inner in prop.any_of.iter_mut().flat_map(|v| v.iter_mut()) {
                    inner.mark_borrowed_property();
                }
                if let Some(inner) = &mut prop.items {
                    inner.mark_borrowed_property();
                }

                prop.mark_borrowed_property();
            }
        }
    }

    /// A generic function to:
    /// 1. (Optional) Copy an inline parameter schema to `components/schemas`.
    /// 2. Unwrap that parameter object into individual query parameters within the `paths` definition.
    fn unwrap_parameters_by_path(
        &mut self,
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
            let params_for_copy = self
                .paths
                .get(path)
                .and_then(|p| p.get(method))
                .and_then(|op| op.parameters.as_ref())
                .ok_or_else(|| format!("Could not find parameters for {} {}", method, path))?;

            let param_to_copy = params_for_copy
                .iter()
                .find(|p| p.name.as_deref() == Some(param_name_to_unwrap))
                .ok_or_else(|| {
                    format!("Parameter '{}' not found for copying", param_name_to_unwrap)
                })?;

            let inline_schema = param_to_copy
                .schema
                .clone()
                .ok_or_else(|| format!("No schema found for '{}'", param_name_to_unwrap))?;

            // Get a mutable borrow to insert the cloned schema into components
            self.components
                .schemas
                .insert(component_name.into(), inline_schema);
        }

        // --- Step 2: Unwrap the parameter object into individual parameters ---
        println!(
            "- Unwrapping parameter object '{}'...",
            param_name_to_unwrap
        );

        // Navigate down to the operation's parameters list (mutable)
        let params_for_unwrap = self
            .paths
            .get_mut(path)
            .and_then(|p| p.get_mut(method))
            .and_then(|op| op.parameters.as_mut())
            .ok_or_else(|| format!("Could not find parameters for {} {}", method, path))?;

        let param_index = params_for_unwrap
            .iter()
            .position(|p| p.name.as_deref() == Some(param_name_to_unwrap))
            .ok_or_else(|| format!("Parameter '{}' not found in {}", param_name_to_unwrap, path))?;

        let param_object = params_for_unwrap.remove(param_index);
        let properties = param_object
            .schema
            .and_then(|s| s.properties)
            .ok_or_else(|| {
                format!(
                    "Could not extract properties from '{}'",
                    param_name_to_unwrap
                )
            })?;

        for (key, value) in properties {
            let new_param = OpenAPIParameter {
                name: Some(key),
                r#in: Some("query".to_owned()),
                schema: Some(value),
                ..Default::default()
            };
            params_for_unwrap.push(new_param);
        }

        Ok(())
    }

    /// Special handler for unwrapping search parameters from `components/schemas`.
    fn unwrap_search_parameters(&mut self) -> Result<(), String> {
        println!("- Unwrapping searchParameters...");
        // Get the definition of SearchParameters from components
        let search_params_props = self
            .components
            .schemas
            .get("SearchParameters")
            .and_then(|sp| sp.properties.as_ref())
            .cloned() // Clone to avoid borrowing issues
            .ok_or_else(|| "Could not find schema for SearchParameters".to_string())?;

        // Navigate to the operation's parameters list
        let params = self
            .paths
            .get_mut("/collections/{collectionName}/documents/search")
            .and_then(|p| p.get_mut("get"))
            .and_then(|op| op.parameters.as_mut())
            .ok_or_else(|| {
                "Could not find parameters for /collections/{collectionName}/documents/search"
                    .to_string()
            })?;

        // Find and remove the old parameter object.
        let param_index = params
            .iter()
            .position(|p| p.name.as_deref() == Some("searchParameters"))
            .ok_or_else(|| "searchParameters object not found".to_string())?;
        params.remove(param_index);

        // Add the new individual parameters.
        for (key, value) in search_params_props {
            let new_param = OpenAPIParameter {
                name: Some(key),
                r#in: Some("query".to_owned()),
                schema: Some(value),
                ..Default::default()
            };
            params.push(new_param);
        }

        Ok(())
    }

    /// Special handler for unwrapping multi-search parameters from `components/schemas`.
    fn unwrap_multi_search_parameters(&mut self) -> Result<(), String> {
        println!("- Unwrapping multiSearchParameters...");
        // Get the definition of MultiSearchParameters from components
        let search_params_props = self
            .components
            .schemas
            .get("MultiSearchParameters")
            .and_then(|sp| sp.properties.as_ref())
            .cloned()
            .ok_or_else(|| "Could not find schema for MultiSearchParameters".to_string())?;

        // Navigate to the operation's parameters list
        let params = self
            .paths
            .get_mut("/multi_search")
            .and_then(|p| p.get_mut("post"))
            .and_then(|op| op.parameters.as_mut())
            .ok_or_else(|| "Could not find parameters for /multi_search".to_string())?;

        // Find and remove the old parameter object.
        let param_index = params
            .iter()
            .position(|p| p.name.as_deref() == Some("multiSearchParameters"))
            .ok_or_else(|| "multiSearchParameters object not found".to_string())?;
        params.remove(param_index);

        // Add the new individual parameters.
        for (key, value) in search_params_props {
            let new_param = OpenAPIParameter {
                name: Some(key),
                r#in: Some("query".to_owned()),
                schema: Some(value),
                ..Default::default()
            };
            params.push(new_param);
        }

        Ok(())
    }
}
