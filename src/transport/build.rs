use std::future::Future;
use std::pin::Pin;

use super::Transport;

pub(super) type HyperClient<C> = hyper::Client<C, hyper::Body>;
pub(super) type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

pub(super) struct WasmClient;

pub struct TransportBuilder<C> {
    client: C,
}

impl TransportBuilder<HyperClient<HttpsConnector>> {
    pub fn new_hyper() -> Self {
        let mut https = HttpsConnector::new();
        https.https_only(true);
        let client = hyper::Client::builder().build(https);

        Self { client }
    }
}

impl<C> TransportBuilder<HyperClient<C>>
where
    C: hyper::client::connect::Connect + Clone,
{
    pub fn new_custom_hyper<E>(executor: E, connector: C) -> Self
    where
        E: hyper::rt::Executor<Pin<Box<dyn Future<Output = ()> + Send>>> + Send + Sync + 'static,
    {
        let client = hyper::Client::builder().executor(executor).build(connector);

        Self { client }
    }
}

impl TransportBuilder<WasmClient> {
    pub fn new_wasm() -> Self {
        Self { client: WasmClient }
    }
}

impl<C> TransportBuilder<C> {
    pub fn build(self) -> Transport<C> {
        Transport {
            client: self.client,
        }
    }
}
