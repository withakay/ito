//! Inline frontend asset handlers.
//!
//! The web UI ships as compile-time–embedded HTML and JavaScript so the binary
//! is fully self-contained with no external asset directory. Each handler
//! returns the corresponding `include_str!` blob with the correct content type.

use axum::{
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
};

/// Serve the root `index.html` page.
pub async fn index() -> Html<&'static str> {
    Html(include_str!("index.html"))
}

/// Serve the client-side JavaScript bundle (`app.js`).
pub async fn app_js() -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/javascript")],
        include_str!("app.js"),
    )
        .into_response()
}
