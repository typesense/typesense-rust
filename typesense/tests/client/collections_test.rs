use typesense::{
    GetCollectionsParameters,
    models::{CollectionSchema, CollectionUpdateSchema, Field},
};
use typesense_codegen::apis::collections_api::GetCollectionParams;

use super::{get_client, new_id};

async fn logic_test_collections_and_collection_lifecycle() {
    let client = get_client();
    let collection_name = new_id("products");

    // --- 1. Create a Collection (via `collections`) ---
    let schema = CollectionSchema {
        name: collection_name.clone(),
        fields: vec![
            Field {
                name: "name".to_string(),
                r#type: "string".to_string(),
                ..Default::default()
            },
            Field {
                name: "price".to_string(),
                r#type: "int32".to_string(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let create_result = client.collections().create(schema).await;
    assert!(create_result.is_ok(), "Failed to create collection");
    let created_collection = create_result.unwrap();
    assert_eq!(created_collection.name, collection_name);

    // --- 2. Retrieve the specific Collection (via `collection`) ---
    let retrieve_one_result = client.collection(&collection_name).retrieve().await;
    assert!(
        retrieve_one_result.is_ok(),
        "Failed to retrieve the newly created collection."
    );
    let retrieved_collection = retrieve_one_result.unwrap();
    assert_eq!(retrieved_collection.name, collection_name);
    assert_eq!(retrieved_collection.fields.len(), 2);

    // --- 3. Retrieve all collections (via `collections`) ---
    let retrieve_all_result = client
        .collections()
        .retrieve(&GetCollectionsParameters::default())
        .await;
    assert!(
        retrieve_all_result.is_ok(),
        "Failed to retrieve the list of collections."
    );
    let all_collections = retrieve_all_result.unwrap();

    // --- 4. Find our specific collection within the list ---
    let our_collection = all_collections.iter().find(|c| c.name == collection_name);
    assert!(
        our_collection.is_some(),
        "The newly created collection was not found in the list."
    );

    // --- 5. Update the Collection to add and drop a field (via `collection`) ---
    let update_schema = CollectionUpdateSchema {
        fields: vec![
            // Add a new field
            Field {
                name: "description".to_string(),
                r#type: "string".to_string(),
                optional: Some(true),
                ..Default::default()
            },
            // Drop an existing field
            Field {
                name: "price".to_string(),
                drop: Some(true),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let update_result = client
        .collection(&collection_name)
        .update(update_schema)
        .await;
    assert!(update_result.is_ok(), "Failed to update collection");

    // The update response contains the fields that were modified
    let updated_fields_response = update_result.unwrap();
    assert_eq!(
        updated_fields_response.fields.len(),
        2,
        "The update response should contain the two modified fields."
    );

    // --- 6. Verify the update by retrieving the full schema again ---
    let retrieve_after_update_result = client.collection(&collection_name).retrieve().await;
    let retrieved_after_update = retrieve_after_update_result.unwrap();

    // Initial fields: name, price. Update: +description, -price. Final fields: name, description.
    assert_eq!(
        retrieved_after_update.fields.len(),
        2,
        "The number of fields should be 2 after the update."
    );
    assert!(
        retrieved_after_update
            .fields
            .iter()
            .any(|f| f.name == "name"),
        "The 'name' field should still exist."
    );
    assert!(
        retrieved_after_update
            .fields
            .iter()
            .any(|f| f.name == "description"),
        "The 'description' field should have been added."
    );
    assert!(
        !retrieved_after_update
            .fields
            .iter()
            .any(|f| f.name == "price"),
        "The 'price' field should have been dropped."
    );

    // --- 7. Delete the Collection (via `collection`) ---
    let delete_result = client.collection(&collection_name).delete().await;
    assert!(delete_result.is_ok(), "Failed to delete collection");

    // --- 8. Verify Deletion ---
    let get_after_delete_result = client.collection(&collection_name).retrieve().await;
    assert!(
        get_after_delete_result.is_err(),
        "Collection should not exist after deletion"
    );
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tokio_test {
    use super::*;

    #[tokio::test]
    async fn test_collections_and_collection_lifecycle() {
        logic_test_collections_and_collection_lifecycle().await;
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_collections_and_collection_lifecycle() {
        console_error_panic_hook::set_once();
        logic_test_collections_and_collection_lifecycle().await;
    }
}
