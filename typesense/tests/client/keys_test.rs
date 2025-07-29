use super::get_client;
use typesense::models::ApiKeySchema;

#[tokio::test]
async fn test_keys_lifecycle() {
    let client = get_client();
    let key_description = "A test search-only key.";

    // --- 1. Create a new API Key (via `keys`) ---
    let key_schema = ApiKeySchema {
        description: key_description.to_string(),
        actions: vec!["documents:search".to_string()], // Grant only search permissions
        collections: vec!["*".to_string()],            // For all collections
        ..Default::default()
    };

    let create_result = client.keys().create(key_schema).await;
    assert!(create_result.is_ok(), "Failed to create the API key.");
    let created_key = create_result.unwrap();

    // The full key value is only returned on creation
    assert!(
        created_key.value.is_some(),
        "The full API key value should be present upon creation."
    );
    assert_eq!(created_key.description, key_description.to_string());

    let key_id = created_key.id.unwrap();

    // --- 2. Retrieve the specific key (via `key`) ---
    let retrieve_one_result = client.key(key_id).retrieve().await;
    assert!(
        retrieve_one_result.is_ok(),
        "Failed to retrieve the specific API key."
    );
    let retrieved_key = retrieve_one_result.unwrap();

    // On retrieval, the value should be None and the prefix should be present
    assert_eq!(retrieved_key.id.unwrap(), key_id);
    assert!(
        retrieved_key.value.is_none(),
        "The retrieved key should not contain the full value."
    );
    assert!(
        retrieved_key.value_prefix.is_some(),
        "The retrieved key should have a value prefix."
    );

    // --- 3. Retrieve all keys (via `keys`) ---
    let retrieve_all_result = client.keys().retrieve().await;
    assert!(
        retrieve_all_result.is_ok(),
        "Failed to retrieve the list of keys."
    );
    let all_keys_response = retrieve_all_result.unwrap();

    // --- 4. Find our specific key within the list ---
    let our_key = all_keys_response
        .keys
        .iter()
        .find(|k| k.id.unwrap() == (key_id));
    assert!(
        our_key.is_some(),
        "The newly created key was not found in the list."
    );

    // --- 5. Delete the key (via `key`) ---
    let delete_result = client.key(key_id).delete().await;
    assert!(delete_result.is_ok(), "Failed to delete the API key.");
    let delete_response = delete_result.unwrap();
    assert_eq!(
        delete_response.id, key_id,
        "The response from delete should contain the correct key ID."
    );

    // --- 6. Verify Deletion ---
    let get_after_delete_result = client.key(key_id).retrieve().await;
    assert!(
        get_after_delete_result.is_err(),
        "API key should not exist after deletion."
    );
}
