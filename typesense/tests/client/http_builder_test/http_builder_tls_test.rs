use std::{
    net::{IpAddr, Ipv4Addr},
    sync::Arc,
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt as _},
    net::TcpListener,
};
use tokio_rustls::{
    TlsAcceptor,
    rustls::{
        self, ServerConfig,
        pki_types::{CertificateDer, PrivateKeyDer},
    },
};
use typesense::{ExponentialBackoff, NodeConfig};

/// Exercise the per-node `http_builder` option by setting up a custom root TLS certificate.
///
/// If the customization doesn't work, reqwest would be unable to connect
/// to the mocked Typesense node because the self-signed cert is not trusted
/// by the system root store.
///
/// This test is non-WASM as it needs TCP.
pub(super) async fn test_http_builder_tls() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install crypto provider");

    let api_key = "xxx-api-key";

    let (cert, key) = generate_self_signed_cert();
    let tls_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert.clone()], key)
        .expect("failed to build TLS config");

    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let listener = TcpListener::bind((localhost, 0))
        .await
        .expect("Failed to bind to address");
    let server_addr = listener.local_addr().expect("Failed to get local address");

    let handler = tokio::spawn(mock_node_handler(listener, tls_config, api_key));

    let client_cert = reqwest::Certificate::from_der(&cert)
        .expect("Failed to convert certificate to Certificate");
    let client = typesense::Client::builder()
        .nodes(vec![
            NodeConfig::new(format!("https://localhost:{}", server_addr.port())).http_builder(
                move |builder| builder.add_root_certificate(client_cert).https_only(true),
            ),
        ])
        .api_key(api_key)
        .healthcheck_interval(Duration::from_secs(9001))
        .retry_policy(ExponentialBackoff::builder().build_with_max_retries(0))
        .build()
        .expect("Failed to create Typesense client");

    client
        .operations()
        .health()
        .await
        .expect("Failed to get collection health");

    handler.await.expect("Failed to join handler");
}

fn generate_self_signed_cert() -> (CertificateDer<'static>, PrivateKeyDer<'static>) {
    let pair = rcgen::generate_simple_self_signed(["localhost".into()])
        .expect("Failed to generate self-signed certificate");
    let cert = pair.cert.der().clone();
    let signing_key = pair.signing_key.serialize_der();
    let signing_key = PrivateKeyDer::try_from(signing_key)
        .expect("Failed to convert signing key to PrivateKeyDer");
    (cert, signing_key)
}

async fn mock_node_handler(listener: TcpListener, tls_config: ServerConfig, api_key: &'static str) {
    let tls_acceptor = TlsAcceptor::from(Arc::new(tls_config));
    let (stream, _addr) = listener
        .accept()
        .await
        .expect("Failed to accept connection");
    let mut stream = tls_acceptor
        .accept(stream)
        .await
        .expect("Failed to accept TLS connection");

    let mut buf = vec![0u8; 1024];
    stream
        .read(&mut buf[..])
        .await
        .expect("Failed to read request");
    let request = String::from_utf8(buf).expect("Failed to parse request as UTF-8");
    assert!(request.contains("/health"));
    assert!(request.contains(api_key));

    let response = "HTTP/1.1 200 OK\r\n\
        Content-Type: application/json\r\n\
        Connection: close\r\n\
        \r\n\
        {\"ok\": true}";
    stream
        .write_all(response.as_bytes())
        .await
        .expect("Failed to write to stream");
    stream.shutdown().await.expect("Failed to shutdown stream");
}
