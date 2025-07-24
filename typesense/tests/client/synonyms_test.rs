use typesense_codegen::models::{CollectionSchema, Field, SearchSynonymSchema};

use super::{get_client, new_id};

#[tokio::test]
async fn test_synonyms_lifecycle() {
    let client = get_client();
    let collection_name = new_id("products");
    let synonym_id = new_id("synonym-123");

    // --- 1. Create a collection to house the synonyms ---
    let collection_schema = CollectionSchema {
        name: collection_name.clone(),
        fields: vec![Field {
            name: "name".to_string(),
            r#type: "string".to_string(),
            ..Default::default()
        }],
        ..Default::default()
    };
    let create_collection_result = client.collections().create(collection_schema).await;
    assert!(
        create_collection_result.is_ok(),
        "Failed to create collection for synonym test"
    );

    // --- 2. Create (Upsert) a Synonym (via `synonyms`) ---
    let synonym_schema = SearchSynonymSchema {
        synonyms: vec![
            "blazer".to_string(),
            "jacket".to_string(),
            "coat".to_string(),
        ],
        ..Default::default()
    };

    let upsert_result = client
        .collection(&collection_name)
        .synonyms()
        .upsert(&synonym_id, synonym_schema)
        .await;

    assert!(upsert_result.is_ok(), "Failed to create synonym");
    let created_synonym = upsert_result.unwrap();
    assert_eq!(created_synonym.id, synonym_id);

    // --- 3. Retrieve the specific synonym (via `synonym`) ---
    let retrieve_one_result = client
        .collection(&collection_name)
        .synonym(&synonym_id)
        .get()
        .await;

    assert!(
        retrieve_one_result.is_ok(),
        "Failed to retrieve the specific synonym."
    );
    let retrieved_synonym = retrieve_one_result.unwrap();
    assert_eq!(retrieved_synonym.id, synonym_id);
    assert_eq!(retrieved_synonym.synonyms.len(), 3);

    // --- 4. Retrieve all synonyms for the collection (via `synonyms`) ---
    let retrieve_all_result = client
        .collection(&collection_name)
        .synonyms()
        .retrieve()
        .await;
    assert!(
        retrieve_all_result.is_ok(),
        "Failed to retrieve the list of synonyms."
    );
    let all_synonyms_response = retrieve_all_result.unwrap();

    // --- 5. Find our specific synonym within the list ---
    let our_synonym = all_synonyms_response
        .synonyms
        .iter()
        .find(|s| s.id == synonym_id);

    assert!(
        our_synonym.is_some(),
        "The newly created synonym was not found in the list."
    );

    // --- 6. Delete the synonym (via `synonym`) ---
    let delete_result = client
        .collection(&collection_name)
        .synonym(&synonym_id)
        .delete()
        .await;
    assert!(delete_result.is_ok(), "Failed to delete synonym");
    let delete_response = delete_result.unwrap();
    assert_eq!(delete_response.id, synonym_id);

    // --- 7. Verify Deletion ---
    let get_after_delete_result = client
        .collection(&collection_name)
        .synonym(&synonym_id)
        .get()
        .await;
    assert!(
        get_after_delete_result.is_err(),
        "Synonym should not exist after deletion"
    );

    // --- 8. Clean up the collection ---
    let delete_collection_result = client.collection(&collection_name).delete().await;
    assert!(
        delete_collection_result.is_ok(),
        "Failed to delete collection after synonym test"
    );
}
