use super::Client;
use crate::transport::Transport;

#[cfg(target_arch = "wasm32")]
use crate::transport::WasmClient;

use crate::{Result, TypesenseError};
/// Builder for the Typesense [`Client`]
pub struct ClientBuilder<'a, T> {
    transport: Option<Transport<T>>,
    host: Option<&'a str>,
    api_key: Option<&'a str>,
}

impl<'a, T> ClientBuilder<'a, T> {
    /// build [`Client`] with the current configurations. Return [`typesense::TypesenseError::ConfigError`]
    /// if a configuration is missing.
    pub fn build(self) -> Result<Client<'a, T>> {
        Ok(Client {
            transport: self.transport.ok_or_else(|| {
                TypesenseError::ConfigError("missing client transport".to_string())
            })?,
            host: self
                .host
                .ok_or_else(|| TypesenseError::ConfigError("missing client host".to_string()))?,
            api_key: self
                .api_key
                .ok_or_else(|| TypesenseError::ConfigError("missing client api key".to_string()))?,
        })
    }

    /// Set host
    pub fn host(mut self, host: &'a str) -> Self {
        self.host = Some(host);
        self
    }

    /// Set api key
    pub fn api_key(mut self, api_key: &'a str) -> Self {
        self.api_key = Some(api_key);
        self
    }

    /// Set transport
    pub fn transport(mut self, transport: Transport<T>) -> Self {
        self.transport = Some(transport);
        self
    }
}

impl<'a, T> Default for ClientBuilder<'a, T> {
    fn default() -> Self {
        Self {
            transport: None,
            host: None,
            api_key: None,
        }
    }
}

#[cfg(all(feature = "tokio-rt", not(target_arch = "wasm32")))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "tokio-rt", not(target_arch = "wasm32"))))
)]
impl<'a> ClientBuilder<'a, crate::transport::HyperHttpsClient> {
    /// Set transport with a new [`hyper`](https://docs.rs/hyper) client.
    /// The connector used is [`HttpsConnector`](hyper_tls::HttpsConnector).
    pub fn with_hyper(mut self) -> Self {
        self.transport = Some(crate::transport::TransportBuilder::new_hyper().build());
        self
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(docsrs, doc(cfg(target_arch = "wasm32")))]
impl<'a> ClientBuilder<'a, WasmClient> {
    /// Set transport using default wasm client
    pub fn with_wasm(mut self) -> Self {
        self.transport = Some(crate::transport::TransportBuilder::new_wasm().build());
        self
    }
}
