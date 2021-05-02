//! The module containing the [`Transport`] struct and
//! its [`Builder`](TransportBuilder).

mod builder;
mod http_low_level;

pub use builder::TransportBuilder;
pub use http_low_level::HttpLowLevel;

/// The [`Transport`] struct.
///
/// It handles the low level HTTP client.
pub struct Transport<C> {
    client: C,
}

#[cfg(feature = "tokio_rt")]
impl Default for Transport<http_low_level::HyperHttpsClient> {
    fn default() -> Self {
        TransportBuilder::new_hyper().build()
    }
}

impl<C> Transport<C>
where
    C: HttpLowLevel,
{
    /// Make a request that will be accepted by
    /// [`send`](Self::send) function.
    pub fn make_request(
        &self,
        method: C::Method,
        uri: &str,
        headers: C::HeaderMap,
        body: C::Body,
    ) -> C::Request {
        C::make_request(method, uri, headers, body)
    }

    /// Send a request and receive a response.
    pub async fn send(&self, request: C::Request) -> C::Response {
        self.client.send(request).await
    }
}

#[cfg(test)]
mod tests {
    use http::Method as HttpMethod;
    use http::{HeaderMap, StatusCode};
    use httpmock::Method as MockMethod;
    use httpmock::MockServer;

    use super::*;

    #[tokio::test]
    async fn hyper() {
        let body = String::from("Test Successful");
        let server = MockServer::start();

        let get = server.mock(|when, then| {
            when.method(MockMethod::GET)
                .path("/")
                .header("Test", "test");
            then.status(200)
                .header("Content-Type", "text/html")
                .body(body.clone());
        });

        let post = server.mock(|when, then| {
            when.method(MockMethod::POST)
                .path("/")
                .header("Test", "test")
                .body(body.clone());
            then.status(200)
                .header("Content-Type", "text/html")
                .body(body.clone());
        });

        let url = server.url("/");
        let mut header = HeaderMap::new();
        header.insert("Test", "test".parse().unwrap());

        let transport = TransportBuilder::new_hyper().build();

        let request = transport.make_request(HttpMethod::GET, &url, header.clone(), vec![].into());
        let response = transport.send(request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = hyper::body::to_bytes(response).await.unwrap();

        assert_eq!(bytes, body.as_bytes());
        get.assert();

        let request = transport.make_request(HttpMethod::POST, &url, header.clone(), body.clone().into());
        let response = transport.send(request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = hyper::body::to_bytes(response).await.unwrap();

        assert_eq!(bytes, body.as_bytes());
        post.assert();
    }
}
