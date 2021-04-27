use super::http_low_level;
use super::Transport;

/// The [`TransportBuilder`] to build [`Transport`].
///
/// Used to build [`Transport`] with custom configuration.
pub struct TransportBuilder<C> {
    client: C,
}

#[cfg(all(feature = "tokio-rt", not(target_arch = "wasm32")))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "tokio-rt", not(target_arch = "wasm32"))))
)]
impl TransportBuilder<http_low_level::HyperHttpsClient> {
    /// Used to make a new [`hyper`](https://docs.rs/hyper) client.
    /// The connector used is [`HttpsConnector`](hyper_tls::HttpsConnector).
    pub fn new_hyper() -> Self {
        let https = http_low_level::HttpsConnector::new();
        let client = hyper::Client::builder().build(https);

        Self { client }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(docsrs, doc(cfg(not(target_arch = "wasm32"))))]
impl<C> TransportBuilder<http_low_level::HyperClient<C>>
where
    C: hyper::client::connect::Connect + Clone,
{
    /// Used to make a new custom [`hyper`](https://docs.rs/hyper) client.
    /// Provide your own executor and connector.
    pub fn new_custom_hyper<E>(executor: E, connector: C) -> Self
    where
        E: hyper::rt::Executor<std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>>
            + Send
            + Sync
            + 'static,
    {
        let client = hyper::Client::builder().executor(executor).build(connector);

        Self { client }
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(docsrs, doc(cfg(target_arch = "wasm32")))]
impl TransportBuilder<http_low_level::WasmClient> {
    /// Used to make a new wasm client.
    pub fn new_wasm() -> Self {
        Self {
            client: http_low_level::WasmClient,
        }
    }
}

impl<C> TransportBuilder<C> {
    /// Make a [`Transport`] struct from the builder.
    pub fn build(self) -> Transport<C> {
        Transport {
            client: self.client,
        }
    }
}
