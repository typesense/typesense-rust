mod aliases_test;
mod client_test;
mod collections_test;
mod conversation_models_test;
mod derive_integration_test;
mod documents_test;
mod keys_test;
mod multi_search_test;
mod operations_test;
mod presets_test;
mod stemming_dictionaries_test;
mod stopwords_test;

use std::time::Duration;
use typesense::{Client, ExponentialBackoff};
use web_time::{SystemTime, UNIX_EPOCH};

/// Helper function to create a new client for all tests in this suite.
pub fn get_client() -> Client {
    Client::builder()
        .nodes(vec!["http://localhost:8108"])
        .api_key("xyz")
        .healthcheck_interval(Duration::from_secs(5))
        .retry_policy(ExponentialBackoff::builder().build_with_max_retries(0))
        .connection_timeout(Duration::from_secs(3))
        .build()
        .expect("Failed to create Typesense client")
}

/// Generates a unique name for a test resource by combining a prefix,
/// a timestamp, and a nano id.
/// e.g., "test_collection_123456789_aB1cD2eF"
pub fn new_id(prefix: &str) -> String {
    let random_part = nanoid::nanoid!(8);
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    format!("test_{}_{}_{}", prefix, timestamp, random_part)
}
