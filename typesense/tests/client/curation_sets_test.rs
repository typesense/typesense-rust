use std::vec;

use super::{get_client, new_id};
use typesense::models;

async fn run_test_curation_sets_and_items_lifecycle() {
    let client = get_client();
    let curation_set_name = new_id("curation_set");
    let item_id = "customize-apple";
    let item2_id = "customize-apple-2";

    //  Create a curation set
    let create_schema = models::CurationSetCreateSchema {
        items: vec![models::CurationItemCreateSchema {
            id: Some(item_id.into()),
            rule: Box::new(models::CurationRule {
                query: Some("apple".into()),
                r#match: Some(models::CurationRuleMatch::Exact),
                ..Default::default()
            }),
            includes: Some(vec![models::CurationInclude {
                id: "1".into(),
                position: 1,
            }]),
            excludes: Some(vec![models::CurationExclude { id: "2".into() }]),
            ..Default::default()
        }],
        ..Default::default()
    };

    let create_result = client
        .curation_sets()
        .upsert(&curation_set_name, create_schema.clone())
        .await;
    assert!(create_result.is_ok(), "Failed to create curation set");
    let created_set = create_result.unwrap();
    assert_eq!(created_set.name, curation_set_name);
    assert_eq!(created_set.items[0].id, create_schema.items[0].id);

    let this_curation_set = client.curation_set(&curation_set_name);

    // Retrieve the specific curation set
    let retrieve_one_result = this_curation_set.retrieve().await;
    assert!(
        retrieve_one_result.is_ok(),
        "Failed to retrieve the newly created set."
    );
    let retrieved_set = retrieve_one_result.unwrap();
    assert_eq!(retrieved_set.name, curation_set_name);

    // Retrieve all curation sets
    let retrieve_all_result = client.curation_sets().retrieve().await;
    assert!(
        retrieve_all_result.is_ok(),
        "Failed to retrieve the list of curation sets."
    );
    let all_sets_response = retrieve_all_result.unwrap();
    assert!(
        all_sets_response.len() >= 1,
        "Expected at least one curation set to be present."
    );
    assert!(
        all_sets_response
            .iter()
            .any(|r| r.name == curation_set_name)
    );

    // Create curation item
    let create_item_result = this_curation_set
        .items()
        .upsert(
            item2_id,
            models::CurationItemCreateSchema {
                rule: Box::new(models::CurationRule {
                    query: Some("apple".into()),
                    r#match: Some(models::CurationRuleMatch::Exact),
                    ..Default::default()
                }),
                includes: Some(vec![models::CurationInclude {
                    id: "1".into(),
                    position: 1,
                }]),
                excludes: Some(vec![models::CurationExclude { id: "2".into() }]),
                ..Default::default()
            },
        )
        .await;

    assert!(
        create_item_result.is_ok(),
        "Failed to upsert curation item."
    );
    assert_eq!(create_item_result.unwrap().id, item2_id);

    // Retrieve a curation item
    let retrieve_item_result = this_curation_set.item(item2_id).retrieve().await;

    assert!(
        retrieve_item_result.is_ok(),
        "Failed to retrieve curation item."
    );
    assert_eq!(retrieve_item_result.unwrap().id, item2_id);

    // Retrieve all curation items
    let retrieve_all_items_result = this_curation_set.items().retrieve().await;
    assert!(
        retrieve_all_items_result.is_ok(),
        "Failed to retrieve all curation items."
    );
    let all_items_response = retrieve_all_items_result.unwrap();
    assert!(
        all_items_response.iter().any(|r| r.id == item2_id),
        "Expected to find the created curation item in the list."
    );

    // delete a curation item
    let delete_item_result = this_curation_set.item(item2_id).delete().await;
    assert!(
        delete_item_result.is_ok(),
        "Failed to delete curation item."
    );
    let deleted_item_response = delete_item_result.unwrap();
    assert_eq!(deleted_item_response.id, item2_id);

    // Verify deletion of curation item
    let get_after_delete_item_result = this_curation_set.item(item2_id).retrieve().await;
    assert!(
        get_after_delete_item_result.is_err(),
        "Curation item should not exist after deletion"
    );

    // Delete a curation set
    let delete_result = client.curation_set(&curation_set_name).delete().await;
    assert!(delete_result.is_ok(), "Failed to delete curation set");
    let deleted_response = delete_result.unwrap();
    assert_eq!(deleted_response.name, curation_set_name);

    // Verify deletion
    let get_after_delete_result = client.curation_set(&curation_set_name).delete().await;
    assert!(
        get_after_delete_result.is_err(),
        "Curation set should not exist after deletion"
    );
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tokio_test {
    use super::*;

    #[tokio::test]
    async fn test_curation_sets_and_items_lifecycl() {
        run_test_curation_sets_and_items_lifecycle().await;
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_curation_sets_and_items_lifecycl() {
        console_error_panic_hook::set_once();
        run_test_curation_sets_and_items_lifecycle().await;
    }
}
