use crate::{get_client, new_id};

#[tokio::test]
async fn test_stemming_dictionary_import_and_retrieve() {
    let client = get_client();
    let dictionary_id = new_id("verb_stems_v2");

    // --- 1. Define and Import the Dictionary ---
    // The JSONL payload uses "word" and "root" keys.
    let dictionary_data = r#"{"word": "running", "root": "run"}
{"word": "flies", "root": "fly"}"#
        .to_string();
    let import_result = client
        .stemming()
        .dictionaries()
        .import(&dictionary_id, dictionary_data)
        .await;
    assert!(
        import_result.is_ok(),
        "Failed to import stemming dictionary. Error: {:?}",
        import_result.err()
    );

    // --- 2. Retrieve the specific dictionary by its ID to verify contents ---
    // This is necessary because the list operation only returns IDs.
    let get_result = client
        .stemming()
        .dictionary(&dictionary_id)
        .retrieve()
        .await;
    assert!(
        get_result.is_ok(),
        "Failed to retrieve the specific stemming dictionary. Error: {:?}",
        get_result.err()
    );

    let dictionary = get_result.unwrap();
    assert_eq!(dictionary.id, dictionary_id);
    assert_eq!(
        dictionary.words.len(),
        2,
        "The number of words in the retrieved dictionary is incorrect."
    );
    assert!(
        dictionary
            .words
            .iter()
            .any(|w| w.word == "running" && w.root == "run"),
        "The mapping for 'running' -> 'run' was not found."
    );

    // --- 3. Retrieve all dictionary IDs and find ours ---
    let list_result = client.stemming().dictionaries().retrieve().await;
    assert!(
        list_result.is_ok(),
        "Failed to retrieve the list of stemming dictionaries. Error: {:?}",
        list_result.err()
    );

    let list_response = list_result.unwrap();
    let dictionary_ids = list_response.dictionaries;

    assert!(
        dictionary_ids.is_some(),
        "The list of dictionary IDs should not be None."
    );

    let ids_vec = dictionary_ids.unwrap();
    assert!(
        ids_vec.iter().any(|id| id == &dictionary_id),
        "The newly imported dictionary's ID was not found in the master list."
    );
}
