use super::get_client;
use typesense::models::{ApiKeySchema, ScopedKeyParameters, SearchParameters};

async fn run_test_keys_lifecycle() {
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

#[test]
fn test_generate_scoped_search_key_with_example_values() {
    // The parent key with `documents:search` permissions.
    let search_only_api_key = "RN23GFr1s6jQ9kgSNg2O7fYcAUXU7127";

    // The parameters to be embedded in the new scoped key.
    let params = ScopedKeyParameters {
        search_params: Some(SearchParameters {
            filter_by: Some("company_id:124".to_string()),
            ..Default::default()
        }),
        expires_at: Some(1906054106),
        ..Default::default()
    };

    // The known correct output from the Typesense documentation.
    let expected_scoped_key = "OW9DYWZGS1Q1RGdSbmo0S1QrOWxhbk9PL2kxbTU1eXA3bCthdmE5eXJKRT1STjIzeyJmaWx0ZXJfYnkiOiJjb21wYW55X2lkOjEyNCIsImV4cGlyZXNfYXQiOjE5MDYwNTQxMDZ9";

    let client = get_client();

    let generated_key_result = client
        .keys()
        .generate_scoped_search_key(search_only_api_key, &params);

    // First, ensure the function returned an Ok result.
    assert!(
        generated_key_result.is_ok(),
        "Function returned an error: {:?}",
        generated_key_result.err()
    );

    // Unwrap the result and compare it with the expected output.
    let generated_key = generated_key_result.unwrap();
    assert_eq!(
        generated_key, expected_scoped_key,
        "The generated key does not match the expected key."
    );
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tokio_test {
    use super::*;

    #[tokio::test]
    async fn test_keys_lifecycle() {
        run_test_keys_lifecycle().await;
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_keys_lifecycle() {
        console_error_panic_hook::set_once();
        run_test_keys_lifecycle().await;
    }
}
