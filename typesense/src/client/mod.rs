// in src/client/mod.rs

// Make the sub-modules public within the client module
pub mod collections;
pub mod documents;

// Re-export the namespace structs for easier access
pub use collections::Collections;
pub use documents::Documents;

use reqwest::Url;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use typesense_codegen::apis::Error; // Use the generated Error type

pub mod collections;
// Public configuration for the user
pub struct MultiNodeConfiguration {
    pub nodes: Vec<Url>,
    pub api_key: String,
    pub retry_policy: ExponentialBackoff,
    pub connection_timeout: Duration,
}

// The main public client
pub struct Client {
    config: MultiNodeConfiguration,
    http_client: ClientWithMiddleware, // The client that handles retries on a single node
    current_node_index: AtomicUsize,
}
// In the `impl Client` block
impl Client {
    pub fn new(config: MultiNodeConfiguration) -> Self {
        let http_client = ClientBuilder::new(
            reqwest::Client::builder()
                .timeout(config.connection_timeout)
                .build()
                .expect("Failed to build reqwest client"),
        )
        // The retry middleware will handle transient errors for a SINGLE request
        .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(config.retry_policy.clone()))
        .build();

        Self {
            config,
            http_client,
            current_node_index: AtomicUsize::new(0),
        }
    }

    // Simple round-robin node selection for trying the next node on failure
    fn get_next_node(&self) -> &Url {
        let index = self.current_node_index.fetch_add(1, Ordering::Relaxed);
        &self.config.nodes[index % self.config.nodes.len()]
    }

    /// A generic POST request handler.
    /// It tries each node in sequence if the previous one fails with a retriable error.
    async fn post<T, U, E>(
        &self,
        path: &str,
        body: &T,
        // We accept optional query params now
        query_params: Option<&[(&str, String)]>,
    ) -> Result<U, Error<E>>
    where
        T: serde::Serialize + ?Sized,
        U: for<'de> serde::Deserialize<'de>,
        E: for<'de> serde::Deserialize<'de> + std::fmt::Debug,
    {
        self.execute_request(reqwest::Method::POST, path, Some(body), query_params).await
    }

    // You would create similar `get`, `delete`, and `patch` helpers
    async fn get<U, E>(&self, path: &str, query_params: Option<&[(&str, String)]>) -> Result<U, Error<E>>
    where
        U: for<'de> serde::Deserialize<'de>,
        E: for<'de> serde::Deserialize<'de> + std::fmt::Debug,
    {
        self.execute_request::<serde_json::Value, U, E>(reqwest::Method::GET, path, None, query_params).await
    }

    /// The single, generic request executor containing all the logic.
    async fn execute_request<T, U, E>(&self, method: reqwest::Method, path: &str, body: Option<&T>, query_params: Option<&[(&str, String)]>) -> Result<U, Error<E>>
    where
        T: serde::Serialize + ?Sized,
        U: for<'de> serde::Deserialize<'de>,
        E: for<'de> serde::Deserialize<'de> + std::fmt::Debug,
    {
        let mut last_error: Option<Error<E>> = None;

        for _ in 0..self.config.nodes.len() {
            let node_url = self.get_next_node();
            let full_url = format!("{}{}", node_url.as_str().trim_end_matches('/'), path);

            let mut request_builder = self.http_client.request(method.clone(), &full_url).header("X-TYPESENSE-API-KEY", &self.config.api_key);

            if let Some(body) = body {
                request_builder = request_builder.json(body);
            }

            if let Some(params) = query_params {
                request_builder = request_builder.query(params);
            }

            match request_builder.send().await {
                Ok(response) => {
                    // If the request was successful, parse the response and return.
                    return Self::handle_response(response).await;
                }
                Err(e) => {
                    // This error is from the reqwest-middleware layer, likely a connection
                    // error or because all retries on this single node were exhausted.
                    // We'll log it and try the next node.
                    eprintln!("Request to node {} failed: {}. Trying next node.", node_url, e);
                    last_error = Some(Error::Middleware(e));
                }
            }
        }
        // If all nodes have been tried and failed, return the last error.
        Err(last_error.expect("No nodes were available to try"))
    }

    /// Generic response handler adapted from the generated code.
    /// This parses a success response or a typed error response.
    async fn handle_response<U, E>(resp: reqwest::Response) -> Result<U, Error<E>>
    where
        U: for<'de> serde::Deserialize<'de>,
        E: for<'de> serde::Deserialize<'de>,
    {
        let status = resp.status();
        let content = resp.text().await.map_err(Error::Reqwest)?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::Serde)
        } else {
            let entity: Option<E> = serde_json::from_str(&content).ok();
            let error = typesense_codegen::apis::ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }
}
