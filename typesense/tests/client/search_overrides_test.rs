use typesense_codegen::models::{
    CollectionSchema, Field, SearchOverrideInclude, SearchOverrideRule, SearchOverrideSchema,
};

use super::{get_client, new_id};

#[tokio::test]
async fn test_search_overrides_lifecycle() {
    let client = get_client();
    let collection_name = new_id("products");
    let override_id = new_id("promo_products");

    // --- 1. Setup: Create a collection and add some documents ---
    let schema = CollectionSchema {
        name: collection_name.clone(),
        fields: vec![
            Field {
                name: "name".to_string(),
                r#type: "string".to_string(),
                ..Default::default()
            },
            Field {
                name: "category".to_string(),
                r#type: "string".to_string(),
                facet: Some(true),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    client.collections().create(schema).await.unwrap();

    // --- 2. Create (Upsert) a Search Override (via `search_overrides`) ---
    let override_schema = SearchOverrideSchema {
        rule: Box::new(SearchOverrideRule {
            query: Some("products".to_string()),
            r#match: Some(typesense::models::search_override_rule::Match::Exact),
            ..Default::default()
        }),
        includes: Some(vec![SearchOverrideInclude {
            id: "3".to_string(),
            position: 1,
        }]),
        ..Default::default()
    };

    let upsert_result = client
        .collection(&collection_name)
        .search_overrides()
        .upsert(&override_id, override_schema)
        .await;

    assert!(upsert_result.is_ok(), "Failed to create search override");
    let created_override = upsert_result.unwrap();
    assert_eq!(created_override.id, override_id);
    assert_eq!(created_override.rule.query.unwrap(), "products");

    // --- 3. Retrieve the specific override (via `search_override`) ---
    let retrieve_one_result = client
        .collection(&collection_name)
        .search_override(&override_id)
        .retrieve()
        .await;

    assert!(
        retrieve_one_result.is_ok(),
        "Failed to retrieve the specific search override."
    );
    let retrieved_override = retrieve_one_result.unwrap();
    assert_eq!(retrieved_override.id, override_id);
    assert_eq!(retrieved_override.includes.unwrap()[0].id, "3");

    // --- 4. List all overrides (via `search_overrides`) ---
    let list_result = client
        .collection(&collection_name)
        .search_overrides()
        .list()
        .await;

    assert!(list_result.is_ok(), "Failed to list search overrides.");
    let list_response = list_result.unwrap();
    assert_eq!(list_response.overrides.len(), 1);
    assert!(
        list_response
            .overrides
            .iter()
            .find(|o| o.id == override_id)
            .is_some(),
        "The newly created override was not found in the list."
    );

    // --- 5. Delete the override (via `search_override`) ---
    let delete_result = client
        .collection(&collection_name)
        .search_override(&override_id)
        .delete()
        .await;

    assert!(delete_result.is_ok(), "Failed to delete search override.");
    let delete_response = delete_result.unwrap();
    assert_eq!(delete_response.id, override_id);

    // --- 6. Verify Deletion ---
    let get_after_delete_result = client
        .collection(&collection_name)
        .search_override(&override_id)
        .retrieve()
        .await;
    assert!(
        get_after_delete_result.is_err(),
        "Search override should not exist after deletion."
    );

    // --- 7. Teardown: Delete the collection ---
    let delete_collection_result = client.collection(&collection_name).delete().await;
    assert!(
        delete_collection_result.is_ok(),
        "Failed to delete collection after test."
    );
}
