use super::*;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::thread;

fn runtime_for(base_url: String) -> BackendRuntime {
    BackendRuntime {
        base_url,
        token: "secret".to_string(),
        timeout: Duration::from_secs(1),
        max_retries: 0,
        backup_dir: PathBuf::from("/tmp/ito-backups"),
        org: "withakay".to_string(),
        repo: "ito".to_string(),
    }
}

fn serve_responses(responses: Vec<&'static str>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind test server");
    let addr = listener.local_addr().expect("local addr");
    thread::spawn(move || {
        for response in responses {
            let (mut stream, _) = listener.accept().expect("accept request");
            let mut request = [0_u8; 2048];
            let _ = stream.read(&mut request).expect("read request");
            stream
                .write_all(response.as_bytes())
                .expect("write response");
        }
    });
    format!("http://{addr}")
}

fn json_response(status: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
}

#[test]
fn backend_health_status_default_is_all_false() {
    let status = BackendHealthStatus {
        server_reachable: false,
        server_healthy: false,
        server_ready: false,
        server_version: None,
        ready_reason: None,
        auth_verified: false,
        token_scope: None,
        error: None,
    };

    assert!(!status.server_reachable);
    assert!(!status.server_healthy);
    assert!(!status.server_ready);
    assert!(!status.auth_verified);
    assert!(status.server_version.is_none());
    assert!(status.ready_reason.is_none());
    assert!(status.token_scope.is_none());
    assert!(status.error.is_none());
}

#[test]
fn backend_health_status_serializes_to_json() {
    let status = BackendHealthStatus {
        server_reachable: true,
        server_healthy: true,
        server_ready: true,
        server_version: Some("0.1.0".to_string()),
        ready_reason: None,
        auth_verified: true,
        token_scope: Some("project".to_string()),
        error: None,
    };

    let json = serde_json::to_string(&status).expect("should serialize");
    assert!(json.contains("\"server_reachable\":true"));
    assert!(json.contains("\"server_healthy\":true"));
    assert!(json.contains("\"server_ready\":true"));
    assert!(json.contains("\"server_version\":\"0.1.0\""));
    assert!(json.contains("\"auth_verified\":true"));
    assert!(json.contains("\"token_scope\":\"project\""));
}

#[test]
fn backend_health_status_serializes_error_state() {
    let status = BackendHealthStatus {
        server_reachable: false,
        server_healthy: false,
        server_ready: false,
        server_version: None,
        ready_reason: None,
        auth_verified: false,
        token_scope: None,
        error: Some("Connection refused".to_string()),
    };

    let json = serde_json::to_string(&status).expect("should serialize");
    assert!(json.contains("\"server_reachable\":false"));
    assert!(json.contains("\"error\":\"Connection refused\""));
}

#[test]
fn check_backend_health_success_records_version_ready_and_scope() {
    let base_url = serve_responses(vec![
        Box::leak(json_response("200 OK", r#"{"status":"ok","version":"1.2.3"}"#).into_boxed_str()),
        Box::leak(json_response("200 OK", r#"{"status":"ready","reason":null}"#).into_boxed_str()),
        Box::leak(json_response("200 OK", r#"{"valid":true,"scope":"project"}"#).into_boxed_str()),
    ]);

    let status = check_backend_health(&runtime_for(base_url));

    assert!(status.server_reachable);
    assert!(status.server_healthy);
    assert!(status.server_ready);
    assert!(status.auth_verified);
    assert_eq!(status.server_version.as_deref(), Some("1.2.3"));
    assert_eq!(status.token_scope.as_deref(), Some("project"));
    assert!(status.error.is_none());
}

#[test]
fn check_backend_health_reports_health_status_and_parse_errors() {
    let http_error = check_backend_health(&runtime_for(serve_responses(vec![Box::leak(
        json_response("500 Internal Server Error", "{}").into_boxed_str(),
    )])));
    assert!(http_error.server_reachable);
    assert_eq!(
        http_error.error.as_deref(),
        Some("Health endpoint returned HTTP 500")
    );

    let parse_error = check_backend_health(&runtime_for(serve_responses(vec![Box::leak(
        json_response("200 OK", "not json").into_boxed_str(),
    )])));
    assert!(
        parse_error
            .error
            .as_deref()
            .expect("parse error")
            .starts_with("Failed to parse health response:")
    );
}

#[test]
fn check_backend_health_handles_ready_not_ready_and_unparseable_503() {
    let not_ready = check_backend_health(&runtime_for(serve_responses(vec![
        Box::leak(json_response("200 OK", r#"{"status":"ok","version":"1.2.3"}"#).into_boxed_str()),
        Box::leak(
            json_response(
                "503 Service Unavailable",
                r#"{"status":"starting","reason":"warming"}"#,
            )
            .into_boxed_str(),
        ),
        Box::leak(json_response("200 OK", r#"{"valid":true,"scope":"admin"}"#).into_boxed_str()),
    ])));
    assert!(!not_ready.server_ready);
    assert_eq!(not_ready.ready_reason.as_deref(), Some("warming"));
    assert!(not_ready.auth_verified);

    let unparseable = check_backend_health(&runtime_for(serve_responses(vec![
        Box::leak(json_response("200 OK", r#"{"status":"ok","version":"1.2.3"}"#).into_boxed_str()),
        Box::leak(json_response("503 Service Unavailable", "busy").into_boxed_str()),
        Box::leak(json_response("200 OK", r#"{"valid":true,"scope":"admin"}"#).into_boxed_str()),
    ])));
    assert_eq!(
        unparseable.ready_reason.as_deref(),
        Some("Server returned 503")
    );
}

#[test]
fn check_backend_health_reports_ready_parse_and_status_errors() {
    let parse_error = check_backend_health(&runtime_for(serve_responses(vec![
        Box::leak(json_response("200 OK", r#"{"status":"ok","version":"1.2.3"}"#).into_boxed_str()),
        Box::leak(json_response("200 OK", "not json").into_boxed_str()),
    ])));
    assert!(
        parse_error
            .error
            .as_deref()
            .expect("ready parse error")
            .starts_with("Failed to parse ready response:")
    );

    let status_error = check_backend_health(&runtime_for(serve_responses(vec![
        Box::leak(json_response("200 OK", r#"{"status":"ok","version":"1.2.3"}"#).into_boxed_str()),
        Box::leak(json_response("418 I'm a teapot", "{}").into_boxed_str()),
    ])));
    assert_eq!(
        status_error.error.as_deref(),
        Some("Ready endpoint returned HTTP 418")
    );
}

#[test]
fn check_backend_health_reports_auth_failures_and_tolerates_scope_parse_error() {
    let unauthorized = check_backend_health(&runtime_for(serve_responses(vec![
        Box::leak(json_response("200 OK", r#"{"status":"ok","version":"1.2.3"}"#).into_boxed_str()),
        Box::leak(json_response("200 OK", r#"{"status":"ready","reason":null}"#).into_boxed_str()),
        Box::leak(json_response("401 Unauthorized", "{}").into_boxed_str()),
    ])));
    assert!(!unauthorized.auth_verified);
    assert!(
        unauthorized
            .error
            .as_deref()
            .expect("auth error")
            .starts_with("Authentication failed.")
    );

    let forbidden = check_backend_health(&runtime_for(serve_responses(vec![
        Box::leak(json_response("200 OK", r#"{"status":"ok","version":"1.2.3"}"#).into_boxed_str()),
        Box::leak(json_response("200 OK", r#"{"status":"ready","reason":null}"#).into_boxed_str()),
        Box::leak(json_response("403 Forbidden", "{}").into_boxed_str()),
    ])));
    assert_eq!(
        forbidden.error.as_deref(),
        Some("Organization/repository 'withakay/ito' is not in the server allowlist.")
    );

    let bad_scope = check_backend_health(&runtime_for(serve_responses(vec![
        Box::leak(json_response("200 OK", r#"{"status":"ok","version":"1.2.3"}"#).into_boxed_str()),
        Box::leak(json_response("200 OK", r#"{"status":"ready","reason":null}"#).into_boxed_str()),
        Box::leak(json_response("200 OK", "not json").into_boxed_str()),
    ])));
    assert!(bad_scope.auth_verified);
    assert!(bad_scope.token_scope.is_none());
}
