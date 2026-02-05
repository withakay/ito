//! Frontend HTML/CSS/JS served inline.

use axum::{
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
};

/// Serve the main application.
pub async fn index() -> Html<&'static str> {
    Html(include_str!("index.html"))
}

/// Serve the JavaScript application.
pub async fn app_js() -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/javascript")],
        include_str!("app.js"),
    )
        .into_response()
}
