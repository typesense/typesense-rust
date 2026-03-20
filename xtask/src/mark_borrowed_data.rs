use indexmap::IndexMap;

use crate::preprocess_openapi::OpenAPIProperty;

pub(crate) fn is_forced_borrow_model(name: &str) -> bool {
    matches!(
        name,
        "SearchParameters"
            | "MultiSearchParameters"
            | "MultiSearchSearchesParameter"
            | "MultiSearchCollectionParameters"
    )
}
// Deeply traverse resolving $refs to ensure all nested request models are tracked.
pub(crate) fn collect_request_schemas(
    property: &OpenAPIProperty,
    schemas: &IndexMap<String, OpenAPIProperty>,
    collected: &mut std::collections::HashSet<String>,
) {
    if let Some(reference) = &property.r#ref {
        let name = reference.trim_start_matches("#/components/schemas/");
        if collected.insert(name.to_owned())
            && let Some(schema) = schemas.get(name)
        {
            collect_request_schemas(schema, schemas, collected);
        }
    }

    if let Some(properties) = &property.properties {
        for nested_property in properties.values() {
            collect_request_schemas(nested_property, schemas, collected);
        }
    }
    if let Some(items) = &property.items {
        collect_request_schemas(items, schemas, collected);
    }
    if let Some(one_of) = &property.one_of {
        for variant in one_of {
            collect_request_schemas(variant, schemas, collected);
        }
    }
    if let Some(any_of) = &property.any_of {
        for variant in any_of {
            collect_request_schemas(variant, schemas, collected);
        }
    }

    if let Some(all_of_value) = property.extra.get("allOf")
        && let Ok(sequence) = serde_yaml::from_value::<Vec<OpenAPIProperty>>(all_of_value.clone())
    {
        for item in sequence {
            collect_request_schemas(&item, schemas, collected);
        }
    }
    if let Some(additional_properties_value) = property.extra.get("additionalProperties")
        && let Ok(additional_property) =
            serde_yaml::from_value::<OpenAPIProperty>(additional_properties_value.clone())
    {
        collect_request_schemas(&additional_property, schemas, collected);
    }
}

// Deeply traverse resolving $refs to ensure all nested response models are tracked.
// IMPORTANT: treat `allOf` as a mixin and do NOT blocklist the mixin schema itself.
pub(crate) fn collect_response_schemas(
    property: &OpenAPIProperty,
    schemas: &IndexMap<String, OpenAPIProperty>,
    collected: &mut std::collections::HashSet<String>,
    is_mixin: bool,
) {
    if let Some(reference) = &property.r#ref {
        let name = reference.trim_start_matches("#/components/schemas/");

        if !is_mixin {
            // It's a direct type reference, blocklist it.
            if collected.insert(name.to_owned())
                && let Some(schema) = schemas.get(name)
            {
                collect_response_schemas(schema, schemas, collected, false);
            }
        } else {
            // It's an allOf mixin. Do NOT blocklist the schema itself.
            if let Some(schema) = schemas.get(name) {
                if let Some(properties) = &schema.properties {
                    for nested_property in properties.values() {
                        collect_response_schemas(nested_property, schemas, collected, false);
                    }
                }
                if let Some(items) = &schema.items {
                    collect_response_schemas(items, schemas, collected, false);
                }
                if let Some(one_of) = &schema.one_of {
                    for variant in one_of {
                        collect_response_schemas(variant, schemas, collected, false);
                    }
                }
                if let Some(any_of) = &schema.any_of {
                    for variant in any_of {
                        collect_response_schemas(variant, schemas, collected, false);
                    }
                }

                if let Some(all_of_value) = schema.extra.get("allOf")
                    && let Ok(sequence) =
                        serde_yaml::from_value::<Vec<OpenAPIProperty>>(all_of_value.clone())
                {
                    for item in sequence {
                        collect_response_schemas(&item, schemas, collected, true);
                    }
                }
                if let Some(additional_properties_value) = schema.extra.get("additionalProperties")
                    && let Ok(additional_property) = serde_yaml::from_value::<OpenAPIProperty>(
                        additional_properties_value.clone(),
                    )
                {
                    collect_response_schemas(&additional_property, schemas, collected, false);
                }
            }
        }
    }

    if let Some(properties) = &property.properties {
        for nested_property in properties.values() {
            collect_response_schemas(nested_property, schemas, collected, false);
        }
    }
    if let Some(items) = &property.items {
        collect_response_schemas(items, schemas, collected, false);
    }
    if let Some(one_of) = &property.one_of {
        for variant in one_of {
            collect_response_schemas(variant, schemas, collected, false);
        }
    }
    if let Some(any_of) = &property.any_of {
        for variant in any_of {
            collect_response_schemas(variant, schemas, collected, false);
        }
    }

    // If we hit `allOf` in a response, flag its targets as mixins (is_mixin = true)
    if let Some(all_of_value) = property.extra.get("allOf")
        && let Ok(sequence) = serde_yaml::from_value::<Vec<OpenAPIProperty>>(all_of_value.clone())
    {
        for item in sequence {
            collect_response_schemas(&item, schemas, collected, true);
        }
    }
    if let Some(additional_properties_value) = property.extra.get("additionalProperties")
        && let Ok(additional_property) =
            serde_yaml::from_value::<OpenAPIProperty>(additional_properties_value.clone())
    {
        collect_response_schemas(&additional_property, schemas, collected, false);
    }
}

// Helper function to recursively check if a schema contains an actual string
pub(crate) fn property_contains_string(
    property: &OpenAPIProperty,
    schemas: &IndexMap<String, OpenAPIProperty>,
    visited: &mut std::collections::HashSet<String>,
    response_schemas: &std::collections::HashSet<String>,
) -> bool {
    let is_enum = property.extra.contains_key("enum");
    let is_uuid = property.extra.get("format").and_then(|v| v.as_str()) == Some("uuid");

    if property.r#type.as_deref() == Some("string")
        && !is_enum
        && !is_uuid
        && property.r#ref.is_none()
    {
        return true;
    }

    if let Some(reference) = &property.r#ref {
        let name = reference.trim_start_matches("#/components/schemas/");

        // Uses the new override helper
        if response_schemas.contains(name) && !is_forced_borrow_model(name) {
            return false;
        }

        if !visited.contains(name) {
            visited.insert(name.to_owned());
            if let Some(schema) = schemas.get(name)
                && property_contains_string(schema, schemas, visited, response_schemas)
            {
                return true;
            }
        }
    }

    if let Some(properties) = &property.properties {
        for nested_property in properties.values() {
            if property_contains_string(nested_property, schemas, visited, response_schemas) {
                return true;
            }
        }
    }
    if let Some(items) = &property.items
        && property_contains_string(items, schemas, visited, response_schemas)
    {
        return true;
    }
    if let Some(one_of) = &property.one_of {
        for variant in one_of {
            if property_contains_string(variant, schemas, visited, response_schemas) {
                return true;
            }
        }
    }
    if let Some(any_of) = &property.any_of {
        for variant in any_of {
            if property_contains_string(variant, schemas, visited, response_schemas) {
                return true;
            }
        }
    }

    if let Some(all_of_value) = property.extra.get("allOf")
        && let Ok(sequence) = serde_yaml::from_value::<Vec<OpenAPIProperty>>(all_of_value.clone())
    {
        for item in sequence {
            if property_contains_string(&item, schemas, visited, response_schemas) {
                return true;
            }
        }
    }
    if let Some(additional_properties_value) = property.extra.get("additionalProperties")
        && let Ok(additional_property) =
            serde_yaml::from_value::<OpenAPIProperty>(additional_properties_value.clone())
        && property_contains_string(&additional_property, schemas, visited, response_schemas)
    {
        return true;
    }

    false
}
