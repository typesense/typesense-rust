use std::time::Duration;

use reqwest_retry::policies::ExponentialBackoff;
use typesense::{
    client::{Error as TypesenseError, MultiNodeConfiguration},
    models::ConversationModelUpdateSchema,
};
use typesense_codegen::models::{CollectionSchema, ConversationModelCreateSchema, Field};

use super::{get_client, new_id};

#[tokio::test]
async fn test_create_model_with_invalid_key_fails_as_expected() {
    let client = get_client();
    let model_id = new_id("gpt-4-invalid-key-test");
    let collection_name = new_id("conversation_store_invalid");

    // --- 1. Setup: Create the prerequisite collection for history ---
    let schema = CollectionSchema {
        name: collection_name.clone(),
        fields: vec![
            Field {
                name: "conversation_id".to_string(),
                r#type: "string".to_string(),
                ..Default::default()
            },
            Field {
                name: "model_id".to_string(),
                r#type: "string".to_string(),
                ..Default::default()
            },
            Field {
                name: "timestamp".to_string(),
                r#type: "int32".to_string(),
                ..Default::default()
            },
            Field {
                name: "role".to_string(),
                r#type: "string".to_string(),
                index: Some(false),
                ..Default::default()
            },
            Field {
                name: "message".to_string(),
                r#type: "string".to_string(),
                index: Some(false),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    let create_collection_result = client.collections().create(schema).await;
    assert!(
        create_collection_result.is_ok(),
        "Setup failed: Could not create the collection needed for the test."
    );

    // --- 2. Action: Attempt to create a model with a deliberately invalid API key ---
    let create_schema = ConversationModelCreateSchema {
        id: Some(model_id.clone()),
        model_name: "openai/gpt-4".to_string(),
        api_key: Some("THIS_IS_AN_INVALID_KEY".to_string()),
        history_collection: collection_name.clone(),
        max_bytes: 10000,
        ..Default::default()
    };
    let create_result = client.conversations().models().create(create_schema).await;

    // --- 3. Assertion: Verify that the creation failed with the correct error ---
    assert!(
        create_result.is_err(),
        "Model creation should have failed due to an invalid API key, but it succeeded."
    );
    match create_result.err() {
        Some(TypesenseError::Api(response_content)) => match response_content {
            typesense::apis::Error::ResponseError(api_error) => {
                assert_eq!(
                    api_error.status.as_u16(),
                    400,
                    "Expected HTTP status code 400 for an invalid key."
                );
                assert!(
                    api_error.content.contains("Incorrect API key provided"),
                    "The error message did not match the expected content. Got: {}",
                    api_error.content
                );
            }
            other_entity => {
                panic!(
                    "Expected a Status400 error entity but got something else: {:?}",
                    other_entity
                );
            }
        },
        other_error => {
            panic!(
                "Expected a Typesense ResponseError, but got a different kind of error: {:?}",
                other_error
            );
        }
    }

    // --- 4. Teardown: Clean up the collection created during setup ---
    let delete_collection_result = client.collection(&collection_name).delete().await;
    assert!(
        delete_collection_result.is_ok(),
        "Teardown failed: Could not delete the test collection."
    );
}

use typesense::client::Client;
use wiremock::{
    matchers::{body_json, method, path},
    Mock, MockServer, ResponseTemplate,
};

// Helper to create a Typesense client configured for a mock server.
fn get_test_client(uri: &str) -> Client {
    let config = MultiNodeConfiguration {
        nodes: vec![uri.parse().unwrap()],
        nearest_node: None, // Not needed for single-node tests
        api_key: "TEST_API_KEY".to_string(),
        // Keep other settings minimal for testing
        healthcheck_interval: Duration::from_secs(60),
        retry_policy: ExponentialBackoff::builder().build_with_max_retries(0),
        connection_timeout: Duration::from_secs(1),
    };
    Client::new(config).unwrap()
}

#[tokio::test]
async fn test_create_model_with_wiremock() {
    // --- 1. Setup: Start a mock server ---
    let mock_server = MockServer::start().await;

    // --- 2. Setup: Configure the Typesense client to use the mock server's URI ---
    let client = get_test_client(&mock_server.uri());

    // --- 3. Setup: Define the request and the expected successful response ---
    let model_id = new_id("conv-model-test");
    let collection_name = new_id("history-collection");

    let create_schema = ConversationModelCreateSchema {
        id: Some(model_id.clone()),
        model_name: "openai/gpt-4".to_string(),
        api_key: Some("A-FAKE-BUT-VALID-LOOKING-KEY".to_string()),
        history_collection: collection_name.clone(),
        system_prompt: Some("You are a helpful assistant.".to_string()),
        ..Default::default()
    };

    // This is the successful JSON body we expect the mock server to return.
    // It should match the structure of `ConversationModelSchema`.
    let mock_response_body = serde_json::json!({
      "id": model_id,
      "model_name": "openai/gpt-4",
      "history_collection": collection_name,
      "api_key": "sk-FA**********************************KEY", // Masked key
      "system_prompt": "You are a helpful assistant.",
      "max_bytes": 16384,
      "ttl": 86400
    });

    // --- 4. Setup: Define the mock server's behavior ---
    Mock::given(method("POST"))
        .and(path("/conversations/models"))
        .and(body_json(&create_schema)) // Ensure the client sends the correct body
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response_body.clone()))
        .expect(1) // Expect this mock to be called exactly once
        .mount(&mock_server)
        .await;

    // --- 5. Action: Call the client method ---
    let create_result = client.conversations().models().create(create_schema).await;

    // --- 6. Assertion: Verify the result ---
    assert!(
        create_result.is_ok(),
        "The client should have successfully parsed the 200 response from the mock server. Error: {:?}",
        create_result.err()
    );

    // Unwrap the successful result and check if its fields match the mocked response
    let created_model = create_result.unwrap();
    assert_eq!(created_model.id, model_id);
    assert_eq!(created_model.model_name, "openai/gpt-4");
    assert_eq!(created_model.history_collection, collection_name);
    assert_eq!(
        created_model.system_prompt,
        Some("You are a helpful assistant.".to_string())
    );
}

#[tokio::test]
async fn test_retrieve_all_models_with_wiremock() {
    // --- 1. Setup ---
    let mock_server = MockServer::start().await;
    let client = get_test_client(&mock_server.uri());

    // The response body should be a Vec<ConversationModelSchema>
    let mock_response_body = serde_json::json!([
        {
            "id": "model-1",
            "model_name": "openai/gpt-3.5-turbo",
            "history_collection": "conversation_store",
            "api_key": "OPENAI_API_KEY",
            "system_prompt": "Hey, you are an **intelligent** assistant for question-answering. You can only make conversations based on the provided context. If a response cannot be formed strictly using the provided context, politely say you do not have knowledge about that topic.",
            "max_bytes": 16384
          },
          {
            "id": "model-2",
            "model_name": "openai/gpt-3.5-turbo",
            "history_collection": "conversation_store",
            "api_key": "OPENAI_API_KEY",
            "system_prompt": "Hey, you are an **intelligent** assistant for question-answering. You can only make conversations based on the provided context. If a response cannot be formed strictly using the provided context, politely say you do not have knowledge about that topic.",
            "max_bytes": 16384
          }
    ]);

    // --- 2. Mocking ---
    Mock::given(method("GET"))
        .and(path("/conversations/models"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    // --- 3. Action ---
    let retrieve_result = client.conversations().models().retrieve().await;

    // --- 4. Assertion ---
    assert!(retrieve_result.is_ok(), "Retrieving all models failed");
    let models = retrieve_result.unwrap();
    assert_eq!(models.len(), 2);
    assert_eq!(models[0].id, "model-1");
    assert_eq!(models[1].id, "model-2");
}

#[tokio::test]
async fn test_retrieve_single_model_with_wiremock() {
    // --- 1. Setup ---
    let mock_server = MockServer::start().await;
    let client = get_test_client(&mock_server.uri());

    let model_id = new_id("conv-model");
    let mock_response_body = serde_json::json!({
        "id": model_id,
        "model_name": "openai/gpt-3.5-turbo",
        "history_collection": "conversation_store",
        "api_key": "OPENAI_API_KEY",
        "system_prompt": "Hey, you are an **intelligent** assistant for question-answering. You can only make conversations based on the provided context. If a response cannot be formed strictly using the provided context, politely say you do not have knowledge about that topic.",
        "max_bytes": 16384
    });

    // --- 2. Mocking ---
    Mock::given(method("GET"))
        .and(path(format!("/conversations/models/{}", model_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    // --- 3. Action ---
    let retrieve_result = client.conversations().model(&model_id).retrieve().await;

    // --- 4. Assertion ---
    assert!(retrieve_result.is_ok());
    assert_eq!(retrieve_result.unwrap().id, model_id);
}

#[tokio::test]
async fn test_update_single_model_with_wiremock() {
    // --- 1. Setup ---
    let mock_server = MockServer::start().await;
    let client = get_test_client(&mock_server.uri());

    let model_id = new_id("conv-model");

    let update_schema = ConversationModelUpdateSchema {
        system_prompt: Some("A new, updated prompt.".to_string()),
        ..Default::default()
    };

    // The response body reflects the updated state of the resource
    let mock_response_body = serde_json::json!({
        "id": model_id,
        "model_name": "openai/gpt-3.5-turbo",
        "history_collection": "conversation_store",
        "api_key": "OPENAI_API_KEY",
        "system_prompt": "A new, updated prompt.",
        "max_bytes": 16384
    });

    // --- 2. Mocking ---
    Mock::given(method("PUT")) // As per docs, update uses PUT
        .and(path(format!("/conversations/models/{}", model_id)))
        .and(body_json(&update_schema)) // Verify the client sends the correct update payload
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    // --- 3. Action ---
    let update_result = client
        .conversations()
        .model(&model_id)
        .update(update_schema)
        .await;

    // --- 4. Assertion ---
    assert!(update_result.is_ok());
    let updated_model = update_result.unwrap();
    assert_eq!(updated_model.id, model_id);
    assert_eq!(
        updated_model.system_prompt.unwrap(),
        "A new, updated prompt."
    );
}

#[tokio::test]
async fn test_delete_single_model_with_wiremock() {
    // --- 1. Setup ---
    let mock_server = MockServer::start().await;
    let client = get_test_client(&mock_server.uri());

    let model_id = new_id("conv-model-to-delete");

    // The API returns the object that was just deleted
    let mock_response_body = serde_json::json!({
        "id": model_id,
        "model_name": "openai/gpt-3.5-turbo",
        "history_collection": "conversation_store",
        "api_key": "OPENAI_API_KEY",
        "system_prompt": "Hey, you are an **intelligent** assistant for question-answering. You can only make conversations based on the provided context. If a response cannot be formed strictly using the provided context, politely say you do not have knowledge about that topic.",
        "max_bytes": 16384
    });

    // --- 2. Mocking ---
    Mock::given(method("DELETE"))
        .and(path(format!("/conversations/models/{}", model_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    // --- 3. Action ---
    let delete_result = client.conversations().model(&model_id).delete().await;

    // --- 4. Assertion ---
    assert!(delete_result.is_ok());
    let deleted_model = delete_result.unwrap();
    assert_eq!(deleted_model.id, model_id);
}
