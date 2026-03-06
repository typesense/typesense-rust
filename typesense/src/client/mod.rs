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
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::builder()
//!         .nodes(vec!["http://localhost:8108"])
//!         .api_key("xyz")
//!         .healthcheck_interval(Duration::from_secs(60))
//!         .retry_policy(ExponentialBackoff::builder().build_with_max_retries(3))
//!         .build()
//!         .unwrap();
//!
//!     // Retrieve details for a collection
//!     let collection = client.collection_schemaless("products").retrieve().await?;
//!     println!("Collection Name: {}", collection.name);
//!
//!     // Search for a document
//!     let search_params = models::SearchParameters {
//!         q: Some("phone".into()),
//!         query_by: Some("name".into()),
//!         ..Default::default()
//!     };
//!
//!     let search_results = client
//!         .collection_schemaless("products")
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
//! Tokio-based features such as middleware and retries are **not available**.
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
//!             .nodes(vec!["http://localhost:8108"])
//!             .api_key("xyz")
//!             .healthcheck_interval(Duration::from_secs(60))
//!             // .retry_policy(...)  <-- not supported in Wasm
//!             .build()
//!             .unwrap();
//!
//!         // Retrieve details for a collection
//!         match client.collection_schemaless("products").retrieve().await {
//!             Ok(collection) => println!("Collection Name: {}", collection.name),
//!             Err(e) => eprintln!("Error retrieving collection: {}", e),
//!         }
//!
//!         // Search for a document
//!         let search_params = models::SearchParameters {
//!             q: Some("phone".into()),
//!             query_by: Some("name".into()),
//!             ..Default::default()
//!         };
//!
//!         match client.collection_schemaless("products").documents().search(search_params).await {
//!             Ok(search_results) => {
//!                 println!("Found {} hits.", search_results.found.unwrap_or(0));
//!             }
//!             Err(e) => eprintln!("Error searching documents: {}", e),
//!         }
//!     });
//! }
//! }
//! ```
mod alias;
mod aliases;
mod analytics;
mod collection;
mod collections;
mod conversations;
mod curation_set;
mod curation_sets;
mod key;
mod keys;
mod multi_search;
mod operations;
mod preset;
mod presets;
mod stemming;
mod stopword;
mod stopwords;
mod synonym_set;
mod synonym_sets;

use crate::{Error, traits::Document};
use alias::Alias;
use aliases::Aliases;
use analytics::Analytics;
use collection::Collection;
use collections::Collections;
use conversations::Conversations;
use curation_set::CurationSet;
use curation_sets::CurationSets;
use key::Key;
use keys::Keys;
use operations::Operations;
use preset::Preset;
use presets::Presets;
use stemming::Stemming;
use stopword::Stopword;
use stopwords::Stopwords;
use synonym_set::SynonymSet;
use synonym_sets::SynonymSets;

#[cfg(not(target_arch = "wasm32"))]
use reqwest_middleware::ClientBuilder as ReqwestMiddlewareClientBuilder;
#[cfg(not(target_arch = "wasm32"))]
use reqwest_retry::RetryTransientMiddleware;
pub use reqwest_retry::policies::ExponentialBackoff;

use ::std::{
    borrow::Cow,
    future::Future,
    sync::{
        RwLock,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};
use serde::{Serialize, de::DeserializeOwned};
use typesense_codegen::apis::{self, configuration};
use web_time::{Duration, Instant};

/// Wraps api call in `client::execute()`
#[macro_export]
macro_rules! execute_wrapper {
    ($self:ident, $call:expr) => {
        $self.client.execute($call).await
    };
    ($self:ident, $call:expr, $params:ident) => {
        $self
            .client
            .execute(
                |config: &typesense_codegen::apis::configuration::Configuration| {
                    $call(config, &$params)
                },
            )
            .await
    };
}

/// Configuration for a single Typesense node.
///
/// Use this to customize the HTTP client for specific nodes,
/// for example to add custom TLS root certificates or configure proxies.
///
/// For simple cases, you can pass a plain URL string to the builder's
/// `.nodes()` method, which will be automatically converted.
///
/// # Examples
///
/// ```
/// use typesense::NodeConfig;
///
/// // Simple URL (same as passing a string directly)
/// let node = NodeConfig::new("https://node1.example.com");
///
/// // With custom HTTP client configuration
/// // (add timeouts, headers, TLS, etc. on native targets)
/// let node = NodeConfig::new("https://node2.example.com")
///     .http_builder(|builder| {
///         // This closure receives a `reqwest::ClientBuilder` and must return it.
///         // You can call any supported builder methods here; for example,
///         // `builder.connect_timeout(...)` on native targets.
///         builder
///     });
/// ```
pub struct NodeConfig {
    url: String,
    http_builder: Option<Box<dyn HttpBuilderFn>>,
}

/// Internal helper to allow storing and calling a boxed `FnOnce`.
trait HttpBuilderFn: Send {
    fn call_once(self: Box<Self>, builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder;
}

impl<F> HttpBuilderFn for F
where
    F: FnOnce(reqwest::ClientBuilder) -> reqwest::ClientBuilder + Send,
{
    fn call_once(self: Box<Self>, builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
        (*self)(builder)
    }
}

impl std::fmt::Debug for NodeConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeConfig")
            .field("url", &self.url)
            .field("http_builder", &self.http_builder.as_ref().map(|_| ".."))
            .finish()
    }
}

impl NodeConfig {
    /// Creates a new `NodeConfig` with the given URL.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            http_builder: None,
        }
    }

    /// Sets a custom HTTP client builder for this node.
    ///
    /// The closure receives a default [`reqwest::ClientBuilder`] and should return
    /// a configured builder. This is useful for adding custom TLS certificates,
    /// proxies, or other reqwest settings.
    ///
    /// When not set, a default builder with a 5-second connect timeout is used
    /// (native targets only; WASM uses the browser's defaults).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use typesense::NodeConfig;
    ///
    /// // You can capture arbitrary configuration here (certs, proxies, etc.)
    /// // and apply it to the `reqwest::ClientBuilder` on platforms that support it.
    /// let node = NodeConfig::new("https://secure.example.com")
    ///     .http_builder(move |builder| {
    ///         // Example (native-only, not shown here to keep the example
    ///         // portable across native and WASM):
    ///         //
    ///         //   builder
    ///         //       .add_root_certificate(cert)
    ///         //       .connect_timeout(std::time::Duration::from_secs(10))
    ///         //
    ///         // For this doctest, we just return the builder unchanged.
    ///         builder
    ///     });
    /// ```
    ///
    /// # Multiple nodes with the same configuration
    ///
    /// The closure is `FnOnce`, so it is consumed when the HTTP client for that node
    /// is built. To use the same configuration (e.g. the same TLS certificate) for
    /// multiple nodes, clone the value once per node when building the configs:
    ///
    /// ```no_run
    /// use typesense::{Client, NodeConfig};
    ///
    /// # fn cert() -> reqwest::Certificate { unimplemented!() }
    /// let cert = cert();
    /// let nodes = ["https://node1:8108", "https://node2:8108"]
    ///     .into_iter()
    ///     .map(|url| {
    ///         let cert_for_node = cert.clone();
    ///         NodeConfig::new(url).http_builder(move |b| {
    ///             b.add_root_certificate(cert_for_node) // reqwest takes ownership
    ///         })
    ///     })
    ///     .collect::<Vec<_>>();
    /// let _client = Client::builder().nodes(nodes).api_key("key").build();
    /// ```
    pub fn http_builder(
        mut self,
        f: impl FnOnce(reqwest::ClientBuilder) -> reqwest::ClientBuilder + Send + 'static,
    ) -> Self {
        self.http_builder = Some(Box::new(f));
        self
    }
}

impl From<String> for NodeConfig {
    fn from(url: String) -> Self {
        Self::new(url)
    }
}

impl<'a> From<&'a str> for NodeConfig {
    fn from(url: &'a str) -> Self {
        Self::new(url)
    }
}

impl From<reqwest::Url> for NodeConfig {
    fn from(url: reqwest::Url) -> Self {
        Self::new(url)
    }
}

// This is an internal detail to track the state of each node.
#[derive(Debug)]
struct Node {
    config: configuration::Configuration,
    is_healthy: AtomicBool,
    last_accessed: RwLock<Instant>,
}

impl Node {
    /// Sets the health status of the node
    #[inline]
    fn set_health(&self, is_healthy: bool) {
        *self.last_accessed.write().unwrap() = Instant::now();
        self.is_healthy.store(is_healthy, Ordering::Relaxed);
    }
}

/// The main entry point for all interactions with the Typesense API.
///
/// The client manages connections to multiple nodes and provides access to different
/// API resource groups (namespaces) like `collections`, `documents`, and `operations`.
#[derive(Debug)]
pub struct Client {
    nodes: Vec<Node>,
    is_nearest_node_set: bool,
    healthcheck_interval: Duration,
    current_node_index: AtomicUsize,
}

#[bon::bon]
impl Client {
    /// Creates a new `Client`.
    ///
    /// Returns an error if the configuration contains no nodes. Default values:
    /// - **nearest_node**: None.
    /// - **healthcheck_interval**: 60 seconds.
    /// - **retry_policy**: Exponential backoff with a maximum of 3 retries. (disabled on WASM)
    /// - **http_builder**: An `FnOnce(reqwest::ClientBuilder) -> reqwest::ClientBuilder` closure
    ///   for per-node HTTP client customization (optional, via [`NodeConfig`]).
    ///
    /// When no custom `http_builder` is configured, a default `reqwest::ClientBuilder` with
    /// a 5-second connect timeout is used (native targets only).
    #[builder]
    pub fn new(
        /// The Typesense API key used for authentication.
        #[builder(into)]
        api_key: String,
        /// A list of all nodes in the Typesense cluster.
        ///
        /// Accepts plain URL strings or [`NodeConfig`] instances for per-node
        /// HTTP client customization.
        #[builder(
            with = |iter: impl IntoIterator<Item = impl Into<NodeConfig>>|
                iter.into_iter().map(Into::into).collect::<Vec<NodeConfig>>()
        )]
        nodes: Vec<NodeConfig>,
        #[builder(into)]
        /// An optional, preferred node to try first for every request.
        /// This is for your server-side load balancer.
        /// Do not add this node to all nodes list, should be a separate one.
        nearest_node: Option<NodeConfig>,
        #[builder(default = Duration::from_secs(60))]
        /// The duration after which an unhealthy node will be retried for requests.
        healthcheck_interval: Duration,
        #[builder(default = ExponentialBackoff::builder().build_with_max_retries(3))]
        /// The retry policy for transient network errors on a *single* node.
        retry_policy: ExponentialBackoff,
    ) -> Result<Self, &'static str> {
        let is_nearest_node_set = nearest_node.is_some();

        let nodes: Vec<_> = nodes
            .into_iter()
            .chain(nearest_node)
            .map(|node_config| {
                let builder = match node_config.http_builder {
                    Some(f) => f.call_once(reqwest::Client::builder()),
                    None => {
                        let b = reqwest::Client::builder();
                        #[cfg(not(target_arch = "wasm32"))]
                        let b = b.connect_timeout(Duration::from_secs(5));
                        b
                    }
                };

                #[cfg(target_arch = "wasm32")]
                let http_client = builder.build().expect("Failed to build reqwest client");

                #[cfg(not(target_arch = "wasm32"))]
                let http_client = ReqwestMiddlewareClientBuilder::new(
                    builder.build().expect("Failed to build reqwest client"),
                )
                .with(RetryTransientMiddleware::new_with_policy(retry_policy))
                .build();

                let mut url = node_config.url;
                if url.len() > 1 && matches!(url.chars().last(), Some('/')) {
                    url.pop();
                }

                let config = configuration::Configuration {
                    base_path: url,
                    api_key: Some(configuration::ApiKey {
                        prefix: None,
                        key: api_key.clone(),
                    }),
                    client: http_client,
                    ..Default::default()
                };

                Node {
                    config,
                    is_healthy: AtomicBool::new(true),
                    last_accessed: RwLock::new(Instant::now()),
                }
            })
            .collect();

        if nodes.is_empty() {
            return Err("Configuration must include at least one node or a nearest_node.");
        }

        Ok(Self {
            nodes,
            is_nearest_node_set,
            healthcheck_interval,
            current_node_index: AtomicUsize::new(0),
        })
    }

    /// Selects the next node to use for a request based on health and priority.
    fn get_next_node(&self) -> &Node {
        // if only one node (including nearest)
        if self.nodes.len() == 1
            && let Some(first) = self.nodes.first()
        {
            return first;
        }

        let (nodes_len, mut index) = if self.is_nearest_node_set {
            let last_node_index = self.nodes.len() - 1;
            (last_node_index, last_node_index)
        } else {
            (
                self.nodes.len(),
                self.current_node_index.fetch_add(1, Ordering::Relaxed) % self.nodes.len(),
            )
        };

        for _ in 0..self.nodes.len() {
            let node = &self.nodes[index];

            if node.is_healthy.load(Ordering::Relaxed)
                || node.last_accessed.read().unwrap().elapsed() >= self.healthcheck_interval
            {
                return node;
            }
            index = self.current_node_index.fetch_add(1, Ordering::Relaxed) % nodes_len;
        }

        // If all nodes are unhealthy and not due for a check, just pick the next one in the round-robin.
        // This gives it a chance to prove it has recovered.
        index = self.current_node_index.load(Ordering::Relaxed) % self.nodes.len();
        &self.nodes[index]
    }

    /// For use in legacy APIs.
    #[inline]
    pub fn get_legacy_config(&self) -> &configuration::Configuration {
        &self.get_next_node().config
    }

    /// The core execution method that handles multi-node failover and retries.
    /// This internal method is called by all public API methods.
    pub(super) async fn execute<F, Fut, T, E, 'a>(&'a self, api_call: F) -> Result<T, Error<E>>
    where
        F: Fn(&'a configuration::Configuration) -> Fut,
        Fut: Future<Output = Result<T, apis::Error<E>>>,
        E: std::fmt::Debug + 'static,
        apis::Error<E>: std::error::Error + 'static,
    {
        let mut last_api_error: Option<apis::Error<E>> = None;
        // Loop up to the total number of available nodes.
        for _ in 0..self.nodes.len() {
            let node = self.get_next_node();
            match api_call(&node.config).await {
                Ok(response) => {
                    node.set_health(true);
                    return Ok(response);
                }
                Err(e) => {
                    if is_retriable(&e) {
                        node.set_health(false);
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

    /// Provides access to the collection aliases-related API endpoints.
    ///
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let all_aliases = client.aliases().retrieve().await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn aliases(&self) -> Aliases<'_> {
        Aliases::new(self)
    }

    /// Provides access to a specific collection alias's-related API endpoints.
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let specific_alias = client.alias("books_alias").retrieve().await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn alias<'a>(&'a self, alias_name: &'a str) -> Alias<'a> {
        Alias::new(self, alias_name)
    }

    /// Provides access to the analytics API endpoints.
    ///
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let all_rules = client.analytics().rules().retrieve(None).await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn analytics(&self) -> Analytics<'_> {
        Analytics::new(self)
    }

    /// Provides access to API endpoints for managing collections like `create()` and `retrieve()`.
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::{Client, models::GetCollectionsParameters};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let all_collections = client.collections().retrieve(GetCollectionsParameters::default()).await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn collections(&self) -> Collections<'_> {
        Collections::new(self)
    }

    /// Provides access to API endpoints for a specific collection.
    ///
    /// This method returns a `Collection<D>` handle, which is generic over the type of document
    /// stored in that collection.
    ///
    /// # Type Parameters
    /// * `D` - The type of the documents in the collection. It must be serializable and deserializable.
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
    /// #
    /// # #[derive(Serialize, Deserialize, Debug)]
    /// # struct Book { id: String, title: String }
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// // Get a typed handle to the "books" collection
    /// let books_collection = client.collection_named::<Book>("books");
    ///
    /// // Retrieve a single book, it returns `Result<Book, ...>`
    /// let book = books_collection.document("123").retrieve().await?;
    /// println!("Retrieved book: {:?}", book);
    /// #
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn collection_named<'c, D>(
        &'c self,
        collection_name: impl Into<Cow<'c, str>>,
    ) -> Collection<'c, D>
    where
        D: DeserializeOwned + Serialize,
    {
        Collection::new(self, collection_name)
    }

    /// Provides access to API endpoints for a specific collection.
    ///
    /// This method returns a `Collection<D>` handle, which is generic over the type of document
    /// stored in that collection.
    ///
    /// # Type Parameters
    /// * `D` - The type of the documents in the collection. It must be of trait Document.
    ///
    /// # Example: Working with a strongly-typed collection
    ///
    /// When you want to retrieve or search for documents and have them automatically
    /// deserialized into your own structs.
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::{Client, Typesense};
    /// # use serde::{Serialize, Deserialize};
    /// #
    /// # #[derive(Typesense, Serialize, Deserialize, Debug)]
    /// # struct Book { id: String, title: String }
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// // Get a typed handle to the "books" collection
    /// let books_collection = client.collection::<Book>();
    ///
    /// // Retrieve a single book, it returns `Result<Book, ...>`
    /// let book = books_collection.document("123").retrieve().await?;
    /// println!("Retrieved book: {:?}", book);
    /// #
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn collection<'c, D>(&'c self) -> Collection<'c, D>
    where
        D: Document,
    {
        Collection::new(self, D::COLLECTION_NAME)
    }

    /// Provides access to API endpoints for a specific collection using schemaless `serde_json::Value` documents.
    ///
    /// This is the simplest way to interact with a collection when you do not need strong typing.
    /// It is a convenient shorthand for `client.collection_named::<serde_json::Value>("...")`.
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
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let products_collection = client.collection_schemaless("products");
    /// #
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn collection_schemaless<'c>(
        &'c self,
        collection_name: impl Into<Cow<'c, str>>,
    ) -> Collection<'c, serde_json::Value> {
        Collection::new(self, collection_name)
    }

    /// Returns a `Conversations` instance for managing conversation models.
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let conversation = client.conversations().models().retrieve().await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn conversations(&self) -> Conversations<'_> {
        Conversations::new(self)
    }

    /// Provides access to endpoints for managing curation sets.
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let curation_sets = client.curation_sets().retrieve().await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn curation_sets(&self) -> CurationSets<'_> {
        CurationSets::new(self)
    }

    /// Provides access to endpoints for managing a specific curation set.
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let curation_set = client.curation_set("curation_set_name").retrieve().await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn curation_set<'a>(&'a self, curation_set_name: &'a str) -> CurationSet<'a> {
        CurationSet::new(self, curation_set_name)
    }

    /// Provides access to endpoints for managing the collection of API keys.
    ///
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::{Client, models};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// # let schema = models::ApiKeySchema {
    /// #     description: "Search-only key.".into(),
    /// #     actions: vec!["documents:search".to_owned()],
    /// #     collections: vec!["*".to_owned()],
    /// #     ..Default::default()
    /// # };
    /// let new_key = client.keys().create(schema).await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
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
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let deleted_key = client.key(123).delete().await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
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
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// # let search_requests = models::MultiSearchBody {
    /// #     searches: vec![models::MultiSearchCollectionParameters {
    /// #         collection: Some("products".into()),
    /// #         q: Some("phone".into()),
    /// #         query_by: Some("name".into()),
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
    #[inline]
    pub fn multi_search(&self) -> multi_search::MultiSearch<'_> {
        multi_search::MultiSearch::new(self)
    }

    /// Provides access to top-level, non-namespaced API endpoints like `health` and `debug`.
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let health = client.operations().health().await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn operations(&self) -> Operations<'_> {
        Operations::new(self)
    }

    /// Provides access to endpoints for managing all of your presets.
    ///
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let list_of_presets = client.presets().retrieve().await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn presets(&self) -> Presets<'_> {
        Presets::new(self)
    }

    /// Provides access to endpoints for managing a single preset.
    ///
    /// # Arguments
    /// * `preset_id` - The ID of the preset to manage.
    ///
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let preset = client.preset("my-preset").retrieve().await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn preset<'a>(&'a self, preset_id: &'a str) -> Preset<'a> {
        Preset::new(self, preset_id)
    }

    /// Provides access to the stemming-related API endpoints.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let response = client.stemming().dictionaries().retrieve().await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn stemming(&self) -> Stemming<'_> {
        Stemming::new(self)
    }

    /// Provides access to endpoints for managing the collection of stopwords sets.
    ///
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let all_stopwords = client.stopwords().retrieve().await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn stopwords(&self) -> Stopwords<'_> {
        Stopwords::new(self)
    }

    /// Provides access to endpoints for managing a single stopwords set.
    ///
    /// # Arguments
    /// * `set_id` - The ID of the stopwords set to manage.
    ///
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let my_stopword_set = client.stopword("common_words").retrieve().await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn stopword<'a>(&'a self, set_id: &'a str) -> Stopword<'a> {
        Stopword::new(self, set_id)
    }

    /// Provides access to endpoints for managing all synonym sets.
    ///
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let all_synonym_sets = client.synonym_sets().retrieve().await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn synonym_sets(&self) -> SynonymSets<'_> {
        SynonymSets::new(self)
    }

    /// Provides access to endpoints for managing a single synonym set.
    ///
    /// # Arguments
    /// * `synonym_set_name` - The name of the synonym set to manage.
    ///
    /// # Example
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec!["http://localhost:8108"])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let my_synonym_set = client.synonym_set("synonym_set_name").retrieve().await.unwrap();
    /// # Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn synonym_set<'a>(&'a self, synonym_set_name: &'a str) -> SynonymSet<'a> {
        SynonymSet::new(self, synonym_set_name)
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
