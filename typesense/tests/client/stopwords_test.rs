use typesense::models::StopwordsSetUpsertSchema;

use super::{get_client, new_id};

async fn run_test_stopwords_and_stopword_lifecycle() {
    let client = get_client();
    let set_id = new_id("custom_stopwords");

    // --- 1. Upsert a Stopwords Set (via `stopwords`) ---
    let schema = StopwordsSetUpsertSchema {
        stopwords: vec!["a".to_owned(), "the".to_owned(), "an".to_owned()],
        ..Default::default()
    };

    let upsert_result = client.stopwords().upsert(&set_id, schema).await;
    assert!(upsert_result.is_ok(), "Failed to upsert stopwords set");
    let upserted_set = upsert_result.unwrap();
    assert_eq!(upserted_set.id, set_id);
    assert_eq!(upserted_set.stopwords, vec!["a", "the", "an"]);

    // --- 2. Retrieve the specific Stopword set (via `stopword`) ---
    let retrieve_one_result = client.stopword(&set_id).retrieve().await;
    assert!(
        retrieve_one_result.is_ok(),
        "Failed to retrieve the newly created stopwords set."
    );
    let retrieved_set = retrieve_one_result.unwrap();
    assert_eq!(retrieved_set.stopwords.id, set_id);
    assert_eq!(retrieved_set.stopwords.stopwords, vec!["a", "the", "an"]);

    // --- 3. Retrieve all stopwords sets (via `stopwords`) ---
    let retrieve_all_result = client.stopwords().retrieve().await;
    assert!(
        retrieve_all_result.is_ok(),
        "Failed to retrieve the list of stopwords sets."
    );
    let all_sets = retrieve_all_result.unwrap();

    // --- 4. Find our specific set within the list ---
    let our_set = all_sets.stopwords.iter().find(|s| s.id == set_id);
    assert!(
        our_set.is_some(),
        "The newly created stopwords set was not found in the list."
    );

    // --- 5. Delete the Stopword set (via `stopword`) ---
    let delete_result = client.stopword(&set_id).delete().await;
    assert!(delete_result.is_ok(), "Failed to delete stopwords set");
    let deleted_response = delete_result.unwrap();
    assert_eq!(deleted_response.id, set_id);

    // --- 6. Verify Deletion ---
    let get_after_delete_result = client.stopword(&set_id).retrieve().await;
    assert!(
        get_after_delete_result.is_err(),
        "Stopwords set should not exist after deletion"
    );
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tokio_test {
    use super::*;

    #[tokio::test]
    async fn test_stopwords_and_stopword_lifecycle() {
        run_test_stopwords_and_stopword_lifecycle().await;
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_stopwords_and_stopword_lifecycle() {
        console_error_panic_hook::set_once();
        run_test_stopwords_and_stopword_lifecycle().await;
    }
}
