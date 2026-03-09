//! REST API endpoint handlers for the multi-tenant backend state API.
//!
//! All project-scoped handlers receive `{org}` and `{repo}` path parameters
//! and resolve the project's `.ito/` directory via [`AppState::ito_path_for`].
//! Repositories are constructed per-request since the filesystem-backed
//! implementations are cheap (they only store a path reference).
//!
//! Routes are nested under `/api/v1/projects/{org}/{repo}/...`.
//! Health and readiness endpoints remain at `/api/v1/health` and `/api/v1/ready`.

use axum::Router;
use axum::extract::{Extension, Path, Query, State};
use axum::http::StatusCode;
use axum::response::Json;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::auth::TokenScope;
use crate::error::ApiErrorResponse;
use crate::state::AppState;
use ito_core::ChangeLifecycleFilter;

// ── Response types ──────────────────────────────────────────────────

/// Health check response.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// Always `"ok"`.
    pub status: String,
    /// API version identifier (crate version).
    pub version: String,
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

/// Auth verification response.
#[derive(Debug, Serialize)]
pub struct AuthVerifyResponse {
    /// Whether the token is valid (always true if this handler is reached).
    pub valid: bool,
    /// Token scope: `"admin"` or `"project"`.
    pub scope: String,
    /// Organization (present only for project-scoped tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org: Option<String>,
    /// Repository (present only for project-scoped tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<String>,
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
    /// Number of shelved tasks.
    pub shelved_tasks: u32,
    /// Number of in-progress tasks.
    pub in_progress_tasks: u32,
    /// Number of pending tasks.
    pub pending_tasks: u32,
    /// Total number of tasks.
    pub total_tasks: u32,
    /// Whether proposal.md exists.
    pub has_proposal: bool,
    /// Whether design.md exists.
    pub has_design: bool,
    /// Whether specs are present.
    pub has_specs: bool,
    /// Whether tasks.md exists and contains tasks.
    pub has_tasks: bool,
    /// Derived work status label.
    pub work_status: String,
    /// ISO-8601 timestamp of last modification.
    pub last_modified: String,
}

/// Query parameters for change list/get requests.
#[derive(Debug, Deserialize)]
pub struct ChangeQuery {
    /// Lifecycle filter: active, archived, or all.
    pub lifecycle: Option<String>,
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

/// Full task detail used by mutation responses.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTaskDetail {
    /// Task identifier.
    pub id: String,
    /// Task name.
    pub name: String,
    /// Wave number when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wave: Option<u32>,
    /// Current status label.
    pub status: String,
    /// Last updated date when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Explicit task dependencies.
    pub dependencies: Vec<String>,
    /// Referenced files.
    pub files: Vec<String>,
    /// Suggested action text.
    pub action: String,
    /// Verification command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify: Option<String>,
    /// Completion criteria.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done_when: Option<String>,
    /// Task kind label.
    pub kind: String,
    /// 0-based header line index.
    pub header_line_index: usize,
}

/// Raw tasks markdown response.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTaskMarkdown {
    /// Change identifier.
    pub change_id: String,
    /// Raw tasks markdown, when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// Task init response payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTaskInitResult {
    /// Change identifier.
    pub change_id: String,
    /// Tracking path, if filesystem-backed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Whether the tasks artifact already existed.
    pub existed: bool,
    /// Revision marker when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}

/// Task mutation response payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTaskMutationResult {
    /// Change identifier.
    pub change_id: String,
    /// Updated task detail.
    pub task: ApiTaskDetail,
    /// Revision marker when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}

/// Request body for adding a task.
#[derive(Debug, Deserialize)]
pub struct AddTaskRequest {
    /// Task title.
    pub title: String,
    /// Optional target wave.
    pub wave: Option<u32>,
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

fn map_task_mutation_err<T>(
    result: ito_core::TaskMutationServiceResult<T>,
) -> Result<T, ApiErrorResponse> {
    result.map_err(|err| match err {
        ito_core::TaskMutationError::Io { .. } => ApiErrorResponse::internal(err.to_string()),
        ito_core::TaskMutationError::Validation(_) => {
            ApiErrorResponse::bad_request(err.to_string())
        }
        ito_core::TaskMutationError::NotFound(_) => ApiErrorResponse::not_found(err.to_string()),
        ito_core::TaskMutationError::Other(_) => ApiErrorResponse::internal(err.to_string()),
    })
}

fn api_task_detail(task: ito_core::TaskItem) -> ApiTaskDetail {
    ApiTaskDetail {
        id: task.id,
        name: task.name,
        wave: task.wave,
        status: task.status.as_enhanced_label().to_string(),
        updated_at: task.updated_at,
        dependencies: task.dependencies,
        files: task.files,
        action: task.action,
        verify: task.verify,
        done_when: task.done_when,
        kind: match task.kind {
            ito_core::TaskKind::Normal => "normal",
            ito_core::TaskKind::Checkpoint => "checkpoint",
        }
        .to_string(),
        header_line_index: task.header_line_index,
    }
}

fn api_task_init_result(result: ito_core::TaskInitResult) -> ApiTaskInitResult {
    ApiTaskInitResult {
        change_id: result.change_id,
        path: result.path.map(|path| path.display().to_string()),
        existed: result.existed,
        revision: result.revision,
    }
}

fn api_task_mutation_result(result: ito_core::TaskMutationResult) -> ApiTaskMutationResult {
    ApiTaskMutationResult {
        change_id: result.change_id,
        task: api_task_detail(result.task),
        revision: result.revision,
    }
}

// ── Top-level handlers (no org/repo) ────────────────────────────────

/// `GET /api/v1/health` — returns `{"status": "ok", "version": "..."}`.
///
/// Unauthenticated. Clients can verify connectivity and API version.
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// `GET /api/v1/ready` — checks whether the data directory exists.
///
/// Unauthenticated. Returns 200 when the backend data directory is accessible,
/// or 503 with a reason when not.
pub async fn ready(State(state): State<Arc<AppState>>) -> (StatusCode, Json<ReadyResponse>) {
    if state.data_dir.is_dir() {
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
                    "Data directory not found at {}",
                    state.data_dir.display()
                )),
            }),
        )
    }
}

// ── Project-scoped handlers ─────────────────────────────────────────

/// `GET /api/v1/projects/{org}/{repo}/auth/verify` — verify token and return scope.
///
/// The auth middleware has already validated the token before this handler runs.
/// This endpoint returns information about the authenticated token's scope.
pub async fn auth_verify(Extension(scope): Extension<TokenScope>) -> Json<AuthVerifyResponse> {
    match scope {
        TokenScope::Admin => Json(AuthVerifyResponse {
            valid: true,
            scope: "admin".to_string(),
            org: None,
            repo: None,
        }),
        TokenScope::Project { org, repo } => Json(AuthVerifyResponse {
            valid: true,
            scope: "project".to_string(),
            org: Some(org),
            repo: Some(repo),
        }),
    }
}

pub async fn list_changes_with_query(
    State(state): State<Arc<AppState>>,
    Path((org, repo)): Path<(String, String)>,
    Query(query): Query<ChangeQuery>,
) -> Result<Json<Vec<ApiChangeSummary>>, ApiErrorResponse> {
    let filter = parse_lifecycle_filter(query.lifecycle)?;
    let change_repo = map_domain_err(state.store.change_repository(&org, &repo))?;
    let changes = map_domain_err(change_repo.list_with_filter(filter))?;

    let mut summaries: Vec<ApiChangeSummary> = Vec::with_capacity(changes.len());
    for c in changes {
        summaries.push(ApiChangeSummary {
            id: c.id.clone(),
            module_id: c.module_id.clone(),
            completed_tasks: c.completed_tasks,
            shelved_tasks: c.shelved_tasks,
            in_progress_tasks: c.in_progress_tasks,
            pending_tasks: c.pending_tasks,
            total_tasks: c.total_tasks,
            has_proposal: c.has_proposal,
            has_design: c.has_design,
            has_specs: c.has_specs,
            has_tasks: c.has_tasks,
            work_status: c.work_status().to_string(),
            last_modified: c.last_modified.to_rfc3339(),
        });
    }

    Ok(Json(summaries))
}

pub async fn get_change_with_query(
    State(state): State<Arc<AppState>>,
    Path((org, repo, change_id)): Path<(String, String, String)>,
    Query(query): Query<ChangeQuery>,
) -> Result<Json<ApiChange>, ApiErrorResponse> {
    let filter = parse_lifecycle_filter(query.lifecycle)?;
    let change_repo = map_domain_err(state.store.change_repository(&org, &repo))?;
    let change = map_domain_err(change_repo.get_with_filter(&change_id, filter))?;
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

/// `GET /api/v1/projects/{org}/{repo}/changes/{change_id}/tasks` — get tasks for a change.
pub async fn get_change_tasks(
    State(state): State<Arc<AppState>>,
    Path((org, repo, change_id)): Path<(String, String, String)>,
) -> Result<Json<ApiTaskList>, ApiErrorResponse> {
    let task_repo = map_domain_err(state.store.task_repository(&org, &repo))?;
    let result = map_domain_err(task_repo.load_tasks(&change_id))?;
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

/// `GET /api/v1/projects/{org}/{repo}/changes/{change_id}/tasks/raw` — get raw tasks markdown.
pub async fn get_change_tasks_markdown(
    State(state): State<Arc<AppState>>,
    Path((org, repo, change_id)): Path<(String, String, String)>,
) -> Result<Json<ApiTaskMarkdown>, ApiErrorResponse> {
    let task_mutations = map_domain_err(state.store.task_mutation_service(&org, &repo))?;
    let content = map_task_mutation_err(task_mutations.load_tasks_markdown(&change_id))?;
    Ok(Json(ApiTaskMarkdown { change_id, content }))
}

/// `POST /api/v1/projects/{org}/{repo}/changes/{change_id}/tasks/init` — initialize tasks.
pub async fn init_change_tasks(
    State(state): State<Arc<AppState>>,
    Path((org, repo, change_id)): Path<(String, String, String)>,
) -> Result<Json<ApiTaskInitResult>, ApiErrorResponse> {
    let task_mutations = map_domain_err(state.store.task_mutation_service(&org, &repo))?;
    let result = map_task_mutation_err(task_mutations.init_tasks(&change_id))?;
    Ok(Json(api_task_init_result(result)))
}

/// `POST /api/v1/projects/{org}/{repo}/changes/{change_id}/tasks/{task_id}/start` — start a task.
pub async fn start_change_task(
    State(state): State<Arc<AppState>>,
    Path((org, repo, change_id, task_id)): Path<(String, String, String, String)>,
) -> Result<Json<ApiTaskMutationResult>, ApiErrorResponse> {
    let task_mutations = map_domain_err(state.store.task_mutation_service(&org, &repo))?;
    let result = map_task_mutation_err(task_mutations.start_task(&change_id, &task_id))?;
    Ok(Json(api_task_mutation_result(result)))
}

/// `POST /api/v1/projects/{org}/{repo}/changes/{change_id}/tasks/{task_id}/complete` — complete a task.
pub async fn complete_change_task(
    State(state): State<Arc<AppState>>,
    Path((org, repo, change_id, task_id)): Path<(String, String, String, String)>,
) -> Result<Json<ApiTaskMutationResult>, ApiErrorResponse> {
    let task_mutations = map_domain_err(state.store.task_mutation_service(&org, &repo))?;
    let result = map_task_mutation_err(task_mutations.complete_task(&change_id, &task_id, None))?;
    Ok(Json(api_task_mutation_result(result)))
}

/// `POST /api/v1/projects/{org}/{repo}/changes/{change_id}/tasks/{task_id}/shelve` — shelve a task.
pub async fn shelve_change_task(
    State(state): State<Arc<AppState>>,
    Path((org, repo, change_id, task_id)): Path<(String, String, String, String)>,
) -> Result<Json<ApiTaskMutationResult>, ApiErrorResponse> {
    let task_mutations = map_domain_err(state.store.task_mutation_service(&org, &repo))?;
    let result = map_task_mutation_err(task_mutations.shelve_task(&change_id, &task_id, None))?;
    Ok(Json(api_task_mutation_result(result)))
}

/// `POST /api/v1/projects/{org}/{repo}/changes/{change_id}/tasks/{task_id}/unshelve` — unshelve a task.
pub async fn unshelve_change_task(
    State(state): State<Arc<AppState>>,
    Path((org, repo, change_id, task_id)): Path<(String, String, String, String)>,
) -> Result<Json<ApiTaskMutationResult>, ApiErrorResponse> {
    let task_mutations = map_domain_err(state.store.task_mutation_service(&org, &repo))?;
    let result = map_task_mutation_err(task_mutations.unshelve_task(&change_id, &task_id))?;
    Ok(Json(api_task_mutation_result(result)))
}

/// `POST /api/v1/projects/{org}/{repo}/changes/{change_id}/tasks/add` — add a task.
pub async fn add_change_task(
    State(state): State<Arc<AppState>>,
    Path((org, repo, change_id)): Path<(String, String, String)>,
    Json(payload): Json<AddTaskRequest>,
) -> Result<Json<ApiTaskMutationResult>, ApiErrorResponse> {
    let task_mutations = map_domain_err(state.store.task_mutation_service(&org, &repo))?;
    let result = map_task_mutation_err(task_mutations.add_task(
        &change_id,
        &payload.title,
        payload.wave,
    ))?;
    Ok(Json(api_task_mutation_result(result)))
}

/// `GET /api/v1/projects/{org}/{repo}/modules` — list all modules as summaries.
pub async fn list_modules(
    State(state): State<Arc<AppState>>,
    Path((org, repo)): Path<(String, String)>,
) -> Result<Json<Vec<ApiModuleSummary>>, ApiErrorResponse> {
    let module_repo = map_domain_err(state.store.module_repository(&org, &repo))?;
    let modules = map_domain_err(module_repo.list())?;

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

/// `GET /api/v1/projects/{org}/{repo}/modules/{module_id}` — get a full module by ID.
pub async fn get_module(
    State(state): State<Arc<AppState>>,
    Path((org, repo, module_id)): Path<(String, String, String)>,
) -> Result<Json<ApiModule>, ApiErrorResponse> {
    let module_repo = map_domain_err(state.store.module_repository(&org, &repo))?;
    let module = map_domain_err(module_repo.get(&module_id))?;
    Ok(Json(ApiModule {
        id: module.id,
        name: module.name,
        description: module.description,
    }))
}

// ── Event ingest types ──────────────────────────────────────────────

/// Request body for the event ingest endpoint.
#[derive(Debug, Deserialize)]
pub struct IngestEventsRequest {
    /// Batch of audit events to ingest.
    pub events: Vec<ito_core::audit::AuditEvent>,
    /// Client-provided idempotency key for safe retries.
    pub idempotency_key: String,
}

/// Response for a successful event ingest.
#[derive(Debug, Serialize)]
pub struct IngestEventsResponse {
    /// Number of events accepted (new).
    pub accepted: usize,
    /// Number of duplicate events (already seen via this idempotency key).
    pub duplicates: usize,
}

/// `POST /api/v1/projects/{org}/{repo}/events` — ingest a batch of audit events.
///
/// Accepts a JSON body with an array of events and an idempotency key.
/// The server writes new events to the project's audit log and deduplicates
/// by idempotency key.
pub async fn ingest_events(
    State(state): State<Arc<AppState>>,
    Path((org, repo)): Path<(String, String)>,
    Json(payload): Json<IngestEventsRequest>,
) -> Result<Json<IngestEventsResponse>, ApiErrorResponse> {
    if payload.events.is_empty() {
        return Ok(Json(IngestEventsResponse {
            accepted: 0,
            duplicates: 0,
        }));
    }

    if payload.idempotency_key.is_empty() {
        return Err(ApiErrorResponse::bad_request(
            "idempotency_key must not be empty",
        ));
    }

    if payload.idempotency_key.len() > 128
        || !payload
            .idempotency_key
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    {
        return Err(ApiErrorResponse::bad_request(
            "idempotency_key contains invalid characters",
        ));
    }

    let ito_path = state
        .ito_path_for(&org, &repo)
        .map_err(|e| ApiErrorResponse::bad_request(e.to_string()))?;

    // Check idempotency (atomic): create a key file using create_new.
    // This prevents concurrent requests with the same key from double-appending.
    let idem_dir = ito_path.join(".state").join("ingest-keys");
    if let Err(e) = std::fs::create_dir_all(&idem_dir) {
        tracing::error!("failed to create ingest-keys dir: {e}");
        return Err(ApiErrorResponse::internal(
            "Failed to process idempotency key",
        ));
    }

    let idem_file = idem_dir.join(&payload.idempotency_key);

    // Idempotency file format: a single integer (UTF-8) representing how many
    // events from this request have been successfully appended so far.
    let mut idem_handle = match std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(&idem_file)
    {
        Ok(f) => f,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&idem_file)
            .map_err(|e| {
                tracing::error!("failed to open idempotency key file: {e}");
                ApiErrorResponse::internal("Failed to process idempotency key")
            })?,
        Err(e) => {
            tracing::error!("failed to open idempotency key file: {e}");
            return Err(ApiErrorResponse::internal(
                "Failed to process idempotency key",
            ));
        }
    };

    use std::io::{Read, Seek, SeekFrom, Write};

    fn read_idem_count(file: &mut std::fs::File) -> usize {
        let mut buf = String::new();
        if file.seek(SeekFrom::Start(0)).is_err() {
            return 0;
        }
        if file.read_to_string(&mut buf).is_err() {
            return 0;
        }
        buf.trim().parse::<usize>().unwrap_or(0)
    }

    fn write_idem_count(file: &mut std::fs::File, count: usize) -> std::io::Result<()> {
        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;
        file.write_all(count.to_string().as_bytes())?;
        file.flush()
    }

    let total = payload.events.len();
    let already_accepted = read_idem_count(&mut idem_handle).min(total);
    let duplicates = already_accepted;

    if duplicates >= total {
        return Ok(Json(IngestEventsResponse {
            accepted: 0,
            duplicates: total,
        }));
    }

    // Write the remaining events to the audit log.
    let writer = ito_core::audit::FsAuditWriter::new(&ito_path);
    let mut accepted = 0usize;
    let mut progress = duplicates;

    for event in payload.events.iter().skip(duplicates) {
        if let Err(e) = ito_core::audit::AuditWriter::append(&writer, event) {
            tracing::warn!("event ingest write failed: {e}");

            // Best-effort: persist progress so retries can skip the already written prefix.
            let _ = write_idem_count(&mut idem_handle, progress);

            return Err(ApiErrorResponse::internal(
                "Failed to ingest events (write error)",
            ));
        }

        accepted += 1;
        progress += 1;

        // Best-effort: persist progress after each successful append.
        let _ = write_idem_count(&mut idem_handle, progress);
    }

    Ok(Json(IngestEventsResponse {
        accepted,
        duplicates,
    }))
}

// ── Router construction ─────────────────────────────────────────────

/// Build the project-scoped router nested under `/projects/{org}/{repo}`.
///
/// These routes require authentication (enforced by middleware in server.rs).
fn project_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/verify", get(auth_verify))
        .route("/changes", get(list_changes_with_query))
        .route("/changes/{change_id}", get(get_change_with_query))
        .route("/changes/{change_id}/tasks", get(get_change_tasks))
        .route(
            "/changes/{change_id}/tasks/raw",
            get(get_change_tasks_markdown),
        )
        .route("/changes/{change_id}/tasks/init", post(init_change_tasks))
        .route(
            "/changes/{change_id}/tasks/{task_id}/start",
            post(start_change_task),
        )
        .route(
            "/changes/{change_id}/tasks/{task_id}/complete",
            post(complete_change_task),
        )
        .route(
            "/changes/{change_id}/tasks/{task_id}/shelve",
            post(shelve_change_task),
        )
        .route(
            "/changes/{change_id}/tasks/{task_id}/unshelve",
            post(unshelve_change_task),
        )
        .route("/changes/{change_id}/tasks/add", post(add_change_task))
        .route("/modules", get(list_modules))
        .route("/modules/{module_id}", get(get_module))
        .route("/events", post(ingest_events))
}

fn parse_lifecycle_filter(value: Option<String>) -> Result<ChangeLifecycleFilter, ApiErrorResponse> {
    let Some(raw) = value else {
        return Ok(ChangeLifecycleFilter::Active);
    };
    ChangeLifecycleFilter::parse(&raw).ok_or_else(|| {
        ApiErrorResponse::bad_request(format!(
            "Invalid lifecycle '{}'. Expected active, archived, or all.",
            raw
        ))
    })
}

/// Build the v1 API router with all endpoints.
///
/// - `/health` and `/ready` are top-level (unauthenticated).
/// - `/projects/{org}/{repo}/...` hosts all project-scoped routes.
pub fn v1_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .nest("/projects/{org}/{repo}", project_router())
}
