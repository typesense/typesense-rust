use super::{get_client, new_id};
use serde_json::json;
use typesense::models::analytics_rule_schema::Type::Counter;
use typesense::models::{
    self, AnalyticsEventCreateSchema, AnalyticsRuleParametersDestination,
    AnalyticsRuleParametersSource, AnalyticsRuleParametersSourceEventsInner, AnalyticsRuleSchema,
};
use typesense_codegen::models::{CollectionSchema, Field};

#[tokio::test]
async fn test_analytics_rules_and_events_lifecycle() {
    let client = get_client();
    let rule_name_1 = new_id("product_clicks");
    let collection_name = new_id("products");
    let event_name = "products_click_event";

    // --- 1. Create a Collection (via `collections`) ---
    let schema = CollectionSchema {
        name: collection_name.clone(),
        fields: vec![
            Field {
                name: "title".to_string(),
                r#type: "string".to_string(),
                ..Default::default()
            },
            Field {
                name: "popularity".to_string(),
                r#type: "int32".to_string(),
                optional: Some(true),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let create_result = client.collections().create(schema).await;
    assert!(create_result.is_ok(), "Failed to create collection");
    let created_collection = create_result.unwrap();
    assert_eq!(created_collection.name, collection_name);

    // --- 2. Create a Rule (via `rules.create`) ---
    let create_schema = AnalyticsRuleSchema {
        name: rule_name_1.clone(),
        r#type: Counter,
        params: Box::new(models::AnalyticsRuleParameters {
            source: Box::new(AnalyticsRuleParametersSource {
                collections: vec!["products".to_string()],
                events: Some(vec![AnalyticsRuleParametersSourceEventsInner {
                    r#type: "click".to_string(),
                    weight: 1.0,
                    name: event_name.to_owned(),
                }]),
            }),
            destination: Box::new(AnalyticsRuleParametersDestination {
                collection: "products".to_string(),
                counter_field: Some("popularity".to_string()),
            }),
            ..Default::default()
        }),
    };

    let create_result = client.analytics().rules().create(create_schema).await;
    assert!(create_result.is_ok(), "Failed to create analytics rule");
    let created_rule = create_result.unwrap();
    assert_eq!(created_rule.name, rule_name_1);
    assert_eq!(created_rule.r#type, Counter);

    // --- 3. Retrieve the specific Rule (via `rule`) ---
    let retrieve_one_result = client.analytics().rule(&rule_name_1).retrieve().await;
    assert!(
        retrieve_one_result.is_ok(),
        "Failed to retrieve the newly created rule."
    );
    let retrieved_rule = retrieve_one_result.unwrap();
    assert_eq!(retrieved_rule.name, rule_name_1);

    // --- 4. Retrieve all Rules (via `rules`) ---
    let retrieve_all_result = client.analytics().rules().retrieve().await;
    assert!(
        retrieve_all_result.is_ok(),
        "Failed to retrieve the list of rules."
    );
    let all_rules_response = retrieve_all_result.unwrap();
    assert!(
        all_rules_response.rules.as_ref().unwrap().len() >= 1,
        "Expected at least one rule to be present."
    );
    assert!(all_rules_response
        .rules
        .unwrap()
        .iter()
        .any(|r| r.name == rule_name_1));

    // --- 5. Sending click events (via `events`) ---
    let event_result = client
        .analytics()
        .events()
        .create(AnalyticsEventCreateSchema {
            r#type: "click".to_string(),
            name: event_name.to_owned(),
            data: json!({
                "doc_id": "1024",
                "user_id": "111112"
            }),
        })
        .await;

    assert!(event_result.is_ok(), "Failed to send the click event.");
    assert!(event_result.unwrap().ok, "Unsuccessful click event.");

    // --- 6. Delete a Rule (via `rule`) ---
    let delete_result = client.analytics().rule(&rule_name_1).delete().await;
    assert!(delete_result.is_ok(), "Failed to delete rule");
    let deleted_response = delete_result.unwrap();
    assert_eq!(deleted_response.name, rule_name_1);

    // --- 7. Verify Deletion ---
    let get_after_delete_result = client.analytics().rule(&rule_name_1).retrieve().await;
    assert!(
        get_after_delete_result.is_err(),
        "Rule should not exist after deletion"
    );
}
