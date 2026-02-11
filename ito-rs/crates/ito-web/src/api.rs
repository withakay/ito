//! File system API endpoints.

use axum::extract::Query;
use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::path::{Path as StdPath, PathBuf};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    root: PathBuf,
}

/// File or directory entry.
#[derive(Debug, Serialize)]
pub struct Entry {
    name: String,
    path: String,
    is_dir: bool,
    size: Option<u64>,
}

/// Directory listing response.
#[derive(Debug, Serialize)]
pub struct ListResponse {
    path: String,
    entries: Vec<Entry>,
}

/// File content response.
#[derive(Debug, Serialize)]
pub struct FileResponse {
    path: String,
    content: String,
    language: String,
}

/// File save request.
#[derive(Debug, Deserialize)]
pub struct SaveRequest {
    content: String,
}

/// Create the API router.
pub fn router(root: PathBuf) -> Router {
    let state = Arc::new(AppState { root });

    Router::new()
        .route("/list/{*path}", get(list_dir))
        .route("/list", get(list_root))
        .route("/file/{*path}", get(read_file).post(save_file))
        .route("/templates/list", get(list_templates))
        .route("/templates/source", get(get_template_source))
        .route("/templates/render", axum::routing::post(render_template))
        .with_state(state)
}

#[derive(Debug, Serialize)]
pub struct TemplateEntry {
    path: String,
    size: usize,
}

#[derive(Debug, Serialize)]
pub struct TemplateListResponse {
    templates: Vec<TemplateEntry>,
}

#[derive(Debug, Serialize)]
pub struct TemplateSourceResponse {
    path: String,
    content: String,
}

#[derive(Debug, Serialize)]
pub struct TemplateRenderResponse {
    path: String,
    output: String,
}

#[derive(Debug, Deserialize)]
pub struct TemplatePathQuery {
    path: String,
}

fn is_safe_embedded_template_path(path: &str) -> bool {
    if path.is_empty() {
        return false;
    }
    if path.len() > 256 {
        return false;
    }
    if path.starts_with('/') {
        return false;
    }
    if path.contains('\\') {
        return false;
    }
    // Avoid any surprising path traversal patterns even though include_dir
    // won't access the filesystem.
    if path.contains("..") {
        return false;
    }
    true
}

async fn list_templates() -> Result<Json<TemplateListResponse>, (StatusCode, String)> {
    let templates = ito_templates::instructions::list_instruction_templates()
        .into_iter()
        .filter(|p| p.ends_with(".j2"))
        .filter_map(|p| {
            let bytes = ito_templates::instructions::get_instruction_template_bytes(p)?;
            Some(TemplateEntry {
                path: p.to_string(),
                size: bytes.len(),
            })
        })
        .collect();

    Ok(Json(TemplateListResponse { templates }))
}

async fn get_template_source(
    Query(q): Query<TemplatePathQuery>,
) -> Result<Json<TemplateSourceResponse>, (StatusCode, String)> {
    if !is_safe_embedded_template_path(&q.path) {
        return Err((StatusCode::BAD_REQUEST, "invalid template path".to_string()));
    }

    let bytes = ito_templates::instructions::get_instruction_template_bytes(&q.path)
        .ok_or((StatusCode::NOT_FOUND, "template not found".to_string()))?;
    let content = std::str::from_utf8(bytes).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "template is not utf-8".to_string(),
        )
    })?;

    Ok(Json(TemplateSourceResponse {
        path: q.path,
        content: content.to_string(),
    }))
}

async fn render_template(
    Query(q): Query<TemplatePathQuery>,
    Json(ctx): Json<serde_json::Value>,
) -> Result<Json<TemplateRenderResponse>, (StatusCode, String)> {
    if !is_safe_embedded_template_path(&q.path) {
        return Err((StatusCode::BAD_REQUEST, "invalid template path".to_string()));
    }

    let output = ito_templates::instructions::render_instruction_template(&q.path, &ctx)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(TemplateRenderResponse {
        path: q.path,
        output,
    }))
}

/// List root directory.
async fn list_root(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ListResponse>, (StatusCode, String)> {
    list_directory(&state.root, "").await
}

/// List a directory.
async fn list_dir(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Json<ListResponse>, (StatusCode, String)> {
    let full_path = safe_path(&state.root, &path)?;
    list_directory(&full_path, &path).await
}

async fn list_directory(
    dir: &StdPath,
    rel_path: &str,
) -> Result<Json<ListResponse>, (StatusCode, String)> {
    let mut entries = Vec::new();

    let mut read_dir = tokio::fs::read_dir(dir)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, format!("Cannot read directory: {e}")))?;

    while let Some(entry) = read_dir
        .next_entry()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    {
        let name = entry.file_name().to_string_lossy().to_string();

        // Skip hidden files and common unwanted directories
        if name.starts_with('.') && name != ".ito" {
            continue;
        }
        if matches!(
            name.as_str(),
            "node_modules" | "target" | "__pycache__" | ".git"
        ) {
            continue;
        }

        let metadata = entry.metadata().await.ok();
        let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
        let size = if is_dir {
            None
        } else {
            metadata.as_ref().map(|m| m.len())
        };

        let entry_path = if rel_path.is_empty() {
            name.clone()
        } else {
            format!("{}/{}", rel_path, name)
        };

        entries.push(Entry {
            name,
            path: entry_path,
            is_dir,
            size,
        });
    }

    // Sort: directories first, then by name
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(Json(ListResponse {
        path: rel_path.to_string(),
        entries,
    }))
}

/// Read a file.
async fn read_file(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Json<FileResponse>, (StatusCode, String)> {
    let full_path = safe_path(&state.root, &path)?;

    let content = tokio::fs::read_to_string(&full_path)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, format!("Cannot read file: {e}")))?;

    let language = detect_language(&path);

    Ok(Json(FileResponse {
        path,
        content,
        language,
    }))
}

/// Save a file.
async fn save_file(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    Json(body): Json<SaveRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let full_path = safe_path(&state.root, &path)?;

    tokio::fs::write(&full_path, &body.content)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Cannot save file: {e}"),
            )
        })?;

    Ok(Json(serde_json::json!({ "ok": true })))
}

/// Safely resolve a path within the root directory.
fn safe_path(root: &StdPath, path: &str) -> Result<PathBuf, (StatusCode, String)> {
    let path = path.trim_start_matches('/');
    let full = root.join(path);

    // Ensure the path doesn't escape the root via ..
    let canonical = full
        .canonicalize()
        .map_err(|_| (StatusCode::NOT_FOUND, "Path not found".to_string()))?;

    let root_canonical = root.canonicalize().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Root not found".to_string(),
        )
    })?;

    if !canonical.starts_with(&root_canonical) {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    Ok(canonical)
}

/// Detect language from file extension for CodeMirror.
fn detect_language(path: &str) -> String {
    let ext = path.rsplit('.').next().unwrap_or("");
    match ext {
        "rs" => "rust",
        "js" | "mjs" | "cjs" => "javascript",
        "ts" | "mts" | "cts" => "typescript",
        "jsx" => "jsx",
        "tsx" => "tsx",
        "py" => "python",
        "rb" => "ruby",
        "go" => "go",
        "java" => "java",
        "c" | "h" => "c",
        "cpp" | "hpp" | "cc" | "cxx" => "cpp",
        "cs" => "csharp",
        "md" | "markdown" => "markdown",
        "json" => "json",
        "yaml" | "yml" => "yaml",
        "toml" => "toml",
        "html" | "htm" => "html",
        "css" => "css",
        "scss" | "sass" => "scss",
        "sql" => "sql",
        "sh" | "bash" | "zsh" => "shell",
        "xml" => "xml",
        "lua" => "lua",
        "swift" => "swift",
        "kt" | "kts" => "kotlin",
        "dockerfile" => "dockerfile",
        _ => "text",
    }
    .to_string()
}
