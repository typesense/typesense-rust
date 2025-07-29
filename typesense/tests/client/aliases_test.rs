use typesense::models::{CollectionAliasSchema, CollectionSchema, Field};

use super::{get_client, new_id};

#[tokio::test]
async fn test_aliases_and_alias_lifecycle() {
    let client = get_client();
    let collection_name = new_id("products");
    let alias_name = new_id("products_alias");

    // --- 1. Create a collection to alias to ---
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
        "Failed to create collection for alias test"
    );

    // --- 2. Create (Upsert) an alias ---
    let alias_schema = CollectionAliasSchema {
        collection_name: collection_name.clone(),
    };

    let upsert_result = client.aliases().upsert(&alias_name, alias_schema).await;
    assert!(upsert_result.is_ok(), "Failed to create alias");
    let created_alias = upsert_result.unwrap();
    assert_eq!(created_alias.name, alias_name);
    assert_eq!(created_alias.collection_name, collection_name);

    // --- 3. Retrieve the specific alias by name ---
    let retrieve_one_result = client.alias(&alias_name).retrieve().await;
    assert!(
        retrieve_one_result.is_ok(),
        "Failed to retrieve the specific alias."
    );
    let retrieved_alias = retrieve_one_result.unwrap();
    assert_eq!(retrieved_alias.name, alias_name);
    assert_eq!(retrieved_alias.collection_name, collection_name);

    // --- 4. Retrieve all aliases ---
    let retrieve_all_result = client.aliases().retrieve().await;
    assert!(
        retrieve_all_result.is_ok(),
        "Failed to retrieve the list of aliases."
    );
    let all_aliases_response = retrieve_all_result.unwrap();

    // --- 5. Find our specific alias within the list ---
    let our_alias = all_aliases_response
        .aliases
        .iter()
        .find(|a| a.name == alias_name);

    assert!(
        our_alias.is_some(),
        "The newly created alias was not found in the list."
    );

    if let Some(alias) = our_alias {
        assert_eq!(alias.name, alias_name);
        assert_eq!(alias.collection_name, collection_name);
    }

    // --- 6. Delete the alias ---
    let delete_result = client.alias(&alias_name).delete().await;
    assert!(delete_result.is_ok(), "Failed to delete alias");
    let deleted_alias = delete_result.unwrap();
    assert_eq!(deleted_alias.name, alias_name);

    // --- 7. Verify Deletion ---
    let get_after_delete_result = client.alias(&alias_name).retrieve().await;
    assert!(
        get_after_delete_result.is_err(),
        "Alias should not exist after deletion"
    );

    // --- 8. Clean up the collection ---
    let delete_collection_result = client.collection(&collection_name).delete().await;
    assert!(
        delete_collection_result.is_ok(),
        "Failed to delete collection after alias test"
    );
}
