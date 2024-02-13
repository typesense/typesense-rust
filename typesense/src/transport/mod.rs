//! The module containing the [`Transport`] struct and
//! its [`Builder`](TransportBuilder).

mod http_low_level;

pub use http_low_level::HttpLowLevel;

#[cfg(target_arch = "wasm32")]
pub(crate) use http_low_level::WasmClient;

#[cfg(all(feature = "tokio-rt", not(target_arch = "wasm32")))]
pub(crate) use http_low_level::HyperHttpsClient;

/// The [`Transport`] struct.
///
/// It handles the low level HTTP client.
#[derive(Clone)]
pub struct Transport<C> {
    client: C,
}

#[cfg(all(feature = "tokio-rt", not(target_arch = "wasm32")))]
impl Default for Transport<HyperHttpsClient> {
    fn default() -> Self {
        Transport::new()
    }
}

impl<C> Transport<C>
where
    C: HttpLowLevel,
{
    /// Send a request and receive a response.
    pub async fn send(
        &self,
        method: http::Method,
        uri: &str,
        headers: http::HeaderMap,
        body: Vec<u8>,
    ) -> crate::Result<http::Response<Vec<u8>>> {
        self.client.send(method, uri, headers, body).await
    }
}

#[allow(missing_docs)]
pub trait TransportCreator {
    /// Create new Transport
    fn new() -> Self;
}

#[cfg(all(feature = "tokio-rt", not(target_arch = "wasm32")))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "tokio-rt", not(target_arch = "wasm32"))))
)]
impl TransportCreator for Transport<HyperHttpsClient> {
    /// Used to make a new [`hyper`](https://docs.rs/hyper) client.
    /// The connector used is [`HttpsConnector`](hyper_tls::HttpsConnector).
    fn new() -> Self {
        let https = http_low_level::HttpsConnector::new();
        let client = hyper::Client::builder().build(https);

        Self { client }
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(docsrs, doc(cfg(target_arch = "wasm32")))]
impl TransportCreator for Transport<WasmClient> {
    /// Used to make a new wasm client.
    pub fn new() -> Self {
        Self {
            client: http_low_level::WasmClient,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(docsrs, doc(cfg(not(target_arch = "wasm32"))))]
impl<C> Transport<http_low_level::HyperClient<C>>
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

#[cfg(all(test, feature = "tokio-rt", not(target_arch = "wasm32")))]
mod hyper_tests {
    use http::Method as HttpMethod;
    use http::{HeaderMap, StatusCode};

    use super::*;

    #[tokio::test]
    async fn hyper() -> crate::Result<()> {
        let _ = dotenvy::dotenv();

        let url = std::env::var("URL").expect("URL must be present in .env");

        let body = String::from("Test Successful");
        let mut header = HeaderMap::new();
        header.insert("Test", "test".parse().unwrap());

        let transport = Transport::new();

        let response = transport
            .send(HttpMethod::GET, &url, header.clone(), vec![].into())
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.into_body(), body.as_bytes());

        let response = transport
            .send(HttpMethod::POST, &url, header.clone(), body.clone().into())
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.into_body(), body.as_bytes());

        Ok(())
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_test {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use http::Method as HttpMethod;
    use http::{HeaderMap, StatusCode};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        fn log(s: &str);
    }

    macro_rules! console_log {
        ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
    }

    #[wasm_bindgen_test]
    async fn wasm() {
        console_error_panic_hook::set_once();

        console_log!("Test Started.");
        match try_wasm().await {
            Ok(_) => console_log!("Test Successful."),
            Err(e) => console_log!("Test failed: {:?}", e),
        }
    }

    async fn try_wasm() -> crate::Result<()> {
        let url = "http://localhost:5000";

        let body = String::from("Test Successful");

        let mut header = HeaderMap::new();
        header.insert("Test", "test".parse().unwrap());

        let transport = Transport::new();

        let response = transport
            .send(HttpMethod::GET, &url, header.clone(), vec![].into())
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.body(), body.as_bytes());

        let response = transport
            .send(HttpMethod::POST, &url, header.clone(), body.clone().into())
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.body(), body.as_bytes());

        Ok(())
    }
}
