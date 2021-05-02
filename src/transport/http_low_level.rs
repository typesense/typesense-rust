use std::pin::Pin;

use async_trait::async_trait;

#[cfg(feature = "hyper")]
pub(crate) type HyperClient<C> = hyper::Client<C, hyper::Body>;

#[cfg(feature = "tokio_rt")]
pub(crate) type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

#[cfg(feature = "tokio_rt")]
pub(crate) type HyperHttpsClient = HyperClient<HttpsConnector>;

#[cfg(feature = "wasm")]
pub(crate) struct WasmClient;

/// A low level HTTP trait.
#[async_trait(?Send)]
pub trait HttpLowLevel {
    /// HTTP Method type.
    type Method;

    /// HTTP Header map type.
    type HeaderMap;

    /// HTTP Body type.
    type Body;

    /// HTTP Request type.
    type Request;

    /// HTTP Response type.
    type Response;

    /// Return a request which is accepted by the client.
    fn make_request(
        method: Self::Method,
        uri: &str,
        headers: Self::HeaderMap,
        body: Self::Body,
    ) -> Self::Request;

    /// Send a request and receive a response.
    async fn send(&self, request: Self::Request) -> Self::Response;
}

#[cfg(feature = "hyper")]
#[async_trait(?Send)]
impl<C> HttpLowLevel for HyperClient<C>
where
    C: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
{
    type Method = http::Method;
    type HeaderMap = http::HeaderMap;
    type Body = Vec<u8>;
    type Request = http::Request<hyper::Body>;
    type Response = http::Response<hyper::Body>;

    fn make_request(
        method: Self::Method,
        uri: &str,
        headers: Self::HeaderMap,
        body: Self::Body,
    ) -> Self::Request {
        let mut builder = http::Request::builder().method(method).uri(uri);
        if let Some(h) = builder.headers_mut() {
            *h = headers;
        }

        builder.body(body.into()).unwrap()
    }

    async fn send(&self, request: Self::Request) -> Self::Response {
        self.request(request).await.unwrap()
    }
}

#[cfg(feature = "wasm")]
#[async_trait(?Send)]
impl HttpLowLevel for WasmClient {
    type Method = http::Method;
    type HeaderMap = http::HeaderMap;
    type Body = Vec<u8>;
    type Request = web_sys::Request;
    type Response = http::Response<Self::Body>;

    fn make_request(
        method: Self::Method,
        uri: &str,
        headers: Self::HeaderMap,
        body: Self::Body,
    ) -> Self::Request {
        let mut opts = web_sys::RequestInit::new();
        opts.method(method.as_str());

        let body_pinned = Pin::new(body);
        if body_pinned.len() > 0 {
            let uint_8_array = unsafe { js_sys::Uint8Array::view(&body_pinned) };
            opts.body(Some(&uint_8_array));
        }

        let request = web_sys::Request::new_with_str_and_init(&uri, &opts).unwrap();

        for (name, value) in headers
            .iter()
            .map(|(x, y)| (x.as_str(), y.to_str().unwrap()))
        {
            request.headers().set(name, value).unwrap();
        }

        request
    }

    async fn send(&self, request: Self::Request) -> Self::Response {
        use js_sys::{Array, ArrayBuffer, Reflect, Uint8Array};
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;

        let scope = WindowOrWorker::new();
        let promise = match scope {
            WindowOrWorker::Window(window) => window.fetch_with_request(&request),
            WindowOrWorker::Worker(worker) => worker.fetch_with_request(&request),
        };

        let res = JsFuture::from(promise).await.unwrap();

        let res: web_sys::Response = res.dyn_into().unwrap();

        let promise_array = res.array_buffer().unwrap();
        let array = JsFuture::from(promise_array).await.unwrap();
        let buf: ArrayBuffer = array.dyn_into().unwrap();
        let slice = Uint8Array::new(&buf);
        let body = slice.to_vec();

        let mut response = http::Response::builder().status(res.status());

        while let Some(mut i) = js_sys::try_iter(&res.headers()).unwrap() {
            let array: Array = i.next().unwrap().unwrap().into();
            let values = array.values();

            let prop = String::from("value").into();
            let key = Reflect::get(&values.next().unwrap(), &prop)
                .unwrap()
                .as_string()
                .unwrap();
            let value = Reflect::get(&values.next().unwrap(), &prop)
                .unwrap()
                .as_string()
                .unwrap();
            response = response.header(&key, &value);
        }

        response.body(body).unwrap()
    }
}

#[cfg(feature = "wasm")]
enum WindowOrWorker {
    Window(web_sys::Window),
    Worker(web_sys::WorkerGlobalScope),
}

#[cfg(feature = "wasm")]
impl WindowOrWorker {
    fn new() -> Self {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        #[wasm_bindgen]
        extern "C" {
            type Global;

            #[wasm_bindgen(method, getter, js_name = Window)]
            fn window(this: &Global) -> wasm_bindgen::JsValue;

            #[wasm_bindgen(method, getter, js_name = WorkerGlobalScope)]
            fn worker(this: &Global) -> wasm_bindgen::JsValue;
        }

        let global: Global = js_sys::global().unchecked_into();

        if !global.window().is_undefined() {
            Self::Window(global.unchecked_into())
        } else if !global.worker().is_undefined() {
            Self::Worker(global.unchecked_into())
        } else {
            panic!("Only supported in a browser or web worker");
        }
    }
}
