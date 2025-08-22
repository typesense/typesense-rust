use super::get_client;
use typesense::models::TakeSnapshotParams;

async fn run_test_health_check() {
    let client = get_client();

    let health_result = client.operations().health().await;
    assert!(health_result.is_ok(), "Failed to get health status");
    let health_status = health_result.unwrap();
    assert!(
        matches!(health_status.ok, true | false),
        "The 'ok' field should be a boolean."
    );
}

async fn run_test_debug_info() {
    let client = get_client();

    let debug_result = client.operations().debug().await;
    assert!(debug_result.is_ok(), "Failed to get debug information");
    let debug_info = debug_result.unwrap();

    assert!(
        debug_info.version.is_some(),
        "Debug info should contain a version"
    );
}

async fn run_test_retrieve_metrics() {
    let client = get_client();
    let metrics_result = client.operations().retrieve_metrics().await;

    assert!(metrics_result.is_ok(), "Failed to retrieve metrics");
    let metrics = metrics_result.unwrap();

    assert!(metrics.is_object(), "Metrics should be a JSON object");
    assert!(
        metrics.get("system_memory_used_bytes").is_some(),
        "Expected system_memory_used_bytes in metrics"
    );
    assert!(
        metrics.get("typesense_memory_active_bytes").is_some(),
        "Expected typesense_memory_active_bytes in metrics"
    );
}

async fn run_test_retrieve_api_stats() {
    let client = get_client();
    let stats_result = client.operations().retrieve_api_stats().await;

    assert!(stats_result.is_ok(), "Failed to retrieve API stats");
    let stats = stats_result.unwrap();
    // The maps might be empty if there are no recent requests,
    // so we just check that the call succeeds and returns the correct structure.
    assert!(
        stats.latency_ms.is_some(),
        "Expected latency_ms field in API stats"
    );
    assert!(
        stats.requests_per_second.is_some(),
        "Expected requests_per_second field in API stats"
    );
}

async fn run_test_take_snapshot() {
    let client = get_client();
    // Note: This requires a directory that Typesense can write to.
    // In a typical Docker setup, `/tmp` is a safe choice.
    let params = TakeSnapshotParams {
        snapshot_path: "/tmp/typesense-snapshots-rust-test".to_string(),
    };
    let snapshot_result = client.operations().take_snapshot(params).await;

    assert!(snapshot_result.is_ok(), "Failed to take snapshot");
    assert!(
        snapshot_result.unwrap().success,
        "Snapshot operation should be successful"
    );
}

async fn run_test_vote() {
    let client = get_client();
    let vote_result = client.operations().vote().await;

    assert!(
        matches!(vote_result.unwrap().success, true | false),
        "The 'success' field should be a boolean."
    );
}

async fn run_test_get_schema_changes() {
    let client = get_client();
    let schema_changes_result = client.operations().get_schema_changes().await;

    assert!(
        schema_changes_result.is_ok(),
        "Failed to get schema changes"
    );
    // The result is a Vec, which is sufficient to confirm the endpoint call was successful.
    // The vec can be empty if no schema changes are in progress.
    let _schema_changes = schema_changes_result.unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
mod tokio_test {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        run_test_health_check().await;
    }

    #[tokio::test]
    async fn test_debug_info() {
        run_test_debug_info().await;
    }

    #[tokio::test]
    async fn test_retrieve_metrics() {
        run_test_retrieve_metrics().await;
    }

    #[tokio::test]
    async fn test_retrieve_api_stats() {
        run_test_retrieve_api_stats().await;
    }

    #[tokio::test]
    async fn test_take_snapshot() {
        run_test_take_snapshot().await;
    }

    #[tokio::test]
    async fn test_vote() {
        run_test_vote().await;
    }

    #[tokio::test]
    async fn test_get_schema_changes() {
        run_test_get_schema_changes().await;
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm_test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_health_check() {
        console_error_panic_hook::set_once();
        run_test_health_check().await;
    }

    #[wasm_bindgen_test]
    async fn test_debug_info() {
        console_error_panic_hook::set_once();
        run_test_debug_info().await;
    }

    #[wasm_bindgen_test]
    async fn test_retrieve_metrics() {
        console_error_panic_hook::set_once();
        run_test_retrieve_metrics().await;
    }

    #[wasm_bindgen_test]
    async fn test_retrieve_api_stats() {
        console_error_panic_hook::set_once();
        run_test_retrieve_api_stats().await;
    }

    #[wasm_bindgen_test]
    async fn test_take_snapshot() {
        console_error_panic_hook::set_once();
        run_test_take_snapshot().await;
    }

    #[wasm_bindgen_test]
    async fn test_vote() {
        console_error_panic_hook::set_once();
        run_test_vote().await;
    }

    #[wasm_bindgen_test]
    async fn test_get_schema_changes() {
        console_error_panic_hook::set_once();
        run_test_get_schema_changes().await;
    }
}
