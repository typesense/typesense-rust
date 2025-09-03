#![cfg(not(target_family = "wasm"))]

use reqwest::Url;
use reqwest_retry::policies::ExponentialBackoff;
use std::time::Duration;
use typesense::models::CollectionResponse;
use typesense::*;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Helper to create a mock Typesense server for a successful collection retrieval.
async fn setup_mock_server_ok(server: &MockServer, collection_name: &str) {
    let response_body = CollectionResponse {
        name: collection_name.to_owned(),
        ..Default::default()
    };

    Mock::given(method("GET"))
        .and(path(format!("/collections/{}", collection_name)))
        .and(header("X-TYPESENSE-API-KEY", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(server)
        .await;
}

// Helper to create a mock Typesense server that returns a server error.
async fn setup_mock_server_503(server: &MockServer, collection_name: &str) {
    Mock::given(method("GET"))
        .and(path(format!("/collections/{}", collection_name)))
        .respond_with(ResponseTemplate::new(503))
        .mount(server)
        .await;
}

// Helper to create a mock Typesense server that returns a 404 Not Found error.
async fn setup_mock_server_404(server: &MockServer, collection_name: &str) {
    Mock::given(method("GET"))
        .and(path(format!("/collections/{}", collection_name)))
        .respond_with(ResponseTemplate::new(404))
        .mount(server)
        .await;
}

// Helper function to create a client configuration for tests.
fn get_client(nodes: Vec<Url>, nearest_node: Option<Url>) -> Client {
    Client::builder()
        .nodes(nodes)
        .maybe_nearest_node(nearest_node)
        .api_key("test-key")
        .healthcheck_interval(Duration::from_secs(60))
        .retry_policy(ExponentialBackoff::builder().build_with_max_retries(0))
        .connection_timeout(Duration::from_secs(1))
        .build()
        .expect("Failed to create client")
}

#[tokio::test]
async fn test_success_on_first_node() {
    let server1 = MockServer::start().await;
    setup_mock_server_ok(&server1, "products").await;

    let client = get_client(vec![Url::parse(&server1.uri()).unwrap()], None);

    let result = client.collection("products").retrieve().await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().name, "products");
    // Check that the server received exactly one request.
    assert_eq!(server1.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn test_failover_to_second_node() {
    let server1 = MockServer::start().await;
    let server2 = MockServer::start().await;
    setup_mock_server_503(&server1, "products").await;
    setup_mock_server_ok(&server2, "products").await;

    let client = get_client(
        vec![
            Url::parse(&server1.uri()).unwrap(),
            Url::parse(&server2.uri()).unwrap(),
        ],
        None,
    );

    let result = client.collection("products").retrieve().await;
    assert!(result.is_ok());

    // The first server should have been tried and failed.
    assert_eq!(server1.received_requests().await.unwrap().len(), 1);
    // The second server should have been tried and succeeded.
    assert_eq!(server2.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn test_nearest_node_is_prioritized() {
    let nearest_server = MockServer::start().await;
    let regular_server = MockServer::start().await;
    setup_mock_server_ok(&nearest_server, "products").await;
    setup_mock_server_ok(&regular_server, "products").await;

    let client = get_client(
        vec![Url::parse(&regular_server.uri()).unwrap()],
        Some(Url::parse(&nearest_server.uri()).unwrap()),
    );

    let result = client.collection("products").retrieve().await;
    assert!(result.is_ok());

    // Only the nearest node should have received a request.
    assert_eq!(nearest_server.received_requests().await.unwrap().len(), 1);
    assert_eq!(regular_server.received_requests().await.unwrap().len(), 0);
}

#[tokio::test]
async fn test_failover_from_nearest_to_regular_node() {
    let nearest_server = MockServer::start().await;
    let regular_server = MockServer::start().await;
    setup_mock_server_503(&nearest_server, "products").await;
    setup_mock_server_ok(&regular_server, "products").await;

    let client = get_client(
        vec![Url::parse(&regular_server.uri()).unwrap()],
        Some(Url::parse(&nearest_server.uri()).unwrap()),
    );

    let result = client.collection("products").retrieve().await;
    assert!(result.is_ok());

    // Nearest node should have failed.
    assert_eq!(nearest_server.received_requests().await.unwrap().len(), 1);
    // Regular node should have succeeded.
    assert_eq!(regular_server.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn test_round_robin_failover() {
    let server1 = MockServer::start().await;
    let server2 = MockServer::start().await;
    let server3 = MockServer::start().await;
    setup_mock_server_503(&server1, "products").await;
    setup_mock_server_503(&server2, "products").await;
    setup_mock_server_ok(&server3, "products").await;

    let client = get_client(
        vec![
            Url::parse(&server1.uri()).unwrap(),
            Url::parse(&server2.uri()).unwrap(),
            Url::parse(&server3.uri()).unwrap(),
        ],
        None,
    );

    // First request should fail over to the third node
    let result = client.collection("products").retrieve().await;
    assert!(result.is_ok());
    assert_eq!(server1.received_requests().await.unwrap().len(), 1);
    assert_eq!(server2.received_requests().await.unwrap().len(), 1);
    assert_eq!(server3.received_requests().await.unwrap().len(), 1);

    // The next request should start from the now-healthy 3rd node, but round-robin
    // logic will have advanced the internal counter. Let's see it wrap around.
    // We expect the next attempt to be on server 3 again, then 1 (if 3 fails).

    // Reset server 3 to also fail
    server3.reset().await;
    setup_mock_server_503(&server3, "products").await;
    // Make server 1 healthy again
    server1.reset().await;
    setup_mock_server_ok(&server1, "products").await;

    let result2 = client.collection("products").retrieve().await;
    assert!(result2.is_ok());

    // Server 3 was tried first and failed.
    assert_eq!(server3.received_requests().await.unwrap().len(), 1);
    // Server 1 was tried next and succeeded.
    assert_eq!(server1.received_requests().await.unwrap().len(), 1);
    // Server 2 was not touched this time.
    assert_eq!(server2.received_requests().await.unwrap().len(), 1); // Remains 1 from first call
}

#[tokio::test]
async fn test_health_check_and_node_recovery() {
    let server1 = MockServer::start().await;
    let server2 = MockServer::start().await;

    setup_mock_server_503(&server1, "products").await;
    setup_mock_server_ok(&server2, "products").await;

    let client = Client::builder()
        .nodes(vec![
            Url::parse(&server1.uri()).unwrap(),
            Url::parse(&server2.uri()).unwrap(),
        ])
        .api_key("test-key")
        .healthcheck_interval(Duration::from_millis(500)) // Use a very short healthcheck interval for the test
        .retry_policy(ExponentialBackoff::builder().build_with_max_retries(0))
        .connection_timeout(Duration::from_secs(1))
        .build()
        .expect("Failed to create client");

    // 1. First request fails over to server2, marking server1 as unhealthy.
    assert!(client.collection("products").retrieve().await.is_ok());
    assert_eq!(server1.received_requests().await.unwrap().len(), 1);
    assert_eq!(server2.received_requests().await.unwrap().len(), 1);

    // 2. Immediate second request should go directly to server2.
    assert!(client.collection("products").retrieve().await.is_ok());
    assert_eq!(server1.received_requests().await.unwrap().len(), 1); // No new request
    assert_eq!(server2.received_requests().await.unwrap().len(), 2); // Got another request

    // 3. Wait for the healthcheck interval to pass.
    tokio::time::sleep(Duration::from_millis(600)).await;

    // 4. Make server1 healthy again.
    server1.reset().await;
    setup_mock_server_ok(&server1, "products").await;

    // 5. The next request should try server1 again (due to healthcheck expiry) and succeed.
    assert!(client.collection("products").retrieve().await.is_ok());
    assert_eq!(server1.received_requests().await.unwrap().len(), 1); // Server 1 received its first successful req
    assert_eq!(server2.received_requests().await.unwrap().len(), 2); // No new request for server 2
}

#[tokio::test]
async fn test_all_nodes_fail() {
    let server1 = MockServer::start().await;
    let server2 = MockServer::start().await;
    setup_mock_server_503(&server1, "products").await;
    setup_mock_server_503(&server2, "products").await;

    let client = get_client(
        vec![
            Url::parse(&server1.uri()).unwrap(),
            Url::parse(&server2.uri()).unwrap(),
        ],
        None,
    );

    let result = client.collection("products").retrieve().await;
    assert!(result.is_err());

    match result.err().unwrap() {
        Error::AllNodesFailed { .. } => { /* This is the expected outcome */ }
        _ => panic!("Expected AllNodesFailed error"),
    }

    // Both servers should have been tried.
    assert_eq!(server1.received_requests().await.unwrap().len(), 1);
    assert_eq!(server2.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn test_fail_fast_on_non_retriable_error() {
    let server1 = MockServer::start().await;
    let server2 = MockServer::start().await;

    setup_mock_server_404(&server1, "products").await;
    setup_mock_server_ok(&server2, "products").await;

    let client = get_client(
        vec![
            Url::parse(&server1.uri()).unwrap(),
            Url::parse(&server2.uri()).unwrap(),
        ],
        None,
    );

    let result = client.collection("products").retrieve().await;
    assert!(result.is_err());

    // Check that the error is the non-retriable API error.
    match result.err().unwrap() {
        Error::Api(typesense_codegen::apis::Error::ResponseError(content)) => {
            assert_eq!(content.status, reqwest::StatusCode::NOT_FOUND);
        }
        e => panic!("Expected an API error, but got {:?}", e),
    }

    // The first server should have been tried.
    assert_eq!(server1.received_requests().await.unwrap().len(), 1);
    // The second server should NOT have been tried.
    assert_eq!(server2.received_requests().await.unwrap().len(), 0);
}

#[tokio::test]
async fn test_load_balancing_with_healthy_nodes() {
    // 1. Setup three healthy mock servers
    let server1 = MockServer::start().await;
    let server2 = MockServer::start().await;
    let server3 = MockServer::start().await;
    setup_mock_server_ok(&server1, "products").await;
    setup_mock_server_ok(&server2, "products").await;
    setup_mock_server_ok(&server3, "products").await;

    // 2. Setup client with the three nodes
    let client = get_client(
        vec![
            Url::parse(&server1.uri()).unwrap(),
            Url::parse(&server2.uri()).unwrap(),
            Url::parse(&server3.uri()).unwrap(),
        ],
        None,
    );

    // 3. Make three consecutive requests
    let result1 = client.collection("products").retrieve().await;
    let result2 = client.collection("products").retrieve().await;
    let result3 = client.collection("products").retrieve().await;

    // 4. Assert all requests were successful
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());

    // 5. Assert that each server received exactly one request, proving round-robin distribution
    assert_eq!(server1.received_requests().await.unwrap().len(), 1);
    assert_eq!(server2.received_requests().await.unwrap().len(), 1);
    assert_eq!(server3.received_requests().await.unwrap().len(), 1);
}
