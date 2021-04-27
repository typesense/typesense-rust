use typesense::Document;
use typesense::document::Document as DocumentTrait;
use serde::{Serialize, Deserialize};

#[test]
fn schema_is_created_properly() {
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
            "default_sorting_field": "num_employees"
          }
    );

    assert_eq!(serde_json::to_value(&schema).unwrap(), expected)
}

#[allow(dead_code)]
#[derive(Document, Serialize, Deserialize)]
#[typesense(default_sorting_field = "num_employees")]
#[typesense(collection_name = "companies")]
struct Company {
    company_name: String,
    num_employees: i32,
    #[typesense(facet)]
    country: String,
    keywords: Option<Vec<String>>,
}
