//! # A batteries-included, multi-node-aware client for the Typesense API.
//!
//! This module provides the main `Client` for interacting with a Typesense cluster.
//! It is designed for resilience and ease of use, incorporating features like
//! automatic failover, health checks, and a structured, ergonomic API.
//!
//! ## Key Features:
//! - **Multi-Node Configuration**: Automatically manages connections to multiple Typesense nodes.
//! - **Health Checks & Failover**: Monitors node health and seamlessly fails over to healthy nodes upon encountering server or network errors.
//! - **Nearest Node Priority**: Can be configured to always prioritize a specific nearest node to reduce latency.
//! - **Built-in Retries**: Handles transient network errors with an exponential backoff policy for each node.
//!
//! ## Example Usage
//!
//! The following example demonstrates how to use the client in a standard
//! server-side **Tokio** environment.
//!
//! ```no_run
//! #[cfg(not(target_family = "wasm"))]
//! {
//! use typesense::{Client, models, ExponentialBackoff};
//! use reqwest::Url;
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::builder()
//!         .nodes(vec![Url::parse("http://localhost:8108").unwrap()])
//!         .api_key("xyz")
//!         .healthcheck_interval(Duration::from_secs(60))
//!         .retry_policy(ExponentialBackoff::builder().build_with_max_retries(3))
//!         .connection_timeout(Duration::from_secs(5))
//!         .build()
//!         .unwrap();
//!
//!     // Retrieve details for a collection
//!     let collection = client.collection("products").retrieve().await?;
//!     println!("Collection Name: {}", collection.name);
//!
//!     // Search for a document
//!     let search_params = models::SearchParameters {
//!         q: Some("phone".to_owned()),
//!         query_by: Some("name".to_owned()),
//!         ..Default::default()
//!     };
//!
//!     let search_results = client
//!         .collection("products")
//!         .documents()
//!         .search(search_params)
//!         .await?;
//!
//!     println!("Found {} hits.", search_results.found.unwrap_or(0));
//!     Ok(())
//! }
//! }
//! ```
//! ---
//!
//! ### WebAssembly (Wasm) Usage
//!
//! When compiling for a WebAssembly target (`wasm32-unknown-unknown`),
//! Tokio-based features such as middleware, retries, and connection
//! timeouts are **not available**.
//!
//! Example:
//!
//! ```no_run
//! #[cfg(target_family = "wasm")]
//! {
//! use typesense::{Client, models};
//! use reqwest::Url;
//! use std::time::Duration;
//! use wasm_bindgen_futures::spawn_local;
//!
//! fn main() {
//!     spawn_local(async {
//!         let client = Client::builder()
//!             .nodes(vec![Url::parse("http://localhost:8108").unwrap()])
//!             .api_key("xyz")
//!             .healthcheck_interval(Duration::from_secs(60))
//!             // .retry_policy(...)       <-- not supported in Wasm
//!             // .connection_timeout(...) <-- not supported in Wasm
//!             .build()
//!             .unwrap();
//!
//!         // Retrieve details for a collection
//!         match client.collection("products").retrieve().await {
//!             Ok(collection) => println!("Collection Name: {}", collection.name),
//!             Err(e) => eprintln!("Error retrieving collection: {}", e),
//!         }
//!
//!         // Search for a document
//!         let search_params = models::SearchParameters {
//!             q: Some("phone".to_owned()),
//!             query_by: Some("name".to_owned()),
//!             ..Default::default()
//!         };
//!
//!         match client.collection("products").documents().search(search_params).await {
//!             Ok(search_results) => {
//!                 println!("Found {} hits.", search_results.found.unwrap_or(0));
//!             }
//!             Err(e) => eprintln!("Error searching documents: {}", e),
//!         }
//!     });
//! }
//! }
//! ```
mod collection;
mod collections;
mod key;
mod keys;
mod multi_search;

use collection::Collection;
use collections::Collections;
use key::Key;
use keys::Keys;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::Error;
use reqwest::Url;
#[cfg(not(target_arch = "wasm32"))]
use reqwest_middleware::ClientBuilder as ReqwestMiddlewareClientBuilder;
#[cfg(not(target_arch = "wasm32"))]
use reqwest_retry::RetryTransientMiddleware;
pub use reqwest_retry::policies::ExponentialBackoff;

use std::future::Future;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};
use typesense_codegen::apis::{self, configuration};
use web_time::{Duration, Instant};

// This is an internal detail to track the state of each node.
#[derive(Debug)]
struct Node {
    url: Url,
    is_healthy: bool,
    last_access_timestamp: Instant,
}
/// The main entry point for all interactions with the Typesense API.
///
/// The client manages connections to multiple nodes and provides access to different
/// API resource groups (namespaces) like `collections`, `documents`, and `operations`.
#[derive(Debug)]
pub struct Client {
    nodes: Vec<Arc<Mutex<Node>>>,
    nearest_node: Option<Arc<Mutex<Node>>>,
    api_key: String,
    healthcheck_interval: Duration,
    current_node_index: AtomicUsize,

    #[cfg(not(target_arch = "wasm32"))]
    retry_policy: ExponentialBackoff,
    #[cfg(not(target_arch = "wasm32"))]
    connection_timeout: Duration,
}

#[bon::bon]
impl Client {
    /// Creates a new `Client`.
    ///
    /// Returns an error if the configuration contains no nodes. Default values:
    /// - **nearest_node**: None.
    /// - **healthcheck_interval**: 60 seconds.
    /// - **retry_policy**: Exponential backoff with a maximum of 3 retries. (disabled on WASM)
    /// - **connection_timeout**: 5 seconds. (disabled on WASM)
    #[builder]
    pub fn new(
        /// The Typesense API key used for authentication.
        api_key: impl Into<String>,
        /// A list of all nodes in the Typesense cluster.
        nodes: Vec<Url>,
        /// An optional, preferred node to try first for every request. This is for your server-side load balancer.
        #[builder(into)]
        nearest_node: Option<Url>,
        #[builder(default = Duration::from_secs(60))]
        /// The duration after which an unhealthy node will be retried for requests.
        healthcheck_interval: Duration,
        #[builder(default = ExponentialBackoff::builder().build_with_max_retries(3))]
        /// The retry policy for transient network errors on a *single* node.
        retry_policy: ExponentialBackoff,
        #[builder(default = Duration::from_secs(5))]
        /// The timeout for each individual network request.
        connection_timeout: Duration,
    ) -> Result<Self, &'static str> {
        if nodes.is_empty() && nearest_node.is_none() {
            return Err("Configuration must include at least one node or a nearest_node.");
        }

        let node_list = nodes
            .into_iter()
            .map(|url| {
                Arc::new(Mutex::new(Node {
                    url,
                    is_healthy: true,
                    last_access_timestamp: Instant::now(),
                }))
            })
            .collect();

        let nearest_node_arc = nearest_node.map(|url| {
            Arc::new(Mutex::new(Node {
                url,
                is_healthy: true,
                last_access_timestamp: Instant::now(),
            }))
        });

        Ok(Self {
            nodes: node_list,
            nearest_node: nearest_node_arc,
            api_key: api_key.into(),
            healthcheck_interval,
            current_node_index: AtomicUsize::new(0),

            #[cfg(not(target_arch = "wasm32"))]
            retry_policy,
            #[cfg(not(target_arch = "wasm32"))]
            connection_timeout,
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
        F: Fn(configuration::Configuration) -> Fut,
        Fut: Future<Output = Result<T, apis::Error<E>>>,
        E: std::fmt::Debug + 'static,
        apis::Error<E>: std::error::Error + 'static,
    {
        let mut last_api_error: Option<apis::Error<E>> = None;
        let num_nodes_to_try = self.nodes.len() + self.nearest_node.is_some() as usize;

        // Loop up to the total number of available nodes.
        for _ in 0..num_nodes_to_try {
            let node_arc = self.get_next_node();
            let node_url = {
                let node = node_arc.lock().unwrap();
                node.url.clone()
            };

            #[cfg(target_arch = "wasm32")]
            let http_client = reqwest::Client::builder()
                .build()
                .expect("Failed to build reqwest client");

            #[cfg(not(target_arch = "wasm32"))]
            let http_client = ReqwestMiddlewareClientBuilder::new(
                reqwest::Client::builder()
                    .timeout(self.connection_timeout)
                    .build()
                    .expect("Failed to build reqwest client"),
            )
            .with(RetryTransientMiddleware::new_with_policy(self.retry_policy))
            .build();

            // Create a temporary config for this attempt.
            let gen_config = configuration::Configuration {
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
            };

            match api_call(gen_config).await {
                Ok(response) => {
                    self.set_node_health(&node_arc, true);
                    return Ok(response);
                }
                Err(e) => {
                    if is_retriable(&e) {
                        self.set_node_health(&node_arc, false);
                        last_api_error = Some(e);
                    } else {
                        return Err(e.into());
                    }
                }
            }
        }

        Err(crate::Error::AllNodesFailed {
            source: last_api_error
                .expect("No nodes were available to try, or all errors were non-retriable."),
        })
    }

    /// Provides access to API endpoints for managing collections like `create()` and `retrieve()`.
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::{Client, GetCollectionsParameters};
    /// # use reqwest::Url;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec![Url::parse("http://localhost:8108").unwrap()])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let all_collections = client.collections().retrieve(GetCollectionsParameters::default()).await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    pub fn collections(&self) -> Collections<'_> {
        Collections::new(self)
    }

    /// Provides access to API endpoints for a specific collection.
    ///
    /// This method returns a `Collection<T>` handle, which is generic over the type of document
    /// stored in that collection.
    ///
    /// # Type Parameters
    /// * `T` - The type of the documents in the collection. It must be serializable and deserializable.
    ///
    /// # Arguments
    /// * `collection_name` - The name of the collection to interact with.
    ///
    /// # Example: Working with a strongly-typed collection
    ///
    /// When you want to retrieve or search for documents and have them automatically
    /// deserialized into your own structs.
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::Client;
    /// # use serde::{Serialize, Deserialize};
    /// # use reqwest::Url;
    /// #
    /// # #[derive(Serialize, Deserialize, Debug)]
    /// # struct Book { id: String, title: String }
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec![Url::parse("http://localhost:8108").unwrap()])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// // Get a typed handle to the "books" collection
    /// let books_collection = client.collection_of::<Book>("books");
    ///
    /// // Retrieve a single book, it returns `Result<Book, ...>`
    /// let book = books_collection.document("123").retrieve().await?;
    /// println!("Retrieved book: {:?}", book);
    /// #
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    pub fn collection_of<'a, T>(&'a self, collection_name: impl Into<String>) -> Collection<'a, T>
    where
        T: DeserializeOwned + Serialize + Send + Sync,
    {
        Collection::new(self, collection_name.into())
    }

    /// Provides access to API endpoints for a specific collection using schemaless `serde_json::Value` documents.
    ///
    /// This is the simplest way to interact with a collection when you do not need strong typing.
    /// It is a convenient shorthand for `client.collection_of::<serde_json::Value>("...")`.
    ///
    /// The returned handle can be used for both document operations (which will return `serde_json::Value`)
    /// and collection-level operations (like `.delete()` or `.retrieve()`).
    ///
    /// # Arguments
    /// * `collection_name` - The name of the collection to interact with.
    ///
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::Client;
    /// # use reqwest::Url;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec![Url::parse("http://localhost:8108").unwrap()])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let products_collection = client.collection("products");
    /// #
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    pub fn collection<'a>(
        &'a self,
        collection_name: impl Into<String>,
    ) -> Collection<'a, serde_json::Value> {
        Collection::new(self, collection_name.into())
    }

    /// Provides access to endpoints for managing the collection of API keys.
    ///
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::{Client, models};
    /// # use reqwest::Url;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec![Url::parse("http://localhost:8108").unwrap()])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// # let schema = models::ApiKeySchema {
    /// #     description: "Search-only key.".to_owned(),
    /// #     actions: vec!["documents:search".to_owned()],
    /// #     collections: vec!["*".to_owned()],
    /// #     ..Default::default()
    /// # };
    /// let new_key = client.keys().create(schema).await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    pub fn keys(&self) -> Keys<'_> {
        Keys::new(self)
    }

    /// Provides access to endpoints for managing a single API key.
    ///
    /// # Arguments
    /// * `key_id` - The ID of the key to manage.
    ///
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::Client;
    /// # use reqwest::Url;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec![Url::parse("http://localhost:8108").unwrap()])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let deleted_key = client.key(123).delete().await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    pub fn key(&self, key_id: i64) -> Key<'_> {
        Key::new(self, key_id)
    }

    /// Provides access to the multi search endpoint.
    ///
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::{Client, models};
    /// # use reqwest::Url;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec![Url::parse("http://localhost:8108").unwrap()])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// # let search_requests = models::MultiSearchBody {
    /// #     searches: vec![models::MultiSearchCollectionParameters {
    /// #         collection: Some("products".to_owned()),
    /// #         q: Some("phone".to_owned()),
    /// #         query_by: Some("name".to_owned()),
    /// #         ..Default::default()
    /// #     }],
    /// #     ..Default::default()
    /// # };
    /// # let common_params = models::MultiSearchParameters::default();
    /// let results = client.multi_search().perform(search_requests, common_params).await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    pub fn multi_search(&self) -> multi_search::MultiSearch<'_> {
        multi_search::MultiSearch::new(self)
    }
}

/// A helper function to determine if an error is worth retrying on another node.
fn is_retriable<E>(error: &apis::Error<E>) -> bool
where
    E: std::fmt::Debug + 'static,
    apis::Error<E>: std::error::Error + 'static,
{
    match error {
        // Server-side errors (5xx) indicate a problem with the node, so we should try another.
        apis::Error::ResponseError(content) => content.status.is_server_error(),

        // Underlying reqwest errors (e.g., connection refused) are retriable.
        apis::Error::Reqwest(_) => true,

        // Network-level errors from middleware are always retriable.
        #[cfg(not(target_arch = "wasm32"))]
        apis::Error::ReqwestMiddleware(_) => true,

        // Client-side (4xx) or parsing errors are not retriable as the request is likely invalid.
        _ => false,
    }
}
