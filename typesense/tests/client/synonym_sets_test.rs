use std::vec;

use super::{get_client, new_id};
use typesense::models::{self, SynonymItemSchema};

async fn run_test_synonym_sets_and_items_lifecycle() {
    let client = get_client();
    let synonym_set_name = new_id("clothing-synonyms");
    let item_id = "customize-apple";
    let item2_id = "customize-apple-2";

    //  Create a synonym set
    let create_schema = models::SynonymSetCreateSchema {
        items: vec![SynonymItemSchema {
            id: item_id.to_owned(),
            synonyms: vec!["blazer".to_owned(), "coat".to_owned(), "jacket".to_owned()],
            ..Default::default()
        }],
        ..Default::default()
    };

    let create_result = client
        .synonym_sets()
        .upsert(&synonym_set_name, create_schema.clone())
        .await;
    assert!(create_result.is_ok(), "Failed to create synonym set");
    let created_set = create_result.unwrap();
    assert_eq!(created_set.name, synonym_set_name);
    assert_eq!(created_set.items[0].id, create_schema.items[0].id);

    let this_synonym_set = client.synonym_set(&synonym_set_name);

    // Retrieve the specific synonym set
    let retrieve_one_result = this_synonym_set.retrieve().await;
    assert!(
        retrieve_one_result.is_ok(),
        "Failed to retrieve the newly created set."
    );
    let retrieved_set = retrieve_one_result.unwrap();
    assert_eq!(retrieved_set.name, synonym_set_name);

    // Retrieve all synonym sets
    let retrieve_all_result = client.synonym_sets().retrieve().await;
    assert!(
        retrieve_all_result.is_ok(),
        "Failed to retrieve the list of synonym sets."
    );
    let all_sets_response = retrieve_all_result.unwrap();
    assert!(
        all_sets_response.len() >= 1,
        "Expected at least one synonym set to be present."
    );
    assert!(all_sets_response.iter().any(|r| r.name == synonym_set_name));

    // Create synonym item
    let create_item_result = this_synonym_set
        .items()
        .upsert(
            item2_id,
            models::SynonymItemUpsertSchema {
                root: Some("smartphone".into()),
                synonyms: vec![
                    "iphone".to_owned(),
                    "android phone".to_owned(),
                    "mobile".to_owned(),
                ],
                ..Default::default()
            },
        )
        .await;

    assert!(create_item_result.is_ok(), "Failed to upsert synonym item.");
    assert_eq!(create_item_result.unwrap().id, item2_id);

    // Retrieve a synonym item
    let retrieve_item_result = this_synonym_set.item(item2_id).retrieve().await;

    assert!(
        retrieve_item_result.is_ok(),
        "Failed to retrieve synonym item."
    );
    assert_eq!(retrieve_item_result.unwrap().id, item2_id);

    // Retrieve all synonym items
    let retrieve_all_items_result = this_synonym_set.items().retrieve().await;
    assert!(
        retrieve_all_items_result.is_ok(),
        "Failed to retrieve all synonym items."
    );
    let all_items_response = retrieve_all_items_result.unwrap();
    assert!(
        all_items_response.iter().any(|r| r.id == item2_id),
        "Expected to find the created synonym item in the list."
    );

    // delete a synonym item
    let delete_item_result = this_synonym_set.item(item2_id).delete().await;
    assert!(delete_item_result.is_ok(), "Failed to delete synonym item.");
    let deleted_item_response = delete_item_result.unwrap();
    assert_eq!(deleted_item_response.id, item2_id);

    // Verify deletion of synonym item
    let get_after_delete_item_result = this_synonym_set.item(item2_id).retrieve().await;
    assert!(
        get_after_delete_item_result.is_err(),
        "Synonym item should not exist after deletion"
    );

    // Delete a synonym set
    let delete_result = this_synonym_set.delete().await;
    assert!(delete_result.is_ok(), "Failed to delete synonym set");
    let deleted_response = delete_result.unwrap();
    assert_eq!(deleted_response.name, synonym_set_name);

    // Verify deletion
    let get_after_delete_result = this_synonym_set.delete().await;
    assert!(
        get_after_delete_result.is_err(),
        "Synonym set should not exist after deletion"
    );
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tokio_test {
    use super::*;

    #[tokio::test]
    async fn test_synonym_sets_and_items_lifecycl() {
        run_test_synonym_sets_and_items_lifecycle().await;
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_synonym_sets_and_items_lifecycl() {
        console_error_panic_hook::set_once();
        run_test_synonym_sets_and_items_lifecycle().await;
    }
}
