use typesense::models::{
    PresetSchema, PresetUpsertSchema, PresetUpsertSchemaValue, SearchParameters,
};

use super::{get_client, new_id};

#[tokio::test]
async fn test_presets_lifecycle() {
    let client = get_client();
    let preset_id = new_id("search-preset");

    // --- 1. Define the Preset's value using the strong types ---
    // This will be the expected value in the response as well.
    let search_params = SearchParameters {
        query_by: Some("title,authors".to_string()),
        sort_by: Some("_text_match:desc,publication_year:desc".to_string()),
        ..Default::default()
    };
    let expected_preset_value = PresetUpsertSchemaValue::SearchParameters(Box::new(search_params));

    // This is the schema to be sent in the request body.
    let upsert_schema = PresetUpsertSchema {
        value: Box::new(expected_preset_value.clone()),
    };

    // --- 2. Create (Upsert) a Preset (via `presets`) ---
    let upsert_result = client.presets().upsert(&preset_id, upsert_schema).await;
    assert!(
        upsert_result.is_ok(),
        "Failed to create preset: {:?}",
        upsert_result.err()
    );

    // The API returns a full PresetSchema object.
    let created_preset: PresetSchema = upsert_result.unwrap();
    assert_eq!(created_preset.name, preset_id);
    // Compare the strongly-typed value field directly.
    assert_eq!(*created_preset.value, expected_preset_value);

    // --- 3. Retrieve the specific preset (via `preset`) ---
    let retrieve_one_result = client.preset(&preset_id).retrieve().await;
    assert!(
        retrieve_one_result.is_ok(),
        "Failed to retrieve the specific preset."
    );
    let retrieved_preset: PresetSchema = retrieve_one_result.unwrap();
    assert_eq!(retrieved_preset.name, preset_id);
    assert_eq!(*retrieved_preset.value, expected_preset_value);

    // --- 4. Retrieve all presets (via `presets`) ---
    let retrieve_all_result = client.presets().retrieve().await;
    assert!(
        retrieve_all_result.is_ok(),
        "Failed to retrieve all presets."
    );
    let all_presets_response = retrieve_all_result.unwrap();

    // --- 5. Find our preset in the list ---
    let our_preset = all_presets_response
        .presets
        .iter()
        .find(|p| p.name == preset_id);

    assert!(
        our_preset.is_some(),
        "The created preset was not found in the list."
    );

    if let Some(preset) = our_preset {
        assert_eq!(preset.name, preset_id);
        assert_eq!(*preset.value, expected_preset_value);
    }

    // --- 6. Delete the preset (via `preset`) ---
    let delete_result = client.preset(&preset_id).delete().await;
    assert!(delete_result.is_ok(), "Failed to delete preset.");
    let deleted_preset = delete_result.unwrap();
    assert_eq!(deleted_preset.name, preset_id);

    // --- 7. Verify Deletion ---
    let get_after_delete_result = client.preset(&preset_id).retrieve().await;
    assert!(
        get_after_delete_result.is_err(),
        "Preset should not exist after deletion."
    );
}
