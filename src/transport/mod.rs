use std::pin::Pin;

use hyper::{body, Body, Request, Response};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

mod build;

pub use build::TransportBuilder;

use build::{HttpsConnector, HyperClient, WasmClient};

pub struct Transport<C> {
    client: C,
}

impl Transport<HyperClient<HttpsConnector>> {
    pub fn new() -> Self {
        TransportBuilder::new_hyper().build()
    }
}

impl<C> Transport<HyperClient<C>>
where
    C: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
{
    pub async fn send(
        &self,
        method: hyper::Method,
        uri: &str,
        headers: hyper::HeaderMap,
        body: Body,
    ) -> Response<Body> {
        let mut request_builder = Request::builder().method(method).uri(uri);
        request_builder.headers_mut().map(|h| *h = headers);
        let request = request_builder.body(body).unwrap();

        self.client.request(request).await.unwrap()
    }
}

impl Transport<WasmClient> {
    pub async fn send(
        &self,
        method: hyper::Method,
        uri: &str,
        headers: hyper::HeaderMap,
        body: Body,
    ) -> Response<Body> {
        let mut opts = web_sys::RequestInit::new();
        opts.method(method.as_str());

        let body_bytes = body::to_bytes(body).await.unwrap();
        let body_pinned = Pin::new(body_bytes);
        if body_pinned.len() > 0 {
            let uint_8_array = unsafe { js_sys::Uint8Array::view(&body_pinned) };
            opts.body(Some(&uint_8_array));
        }

        let request = web_sys::Request::new_with_str_and_init(&uri, &opts).unwrap();

        for (name, value) in headers.iter().map(|(x, y)| (x.as_str(), x.as_str())) {
            request.headers().set(name, value).unwrap();
        }

        let scope = WindowOrWorker::new();
        let promise = match scope {
            WindowOrWorker::Window(window) => window.fetch_with_request(&request),
            WindowOrWorker::Worker(worker) => worker.fetch_with_request(&request),
        };

        let res = JsFuture::from(promise).await.unwrap();

        let res: web_sys::Response = res.dyn_into().unwrap();

        let promise_array = res.array_buffer().unwrap();
        let array = JsFuture::from(promise_array).await.unwrap();
        let buf: js_sys::ArrayBuffer = array.dyn_into().unwrap();
        let slice = js_sys::Uint8Array::new(&buf);
        let body = slice.to_vec();

        let mut response = Response::builder().status(res.status());

        for mut i in js_sys::try_iter(&res.headers()).unwrap() {
            let array: js_sys::Array = i.next().unwrap().unwrap().into();
            let values = array.values();

            let prop = String::from("value").into();
            let key = js_sys::Reflect::get(&values.next().unwrap(), &prop)
                .unwrap()
                .as_string()
                .unwrap();
            let value = js_sys::Reflect::get(&values.next().unwrap(), &prop)
                .unwrap()
                .as_string()
                .unwrap();
            response = response.header(&key, &value);
        }

        response.body(body.into()).unwrap()
    }
}

enum WindowOrWorker {
    Window(web_sys::Window),
    Worker(web_sys::WorkerGlobalScope),
}

impl WindowOrWorker {
    fn new() -> Self {
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

#[tokio::test]
async fn test() {
    use hyper::{Body, HeaderMap, Method};

    let transport = Transport::new();
    let response = transport
        .send(
            Method::GET,
            "https://www.rust-lang.org",
            HeaderMap::new(),
            Body::empty(),
        )
        .await;

    println!("{:?}", response);
}
