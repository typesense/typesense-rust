use serde::Deserialize;
use typesense::{
    models::{
        CollectionSchema, Field, ImportDocumentsParameters, MultiSearchBody,
        MultiSearchCollectionParameters, MultiSearchParameters, SearchResult,
    },
    prelude::*,
};

use super::{get_client, new_id};

async fn setup_multi_search_tests(
    client: &typesense::Client,
    products_collection_name: &str,
    brands_collection_name: &str,
) {
    // --- Create collections ---
    let products_schema = CollectionSchema {
        name: products_collection_name.to_owned(),
        fields: vec![
            Field::new("name".to_owned(), "string".to_owned()),
            Field::new("price".to_owned(), "int32".to_owned()),
        ],
        ..Default::default()
    };
    client.collections().create(products_schema).await.unwrap();

    let brands_schema = CollectionSchema {
        name: brands_collection_name.to_owned(),
        fields: vec![
            Field::new("company_name".to_owned(), "string".to_owned()),
            Field::new("country".to_owned(), "string".to_owned()),
        ],
        ..Default::default()
    };
    client.collections().create(brands_schema).await.unwrap();

    // --- Index documents ---
    let product_docs = r#"
        {"id": "p1", "name": "iPhone 15", "price": 999}
        {"id": "p2", "name": "MacBook Pro", "price": 1999}
    "#
    .trim()
    .lines()
    .map(|s| s.trim())
    .collect::<Vec<_>>()
    .join("\n");

    client
        .collection(products_collection_name)
        .documents()
        .import_jsonl(
            product_docs,
            ImportDocumentsParameters {
                action: Some(typesense::models::IndexAction::Create),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    let brand_docs = r#"
        {"id": "b1", "company_name": "Apple Inc.", "country": "USA"}
        {"id": "b2", "company_name": "Samsung", "country": "South Korea"}
    "#
    .trim()
    .lines()
    .map(|s| s.trim())
    .collect::<Vec<_>>()
    .join("\n");

    client
        .collection(brands_collection_name)
        .documents()
        .import_jsonl(
            brand_docs,
            ImportDocumentsParameters {
                action: Some(typesense::models::IndexAction::Create),
                ..Default::default()
            },
        )
        .await
        .unwrap();
}

async fn run_test_multi_search_federated() {
    let client = get_client();
    let products_collection_name = new_id("products");
    let brands_collection_name = new_id("brands");
    setup_multi_search_tests(&client, &products_collection_name, &brands_collection_name).await;

    let search_requests = MultiSearchBody {
        searches: vec![
            MultiSearchCollectionParameters {
                q: Some("pro".into()),
                query_by: Some("name".into()),
                collection: Some(products_collection_name.clone()),
                ..Default::default()
            },
            MultiSearchCollectionParameters {
                q: Some("USA".into()),
                query_by: Some("country".into()),
                collection: Some(brands_collection_name.clone()),
                ..Default::default()
            },
        ],
    };

    let common_params = MultiSearchParameters::default();

    let result = client
        .multi_search()
        .perform(search_requests, common_params)
        .await;

    assert!(result.is_ok(), "Multi-search request failed");
    let response = result.unwrap();

    assert_eq!(
        response.results.len(),
        2,
        "Expected 2 sets of search results"
    );

    // --- Assert products result ---
    let products_result = &response.results[0];
    assert!(
        products_result.error.is_none(),
        "First search returned an error"
    );
    assert_eq!(products_result.found, Some(1));
    let product_hit = &products_result.hits.as_ref().unwrap()[0];
    let product_doc = product_hit.document.as_ref().unwrap().as_object().unwrap();
    assert_eq!(
        product_doc.get("name").unwrap().as_str(),
        Some("MacBook Pro")
    );

    // --- Assert brands result ---
    let brands_result = &response.results[1];
    assert!(
        brands_result.error.is_none(),
        "Second search returned an error"
    );
    assert_eq!(brands_result.found, Some(1));
    let brand_hit = &brands_result.hits.as_ref().unwrap()[0];
    let brand_doc = brand_hit.document.as_ref().unwrap().as_object().unwrap();
    assert_eq!(
        brand_doc.get("company_name").unwrap().as_str(),
        Some("Apple Inc.")
    );
}

async fn run_test_multi_search_with_common_params() {
    let client = get_client();
    let products_collection_name = new_id("products_common");
    let brands_collection_name = new_id("brands_common");
    setup_multi_search_tests(&client, &products_collection_name, &brands_collection_name).await;

    // Define individual searches, each with the correct `query_by` for its schema.
    let search_requests = MultiSearchBody {
        searches: vec![
            MultiSearchCollectionParameters {
                collection: Some(products_collection_name.clone()),
                q: Some("pro".into()),         // This should find "Macbook Pro"
                query_by: Some("name".into()), // Specific to the products schema
                ..Default::default()
            },
            MultiSearchCollectionParameters {
                collection: Some(brands_collection_name.clone()),
                q: Some("inc".into()), // This should find "Apple Inc."
                query_by: Some("company_name".into()), // Specific to the brands schema
                ..Default::default()
            },
        ],
    };

    let common_params = MultiSearchParameters {
        limit: Some(1),
        ..Default::default()
    };

    let result = client
        .multi_search()
        .perform(search_requests, common_params)
        .await;

    assert!(
        result.is_ok(),
        "Multi-search request failed: {:?}",
        result.err()
    );
    let response = result.unwrap();

    assert_eq!(response.results.len(), 2);

    // --- Assert products result ---
    let products_result = &response.results[0];
    assert!(
        products_result.error.is_none(),
        "Products search returned an error: {:?}",
        products_result.error
    );
    assert_eq!(products_result.found, Some(1));
    let product_hit = &products_result.hits.as_ref().unwrap()[0];
    assert_eq!(
        product_hit.document.as_ref().unwrap()["name"],
        "MacBook Pro"
    );

    // --- Assert brands result ---
    let brands_result = &response.results[1];
    assert!(
        brands_result.error.is_none(),
        "Brands search returned an error: {:?}",
        brands_result.error
    );
    assert_eq!(brands_result.found, Some(1));
    let brand_hit = &brands_result.hits.as_ref().unwrap()[0];
    assert_eq!(
        brand_hit.document.as_ref().unwrap()["company_name"],
        "Apple Inc."
    );
}

#[derive(Debug, Deserialize, PartialEq)]
struct Product {
    id: String,
    name: String,
    price: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Brand {
    id: String,
    company_name: String,
    country: String,
}

async fn run_test_multi_search_generic_parsing() {
    let client = get_client();
    let products_collection_name = new_id("products_generic");
    let brands_collection_name = new_id("brands_generic");
    setup_multi_search_tests(&client, &products_collection_name, &brands_collection_name).await;

    let search_requests = MultiSearchBody {
        searches: vec![
            // Search #0 for products
            MultiSearchCollectionParameters {
                q: Some("pro".into()),
                query_by: Some("name".into()),
                collection: Some(products_collection_name.clone()),
                ..Default::default()
            },
            // Search #1 for brands
            MultiSearchCollectionParameters {
                q: Some("USA".into()),
                query_by: Some("country".into()),
                collection: Some(brands_collection_name.clone()),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let common_params = MultiSearchParameters::default();

    // Perform the search and get the raw, untyped response
    let raw_response = client
        .multi_search()
        .perform(search_requests, common_params)
        .await
        .unwrap();

    // --- Use the new generic parsing feature ---

    // Parse the first result set (index 0) into SearchResult<Product>
    let products_result: SearchResult<Product> =
        raw_response.parse_at(0).expect("Parsing products failed");

    // Parse the second result set (index 1) into SearchResult<Brand>
    let brands_result: SearchResult<Brand> =
        raw_response.parse_at(1).expect("Parsing brands failed");

    // --- Assert the strongly-typed results ---

    // Assert products result
    assert_eq!(products_result.found, Some(1), "Expected to find 1 product");
    let product_hit = &products_result
        .hits
        .as_ref()
        .unwrap()
        .get(0)
        .expect("No product hits found");
    let product_doc = product_hit
        .document
        .as_ref()
        .expect("Product hit has no document");

    assert_eq!(product_doc.name, "MacBook Pro");
    assert_eq!(product_doc.price, 1999);
    assert_eq!(
        *product_doc,
        Product {
            id: "p2".to_owned(),
            name: "MacBook Pro".to_owned(),
            price: 1999,
        }
    );

    // Assert brands result
    assert_eq!(brands_result.found, Some(1), "Expected to find 1 brand");
    let brand_hit = &brands_result
        .hits
        .as_ref()
        .unwrap()
        .get(0)
        .expect("No brand hits found");
    let brand_doc = brand_hit
        .document
        .as_ref()
        .expect("Brand hit has no document");

    assert_eq!(brand_doc.company_name, "Apple Inc.");
    assert_eq!(
        *brand_doc,
        Brand {
            id: "b1".to_owned(),
            company_name: "Apple Inc.".to_owned(),
            country: "USA".to_owned(),
        }
    );
}

async fn run_test_multi_search_union_heterogeneous() {
    let client = get_client();
    let products_collection_name = new_id("products_union");
    let brands_collection_name = new_id("brands_union");
    setup_multi_search_tests(&client, &products_collection_name, &brands_collection_name).await;

    // We will search for "pro" in products and "samsung" in brands.
    // This should yield one hit from each collection.
    let search_requests = MultiSearchBody {
        searches: vec![
            MultiSearchCollectionParameters {
                q: Some("pro".into()),
                query_by: Some("name".into()),
                collection: Some(products_collection_name.clone()),
                ..Default::default()
            },
            MultiSearchCollectionParameters {
                q: Some("samsung".into()),
                query_by: Some("company_name".into()),
                collection: Some(brands_collection_name.clone()),
                ..Default::default()
            },
        ],
    };

    let common_params = MultiSearchParameters::default();

    // Call the new union function
    let result = client
        .multi_search()
        .perform_union::<serde_json::Value>(search_requests, common_params)
        .await;

    assert!(
        result.is_ok(),
        "Union multi-search request failed: {:?}",
        result.err()
    );
    let response = result.unwrap();

    // In a union search, we expect a single merged result set.
    // We found "MacBook Pro" and "Samsung".
    assert_eq!(response.found, Some(2));
    let hits = response.hits.expect("Expected to find hits");
    assert_eq!(hits.len(), 2);

    // --- Process the heterogeneous hits ---
    // This demonstrates how a user would handle the `serde_json::Value` documents.
    let mut product_count = 0;
    let mut brand_count = 0;

    for hit in hits {
        let document = hit.document.as_ref().unwrap();

        // Check for a field unique to the Product schema to identify the document type.
        if document.get("price").is_some() {
            let product: Product =
                serde_json::from_value(document.clone()).expect("Failed to parse Product");
            assert_eq!(product.name, "MacBook Pro");
            product_count += 1;
        }
        // Check for a field unique to the Brand schema.
        else if document.get("company_name").is_some() {
            let brand: Brand =
                serde_json::from_value(document.clone()).expect("Failed to parse Brand");
            assert_eq!(brand.company_name, "Samsung");
            brand_count += 1;
        }
    }

    // Verify that we correctly identified one of each type from the merged results.
    assert_eq!(
        product_count, 1,
        "Expected to find 1 product in the union result"
    );
    assert_eq!(
        brand_count, 1,
        "Expected to find 1 brand in the union result"
    );
}

async fn run_test_multi_search_union_homogeneous_and_typed_conversion() {
    let client = get_client();
    let products_collection_name = new_id("products_union_homo");
    // We only need one collection for this test, but the setup creates two.
    let brands_collection_name = new_id("brands_union_homo_unused");
    setup_multi_search_tests(&client, &products_collection_name, &brands_collection_name).await;

    // Both search queries target the *same* products collection.
    let search_requests = MultiSearchBody {
        searches: vec![
            // This query should find "iPhone 15"
            MultiSearchCollectionParameters {
                q: Some("iphone".into()),
                query_by: Some("name".into()),
                collection: Some(products_collection_name.clone()),
                ..Default::default()
            },
            // This query should find "MacBook Pro"
            MultiSearchCollectionParameters {
                q: Some("macbook".into()),
                query_by: Some("name".into()),
                collection: Some(products_collection_name.clone()),
                ..Default::default()
            },
        ],
    };

    let typed_result: SearchResult<Product> = client
        .multi_search()
        .perform_union(search_requests, MultiSearchParameters::default())
        .await
        .expect("Union search failed");

    assert_eq!(typed_result.found, Some(2));
    let mut hits = typed_result.hits.expect("Expected hits");

    // Sort by price to have a predictable order for assertions.
    hits.sort_by_key(|h| h.document.as_ref().unwrap().price);

    // Assert the first hit (iPhone)
    let iphone = &hits[0].document.as_ref().unwrap();
    assert_eq!(iphone.name, "iPhone 15");
    assert_eq!(iphone.price, 999);

    // Assert the second hit (MacBook Pro)
    let macbook = &hits[1].document.as_ref().unwrap();
    assert_eq!(macbook.name, "MacBook Pro");
    assert_eq!(macbook.price, 1999);
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tokio_test {
    use super::*;

    #[tokio::test]
    async fn test_multi_search_federated() {
        run_test_multi_search_federated().await;
    }
    #[tokio::test]
    async fn test_multi_search_with_common_params() {
        run_test_multi_search_with_common_params().await;
    }
    #[tokio::test]
    async fn test_multi_search_generic_parsing() {
        run_test_multi_search_generic_parsing().await;
    }
    #[tokio::test]
    async fn test_multi_search_union_heterogeneous() {
        run_test_multi_search_union_heterogeneous().await;
    }
    #[tokio::test]
    async fn test_multi_search_union_homogeneous_and_typed_conversion() {
        run_test_multi_search_union_homogeneous_and_typed_conversion().await;
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_multi_search_federated() {
        console_error_panic_hook::set_once();
        run_test_multi_search_federated().await;
    }
    #[wasm_bindgen_test]
    async fn test_multi_search_with_common_params() {
        console_error_panic_hook::set_once();
        run_test_multi_search_with_common_params().await;
    }
    #[wasm_bindgen_test]
    async fn test_multi_search_generic_parsing() {
        console_error_panic_hook::set_once();
        run_test_multi_search_generic_parsing().await;
    }
    #[wasm_bindgen_test]
    async fn test_multi_search_union_heterogeneous() {
        console_error_panic_hook::set_once();
        run_test_multi_search_union_heterogeneous().await;
    }
    #[wasm_bindgen_test]
    async fn test_multi_search_union_homogeneous_and_typed_conversion() {
        console_error_panic_hook::set_once();
        run_test_multi_search_union_homogeneous_and_typed_conversion().await;
    }
}
