#[cfg(all(test, not(target_arch = "wasm32")))]
mod http_builder_tls_test;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use typesense::NodeConfig;

/// Verify that the `http_builder` closure is actually invoked when constructing the client.
///
/// Uses an atomic flag as a side-effect observable. This test should also work on WASM
/// since it doesn't depend on TCP or TLS.
async fn test_http_builder_sideeffect() {
    let builder_called = Arc::new(AtomicBool::new(false));
    let client = typesense::Client::builder()
        .nodes(vec![NodeConfig::new("http://localhost:9001").http_builder(
            {
                let builder_called = builder_called.clone();
                move |b| {
                    builder_called.store(true, Ordering::SeqCst);
                    b
                }
            },
        )])
        .api_key("xyz")
        .build()
        .expect("Failed to create Typesense client");

    // call the health endpoint, this will fail (no server), but the builder should have been called
    client.operations().health().await.unwrap_err();

    assert!(builder_called.load(Ordering::SeqCst));
}

/// Verify that per-node http_builder works independently.
async fn test_per_node_http_builder() {
    let node1_called = Arc::new(AtomicBool::new(false));
    let node2_called = Arc::new(AtomicBool::new(false));

    let _client = typesense::Client::builder()
        .nodes(vec![
            NodeConfig::new("http://localhost:9001").http_builder({
                let flag = node1_called.clone();
                move |b| {
                    flag.store(true, Ordering::SeqCst);
                    b
                }
            }),
            NodeConfig::new("http://localhost:9002").http_builder({
                let flag = node2_called.clone();
                move |b| {
                    flag.store(true, Ordering::SeqCst);
                    b
                }
            }),
        ])
        .api_key("xyz")
        .build()
        .expect("Failed to create Typesense client");

    assert!(node1_called.load(Ordering::SeqCst));
    assert!(node2_called.load(Ordering::SeqCst));
}

/// Verify that plain string URLs still work (backward compatibility).
async fn test_plain_string_nodes() {
    let _client = typesense::Client::builder()
        .nodes(vec!["http://localhost:9001"])
        .api_key("xyz")
        .build()
        .expect("Failed to create Typesense client with plain string nodes");
}

/// Verify that mixing NodeConfig and string nodes works via the iterator API.
async fn test_mixed_node_configs() {
    let _client = typesense::Client::builder()
        .nodes(vec![
            NodeConfig::new("http://localhost:9001"),
            NodeConfig::new("http://localhost:9002").http_builder(|b| b),
        ])
        .api_key("xyz")
        .build()
        .expect("Failed to create Typesense client with mixed node configs");
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tokio_test {
    #[tokio::test]
    async fn test_http_builder_sideeffect() {
        super::test_http_builder_sideeffect().await;
    }

    #[tokio::test]
    async fn test_per_node_http_builder() {
        super::test_per_node_http_builder().await;
    }

    #[tokio::test]
    async fn test_plain_string_nodes() {
        super::test_plain_string_nodes().await;
    }

    #[tokio::test]
    async fn test_mixed_node_configs() {
        super::test_mixed_node_configs().await;
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

    #[wasm_bindgen_test]
    async fn test_plain_string_nodes() {
        console_error_panic_hook::set_once();
        super::test_plain_string_nodes().await;
    }
}
