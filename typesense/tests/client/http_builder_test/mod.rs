#[cfg(all(test, not(target_arch = "wasm32")))]
mod http_builder_tls_test;

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

/// Test that the `http_builder` option can be used to set up a custom DNS resolver.
///
/// In this test we exercise the `http_builder` option by setting a flag when the builder is called.
/// This test should run on WASM as well; due to the constrains of WASM we can't really do a better test than that.
async fn test_http_builder_sideeffect() {
    let builder_called = Arc::new(AtomicBool::new(false));
    let client = typesense::Client::builder()
        .nodes(vec!["http://localhost:9001"]) // does not exist
        .api_key("xyz")
        .http_builder({
            let builder_called = builder_called.clone();
            move || {
                builder_called.store(true, Ordering::SeqCst);
                reqwest::Client::builder()
            }
        })
        .connection_timeout(Duration::from_millis(10))
        .build()
        .expect("Failed to create Typesense client");

    // call the health endpoint, this will fail
    client.operations().health().await.unwrap_err();

    // make sure the builder was called
    assert!(builder_called.load(Ordering::SeqCst));
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tokio_test {
    #[tokio::test]
    async fn test_http_builder_sideeffect() {
        super::test_http_builder_sideeffect().await;
    }

    #[tokio::test]
    async fn test_http_builder_tls() {
        super::http_builder_tls_test::test_http_builder_tls().await;
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_test {
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_http_builder_sideeffect() {
        console_error_panic_hook::set_once();
        super::test_http_builder_sideeffect().await;
    }
}
