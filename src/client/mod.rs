use bytes::Bytes;

use crate::transport::HttpLowLevel;
use crate::transport::Transport;
use crate::Result;

mod builder;
pub use builder::ClientBuilder;

#[allow(dead_code)]
pub const TYPESENSE_API_KEY_HEADER_NAME: &str = "X-TYPESENSE-API-KEY";

/// Root client for top level APIs
pub struct Client<'a, T> {
    transport: Transport<T>,
    host: &'a str,
    api_key: &'a str,
}

impl<'a, T> Client<'a, T> {
    /// Gets the transport of the client
    pub fn transport(&self) -> &Transport<T> {
        &self.transport
    }
}

#[allow(dead_code)]
impl<'a, C> Client<'a, C>
where
    C: HttpLowLevel,
{
    pub(crate) async fn send(
        &self,
        method: http::Method,
        path: &str,
        body: Bytes,
    ) -> Result<C::Response> {
        let uri = format!("{}{}", self.host, path);
        let mut headers = http::HeaderMap::default();
        headers.insert(TYPESENSE_API_KEY_HEADER_NAME, self.api_key.parse().unwrap());
        self.transport.send(method, &uri, headers, body).await
    }

    pub(crate) async fn get(&self, path: &str) -> Result<C::Response> {
        self.send(http::Method::GET, path, Bytes::new()).await
    }

    pub(crate) async fn post(&self, path: &str, body: Bytes) -> Result<C::Response> {
        self.send(http::Method::POST, path, body).await
    }
}

#[cfg(all(test, feature = "tokio-rt", not(target_arch = "wasm32")))]
mod hyper_tests {
    use http::StatusCode;

    use super::*;

    #[tokio::test]
    async fn hyper() -> crate::Result<()> {
        let body = String::from("Test with api key successful");
        let host = "http://localhost:5000";
        let api_key = "VerySecretKey";

        let client = ClientBuilder::new_hyper()
            .host(host)
            .api_key(api_key)
            .build()
            .unwrap();

        let response = client.get("/test_api_key").await?;

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = hyper::body::to_bytes(response).await?;
        assert_eq!(bytes, body.as_bytes());

        let response = client.post("/test_api_key", body.clone().into()).await?;

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = hyper::body::to_bytes(response).await?;
        assert_eq!(bytes, body.as_bytes());

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
        let body = String::from("Test with api key successful");

        let host = "http://localhost:5000";
        let api_key = "VerySecretKey";

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
