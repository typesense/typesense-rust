use serde::{Deserialize, Serialize};
use typesense::document::Document;
use typesense::Typesense;

#[test]
fn derived_document_generates_schema() {
    let schema = Company::collection_schema();

    let expected = serde_json::json!(
        {
            "name": "companies",
            "fields": [
              {
                "name"  :  "company_name",
                "type"  :  "string"
              },
              {
                "name"  :  "num_employees",
                "type"  :  "int32"
              },
              {
                "name"  :  "country",
                "type"  :  "string",
                "facet" :  true
              },
              {
                "name"  :  "keywords",
                "type"  :  "string[]",
                "optional" :  true
              }
            ],
            "default_sorting_field": "num_employees",
            "enable_nested_fields": true
          }
    );

    assert_eq!(serde_json::to_value(&schema).unwrap(), expected)
}

#[allow(dead_code)]
#[derive(Typesense, Serialize, Deserialize)]
#[typesense(
    collection_name = "companies",
    default_sorting_field = "num_employees",
    enable_nested_fields = true
)]
struct Company {
    company_name: String,
    num_employees: i32,
    #[typesense(facet)]
    country: String,
    keywords: Option<Vec<String>>,
}
