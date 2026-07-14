use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use serde_json::Value;
use tower::ServiceExt;

use super::{router, safe_path};

async fn send(app: &Router, request: Request<Body>) -> (StatusCode, String) {
    let response = app.clone().oneshot(request).await.expect("router response");
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("response body");
    (
        status,
        String::from_utf8(body.to_vec()).expect("UTF-8 response body"),
    )
}

fn get(uri: &str) -> Request<Body> {
    Request::builder()
        .uri(uri)
        .body(Body::empty())
        .expect("GET request")
}

#[tokio::test]
async fn list_root_filters_noise_and_sorts_directories_first() {
    let project = tempfile::tempdir().expect("project root");
    std::fs::create_dir(project.path().join("zeta")).expect("zeta directory");
    std::fs::create_dir(project.path().join(".ito")).expect("Ito directory");
    std::fs::create_dir(project.path().join("target")).expect("target directory");
    std::fs::write(project.path().join("Alpha.md"), "alpha").expect("visible file");
    std::fs::write(project.path().join(".secret"), "secret").expect("hidden file");

    let (status, body) = send(&router(project.path().to_path_buf()), get("/list")).await;

    assert_eq!(status, StatusCode::OK);
    let body: Value = serde_json::from_str(&body).expect("listing JSON");
    assert_eq!(body["path"], "");
    let entries = body["entries"].as_array().expect("entries array");
    let names = entries
        .iter()
        .map(|entry| entry["name"].as_str().expect("entry name"))
        .collect::<Vec<_>>();
    assert_eq!(names, [".ito", "zeta", "Alpha.md"]);
    assert_eq!(entries[0]["is_dir"], true);
    assert_eq!(entries[1]["is_dir"], true);
    assert_eq!(entries[2]["is_dir"], false);
    assert_eq!(entries[2]["size"], 5);
}

#[tokio::test]
async fn file_route_reads_detects_language_and_saves_existing_file() {
    let project = tempfile::tempdir().expect("project root");
    std::fs::create_dir(project.path().join("src")).expect("source directory");
    let source = project.path().join("src/lib.rs");
    std::fs::write(&source, "fn before() {}\n").expect("source file");
    let app = router(project.path().to_path_buf());

    let (read_status, read_body) = send(&app, get("/file/src/lib.rs")).await;
    assert_eq!(read_status, StatusCode::OK);
    let read_body: Value = serde_json::from_str(&read_body).expect("file JSON");
    assert_eq!(read_body["path"], "src/lib.rs");
    assert_eq!(read_body["content"], "fn before() {}\n");
    assert_eq!(read_body["language"], "rust");

    let replacement = "fn after() {}\n";
    let save = Request::builder()
        .method("POST")
        .uri("/file/src/lib.rs")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            serde_json::json!({ "content": replacement }).to_string(),
        ))
        .expect("save request");
    let (save_status, save_body) = send(&app, save).await;

    assert_eq!(save_status, StatusCode::OK);
    assert_eq!(
        serde_json::from_str::<Value>(&save_body).expect("save JSON")["ok"],
        true
    );
    assert_eq!(
        std::fs::read_to_string(source).expect("saved source"),
        replacement
    );
}

#[test]
fn safe_path_rejects_parent_traversal_before_filesystem_access() {
    let project = tempfile::tempdir().expect("project root");

    let error = safe_path(project.path(), "../outside.md").expect_err("traversal rejected");

    assert_eq!(error.0, StatusCode::BAD_REQUEST);
    assert_eq!(error.1, "invalid path");
}

#[cfg(unix)]
#[tokio::test]
async fn file_route_forbids_symlinks_that_escape_the_root() {
    let fixture = tempfile::tempdir().expect("fixture root");
    let project = fixture.path().join("project");
    std::fs::create_dir(&project).expect("project directory");
    let outside = fixture.path().join("outside.md");
    std::fs::write(&outside, "outside").expect("outside file");
    std::os::unix::fs::symlink(&outside, project.join("escape.md")).expect("escape symlink");

    let (status, body) = send(&router(project), get("/file/escape.md")).await;

    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, "Access denied");
}

#[tokio::test]
async fn template_routes_list_validate_and_render_embedded_templates() {
    let project = tempfile::tempdir().expect("project root");
    let app = router(project.path().to_path_buf());

    let (list_status, list_body) = send(&app, get("/templates/list")).await;
    assert_eq!(list_status, StatusCode::OK);
    let list_body: Value = serde_json::from_str(&list_body).expect("template list JSON");
    assert!(
        list_body["templates"]
            .as_array()
            .expect("templates array")
            .iter()
            .any(|entry| entry["path"] == "agent/backend.md.j2")
    );

    let (invalid_status, invalid_body) = send(
        &app,
        get("/templates/source?path=../private-template.md.j2"),
    )
    .await;
    assert_eq!(invalid_status, StatusCode::BAD_REQUEST);
    assert_eq!(invalid_body, "invalid template path");

    let render = Request::builder()
        .method("POST")
        .uri("/templates/render?path=agent/backend.md.j2")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from("{}"))
        .expect("render request");
    let (render_status, render_body) = send(&app, render).await;
    assert_eq!(render_status, StatusCode::OK);
    let render_body: Value = serde_json::from_str(&render_body).expect("render JSON");
    assert_eq!(render_body["path"], "agent/backend.md.j2");
    assert!(
        render_body["output"]
            .as_str()
            .expect("rendered output")
            .contains("# Ito Backend Configuration Guide")
    );
}
