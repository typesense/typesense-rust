//! # A batteries-included, multi-node-aware client for the Typesense API.
//!
//! This module provides the main `Client` for interacting with a Typesense cluster.
//! It is designed for resilience and ease of use, incorporating features like
//! automatic failover, health checks, and a structured, ergonomic API.
//!
//! ## Key Features:
//! - **Multi-Node Operation**: Automatically manages connections to multiple Typesense nodes.
//! - **Health Checks & Failover**: Monitors node health and seamlessly fails over to healthy nodes upon encountering server or network errors.
//! - **Nearest Node Priority**: Can be configured to always prioritize a specific "nearest" node to reduce latency.
//! - **Fluent, Namespaced API**: Operations are grouped into logical namespaces like `.collections()`, `.documents("books")`, and `.operations()`, making the API discoverable and easy to use.
//! - **Built-in Retries**: Handles transient network errors with an exponential backoff policy for each node.
//!
//! ## Example Usage
//!
//! ```no_run
//! use typesense_client::client::{Client, MultiNodeConfiguration};
//! use typesense_codegen::models;
//! use reqwest::Url;
//! use reqwest_retry::policies::ExponentialBackoff;
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = MultiNodeConfiguration {
//!         nodes: vec![Url::parse("http://localhost:8108")?],
//!         nearest_node: None,
//!         api_key: "your-api-key".to_string(),
//!         healthcheck_interval: Duration::from_secs(60),
//!         retry_policy: ExponentialBackoff::builder().build_with_max_retries(3),
//!         connection_timeout: Duration::from_secs(10),
//!     };
//!
//!     let client = Client::new(config)?;
//!
//!     // Retrieve details for a collection
//!     let collection = client.collections().get("products").await?;
//!     println!("Collection Name: {}", collection.name);
//!
//!     // Search for a document
//!     let search_params = models::SearchCollectionParams {
//!         q: "phone".to_string(),
//!         query_by: "name".to_string(),
//!         ..Default::default()
//!     };
//!     let search_results = client.documents("products").search(search_params).await?;
//!     println!("Found {} hits.", search_results.found.unwrap_or(0));
//!
//!     Ok(())
//! }
//! ```

pub mod analytics;
pub mod collection;
pub mod collections;
pub mod conversations;
pub mod key;
pub mod keys;
pub mod multi_search;
pub mod operations;
pub mod preset;
pub mod presets;
pub mod stemming;
pub mod stopword;
pub mod stopwords;

pub use analytics::Analytics;
pub use collection::Collection;
pub use collections::Collections;
pub use conversations::Conversations;
pub use key::Key;
pub use keys::Keys;
pub use operations::Operations;
pub use preset::Preset;
pub use presets::Presets;
pub use stemming::Stemming;
pub use stopword::Stopword;
pub use stopwords::Stopwords;

use reqwest::Url;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use std::future::Future;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::time::{Duration, Instant};
use thiserror::Error;
use typesense_codegen::apis::{self, configuration};

// --- Internal Node Health Struct ---
// This is an internal detail to track the state of each node.
#[derive(Debug)]
struct Node {
    url: Url,
    is_healthy: bool,
    last_access_timestamp: Instant,
}

/// Configuration for the multi-node Typesense client.
#[derive(Clone, Debug)]
pub struct MultiNodeConfiguration {
    /// A list of all nodes in the Typesense cluster.
    pub nodes: Vec<Url>,
    /// An optional, preferred node to try first for every request. Ideal for reducing latency.
    pub nearest_node: Option<Url>,
    /// The Typesense API key used for authentication.
    pub api_key: String,
    /// The duration after which an unhealthy node will be retried for requests.
    pub healthcheck_interval: Duration,
    /// The retry policy for transient network errors on a *single* node.
    pub retry_policy: ExponentialBackoff,
    /// The timeout for each individual network request.
    pub connection_timeout: Duration,
}

/// The primary error type for the Typesense client.
#[derive(Debug, Error)]
pub enum Error<E>
where
    E: std::fmt::Debug,
    apis::Error<E>: std::fmt::Display + std::fmt::Debug,
{
    /// Indicates that all configured nodes failed to process a request.
    /// The source contains the last error received.
    #[error("All API nodes failed to respond.")]
    AllNodesFailed(#[source] Box<Error<E>>),

    /// A network-level error occurred within the `reqwest` middleware stack (e.g., a connection timeout).
    #[error("A single node failed with a middleware error")]
    Middleware(#[from] reqwest_middleware::Error),

    /// An API-level error returned by the Typesense server (e.g., 404 Not Found, 400 Bad Request).
    #[error("A single node failed with an API error")]
    Api(#[from] apis::Error<E>),
}

/// The main entry point for all interactions with the Typesense API.
///
/// The client manages connections to multiple nodes and provides access to different
/// API resource groups (namespaces) like `collections`, `documents`, and `operations`.
#[derive(Debug)]
pub struct Client {
    // The Client now holds the stateful Node list.
    nodes: Vec<Arc<Mutex<Node>>>,
    nearest_node: Option<Arc<Mutex<Node>>>,
    api_key: String,
    healthcheck_interval: Duration,
    retry_policy: ExponentialBackoff,
    connection_timeout: Duration,
    current_node_index: AtomicUsize,
}

impl Client {
    /// Creates a new `Client` with the given configuration.
    ///
    /// Returns an error if the configuration contains no nodes.
    pub fn new(config: MultiNodeConfiguration) -> Result<Self, &'static str> {
        if config.nodes.is_empty() && config.nearest_node.is_none() {
            return Err("Configuration must include at least one node or a nearest_node.");
        }

        let nodes = config
            .nodes
            .into_iter()
            .map(|url| {
                Arc::new(Mutex::new(Node {
                    url,
                    is_healthy: true,
                    last_access_timestamp: Instant::now(),
                }))
            })
            .collect();

        let nearest_node = config.nearest_node.map(|url| {
            Arc::new(Mutex::new(Node {
                url,
                is_healthy: true,
                last_access_timestamp: Instant::now(),
            }))
        });

        Ok(Self {
            nodes,
            nearest_node,
            api_key: config.api_key,
            healthcheck_interval: config.healthcheck_interval,
            retry_policy: config.retry_policy,
            connection_timeout: config.connection_timeout,
            current_node_index: AtomicUsize::new(0),
        })
    }

    /// Selects the next node to use for a request based on health and priority.
    fn get_next_node(&self) -> Arc<Mutex<Node>> {
        // 1. Always try the nearest_node first if it exists.
        if let Some(nearest_node_arc) = &self.nearest_node {
            let node = nearest_node_arc.lock().unwrap();
            let is_due_for_check = Instant::now().duration_since(node.last_access_timestamp)
                >= self.healthcheck_interval;

            if node.is_healthy || is_due_for_check {
                return Arc::clone(nearest_node_arc);
            }
        }

        // 2. Fallback to the main list of nodes if no healthy nearest_node is available.
        if self.nodes.is_empty() {
            // This can only happen if ONLY a nearest_node was provided and it's unhealthy.
            // We must return it to give it a chance to recover.
            return Arc::clone(self.nearest_node.as_ref().unwrap());
        }

        // 3. Loop through all nodes once to find a healthy one.
        for _ in 0..self.nodes.len() {
            let index = self.current_node_index.fetch_add(1, Ordering::Relaxed) % self.nodes.len();
            let node_arc = &self.nodes[index];
            let node = node_arc.lock().unwrap();
            let is_due_for_check = Instant::now().duration_since(node.last_access_timestamp)
                >= self.healthcheck_interval;

            if node.is_healthy || is_due_for_check {
                return Arc::clone(node_arc);
            }
        }

        // 4. If all nodes are unhealthy and not due for a check, just pick the next one in the round-robin.
        // This gives it a chance to prove it has recovered.
        let index = self.current_node_index.load(Ordering::Relaxed) % self.nodes.len();
        Arc::clone(&self.nodes[index])
    }

    /// Sets the health status of a given node after a request attempt.
    fn set_node_health(&self, node_arc: &Arc<Mutex<Node>>, is_healthy: bool) {
        let mut node = node_arc.lock().unwrap();
        node.is_healthy = is_healthy;
        node.last_access_timestamp = Instant::now();
    }

    /// The core execution method that handles multi-node failover and retries.
    /// This internal method is called by all public API methods.
    pub(super) async fn execute<F, Fut, T, E>(&self, api_call: F) -> Result<T, Error<E>>
    where
        F: Fn(Arc<configuration::Configuration>) -> Fut,
        Fut: Future<Output = Result<T, apis::Error<E>>>,
        E: std::fmt::Debug,
        apis::Error<E>: std::fmt::Display + std::fmt::Debug,
    {
        let mut last_error: Option<Error<E>> = None;
        let num_nodes_to_try = self.nodes.len() + self.nearest_node.is_some() as usize;

        // Loop up to the total number of available nodes.
        for _ in 0..num_nodes_to_try {
            let node_arc = self.get_next_node();
            let node_url = {
                // Lock is held for a very short duration.
                let node = node_arc.lock().unwrap();
                node.url.clone()
            };

            // This client handles transient retries (e.g. network blips) on the *current node*.
            let http_client = ClientBuilder::new(
                reqwest::Client::builder()
                    .timeout(self.connection_timeout)
                    .build()
                    .expect("Failed to build reqwest client"),
            )
            .with(RetryTransientMiddleware::new_with_policy(
                self.retry_policy.clone(),
            ))
            .build();

            // Create a temporary, single-node config for the generated API function.
            let gen_config = Arc::new(configuration::Configuration {
                base_path: node_url
                    .to_string()
                    .strip_suffix('/')
                    .unwrap_or(node_url.as_str())
                    .to_string(),
                api_key: Some(configuration::ApiKey {
                    prefix: None,
                    key: self.api_key.clone(),
                }),
                client: http_client,
                ..Default::default()
            });

            match api_call(gen_config).await {
                Ok(response) => {
                    self.set_node_health(&node_arc, true); // Mark as healthy on success.
                    return Ok(response);
                }
                Err(e) => {
                    let wrapped_error: Error<E> = e.into();
                    if is_retriable(&wrapped_error) {
                        self.set_node_health(&node_arc, false); // Mark as unhealthy on retriable error.
                        last_error = Some(wrapped_error);
                        // Continue loop to try the next node.
                    } else {
                        // Non-retriable error (e.g., 404 Not Found), fail fast.
                        return Err(wrapped_error);
                    }
                }
            }
        }

        // If the loop finishes, all nodes have failed.
        Err(Error::AllNodesFailed(Box::new(last_error.expect(
            "No nodes were available to try, or all errors were non-retriable.",
        ))))
    }

    /// Provides access to API endpoints for managing collections like `create()` and `retrieve()`.
    pub fn collections(&self) -> collections::Collections<'_> {
        collections::Collections::new(self)
    }

    /// Provides access to API endpoints of a specific collection.
    pub fn collection<'a>(&'a self, collection_name: &'a str) -> Collection<'a> {
        Collection::new(self, collection_name)
    }

    /// Provides access to the analytics-related API endpoints.
    pub fn analytics(&self) -> Analytics<'_> {
        Analytics::new(self)
    }

    /// Returns a `Conversations` instance for managing conversation models.
    pub fn conversations(&self) -> Conversations {
        Conversations::new(self)
    }

    /// Provides access to top-level, non-namespaced API endpoints like `health` and `debug`.
    pub fn operations(&self) -> Operations<'_> {
        Operations::new(self)
    }

    /// Provides access to endpoints for managing the collection of API keys.
    ///
    /// Example: `client.keys().create(schema).await`
    pub fn keys(&self) -> Keys<'_> {
        Keys::new(self)
    }

    /// Provides access to endpoints for managing a single API key.
    ///
    /// # Arguments
    /// * `key_id` - The ID of the key to manage.
    ///
    /// Example: `client.key(123).delete().await`
    pub fn key(&self, key_id: i64) -> Key<'_> {
        Key::new(self, key_id)
    }

    /// Provides access to endpoints for managing all of your presets.
    ///
    /// # Example
    /// ```
    /// client.presets().list().await?;
    /// ```
    pub fn presets(&self) -> Presets {
        Presets::new(self)
    }

    /// Provides access to endpoints for managing a single preset.
    ///
    /// # Arguments
    /// * `preset_id` - The ID of the preset to manage.
    ///
    /// # Example
    /// ```
    /// client.preset("my-preset").retrieve().await?;
    /// ```
    pub fn preset<'a>(&'a self, preset_id: &'a str) -> Preset<'a> {
        Preset::new(self, preset_id)
    }

    /// Provides access to the stemming-related API endpoints.
    ///
    /// # Example
    ///
    /// ```no_run
    /// client.stemming().dictionaries().retrieve().await?;
    /// ```
    pub fn stemming(&self) -> Stemming {
        Stemming::new(self)
    }

    // --- Stopwords Accessors ---

    /// Provides access to endpoints for managing the collection of stopwords sets.
    ///
    /// Example: `client.stopwords().retrieve().await`
    pub fn stopwords(&self) -> Stopwords<'_> {
        Stopwords::new(self)
    }

    /// Provides access to endpoints for managing a single stopwords set.
    ///
    /// # Arguments
    /// * `set_id` - The ID of the stopwords set to manage.
    ///
    /// Example: `client.stopword("common_words").retrieve().await`
    pub fn stopword<'a>(&'a self, set_id: &'a str) -> Stopword<'a> {
        Stopword::new(self, set_id)
    }
}

/// A helper function to determine if an error is worth retrying on another node.
fn is_retriable<E>(error: &Error<E>) -> bool
where
    E: std::fmt::Debug,
    apis::Error<E>: std::fmt::Display + std::fmt::Debug,
{
    match error {
        // Network-level errors from middleware are always retriable.
        Error::Middleware(_) => true,
        Error::Api(api_err) => match api_err {
            // Server-side errors (5xx) indicate a problem with the node, so we should try another.
            apis::Error::ResponseError(content) => content.status.is_server_error(),
            // Underlying reqwest errors (e.g. connection refused) are retriable.
            apis::Error::Reqwest(_) => true,
            // Client-side (4xx) or parsing errors are not retriable as the request is likely invalid.
            _ => false,
        },
        Error::AllNodesFailed(_) => false,
    }
}
