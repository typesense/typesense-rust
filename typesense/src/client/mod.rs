use std::sync::Arc;

use http::Response;

use crate::collection::CollectionClient;
use crate::transport::HttpLowLevel;
use crate::transport::Transport;
use crate::Result;

#[cfg(target_arch = "wasm32")]
use crate::transport::WasmClient;

pub mod keys;

pub use keys::ClientKeys;

pub const TYPESENSE_API_KEY_HEADER_NAME: &str = "X-TYPESENSE-API-KEY";

/// Root client for top level APIs
#[derive(Clone)]
pub struct Client<T> {
    transport: Transport<T>,
    host: Arc<String>,
    api_key: Arc<String>,
}

impl<T> Client<T> {
    /// Gets the transport of the client
    pub fn transport(&self) -> &Transport<T> {
        &self.transport
    }
}

#[cfg(all(feature = "tokio-rt", not(target_arch = "wasm32")))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "tokio-rt", not(target_arch = "wasm32"))))
)]
impl Client<crate::transport::HyperHttpsClient> {
    /// Create client builder with a [`hyper`](https://docs.rs/hyper) client.
    /// The connector used is [`HttpsConnector`](hyper_tls::HttpsConnector).
    pub fn new_hyper(host: String, api_key: String) -> Self {
        let transport = crate::transport::TransportBuilder::new_hyper().build();
        Self {
            transport,
            host: Arc::new(host),
            api_key: Arc::new(api_key),
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(docsrs, doc(cfg(target_arch = "wasm32")))]
impl Client<WasmClient> {
    /// Create client builder using default wasm client
    pub fn new_wasm(host: String, api_key: String) -> Self {
        let transport = crate::transport::TransportBuilder::new_wasm().build();
        Self {
            transport,
            host: Arc::new(host),
            api_key: Arc::new(api_key),
        }
    }
}

impl<T> Client<T>
where
    T: Clone,
{
    /// Make the ClientKeys struct, to interact with the Keys API.
    pub fn keys(&self) -> ClientKeys<T> {
        ClientKeys {
            client: self.clone(),
        }
    }
    /// Creates a [`CollectionClient`] to interact with the Typesense Collection API
    pub fn collection(&self) -> CollectionClient<T> {
        CollectionClient {
            client: self.clone(),
        }
    }
}

#[allow(dead_code)]
impl<C> Client<C>
where
    C: HttpLowLevel,
{
    pub(crate) async fn send(
        &self,
        method: http::Method,
        path: &str,
        body: Vec<u8>,
    ) -> Result<Response<Vec<u8>>> {
        let uri = format!("{}{}", self.host, path);
        let mut headers = http::HeaderMap::default();
        headers.insert(TYPESENSE_API_KEY_HEADER_NAME, self.api_key.parse().unwrap());
        self.transport.send(method, &uri, headers, body).await
    }

    pub(crate) async fn get(&self, path: &str) -> Result<Response<Vec<u8>>> {
        self.send(http::Method::GET, path, Vec::new()).await
    }

    pub(crate) async fn post(&self, path: &str, body: Vec<u8>) -> Result<Response<Vec<u8>>> {
        self.send(http::Method::POST, path, body).await
    }

    pub(crate) async fn delete(&self, path: &str) -> Result<Response<Vec<u8>>> {
        self.send(http::Method::DELETE, path, Vec::new()).await
    }
}

#[cfg(all(test, feature = "tokio-rt", not(target_arch = "wasm32")))]
mod hyper_tests {
    use http::StatusCode;

    use super::*;

    #[tokio::test]
    async fn hyper() -> crate::Result<()> {
        dotenvy::dotenv().unwrap();

        let host = std::env::var("HOST").expect("HOST must be present in .env");
        let api_key = std::env::var("API_KEY").expect("API_KEY must be present in .env");

        let body = String::from("Test with api key successful");

        let client = Client::new_hyper(host, api_key);

        let response = client.get("/test_api_key").await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.into_body(), body.as_bytes());

        let response = client.post("/test_api_key", body.clone().into()).await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.into_body(), body.as_bytes());

        Ok(())
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_test {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use http::StatusCode;
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
        dotenvy::dotenv().unwrap();

        let host = std::env::var("HOST").expect("HOST must be present in .env");
        let api_key = std::env::var("API_KEY").expect("API_KEY must be present in .env");

        let body = String::from("Test with api key successful");
        let client = ClientBuilder::new_wasm()
            .host(host)
            .api_key(api_key)
            .build()
            .unwrap();

        let response = client.get("/test_api_key").await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.body(), body.as_bytes());

        let response = client.post("/test_api_key", body.clone().into()).await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.body(), body.as_bytes());

        Ok(())
    }
}
