//! The module containing the [`Transport`] struct and
//! its [`Builder`](TransportBuilder).

mod builder;
mod http_low_level;

pub use builder::TransportBuilder;
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
impl Default for Transport<http_low_level::HyperHttpsClient> {
    fn default() -> Self {
        TransportBuilder::new_hyper().build()
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

#[cfg(all(test, feature = "tokio-rt", not(target_arch = "wasm32")))]
mod hyper_tests {
    use http::Method as HttpMethod;
    use http::{HeaderMap, StatusCode};

    use super::*;

    #[tokio::test]
    async fn hyper() -> crate::Result<()> {
        let body = String::from("Test Successful");

        let url = "http://localhost:5000";
        let mut header = HeaderMap::new();
        header.insert("Test", "test".parse().unwrap());

        let transport = TransportBuilder::new_hyper().build();

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
        let body = String::from("Test Successful");

        let url = "http://localhost:5000";
        let mut header = HeaderMap::new();
        header.insert("Test", "test".parse().unwrap());

        let transport = TransportBuilder::new_wasm().build();

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
