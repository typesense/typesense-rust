use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::json;
use typesense::prelude::*;
use typesense::Typesense;

// Helper to convert schema to BTreeMap for order-independent comparison
fn schema_to_map(
    schema: &typesense::models::CollectionSchema,
) -> BTreeMap<String, serde_json::Value> {
    serde_json::from_value(serde_json::to_value(schema).unwrap()).unwrap()
}

// Test 1: Basic Schema Generation

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

#[test]
fn derived_document_generates_basic_schema() {
    let schema = Company::collection_schema();

    let expected = json!({
        "name": "companies",
        "fields": [
          {
            "name":  "company_name",
            "type": "string"
          },
          {
            "name": "num_employees",
            "type": "int32"
          },
          {
            "name": "country",
            "type": "string",
            "facet": true
          },
          {
            "name": "keywords",
            "type": "string[]",
            "optional": true
          }
        ],
        "default_sorting_field": "num_employees",
        "enable_nested_fields": true
    });

    assert_eq!(serde_json::to_value(&schema).unwrap(), expected);
}

// Test 2: All Field-Level and Collection-Level Attributes

type GeoPoint = (f32, f32);

#[allow(dead_code)]
#[derive(Typesense, Serialize, Deserialize)]
#[typesense(
    collection_name = "kitchen_sink_products",
    default_sorting_field = "price",
    token_separators = ["-", "/"],
    symbols_to_index = ["+"]
)]
struct KitchenSinkProduct {
    // Basic types and rename
    #[typesense(rename = "product_name")]
    name: String,
    #[typesense(sort = false)]
    price: f32,

    // Booleans for index, store, stem, infix, range_index
    #[typesense(index = false, store = false)]
    internal_id: u64,
    #[typesense(stem = true, infix = true)]
    description: String,
    #[typesense(range_index = true)]
    review_score: f32,

    // Facet and explicit optional
    #[typesense(facet = true, optional = true)]
    brand: String,

    // Locale and type override
    #[typesense(locale = "ja")]
    description_jp: String,
    #[typesense(type = "geopoint")]
    location: GeoPoint,

    // Vector search attributes
    #[typesense(num_dim = 256, vec_dist = "cosine")]
    image_embedding: Vec<f32>,

    // Auto type
    #[typesense(type = "auto")]
    misc_data: String,
}

#[test]
fn derived_document_handles_all_attributes() {
    let schema = KitchenSinkProduct::collection_schema();

    let expected = json!({
      "name": "kitchen_sink_products",
      "fields": [
        { "name": "product_name", "type": "string" },
        { "name": "price", "type": "float", "sort": false },
        { "name": "internal_id", "type": "int64", "index": false, "store": false },
        { "name": "description", "type": "string", "stem": true, "infix": true },
        { "name": "review_score", "type": "float", "range_index": true },
        { "name": "brand", "type": "string", "facet": true, "optional": true },
        { "name": "description_jp", "type": "string", "locale": "ja" },
        { "name": "location", "type": "geopoint" },
        { "name": "image_embedding", "type": "float[]", "num_dim": 256, "vec_dist": "cosine" },
        { "name": "misc_data", "type": "auto" }
      ],
      "default_sorting_field": "price",
      "token_separators": ["-", "/"],
      "symbols_to_index": ["+"]
    });

    assert_eq!(serde_json::to_value(&schema).unwrap(), expected);
}

// Test 3: Nested Objects, Flattening, and Cherry-Picking

#[derive(Typesense, Serialize, Deserialize)]
struct Address {
    line_1: String,
    city: String,
}

#[derive(Typesense, Serialize, Deserialize)]
struct Profile {
    #[typesense(facet = true)]
    name: String,
    email: Option<String>,
}

#[allow(dead_code)]
#[derive(Typesense, Serialize, Deserialize)]
#[typesense(collection_name = "nested_users", enable_nested_fields = true)]
struct User {
    // --- Strategy 1: Indexing as an object ---
    primary_address: Address,
    work_addresses: Vec<Address>,
    optional_profile: Option<Profile>,

    // --- Strategy 2: Flattening ---
    #[typesense(flatten)]
    profile: Profile,
    #[typesense(flatten)]
    previous_addresses: Vec<Address>,

    // --- Strategy 3: manually flattened object ---
    #[typesense(rename = "primary_address.city")]
    primary_city: String,
    #[typesense(rename = "work_addresses.zip", type = "string[]")]
    work_zips: Vec<String>,
}

#[test]
fn derived_document_handles_nested_and_flattened_fields() {
    let schema = User::collection_schema();

    let expected = json!({
      "name": "nested_users",
      "enable_nested_fields": true,
      "fields": [
        // --- Strategy 1: Object Indexing ---
        { "name": "primary_address", "type": "object" },
        { "name": "work_addresses", "type": "object[]" },
        { "name": "optional_profile", "type": "object", "optional": true },

        // --- Strategy 2: Flattened fields ---
        { "name": "profile.name", "type": "string", "facet": true },
        { "name": "profile.email", "type": "string", "optional": true },
        { "name": "previous_addresses.line_1", "type": "string[]" },
        { "name": "previous_addresses.city", "type": "string[]" },

        // --- Strategy 3: manually flattened object ---
        { "name": "primary_address.city", "type": "string" },
        { "name": "work_addresses.zip", "type": "string[]" }
      ]
    });

    // Using BTreeMap to allow comparing JSON without worrying about field order
    let expected_map: std::collections::BTreeMap<String, serde_json::Value> =
        serde_json::from_value(expected).unwrap();
    let actual_map: std::collections::BTreeMap<String, serde_json::Value> =
        serde_json::from_value(serde_json::to_value(&schema).unwrap()).unwrap();

    assert_eq!(actual_map, expected_map);
}

// Test 4: All Boolean Shorthand Attributes

#[allow(dead_code)]
#[derive(Typesense, Serialize, Deserialize)]
#[typesense(collection_name = "shorthand_products")]
struct ShorthandProduct {
    // Shorthand for facet = true
    #[typesense(facet)]
    brand: String,

    // Shorthand for sort = true
    #[typesense(sort)]
    name: String,

    // Shorthand for index = true
    #[typesense(index)]
    category: String,

    // Shorthand for store = true
    #[typesense(store)]
    description: String,

    // Shorthand for infix = true
    #[typesense(infix)]
    tags: String,

    // Shorthand for stem = true
    #[typesense(stem)]
    title: String,

    // Shorthand for range_index = true
    #[typesense(range_index)]
    price: f32,

    // Shorthand for optional = true, overriding the non-Option type
    #[typesense(optional)]
    variant: String,
}

#[test]
fn derived_document_handles_boolean_shorthand() {
    let schema = ShorthandProduct::collection_schema();

    let expected = json!({
      "name": "shorthand_products",
      "fields": [
        { "name": "brand", "type": "string", "facet": true },
        { "name": "name", "type": "string", "sort": true },
        { "name": "category", "type": "string", "index": true },
        { "name": "description", "type": "string", "store": true },
        { "name": "tags", "type": "string", "infix": true },
        { "name": "title", "type": "string", "stem": true },
        { "name": "price", "type": "float", "range_index": true },
        { "name": "variant", "type": "string", "optional": true }
      ]
    });

    // Using BTreeMap to allow comparing JSON without worrying about field order
    let expected_map: std::collections::BTreeMap<String, serde_json::Value> =
        serde_json::from_value(expected).unwrap();
    let actual_map: std::collections::BTreeMap<String, serde_json::Value> =
        serde_json::from_value(serde_json::to_value(&schema).unwrap()).unwrap();

    assert_eq!(actual_map, expected_map);
}

#[allow(dead_code)]
#[derive(Typesense, Serialize, Deserialize)]
#[typesense(collection_name = "skipped_field_products")]
struct SkippedFieldProduct {
    // This field will be in the schema
    product_id: String,

    // This field is for internal Rust logic only and should NOT be in the schema
    #[typesense(skip)]
    internal_metadata: String,

    // This field will also be in the schema
    price: i32,
}

#[test]
fn derived_document_handles_skipped_field() {
    let schema = SkippedFieldProduct::collection_schema();
    let expected_map: BTreeMap<String, serde_json::Value> = serde_json::from_value(json!({
      "name": "skipped_field_products",
      "fields": [
        { "name": "product_id", "type": "string" },
        { "name": "price", "type": "int32" }
        // Note: `internal_metadata` is correctly omitted from the fields array
      ]
    }))
    .unwrap();

    assert_eq!(schema_to_map(&schema), expected_map);
}
