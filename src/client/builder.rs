use std::sync::Arc;

use super::Client;
use crate::transport::Transport;

#[cfg(target_arch = "wasm32")]
use crate::transport::WasmClient;

use crate::{Result, TypesenseError};
/// Builder for the Typesense [`Client`]
pub struct ClientBuilder<T> {
    transport: Option<Transport<T>>,
    host: Option<Arc<String>>,
    api_key: Option<Arc<String>>,
}

impl<T> ClientBuilder<T> {
    /// build [`Client`] with the current configurations. Return [`typesense::TypesenseError::ConfigError`]
    /// if a configuration is missing.
    pub fn build(self) -> Result<Client<T>> {
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
    pub fn host(mut self, host: impl AsRef<str>) -> Self {
        self.host = Some(Arc::new(host.as_ref().to_string()));
        self
    }

    /// Set api key
    pub fn api_key(mut self, api_key: impl AsRef<str>) -> Self {
        self.api_key = Some(Arc::new(api_key.as_ref().to_string()));
        self
    }

    /// Set transport
    pub fn transport(mut self, transport: Transport<T>) -> Self {
        self.transport = Some(transport);
        self
    }
}

impl<T> Default for ClientBuilder<T> {
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
impl ClientBuilder<crate::transport::HyperHttpsClient> {
    /// Create client builder with a [`hyper`](https://docs.rs/hyper) client.
    /// The connector used is [`HttpsConnector`](hyper_tls::HttpsConnector).
    pub fn new_hyper() -> Self {
        let transport = Some(crate::transport::TransportBuilder::new_hyper().build());
        Self {
            transport,
            host: None,
            api_key: None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(docsrs, doc(cfg(target_arch = "wasm32")))]
impl ClientBuilder<WasmClient> {
    /// Create client builder using default wasm client
    pub fn new_wasm() -> Self {
        let transport = Some(crate::transport::TransportBuilder::new_wasm().build());
        Self {
            transport,
            host: None,
            api_key: None,
        }
    }
}
