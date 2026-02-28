//! REST API endpoint handlers for the backend state API.
//!
//! All handlers receive shared [`AppState`] via axum's `State` extractor.
//! Repositories are constructed per-request since the filesystem-backed
//! implementations are cheap (they only store a path reference).
//!
//! API response types are defined here to decouple the HTTP contract from
//! domain models (which do not derive `Serialize`).

use axum::Router;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use axum::routing::get;
use serde::Serialize;
use std::sync::Arc;

use ito_core::DomainTaskRepository as _;
use ito_core::change_repository::FsChangeRepository;
use ito_core::module_repository::FsModuleRepository;
use ito_core::task_repository::FsTaskRepository;

use crate::error::ApiErrorResponse;
use crate::state::AppState;

// ── Response types ──────────────────────────────────────────────────

/// Health check response.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// Always `"ok"`.
    pub status: String,
    /// API version identifier (crate version).
    pub version: String,
}

/// Token introspection response for bootstrap.
#[derive(Debug, Serialize)]
pub struct WhoamiResponse {
    /// The project ID (directory name) bound to the token.
    pub project_id: String,
    /// The canonical project root path on the server.
    pub project_root: String,
}

/// Readiness check response.
#[derive(Debug, Serialize)]
pub struct ReadyResponse {
    /// Either `"ready"` or `"not_ready"`.
    pub status: String,
    /// Present only when not ready, describing the reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Lightweight change summary returned by list endpoint.
#[derive(Debug, Serialize)]
pub struct ApiChangeSummary {
    /// Change identifier.
    pub id: String,
    /// Module ID if part of a module.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,
    /// Number of completed tasks.
    pub completed_tasks: u32,
    /// Total number of tasks.
    pub total_tasks: u32,
    /// Derived work status label.
    pub work_status: String,
    /// ISO-8601 timestamp of last modification.
    pub last_modified: String,
}

/// Full change detail returned by get endpoint.
#[derive(Debug, Serialize)]
pub struct ApiChange {
    /// Change identifier.
    pub id: String,
    /// Module ID if part of a module.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,
    /// Proposal content (raw markdown).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal: Option<String>,
    /// Design content (raw markdown).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub design: Option<String>,
    /// Spec deltas.
    pub specs: Vec<ApiSpec>,
    /// Task progress summary.
    pub progress: ApiProgress,
    /// ISO-8601 timestamp of last modification.
    pub last_modified: String,
}

/// Specification within a change.
#[derive(Debug, Serialize)]
pub struct ApiSpec {
    /// Spec name (directory name under specs/).
    pub name: String,
    /// Spec content (raw markdown).
    pub content: String,
}

/// Progress information for tasks.
#[derive(Debug, Serialize)]
pub struct ApiProgress {
    /// Total tasks.
    pub total: usize,
    /// Completed tasks.
    pub complete: usize,
    /// Shelved tasks.
    pub shelved: usize,
    /// In-progress tasks.
    pub in_progress: usize,
    /// Pending tasks.
    pub pending: usize,
    /// Remaining work (total - complete - shelved).
    pub remaining: usize,
}

/// Individual task item.
#[derive(Debug, Serialize)]
pub struct ApiTaskItem {
    /// Task identifier.
    pub id: String,
    /// Task name.
    pub name: String,
    /// Wave number (enhanced format only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wave: Option<u32>,
    /// Current status label.
    pub status: String,
}

/// Task list response for a change.
#[derive(Debug, Serialize)]
pub struct ApiTaskList {
    /// Change identifier.
    pub change_id: String,
    /// Individual task items.
    pub tasks: Vec<ApiTaskItem>,
    /// Progress summary.
    pub progress: ApiProgress,
    /// Detected format: `"enhanced"` or `"checkbox"`.
    pub format: String,
}

/// Module summary for listings.
#[derive(Debug, Serialize)]
pub struct ApiModuleSummary {
    /// Module identifier.
    pub id: String,
    /// Module name.
    pub name: String,
    /// Number of changes in this module.
    pub change_count: u32,
}

/// Full module detail.
#[derive(Debug, Serialize)]
pub struct ApiModule {
    /// Module identifier.
    pub id: String,
    /// Module name.
    pub name: String,
    /// Optional description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// ── Conversion helpers ──────────────────────────────────────────────

/// Convert a `DomainResult<T>` to a `Result<T, ApiErrorResponse>`.
///
/// Domain repositories return `DomainResult`; we convert `DomainError` to
/// `CoreError` first (via `From`) to reuse the centralized error mapping.
fn map_domain_err<T>(result: Result<T, ito_core::DomainError>) -> Result<T, ApiErrorResponse> {
    result.map_err(|e| {
        let core_err: ito_core::errors::CoreError = e.into();
        ApiErrorResponse::from(core_err)
    })
}

// ── Handlers ────────────────────────────────────────────────────────

/// `GET /api/v1/health` — returns `{"status": "ok", "version": "..."}`.
///
/// The version is the crate version, which corresponds to the API version.
/// This endpoint is unauthenticated so clients can verify connectivity.
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// `GET /api/v1/auth/whoami` — returns the project identity bound to the token.
///
/// Requires valid bearer token authentication. Returns the project ID and
/// canonical project root so the client can discover its effective scope
/// without knowing the project ID upfront.
pub async fn whoami(State(state): State<Arc<AppState>>) -> Json<WhoamiResponse> {
    let project_id = state
        .project_root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    Json(WhoamiResponse {
        project_id,
        project_root: state.project_root.display().to_string(),
    })
}

/// `GET /api/v1/ready` — checks whether the `.ito/` directory exists.
pub async fn ready(State(state): State<Arc<AppState>>) -> (StatusCode, Json<ReadyResponse>) {
    if state.ito_path.is_dir() {
        (
            StatusCode::OK,
            Json(ReadyResponse {
                status: "ready".to_string(),
                reason: None,
            }),
        )
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ReadyResponse {
                status: "not_ready".to_string(),
                reason: Some(format!(
                    ".ito directory not found at {}",
                    state.project_root.display()
                )),
            }),
        )
    }
}

/// `GET /api/v1/changes` — list all changes as summaries.
pub async fn list_changes(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ApiChangeSummary>>, ApiErrorResponse> {
    let repo = FsChangeRepository::new(&state.ito_path);
    let changes = map_domain_err(repo.list())?;

    let mut summaries: Vec<ApiChangeSummary> = Vec::with_capacity(changes.len());
    for c in changes {
        summaries.push(ApiChangeSummary {
            id: c.id.clone(),
            module_id: c.module_id.clone(),
            completed_tasks: c.completed_tasks,
            total_tasks: c.total_tasks,
            work_status: c.work_status().to_string(),
            last_modified: c.last_modified.to_rfc3339(),
        });
    }

    Ok(Json(summaries))
}

/// `GET /api/v1/changes/{change_id}` — get a full change by ID.
pub async fn get_change(
    State(state): State<Arc<AppState>>,
    Path(change_id): Path<String>,
) -> Result<Json<ApiChange>, ApiErrorResponse> {
    let repo = FsChangeRepository::new(&state.ito_path);
    let change = map_domain_err(repo.get(&change_id))?;
    let api_change = ApiChange {
        id: change.id,
        module_id: change.module_id,
        proposal: change.proposal,
        design: change.design,
        specs: change
            .specs
            .into_iter()
            .map(|s| ApiSpec {
                name: s.name,
                content: s.content,
            })
            .collect(),
        progress: ApiProgress {
            total: change.tasks.progress.total,
            complete: change.tasks.progress.complete,
            shelved: change.tasks.progress.shelved,
            in_progress: change.tasks.progress.in_progress,
            pending: change.tasks.progress.pending,
            remaining: change.tasks.progress.remaining,
        },
        last_modified: change.last_modified.to_rfc3339(),
    };
    Ok(Json(api_change))
}

/// `GET /api/v1/changes/{change_id}/tasks` — get tasks for a change.
pub async fn get_change_tasks(
    State(state): State<Arc<AppState>>,
    Path(change_id): Path<String>,
) -> Result<Json<ApiTaskList>, ApiErrorResponse> {
    let repo = FsTaskRepository::new(&state.ito_path);
    let result = map_domain_err(repo.load_tasks(&change_id))?;
    let format_label = match result.format {
        ito_core::TasksFormat::Enhanced => "enhanced",
        ito_core::TasksFormat::Checkbox => "checkbox",
    };
    let mut tasks: Vec<ApiTaskItem> = Vec::with_capacity(result.tasks.len());
    for t in result.tasks {
        tasks.push(ApiTaskItem {
            id: t.id,
            name: t.name,
            wave: t.wave,
            status: t.status.as_enhanced_label().to_string(),
        });
    }
    let progress = ApiProgress {
        total: result.progress.total,
        complete: result.progress.complete,
        shelved: result.progress.shelved,
        in_progress: result.progress.in_progress,
        pending: result.progress.pending,
        remaining: result.progress.remaining,
    };
    Ok(Json(ApiTaskList {
        change_id,
        tasks,
        progress,
        format: format_label.to_string(),
    }))
}

/// `GET /api/v1/modules` — list all modules as summaries.
pub async fn list_modules(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ApiModuleSummary>>, ApiErrorResponse> {
    let repo = FsModuleRepository::new(&state.ito_path);
    let modules = map_domain_err(repo.list())?;

    let mut summaries: Vec<ApiModuleSummary> = Vec::with_capacity(modules.len());
    for m in modules {
        summaries.push(ApiModuleSummary {
            id: m.id,
            name: m.name,
            change_count: m.change_count,
        });
    }

    Ok(Json(summaries))
}

/// `GET /api/v1/modules/{module_id}` — get a full module by ID.
pub async fn get_module(
    State(state): State<Arc<AppState>>,
    Path(module_id): Path<String>,
) -> Result<Json<ApiModule>, ApiErrorResponse> {
    let repo = FsModuleRepository::new(&state.ito_path);
    let module = map_domain_err(repo.get(&module_id))?;
    Ok(Json(ApiModule {
        id: module.id,
        name: module.name,
        description: module.description,
    }))
}

/// Build the v1 API router with all endpoints.
pub fn v1_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/auth/whoami", get(whoami))
        .route("/changes", get(list_changes))
        .route("/changes/{change_id}", get(get_change))
        .route("/changes/{change_id}/tasks", get(get_change_tasks))
        .route("/modules", get(list_modules))
        .route("/modules/{module_id}", get(get_module))
}
