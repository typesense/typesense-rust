pub mod aliases_test;
pub mod analytics_test;
pub mod client_test;
pub mod collections_test;
pub mod conversation_models_test;
pub mod documents_test;
pub mod keys_test;
pub mod multi_search_test;
pub mod presets_test;
pub mod search_overrides_test;
pub mod stemming_dictionaries_test;
pub mod stopwords_test;
pub mod synonyms_test;

use reqwest::Url;
use reqwest_retry::policies::ExponentialBackoff;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use typesense::client::{Client, MultiNodeConfiguration};

/// Helper function to create a new client for all tests in this suite.
pub fn get_client() -> Client {
    let config = MultiNodeConfiguration {
        nodes: vec![Url::parse("http://localhost:8108").unwrap()],
        nearest_node: None,
        api_key: "xyz".to_string(),
        healthcheck_interval: Duration::from_secs(60),
        retry_policy: ExponentialBackoff::builder().build_with_max_retries(3),
        connection_timeout: Duration::from_secs(10),
    };
    Client::new(config).unwrap()
}

/// Generates a unique name for a test resource by combining a prefix,
/// a nanoid, and an optional suffix.
/// e.g., "test_collection_aB1cD2eF_create"
pub fn new_id(suffix: &str) -> String {
    // Using nanoid for a short, URL-friendly, and collision-resistant random ID.
    // The default length of 21 is more than enough. We use 8 for conciseness.
    let random_part = nanoid::nanoid!(8); // e.g., "fX3a-b_1"

    // The timestamp helps ensure IDs are unique even across test runs that happen close together,
    // although nanoid is likely sufficient on its own.
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    format!("test_{}_{}_{}", suffix, timestamp, random_part)
}
