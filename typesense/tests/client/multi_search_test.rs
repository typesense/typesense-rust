use typesense_codegen::models::{
    CollectionSchema, Field, ImportDocumentsParameters, MultiSearchCollectionParameters,
    MultiSearchParameters, MultiSearchSearchesParameter,
};

use super::{get_client, new_id};

async fn setup_multi_search_tests(
    client: &typesense::client::Client,
    products_collection_name: &str,
    brands_collection_name: &str,
) {
    // --- Create collections ---
    let products_schema = CollectionSchema {
        name: products_collection_name.to_string(),
        fields: vec![
            Field::new("name".to_string(), "string".to_string()),
            Field::new("price".to_string(), "int32".to_string()),
        ],
        ..Default::default()
    };
    client.collections().create(products_schema).await.unwrap();

    let brands_schema = CollectionSchema {
        name: brands_collection_name.to_string(),
        fields: vec![
            Field::new("company_name".to_string(), "string".to_string()),
            Field::new("country".to_string(), "string".to_string()),
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
        .import(
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
        .import(
            brand_docs,
            ImportDocumentsParameters {
                action: Some(typesense::models::IndexAction::Create),
                ..Default::default()
            },
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn test_multi_search_federated() {
    let client = get_client();
    let products_collection_name = new_id("products");
    let brands_collection_name = new_id("brands");
    setup_multi_search_tests(&client, &products_collection_name, &brands_collection_name).await;

    let search_requests = MultiSearchSearchesParameter {
        union: Some(false),
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

    // --- Cleanup ---
    client
        .collection(&products_collection_name)
        .delete()
        .await
        .unwrap();
    client
        .collection(&brands_collection_name)
        .delete()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_multi_search_with_common_params() {
    let client = get_client();
    let products_collection_name = new_id("products_common");
    let brands_collection_name = new_id("brands_common");
    setup_multi_search_tests(&client, &products_collection_name, &brands_collection_name).await;

    // Define individual searches, each with the correct `query_by` for its schema.
    let search_requests = MultiSearchSearchesParameter {
        union: Some(false),
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

    // --- Cleanup ---
    client
        .collection(&products_collection_name)
        .delete()
        .await
        .unwrap();
    client
        .collection(&brands_collection_name)
        .delete()
        .await
        .unwrap();
}
