use async_trait::async_trait;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) type HyperClient<C> = hyper::Client<C, hyper::Body>;

#[cfg(all(feature = "tokio-rt", not(target_arch = "wasm32")))]
pub(crate) type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

#[cfg(all(feature = "tokio-rt", not(target_arch = "wasm32")))]
pub(crate) type HyperHttpsClient = HyperClient<HttpsConnector>;

#[cfg(target_arch = "wasm32")]
pub struct WasmClient;

/// A low level HTTP trait.
#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
pub trait HttpLowLevel<M = http::Method, H = http::HeaderMap> {
    /// Send a request and receive a response.
    async fn send(
        &self,
        method: M,
        uri: &str,
        headers: H,
        body: Vec<u8>,
    ) -> crate::Result<http::Response<Vec<u8>>>;
}

/// A low level HTTP trait.
#[cfg(target_arch = "wasm32")]
#[async_trait(?Send)]
pub trait HttpLowLevel<M = http::Method, H = http::HeaderMap> {
    /// Send a request and receive a response.
    async fn send(
        &self,
        method: M,
        uri: &str,
        headers: H,
        body: Vec<u8>,
    ) -> crate::Result<http::Response<Vec<u8>>>;
}

#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
impl<C> HttpLowLevel for HyperClient<C>
where
    C: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
{
    async fn send(
        &self,
        method: http::Method,
        uri: &str,
        headers: http::HeaderMap,
        body: Vec<u8>,
    ) -> crate::Result<http::Response<Vec<u8>>> {
        // Making a builder
        let mut builder = http::Request::builder().method(method).uri(uri);
        // Adding headers
        if let Some(h) = builder.headers_mut() {
            *h = headers;
        }

        // Building it to a request
        let request = builder.body(body.into())?;
        // Sending and waiting for a response
        let response = self.request(request).await?;

        if response.status().is_success() {
            let (parts, body) = response.into_parts();
            let body = hyper::body::to_bytes(body).await?.to_vec();
            let response = http::Response::from_parts(parts, body);

            Ok(response)
        } else {
            return Err(response.status().into());
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[async_trait(?Send)]
impl HttpLowLevel for WasmClient {
    async fn send(
        &self,
        method: http::Method,
        uri: &str,
        headers: http::HeaderMap,
        body: Vec<u8>,
    ) -> crate::Result<http::Response<Vec<u8>>> {
        use js_sys::{Array, ArrayBuffer, Reflect, Uint8Array};
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;

        // Options to configure the request
        let mut opts = web_sys::RequestInit::new();
        // Specifying the method
        opts.method(method.as_str());

        // Pinning the body.
        let body_pinned = std::pin::Pin::new(body);
        if body_pinned.len() > 0 {
            // Creating a JS Typed Array which is a view to into wasm's linear memory.
            // It could be invalidated if the contents is moved, which is why
            // we are using `Pin`.
            // Read more [here](https://docs.rs/js-sys/0.3.51/js_sys/struct.Uint8Array.html#unsafety).
            let uint_8_array = unsafe { Uint8Array::view(&body_pinned) };
            opts.body(Some(&uint_8_array));
        }

        // Setting the request mode
        opts.mode(web_sys::RequestMode::Cors);

        // Making a request
        let request = web_sys::Request::new_with_str_and_init(&uri, &opts)?;

        // Adding headers
        for (name, value) in headers
            .iter()
            .map(|(x, y)| (x.as_str(), y.to_str().unwrap()))
        {
            request.headers().set(name, value)?;
        }

        let scope = WindowOrWorker::new();
        // Fetching the request
        let promise = match scope {
            WindowOrWorker::Window(window) => window.fetch_with_request(&request),
            WindowOrWorker::Worker(worker) => worker.fetch_with_request(&request),
        };

        // Converting a JS Promise to a Rust Future and awaiting
        let res = JsFuture::from(promise).await?;
        debug_assert!(res.is_instance_of::<web_sys::Response>());
        let res: web_sys::Response = res.dyn_into().unwrap();

        // Taking the response body
        let promise_array = res.array_buffer()?;
        let array = JsFuture::from(promise_array).await?;
        debug_assert!(array.is_instance_of::<js_sys::ArrayBuffer>());
        let buf: ArrayBuffer = array.dyn_into().unwrap();
        // Making a uint8 array
        let slice = Uint8Array::new(&buf);
        // Converting it to a Vec
        let body = slice.to_vec();

        // Making a builder
        let mut builder = http::Response::builder().status(res.status());

        // Adding headers
        for i in js_sys::try_iter(&res.headers())?.unwrap() {
            let array: Array = i?.into();
            let values = array.values();

            let prop = String::from("value").into();
            let key = Reflect::get(values.next()?.as_ref(), &prop)?
                .as_string()
                .unwrap();
            let value = Reflect::get(values.next()?.as_ref(), &prop)?
                .as_string()
                .unwrap();
            builder = builder.header(&key, &value);
        }

        // Building it to a response
        let response = builder.body(body)?;

        if response.status().is_success() {
            Ok(response)
        } else {
            return Err(response.status().into());
        }
    }
}

#[cfg(target_arch = "wasm32")]
enum WindowOrWorker {
    Window(web_sys::Window),
    Worker(web_sys::WorkerGlobalScope),
}

#[cfg(target_arch = "wasm32")]
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
