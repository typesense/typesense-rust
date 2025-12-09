use std::vec;

use super::{get_client, new_id};
use typesense::models::{
    self, AnalyticsEvent, AnalyticsEventData, AnalyticsRuleCreate,
    AnalyticsRuleType::PopularQueries, CollectionSchema, Field,
};

async fn logic_test_analytics_rules_and_events_lifecycle() {
    let client = get_client();
    let rule_name_1 = new_id("product_clicks");
    let rule_name_2 = new_id("product_clicks");
    let rule_name_3 = new_id("product_clicks");
    let collection_name = new_id("products");
    let queries_collection_name = new_id("queries");

    // Create a Collection
    let schema = CollectionSchema {
        name: collection_name.as_str().into(),
        fields: vec![
            Field {
                name: "title".into(),
                r#type: "string".into(),
                ..Default::default()
            },
            Field {
                name: "popularity".into(),
                r#type: "int32".into(),
                optional: Some(true),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let queries_collection_schema = CollectionSchema {
        name: queries_collection_name.as_str().into(),
        fields: vec![
            Field {
                name: "q".into(),
                r#type: "string".into(),
                ..Default::default()
            },
            Field {
                name: "count".into(),
                r#type: "int32".into(),
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

    let create_result_2 = client.collections().create(queries_collection_schema).await;
    assert!(
        create_result_2.is_ok(),
        "Failed to create queries collection"
    );

    //  Create a Rule
    let create_schema = AnalyticsRuleCreate {
        name: rule_name_1.as_str().into(),
        r#type: PopularQueries,
        collection: collection_name.as_str().into(),
        event_type: "search".into(),
        rule_tag: Some("homepage".into()),
        params: Some(Box::new(models::AnalyticsRuleCreateParams {
            destination_collection: Some(queries_collection_name.as_str().into()),
            limit: Some(100),
            capture_search_requests: Some(true),
            ..Default::default()
        })),
    };

    let create_result = client
        .analytics()
        .rules()
        .create(create_schema.clone())
        .await;
    assert!(create_result.is_ok(), "Failed to create analytics rule");
    let created_rule = create_result.unwrap();
    assert_eq!(created_rule.name, rule_name_1);
    assert_eq!(created_rule.r#type, PopularQueries);
    assert_eq!(created_rule.params, create_schema.params);

    let create_many_result = client
        .analytics()
        .rules()
        .create_many(vec![
            AnalyticsRuleCreate {
                name: rule_name_2.as_str().into(),
                r#type: PopularQueries,
                collection: collection_name.as_str().into(),
                event_type: "search".into(),
                rule_tag: Some("homepage".into()),
                params: Some(Box::new(models::AnalyticsRuleCreateParams {
                    destination_collection: Some(queries_collection_name.as_str().into()),
                    limit: Some(25),
                    ..Default::default()
                })),
            },
            AnalyticsRuleCreate {
                name: rule_name_3.as_str().into(),
                r#type: PopularQueries,
                collection: collection_name.as_str().into(),
                event_type: "search".into(),
                rule_tag: Some("homepage".into()),
                params: Some(Box::new(models::AnalyticsRuleCreateParams {
                    destination_collection: Some(queries_collection_name.as_str().into()),
                    limit: Some(50),
                    ..Default::default()
                })),
            },
        ])
        .await;
    println!("{:?}", create_many_result);
    assert!(
        create_many_result.is_ok(),
        "Failed to create analytics rule"
    );
    let created_rules = create_many_result.unwrap();
    if let models::CreateAnalyticsRule200ResponseOneOfInner::AnalyticsRule(rule) = &created_rules[0]
    {
        assert_eq!(rule.name, rule_name_2);
    } else {
        panic!("Expected AnalyticsRule variant");
    }

    if let models::CreateAnalyticsRule200ResponseOneOfInner::AnalyticsRule(rule) = &created_rules[1]
    {
        assert_eq!(rule.name, rule_name_3);
    } else {
        panic!("Expected AnalyticsRule variant");
    }

    // Retrieve the specific Rule
    let retrieve_one_result = client.analytics().rule(&rule_name_1).retrieve().await;
    assert!(
        retrieve_one_result.is_ok(),
        "Failed to retrieve the newly created rule."
    );
    let retrieved_rule = retrieve_one_result.unwrap();
    assert_eq!(retrieved_rule.name, rule_name_1);

    // Retrieve all Rules
    let retrieve_all_result = client.analytics().rules().retrieve(None).await;
    assert!(
        retrieve_all_result.is_ok(),
        "Failed to retrieve the list of rules."
    );
    let all_rules_response = retrieve_all_result.unwrap();
    assert!(
        all_rules_response.len() >= 1,
        "Expected at least one rule to be present."
    );
    assert!(all_rules_response.iter().any(|r| r.name == rule_name_1));

    // Sending click events
    let event_result = client
        .analytics()
        .events()
        .create(AnalyticsEvent {
            name: rule_name_1.as_str().into(),
            event_type: "search".into(),
            data: Box::new(AnalyticsEventData {
                q: Some("running shoes".into()),
                user_id: Some("111112".into()),
                ..Default::default()
            }),
        })
        .await;
    println!("{:?}", event_result);

    assert!(event_result.is_ok(), "Failed to send the click event.");
    assert!(event_result.unwrap().ok, "Unsuccessful click event.");

    // Delete a Rule
    let delete_result = client.analytics().rule(&rule_name_1).delete().await;
    assert!(delete_result.is_ok(), "Failed to delete rule");
    let deleted_response = delete_result.unwrap();
    assert_eq!(deleted_response.name, rule_name_1);

    // Verify deletion
    let get_after_delete_result = client.analytics().rule(&rule_name_1).retrieve().await;
    assert!(
        get_after_delete_result.is_err(),
        "Rule should not exist after deletion"
    );
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tokio_test {
    use super::*;

    #[tokio::test]
    async fn test_analytics_rules_and_events_lifecycle() {
        logic_test_analytics_rules_and_events_lifecycle().await;
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_analytics_rules_and_events_lifecycle() {
        console_error_panic_hook::set_once();
        logic_test_analytics_rules_and_events_lifecycle().await;
    }
}
