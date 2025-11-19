use std::collections::BTreeMap;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;
use typesense::Typesense;
use typesense::prelude::*;

// Test 1: Basic Schema Generation (keeping the old test to ensure backward compatibility)
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

// Test 2: All Field-Level and Collection-Level Attributes

type GeoPoint = (f32, f32);

#[allow(dead_code)]
#[derive(Typesense, Serialize, Deserialize)]
#[typesense(
    collection_name = "kitchen_sink_products",
    default_sorting_field = "renamed_price",
    token_separators = ["-", "/"],
    symbols_to_index = ["+"]
)]
struct KitchenSinkProduct {
    // Basic types and rename
    #[typesense(rename = "product_name")]
    name: String,
    #[typesense(sort = false, rename = "renamed_price")]
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

    hash_map: HashMap<String, i32>,
    btree_map: BTreeMap<String, i32>,

    hash_map_vec: Vec<HashMap<String, i32>>,
    btree_map_vec: Vec<BTreeMap<String, i32>>,
    // optional = false should override the Option<T> type
    #[typesense(optional = false)]
    optional_field: Option<i32>,

    #[typesense(reference = "company.id")]
    company_id: String,
}

#[test]
fn derived_document_handles_all_attributes() {
    let schema = KitchenSinkProduct::collection_schema();

    let expected = json!({
      "name": "kitchen_sink_products",
      "fields": [
        { "name": "product_name", "type": "string" },
        { "name": "renamed_price", "type": "float", "sort": false },
        { "name": "internal_id", "type": "int64", "index": false, "store": false },
        { "name": "description", "type": "string", "stem": true, "infix": true },
        { "name": "review_score", "type": "float", "range_index": true },
        { "name": "brand", "type": "string", "facet": true, "optional": true },
        { "name": "description_jp", "type": "string", "locale": "ja" },
        { "name": "location", "type": "geopoint" },
        { "name": "image_embedding", "type": "float[]", "num_dim": 256, "vec_dist": "cosine" },
        { "name": "misc_data", "type": "auto" },

        { "name": "hash_map", "type": "object" },
        { "name": "btree_map", "type": "object" },

        { "name": "hash_map_vec", "type": "object[]" },
        { "name": "btree_map_vec", "type": "object[]" },

        { "name": "optional_field", "type": "int32", "optional": false },
        {"name": "company_id", "type": "string", "reference": "company.id"}
      ],
      "default_sorting_field": "renamed_price",
      "token_separators": ["-", "/"],
      "symbols_to_index": ["+"]
    });

    assert_eq!(serde_json::to_value(&schema).unwrap(), expected);
}

// Test 3: Nested Objects and Flattening

#[derive(Typesense, Serialize, Deserialize)]
struct Address {
    line_1: String,
    number: i32,
    optional_field: Option<String>,
    #[typesense(skip)]
    city: String,
}

#[derive(Typesense, Serialize, Deserialize)]
struct Profile {
    #[typesense(facet, sort)]
    name: String,
    email: Option<String>,
}

#[derive(Typesense, Serialize, Deserialize)]
struct AddressData {
    primary_city: String,
    work_zips: Vec<String>,
}

#[derive(Typesense, Serialize, Deserialize)]
struct NestedStruct {
    name: String,
    #[typesense(flatten)]
    address: AddressData,
}

#[allow(dead_code)]
#[derive(Typesense, Serialize, Deserialize)]
#[typesense(collection_name = "nested_users", enable_nested_fields = true)]
struct User {
    // --- Indexing as an object ---
    primary_address: Address,
    work_addresses: Vec<Address>,
    optional_profile: Option<Profile>,

    // --- Sub-fields indexing ---
    #[typesense(flatten)]
    profile: Profile,
    #[typesense(flatten)]
    previous_addresses: Vec<Address>,
    #[typesense(flatten, skip)]
    sub_fields_only: Profile,

    #[typesense(flatten, skip)]
    nested_struct: NestedStruct,
    #[typesense(flatten)]
    nested_struct_vec: Vec<NestedStruct>,

    // --- Manually flattened object ---
    #[typesense(skip)]
    data: AddressData,
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
        // --- Object Indexing ---
        { "name": "primary_address", "type": "object" },
        { "name": "work_addresses", "type": "object[]" },
        { "name": "optional_profile", "type": "object", "optional": true },

        // --- Sub-fields indexing ---
        { "name": "profile", "type": "object" },
        { "name": "profile.name", "type": "string", "facet": true, "sort": true},
        { "name": "profile.email", "type": "string", "optional": true },

        { "name": "previous_addresses", "type": "object[]" },
        { "name": "previous_addresses.line_1", "type": "string[]" },
        { "name": "previous_addresses.number", "type": "int32[]" },
        { "name": "previous_addresses.optional_field", "type": "string[]", "optional": true},
        // { "name": "previous_addresses.city", "type": "string[]" }, correctly skipped

        { "name": "sub_fields_only.name", "type": "string", "facet": true, "sort": true},
        { "name": "sub_fields_only.email", "type": "string", "optional": true },

        { "name": "nested_struct.name", "type": "string"},
        { "name": "nested_struct.address", "type": "object" },
        { "name": "nested_struct.address.primary_city", "type": "string" },
        { "name": "nested_struct.address.work_zips", "type": "string[]" },

        { "name": "nested_struct_vec", "type": "object[]"},
        { "name": "nested_struct_vec.name", "type": "string[]"},
        { "name": "nested_struct_vec.address", "type": "object[]" },
        { "name": "nested_struct_vec.address.primary_city", "type": "string[]" },
        { "name": "nested_struct_vec.address.work_zips", "type": "string[]" },

        // --- Manually flattened object ---
        // correctly skipped `data`
        { "name": "primary_address.city", "type": "string" },
        { "name": "work_addresses.zip", "type": "string[]" }
      ]
    });

    assert_eq!(serde_json::to_value(schema).unwrap(), expected);
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

    // This field is for internal Rust logic only and should NOT be in the schema
    #[typesense(skip)]
    internal_metadata: String,
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
        // `internal_metadata` is correctly omitted from the fields array
      ]
    });

    assert_eq!(serde_json::to_value(schema).unwrap(), expected);
}
