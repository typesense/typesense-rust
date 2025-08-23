use serde::{Deserialize, Serialize};
use typesense::Field;
use typesense::Typesense;
use typesense::models::SearchParameters;
use typesense::prelude::*;

use crate::{get_client, new_id};

/// A nested struct that will be flattened into the parent.
#[derive(Typesense, Serialize, Deserialize, Debug, PartialEq, Clone)]
struct ProductDetails {
    #[typesense(facet)]
    part_number: String,
    #[typesense(sort = false)]
    weight_kg: f32,
    #[typesense(skip)]
    desc: String,
}
/// A nested struct that will be flattened and renamed.

#[derive(Typesense, Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Logistics {
    warehouse_code: String,
    shipping_class: String,
}

/// A nested struct that will be indexed as a single "object".
#[derive(Typesense, Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Manufacturer {
    name: String,
    city: String,
}

/// The main struct that uses every feature of the derive macro.
#[derive(Typesense, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[typesense(
    collection_name = "mega_products",
    default_sorting_field = "price",
    enable_nested_fields = true,
    token_separators = ["-", "/"],
    symbols_to_index = ["+"]
)]
struct MegaProduct {
    id: String,

    #[typesense(infix, stem)]
    title: String,

    #[typesense(rename = "product_name")]
    #[serde(rename = "product_name")]
    official_name: String,

    #[typesense(facet)]
    brand: String,

    #[typesense(sort)]
    price: f32,

    #[typesense(range_index)]
    review_score: f32,

    #[typesense(index = false, store = false)]
    internal_sku: Option<String>,

    #[typesense(type = "geopoint")]
    location: (f32, f32),

    #[typesense(num_dim = 4, vec_dist = "cosine")]
    embedding: Vec<f32>,

    #[typesense(flatten)]
    details: ProductDetails,

    #[typesense(flatten, rename = "logistics_data")]
    #[serde(rename = "logistics_data")]
    logistics: Logistics,

    manufacturer: Manufacturer,

    tags: Option<Vec<String>>,
}

async fn logic_test_derive_macro_with_generic_client_lifecycle() {
    let client = get_client();
    let collection_name = new_id("mega_products_test");

    // Create Collection using the schema from the derive macro
    let schema = MegaProduct::collection_schema();
    let mut schema_for_creation = schema.clone();
    schema_for_creation.name = collection_name.clone(); // Use the unique name

    let create_res = client.collections().create(schema_for_creation).await;
    assert!(
        create_res.is_ok(),
        "Failed to create collection: {:?}",
        create_res.err()
    );

    // Verify the schema on the server with targeted assertions
    let retrieved_schema = client
        .collection(&collection_name)
        .retrieve()
        .await
        .unwrap();

    // Create a map of the actual fields for easy lookup.
    let actual_fields_map: std::collections::HashMap<String, Field> = retrieved_schema
        .fields
        .into_iter()
        .map(|f| (f.name.clone(), f))
        .collect();

    // Iterate through our *expected* fields and assert only the attributes we set.
    for expected_field in schema.fields {
        let field_name = &expected_field.name;
        // The 'id' field is a special primary key and not listed in the schema's "fields" array.
        if field_name == "id" {
            continue;
        }
        let actual_field = actual_fields_map.get(field_name).unwrap_or_else(|| {
            panic!(
                "Field '{}' expected but not found in retrieved schema",
                field_name
            )
        });

        // Perform targeted checks based on the attributes set in MegaProduct struct
        match field_name.as_str() {
            "title" => {
                assert_eq!(
                    actual_field.infix,
                    Some(true),
                    "Field 'title' should have infix: true"
                );
                assert_eq!(
                    actual_field.stem,
                    Some(true),
                    "Field 'title' should have stem: true"
                );
            }
            "product_name" => {
                // This is the renamed `official_name`
                assert_eq!(
                    actual_field.name, "product_name",
                    "Field 'official_name' should be renamed to 'product_name'"
                );
            }
            "brand" => {
                assert_eq!(
                    actual_field.facet,
                    Some(true),
                    "Field 'brand' should have facet: true"
                );
            }
            "price" => {
                assert_eq!(
                    actual_field.sort,
                    Some(true),
                    "Field 'price' should have sort: true"
                );
            }
            "review_score" => {
                assert_eq!(
                    actual_field.range_index,
                    Some(true),
                    "Field 'review_score' should have range_index: true"
                );
            }
            "internal_sku" => {
                assert_eq!(
                    actual_field.index,
                    Some(false),
                    "Field 'internal_sku' should have index: false"
                );
                assert_eq!(
                    actual_field.store,
                    Some(false),
                    "Field 'internal_sku' should have store: false"
                );
            }
            "location" => {
                assert_eq!(
                    actual_field.r#type, "geopoint",
                    "Field 'location' should have type: 'geopoint'"
                );
            }
            "embedding" => {
                assert_eq!(
                    actual_field.num_dim,
                    Some(4),
                    "Field 'embedding' should have num_dim: 4"
                );
                assert_eq!(
                    actual_field.vec_dist.as_deref(),
                    Some("cosine"),
                    "Field 'embedding' should have vec_dist: 'cosine'"
                );
            }
            "manufacturer" => {
                assert_eq!(
                    actual_field.r#type, "object",
                    "Field 'manufacturer' should have type: 'object'"
                );
            }
            "tags" => {
                assert_eq!(
                    actual_field.optional,
                    Some(true),
                    "Field 'tags' should be optional"
                );
                assert_eq!(
                    actual_field.r#type, "string[]",
                    "Field 'tags' should have type 'string[]'"
                );
            }
            "details.part_number" => {
                assert_eq!(
                    actual_field.facet,
                    Some(true),
                    "Flattened field 'details.part_number' should have facet: true"
                );
            }
            "details.weight_kg" => {
                assert_eq!(
                    actual_field.sort,
                    Some(false),
                    "Flattened field 'details.weight_kg' should have sort: false"
                );
            }
            "details.desc" => {
                assert!(
                    false,
                    "Flattened field 'details.desc' should have been skipped"
                );
            }

            "logistics_data.warehouse_code" => {
                assert_eq!(actual_field.name, "logistics_data.warehouse_code");
            }
            "logistics_data.shipping_class" => {
                assert_eq!(actual_field.name, "logistics_data.shipping_class");
            }
            _ => {
                // If we add a new field to MegaProduct, this panic will remind us to add a check for it.
                if !expected_field.is_default_for_comparison() {
                    panic!(
                        "Unhandled field '{}' in test assertion. Please add a check.",
                        field_name
                    );
                }
            }
        }
    }
    // Add a helper trait to check if a field is just a default name/type pair
    trait IsDefault {
        fn is_default_for_comparison(&self) -> bool;
    }
    impl IsDefault for Field {
        fn is_default_for_comparison(&self) -> bool {
            self.facet.is_none()
                && self.optional.is_none()
                && self.index.is_none()
                && self.store.is_none()
                && self.sort.is_none()
                && self.infix.is_none()
                && self.locale.is_none()
                && self.num_dim.is_none()
                && self.vec_dist.is_none()
                && self.range_index.is_none()
                && self.stem.is_none()
        }
    }

    // Create Documents using the strongly-typed client
    let typed_collection = client.collection_of::<MegaProduct>(&collection_name);
    let documents_client = typed_collection.documents();

    let mut product1 = MegaProduct {
        id: "product-1".to_string(),
        title: "Durable Steel Wrench".to_string(),
        official_name: "The Wrenchmaster 3000+".to_string(),
        brand: "MegaTools".to_string(),
        price: 29.99,
        review_score: 4.8,
        internal_sku: Some("INTERNAL-123".to_string()),
        location: (34.05, -118.24),
        embedding: vec![0.1, 0.2, 0.3, 0.4],
        details: ProductDetails {
            part_number: "MT-WM-3000".to_string(),
            weight_kg: 1.5,
            desc: "A high-quality wrench for all your needs.".to_string(),
        },
        logistics: Logistics {
            warehouse_code: "WH-US-WEST-05".to_string(),
            shipping_class: "GROUND_FREIGHT".to_string(),
        },
        manufacturer: Manufacturer {
            name: "MegaTools Inc.".to_string(),
            city: "Toolsville".to_string(),
        },
        tags: Some(vec!["steel".to_string(), "heavy-duty".to_string()]),
    };

    let create_res = documents_client.create(&product1, None).await;
    assert!(
        create_res.is_ok(),
        "Failed to create typed document: {:?}",
        create_res.err()
    );
    // we set store: false for internal_sku so it should not be present in the response
    product1.internal_sku = None;
    assert_eq!(create_res.unwrap(), product1);

    //  Retrieve Document and verify deserialization
    let retrieve_res = typed_collection.document("product-1").retrieve().await;
    assert!(retrieve_res.is_ok(), "Failed to retrieve typed document");
    assert_eq!(retrieve_res.unwrap(), product1);

    // Search and Filter (Testing attributes)
    // A. Search a normal field
    let search_res1: Result<
        typesense::SearchResult<MegaProduct>,
        typesense::Error<typesense_codegen::apis::documents_api::SearchCollectionError>,
    > = documents_client
        .search(SearchParameters {
            q: Some("Wrench".to_string()),
            query_by: Some("title".to_string()),
            ..Default::default()
        })
        .await;
    assert_eq!(search_res1.unwrap().found, Some(1));

    // B. Search a renamed field
    let search_res2 = documents_client
        .search(SearchParameters {
            q: Some("Wrenchmaster".to_string()),
            query_by: Some("product_name".to_string()),
            ..Default::default()
        })
        .await;
    assert_eq!(search_res2.unwrap().found, Some(1));

    // C. Filter by a facet
    let search_params3 = SearchParameters {
        q: Some("*".to_string()),
        query_by: Some("title".to_string()),
        filter_by: Some("brand:='MegaTools'".to_string()),
        ..Default::default()
    };
    let search_res3 = documents_client.search(search_params3).await;
    assert_eq!(search_res3.unwrap().found, Some(1));

    // D. Filter by a range_index
    let search_params4 = SearchParameters {
        q: Some("*".to_string()),
        query_by: Some("title".to_string()),
        filter_by: Some("review_score:>4.5".to_string()),
        ..Default::default()
    };
    let search_res4 = documents_client.search(search_params4).await;
    assert_eq!(search_res4.unwrap().found, Some(1));

    // E. Search a flattened field
    let search_params5 = SearchParameters {
        q: Some("MT-WM-3000".to_string()),
        query_by: Some("details.part_number".to_string()),
        ..Default::default()
    };
    let search_res5 = documents_client.search(search_params5).await;
    assert_eq!(search_res5.unwrap().found, Some(1));

    let search_params6 = SearchParameters {
        q: Some("WH-US-WEST-05".to_string()),
        query_by: Some("logistics_data.warehouse_code".to_string()),
        ..Default::default()
    };
    let search_res6 = documents_client.search(search_params6).await;
    assert_eq!(
        search_res6.unwrap().found,
        Some(1),
        "Should find by flattened field with a custom prefix"
    );

    //  Update Document (with a partial struct)
    #[derive(Serialize)]
    struct ProductUpdate {
        price: f32,
        tags: Vec<String>,
    }
    let update_payload = ProductUpdate {
        price: 25.99,
        tags: vec!["steel".to_string(), "sale".to_string()],
    };

    let update_res = typed_collection
        .document("product-1")
        .update(&update_payload, None)
        .await;
    assert!(update_res.is_ok(), "Failed to update document");

    // Retrieve again and check updated fields
    let updated_product = typed_collection
        .document("product-1")
        .retrieve()
        .await
        .unwrap();
    assert_eq!(updated_product.price, 25.99);
    assert_eq!(
        updated_product.tags,
        Some(vec!["steel".to_string(), "sale".to_string()])
    );
    assert_eq!(updated_product.title, product1.title); // Unchanged field

    //  Delete Document
    let delete_res = typed_collection.document("product-1").delete().await;
    assert!(delete_res.is_ok(), "Failed to delete document");
    // Returned document should be the state before deletion
    assert_eq!(delete_res.unwrap().id, "product-1");

    // Verify Deletion
    let retrieve_after_delete = typed_collection.document("product-1").retrieve().await;
    assert!(
        retrieve_after_delete.is_err(),
        "Document should not exist after deletion"
    );
}

// Indexing nested objects via flattening test

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct ManualProductDetails {
    part_number: String,
    weight_kg: f32,
}

#[derive(Typesense, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[typesense(
    collection_name = "manual_flat_products",
    // IMPORTANT: Nested fields are disabled for this strategy.
    enable_nested_fields = false
)]
struct ManualFlattenedProduct {
    id: String,
    title: String,

    // This field is part of the Rust struct and will be in the JSON document,
    // but it will NOT be part of the Typesense schema.
    #[typesense(skip)]
    details: ManualProductDetails,

    // These fields represent the flattened data in the Typesense schema.
    // Both `typesense(rename)` and `serde(rename)` are used to achieve the desired structure.
    #[typesense(rename = "details.part_number")]
    #[serde(rename = "details.part_number")]
    details_part_number: String,

    #[typesense(rename = "details.weight_kg")]
    #[serde(rename = "details.weight_kg")]
    details_weight_kg: f32,
}

async fn logic_test_manual_flattening_lifecycle() {
    let client = get_client();
    let collection_name = new_id("manual_flat_test");

    // 1. Create collection from the schema derived from `ManualFlattenedProduct`
    let mut schema = ManualFlattenedProduct::collection_schema();
    schema.name = collection_name.clone();

    // Verify the generated schema is correct *before* creating it
    let schema_fields: Vec<_> = schema.fields.iter().map(|f| f.name.as_str()).collect();
    assert!(
        !schema_fields.contains(&"details"),
        "Schema should not contain the skipped 'details' field"
    );
    assert!(
        schema_fields.contains(&"details.part_number"),
        "Schema must contain the renamed 'details.part_number' field"
    );

    let create_res = client.collections().create(schema).await;
    assert!(
        create_res.is_ok(),
        "Failed to create collection: {:?}",
        create_res.err()
    );

    let typed_collection = client.collection_of::<ManualFlattenedProduct>(&collection_name);

    // 2. Create the document. Note how we populate all fields of the Rust struct.
    let product = ManualFlattenedProduct {
        id: "manual-1".to_string(),
        title: "Portable Generator".to_string(),
        details: ManualProductDetails {
            part_number: "PG-123".to_string(),
            weight_kg: 25.5,
        },
        details_part_number: "PG-123".to_string(),
        details_weight_kg: 25.5,
    };

    let create_res = typed_collection.documents().create(&product, None).await;
    assert!(
        create_res.is_ok(),
        "Failed to create document with manual flattening"
    );

    // The created document in the response should be equal to our input struct.
    assert_eq!(create_res.unwrap(), product);

    // 3. Retrieve and verify the document.
    let retrieved_product = typed_collection
        .document("manual-1")
        .retrieve()
        .await
        .unwrap();
    assert_eq!(retrieved_product, product);
    // Crucially, we can access the nested struct for display purposes, even though it wasn't indexed.
    assert_eq!(retrieved_product.details.part_number, "PG-123");

    // 4. Search using the flattened (and indexed) field.
    let search_res_indexed = typed_collection
        .documents()
        .search(SearchParameters {
            q: Some("PG-123".to_string()),
            query_by: Some("details.part_number".to_string()),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(
        search_res_indexed.found,
        Some(1),
        "Should find document by indexed flattened field"
    );
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tokio_test {
    use super::*;

    #[tokio::test]
    async fn test_derive_macro_with_generic_client_lifecycle() {
        logic_test_derive_macro_with_generic_client_lifecycle().await;
    }

    #[tokio::test]
    async fn test_manual_flattening_lifecycle() {
        logic_test_manual_flattening_lifecycle().await;
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_derive_macro_with_generic_client_lifecycle() {
        console_error_panic_hook::set_once();
        logic_test_derive_macro_with_generic_client_lifecycle().await;
    }

    #[wasm_bindgen_test]
    async fn test_manual_flattening_lifecycle() {
        console_error_panic_hook::set_once();
        logic_test_manual_flattening_lifecycle().await;
    }
}
