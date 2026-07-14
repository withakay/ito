use super::{AuthState, auth_middleware, generate_token, is_loopback};
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode, header},
    middleware,
    routing::get,
};
use std::sync::Arc;
use tower::ServiceExt;

fn app(token: Option<&str>) -> Router {
    let state = Arc::new(AuthState {
        token: token.map(str::to_owned),
    });

    Router::new()
        .route("/", get(|| async { "accepted" }))
        .layer(middleware::from_fn_with_state(state, auth_middleware))
}

async fn request(app: Router, uri: &str, cookie: Option<&str>) -> axum::response::Response {
    let mut builder = Request::builder().uri(uri);
    if let Some(cookie) = cookie {
        builder = builder.header(header::COOKIE, cookie);
    }
    app.oneshot(builder.body(Body::empty()).unwrap())
        .await
        .unwrap()
}

#[test]
fn generated_tokens_are_stable_project_scoped_hex_values() {
    let base = std::env::temp_dir().join(format!("ito-web-auth-{}", std::process::id()));
    let other = base.with_extension("other");
    std::fs::create_dir_all(&base).unwrap();
    std::fs::create_dir_all(&other).unwrap();

    let token = generate_token(&base);
    assert_eq!(token, generate_token(&base));
    assert_ne!(token, generate_token(&other));
    assert_eq!(token.len(), 32);
    assert!(token.bytes().all(|byte| byte.is_ascii_hexdigit()));

    std::fs::remove_dir_all(base).unwrap();
    std::fs::remove_dir_all(other).unwrap();
}

#[test]
fn generated_token_falls_back_to_the_supplied_nonexistent_path() {
    let missing = std::env::temp_dir().join(format!(
        "ito-web-auth-missing-{}-{}",
        std::process::id(),
        std::thread::current().name().unwrap_or("unnamed")
    ));
    assert!(!missing.exists());
    assert_eq!(generate_token(&missing).len(), 32);
}

#[test]
fn loopback_detection_accepts_only_supported_loopback_spellings() {
    for bind in ["127.0.0.1", "localhost", "::1", "0:0:0:0:0:0:0:1"] {
        assert!(is_loopback(bind), "{bind} should be loopback");
    }
    for bind in ["0.0.0.0", "::", "127.0.0.2", "example.test"] {
        assert!(!is_loopback(bind), "{bind} should require auth");
    }
}

#[tokio::test]
async fn disabled_auth_passes_requests_through() {
    let response = request(app(None), "/", None).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        to_bytes(response.into_body(), usize::MAX).await.unwrap(),
        "accepted"
    );
}

#[tokio::test]
async fn valid_query_token_sets_cookie_and_valid_cookie_is_accepted() {
    let response = request(app(Some("secret")), "/?token=secret", None).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()[header::SET_COOKIE],
        "ito_token=secret; Path=/; HttpOnly; SameSite=Strict; Max-Age=86400"
    );

    let response = request(app(Some("secret")), "/", Some("ito_token=secret")).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(!response.headers().contains_key(header::SET_COOKIE));
}

#[tokio::test]
async fn missing_or_invalid_credentials_return_helpful_forbidden_page() {
    for (uri, cookie) in [
        ("/", None),
        ("/?token=wrong", None),
        ("/", Some("ito_token=wrong")),
    ] {
        let response = request(app(Some("secret")), uri, cookie).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        assert_eq!(response.headers()[header::CONTENT_TYPE], "text/html");
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body = String::from_utf8(body.to_vec()).unwrap();
        assert!(body.contains("Access Denied"));
        assert!(body.contains("?token=secret"));
    }
}
