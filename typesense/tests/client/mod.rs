mod aliases_test;
mod analytics_test;
mod client_test;
mod collections_test;
mod conversation_models_test;
mod derive_integration_test;
mod documents_test;
mod keys_test;
mod multi_search_test;
mod presets_test;
mod search_overrides_test;
mod stemming_dictionaries_test;
mod stopwords_test;
mod synonyms_test;

use reqwest::Url;
use reqwest_retry::policies::ExponentialBackoff;
use std::time::Duration;
use typesense::Client;
use web_time::{SystemTime, UNIX_EPOCH};

/// Helper function to create a new client for all tests in this suite.
pub fn get_client() -> Client {
    Client::builder()
        .nodes(vec![Url::parse("http://localhost:8108").unwrap()])
        .api_key("xyz")
        .healthcheck_interval(Duration::from_secs(5))
        .retry_policy(ExponentialBackoff::builder().build_with_max_retries(1))
        .connection_timeout(Duration::from_secs(3))
        .build()
        .expect("Failed to create Typesense client")
}

/// Generates a unique name for a test resource by combining a prefix,
/// a nanoid, and an optional suffix.
/// e.g., "test_collection_aB1cD2eF_create"
pub fn new_id(prefix: &str) -> String {
    // Using nanoid for a short, URL-friendly, and collision-resistant random ID.
    // The default length of 21 is more than enough. We use 8 for conciseness.
    let random_part = nanoid::nanoid!(8); // e.g., "fX3a-b_1"

    // The timestamp helps ensure IDs are unique even across test runs that happen close together,
    // although nanoid is likely sufficient on its own.
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    format!("test_{}_{}_{}", prefix, timestamp, random_part)
}
