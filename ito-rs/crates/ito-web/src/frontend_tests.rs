use super::{app_js, index};
use axum::{
    body::to_bytes,
    http::{StatusCode, header},
    response::IntoResponse,
};

#[tokio::test]
async fn index_returns_the_embedded_application_page() {
    let response = index().await.into_response();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()[header::CONTENT_TYPE],
        "text/html; charset=utf-8"
    );
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8(body.to_vec()).unwrap();
    assert!(body.contains("<title>Ito</title>"));
    assert!(body.contains("/app.js"));
}

#[tokio::test]
async fn app_js_returns_the_embedded_javascript_bundle() {
    let response = app_js().await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()[header::CONTENT_TYPE],
        "application/javascript"
    );
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8(body.to_vec()).unwrap();
    assert!(body.contains("WebSocket"));
    assert!(body.contains("terminal"));
}
