//! # Typesense asynchronous client.
use serde::{de::DeserializeOwned, Serialize};

use crate::request::HttpMethod;
use crate::Result;
use std::time::Duration;

/// An asynchronous Typesense client.
pub struct Client<'a> {
    _api_key: &'a str,
    _connection_timeout: Duration,
    _node: &'a str,
}

impl<'a> Client<'a> {
    /// Make a request to the Typesense API.
    async fn _make_request<T: Serialize, Output: DeserializeOwned>(
        &self,
        method: HttpMethod<T>,
        endpoint: &str,
    ) -> Result<Output> {
        let _url = format!("{}{}", self._node, endpoint);
        match &method {
            HttpMethod::Get => todo!(),
            HttpMethod::Post(_payload) => todo!(),
            HttpMethod::Delete => todo!(),
            HttpMethod::Put(_payload) => todo!(),
        }
    }
}
/// Builder for the Typesense `Client`.
///
/// This type can be used to construct an instance of `Client` through a
/// builder-like pattern.
pub struct ClientBuilder<'a> {
    api_key: &'a str,
    connection_timeout: Duration,
    node: &'a str,
}

impl<'a> Default for ClientBuilder<'a> {
    fn default() -> Self {
        Self {
            api_key: "",
            connection_timeout: Duration::from_secs(3),
            node: "",
        }
    }
}

impl<'a> ClientBuilder<'a> {
    /// Set API Key.
    pub fn api_key(mut self, key: &'a str) -> Self {
        self.api_key = key;
        self
    }
    /// Create `Client` instance
    pub fn build(self) -> Client<'a> {
        Client {
            _api_key: self.api_key,
            _connection_timeout: self.connection_timeout,
            _node: self.node,
        }
    }

    /// Set connection timeout.
    ///
    /// Default value is 3 seconds.
    pub fn connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;

        self
    }

    /// Set the node used by the client. Node must be in the format {protocol}://{host}:{port}/{path}
    pub fn node(mut self, node: &'a str) -> Self {
        self.node = node;

        self
    }
}
