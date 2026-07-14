use std::net::TcpListener;
use std::path::PathBuf;

use super::{ServeConfig, serve};

#[test]
fn serve_config_defaults_to_local_project_server() {
    let ServeConfig { root, bind, port } = ServeConfig::default();

    assert_eq!(root, PathBuf::from("."));
    assert_eq!(bind, "127.0.0.1");
    assert_eq!(port, 9009);
}

#[tokio::test]
async fn serve_rejects_invalid_bind_address_after_router_construction() {
    let error = serve(ServeConfig {
        root: PathBuf::from("path-that-does-not-need-to-exist"),
        bind: "not-an-ip-address".to_string(),
        port: 9009,
    })
    .await
    .expect_err("invalid bind address must fail");

    assert!(
        error.to_string().contains("Invalid address"),
        "unexpected error: {error}"
    );
}

#[tokio::test]
async fn serve_reports_bind_failure_for_occupied_local_port() {
    let occupied = TcpListener::bind("127.0.0.1:0").expect("reserve local port");
    let address = occupied.local_addr().expect("reserved address");

    let error = serve(ServeConfig {
        root: PathBuf::from("."),
        bind: address.ip().to_string(),
        port: address.port(),
    })
    .await
    .expect_err("occupied port must fail");

    assert!(
        error
            .to_string()
            .contains(&format!("Failed to bind to {address}")),
        "unexpected error: {error}"
    );
}
