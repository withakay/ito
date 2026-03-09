//! HTTP client for backend repository reads and task mutations.

use std::collections::BTreeSet;
use std::io::{Error as IoError, ErrorKind};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::backend_client::{BackendRuntime, is_retriable_status};
use ito_domain::backend::{
    ArchiveResult, ArtifactBundle, BackendArchiveClient, BackendChangeReader, BackendModuleReader,
    BackendSpecReader, BackendSyncClient, PushResult,
};
use ito_domain::changes::{Change, ChangeLifecycleFilter, ChangeSummary, Spec};
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::modules::{Module, ModuleSummary};
use ito_domain::specs::{SpecDocument, SpecSummary};
use ito_domain::tasks::{
    DiagnosticLevel, ProgressInfo, TaskDiagnostic, TaskInitResult, TaskItem, TaskKind,
    TaskMutationError, TaskMutationResult, TaskMutationService, TaskMutationServiceResult,
    TaskStatus, TasksFormat, TasksParseResult, WaveInfo,
};

/// Backend HTTP client shared across repository adapters.
#[derive(Debug, Clone)]
pub struct BackendHttpClient {
    inner: Arc<BackendHttpClientInner>,
}

#[derive(Debug)]
struct BackendHttpClientInner {
    runtime: BackendRuntime,
    agent: ureq::Agent,
}

impl BackendHttpClient {
    /// Create a backend HTTP client from a resolved runtime.
    pub fn new(runtime: BackendRuntime) -> Self {
        let agent = ureq::Agent::config_builder()
            .timeout_global(Some(runtime.timeout))
            .http_status_as_error(false)
            .build()
            .into();
        Self {
            inner: Arc::new(BackendHttpClientInner { runtime, agent }),
        }
    }

    pub(crate) fn load_tasks_parse_result(
        &self,
        change_id: &str,
    ) -> DomainResult<TasksParseResult> {
        let url = format!(
            "{}/changes/{change_id}/tasks",
            self.inner.runtime.project_api_prefix()
        );
        let list: ApiTaskList = self.get_json(&url, "task", Some(change_id))?;
        Ok(task_list_to_parse_result(list))
    }

    fn get_json<T: DeserializeOwned>(
        &self,
        url: &str,
        entity: &'static str,
        id: Option<&str>,
    ) -> DomainResult<T> {
        let response = self.request_with_retry("GET", url, None)?;
        let status = response.status().as_u16();
        let body = read_response_body(response)?;
        if status != 200 {
            return Err(map_status_to_domain_error(status, entity, id, &body));
        }
        serde_json::from_str(&body)
            .map_err(|err| DomainError::io("parsing backend response", IoError::other(err)))
    }

    fn task_get_json<T: DeserializeOwned>(&self, url: &str) -> TaskMutationServiceResult<T> {
        let response = self
            .request_with_retry("GET", url, None)
            .map_err(task_error_from_domain)?;
        parse_task_response(response)
    }

    fn task_post_json<T: DeserializeOwned>(
        &self,
        url: &str,
        body: Option<&str>,
    ) -> TaskMutationServiceResult<T> {
        let response = self
            .request_with_retry("POST", url, body)
            .map_err(task_error_from_domain)?;
        parse_task_response(response)
    }

    fn backend_get_json<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T, ito_domain::backend::BackendError> {
        let response = self
            .request_with_retry("GET", url, None)
            .map_err(backend_error_from_domain)?;
        parse_backend_response(response)
    }

    fn backend_post_json<T: DeserializeOwned>(
        &self,
        url: &str,
        body: Option<&str>,
    ) -> Result<T, ito_domain::backend::BackendError> {
        let response = self
            .request_with_retry("POST", url, body)
            .map_err(backend_error_from_domain)?;
        parse_backend_response(response)
    }

    fn request_with_retry(
        &self,
        method: &str,
        url: &str,
        body: Option<&str>,
    ) -> DomainResult<ureq::http::Response<ureq::Body>> {
        let max_retries = self.inner.runtime.max_retries;
        let mut attempt = 0u32;
        loop {
            let response: Result<ureq::http::Response<ureq::Body>, ureq::Error> = match method {
                "GET" => self
                    .inner
                    .agent
                    .get(url)
                    .header(
                        "Authorization",
                        &format!("Bearer {}", self.inner.runtime.token),
                    )
                    .call(),
                "POST" => {
                    // In ureq v3, POST always uses send() — use empty string when no body.
                    let payload = body.unwrap_or("{}");
                    self.inner
                        .agent
                        .post(url)
                        .header(
                            "Authorization",
                            &format!("Bearer {}", self.inner.runtime.token),
                        )
                        .header("Content-Type", "application/json")
                        .send(payload)
                }
                _ => unreachable!("unsupported backend http method"),
            };

            match response {
                Ok(resp) => {
                    let status: u16 = resp.status().as_u16();
                    if is_retriable_status(status) && attempt < max_retries {
                        attempt += 1;
                        sleep_backoff(attempt);
                        continue;
                    }
                    return Ok(resp);
                }
                Err(err) => {
                    if attempt < max_retries {
                        attempt += 1;
                        sleep_backoff(attempt);
                        continue;
                    }
                    return Err(DomainError::io(
                        "backend request",
                        IoError::other(err.to_string()),
                    ));
                }
            }
        }
    }
}

impl BackendChangeReader for BackendHttpClient {
    fn list_changes(&self, filter: ChangeLifecycleFilter) -> DomainResult<Vec<ChangeSummary>> {
        let url = format!(
            "{}/changes?lifecycle={}",
            self.inner.runtime.project_api_prefix(),
            filter.as_str()
        );
        let summaries: Vec<ApiChangeSummary> = self.get_json(&url, "change", None)?;
        let mut out = Vec::with_capacity(summaries.len());
        for summary in summaries {
            let last_modified = parse_timestamp(&summary.last_modified)?;
            out.push(ChangeSummary {
                id: summary.id,
                module_id: summary.module_id,
                completed_tasks: summary.completed_tasks,
                shelved_tasks: summary.shelved_tasks,
                in_progress_tasks: summary.in_progress_tasks,
                pending_tasks: summary.pending_tasks,
                total_tasks: summary.total_tasks,
                last_modified,
                has_proposal: summary.has_proposal,
                has_design: summary.has_design,
                has_specs: summary.has_specs,
                has_tasks: summary.has_tasks,
            });
        }
        Ok(out)
    }

    fn get_change(&self, change_id: &str, filter: ChangeLifecycleFilter) -> DomainResult<Change> {
        let url = format!(
            "{}/changes/{change_id}?lifecycle={}",
            self.inner.runtime.project_api_prefix(),
            filter.as_str()
        );
        let change: ApiChange = self.get_json(&url, "change", Some(change_id))?;
        let tasks = match self.load_tasks_parse_result(change_id) {
            Ok(tasks) => tasks,
            Err(err) => {
                if filter.includes_archived() {
                    tasks_from_progress(&change.progress)
                } else {
                    return Err(err);
                }
            }
        };
        let last_modified = parse_timestamp(&change.last_modified)?;
        Ok(Change {
            id: change.id,
            module_id: change.module_id,
            path: PathBuf::new(),
            proposal: change.proposal,
            design: change.design,
            specs: change
                .specs
                .into_iter()
                .map(|spec| Spec {
                    name: spec.name,
                    content: spec.content,
                })
                .collect(),
            tasks,
            last_modified,
        })
    }
}

impl BackendModuleReader for BackendHttpClient {
    fn list_modules(&self) -> DomainResult<Vec<ModuleSummary>> {
        let url = format!("{}/modules", self.inner.runtime.project_api_prefix());
        let modules: Vec<ApiModuleSummary> = self.get_json(&url, "module", None)?;
        Ok(modules
            .into_iter()
            .map(|m| ModuleSummary {
                id: m.id,
                name: m.name,
                change_count: m.change_count,
            })
            .collect())
    }

    fn get_module(&self, module_id: &str) -> DomainResult<Module> {
        let url = format!(
            "{}/modules/{module_id}",
            self.inner.runtime.project_api_prefix()
        );
        let module: ApiModule = self.get_json(&url, "module", Some(module_id))?;
        Ok(Module {
            id: module.id,
            name: module.name,
            description: module.description,
            path: PathBuf::new(),
        })
    }
}

impl BackendSpecReader for BackendHttpClient {
    fn list_specs(&self) -> DomainResult<Vec<SpecSummary>> {
        let url = format!("{}/specs", self.inner.runtime.project_api_prefix());
        let specs: Vec<ApiSpecSummary> = self.get_json(&url, "spec", None)?;
        Ok(specs
            .into_iter()
            .map(|spec| SpecSummary {
                id: spec.id,
                path: PathBuf::from(spec.path),
                last_modified: parse_timestamp(&spec.last_modified).unwrap_or_else(|_| Utc::now()),
            })
            .collect())
    }

    fn get_spec(&self, spec_id: &str) -> DomainResult<SpecDocument> {
        let url = format!(
            "{}/specs/{spec_id}",
            self.inner.runtime.project_api_prefix()
        );
        let spec: ApiSpecDocument = self.get_json(&url, "spec", Some(spec_id))?;
        Ok(SpecDocument {
            id: spec.id,
            path: PathBuf::from(spec.path),
            markdown: spec.markdown,
            last_modified: parse_timestamp(&spec.last_modified).unwrap_or_else(|_| Utc::now()),
        })
    }
}

impl TaskMutationService for BackendHttpClient {
    fn load_tasks_markdown(&self, change_id: &str) -> TaskMutationServiceResult<Option<String>> {
        let url = format!(
            "{}/changes/{change_id}/tasks/raw",
            self.inner.runtime.project_api_prefix()
        );
        let response: ApiTaskMarkdown = self.task_get_json(&url)?;
        Ok(response.content)
    }

    fn init_tasks(&self, change_id: &str) -> TaskMutationServiceResult<TaskInitResult> {
        let url = format!(
            "{}/changes/{change_id}/tasks/init",
            self.inner.runtime.project_api_prefix()
        );
        let response: ApiTaskInitResult = self.task_post_json(&url, Some("{}"))?;
        Ok(TaskInitResult {
            change_id: response.change_id,
            path: response.path.map(PathBuf::from),
            existed: response.existed,
            revision: response.revision,
        })
    }

    fn start_task(
        &self,
        change_id: &str,
        task_id: &str,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        let url = format!(
            "{}/changes/{change_id}/tasks/{task_id}/start",
            self.inner.runtime.project_api_prefix()
        );
        let response: ApiTaskMutationEnvelope = self.task_post_json(&url, Some("{}"))?;
        Ok(task_mutation_from_api(response))
    }

    fn complete_task(
        &self,
        change_id: &str,
        task_id: &str,
        _note: Option<String>,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        let url = format!(
            "{}/changes/{change_id}/tasks/{task_id}/complete",
            self.inner.runtime.project_api_prefix()
        );
        let response: ApiTaskMutationEnvelope = self.task_post_json(&url, Some("{}"))?;
        Ok(task_mutation_from_api(response))
    }

    fn shelve_task(
        &self,
        change_id: &str,
        task_id: &str,
        _reason: Option<String>,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        let url = format!(
            "{}/changes/{change_id}/tasks/{task_id}/shelve",
            self.inner.runtime.project_api_prefix()
        );
        let response: ApiTaskMutationEnvelope = self.task_post_json(&url, Some("{}"))?;
        Ok(task_mutation_from_api(response))
    }

    fn unshelve_task(
        &self,
        change_id: &str,
        task_id: &str,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        let url = format!(
            "{}/changes/{change_id}/tasks/{task_id}/unshelve",
            self.inner.runtime.project_api_prefix()
        );
        let response: ApiTaskMutationEnvelope = self.task_post_json(&url, Some("{}"))?;
        Ok(task_mutation_from_api(response))
    }

    fn add_task(
        &self,
        change_id: &str,
        title: &str,
        wave: Option<u32>,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        let url = format!(
            "{}/changes/{change_id}/tasks/add",
            self.inner.runtime.project_api_prefix()
        );
        let body = serde_json::json!({ "title": title, "wave": wave }).to_string();
        let response: ApiTaskMutationEnvelope = self.task_post_json(&url, Some(&body))?;
        Ok(task_mutation_from_api(response))
    }
}

impl BackendSyncClient for BackendHttpClient {
    fn pull(&self, change_id: &str) -> Result<ArtifactBundle, ito_domain::backend::BackendError> {
        let url = format!(
            "{}/changes/{change_id}/sync",
            self.inner.runtime.project_api_prefix()
        );
        self.backend_get_json(&url)
    }

    fn push(
        &self,
        change_id: &str,
        bundle: &ArtifactBundle,
    ) -> Result<PushResult, ito_domain::backend::BackendError> {
        let url = format!(
            "{}/changes/{change_id}/sync",
            self.inner.runtime.project_api_prefix()
        );
        let body = serde_json::to_string(bundle)
            .map_err(|err| ito_domain::backend::BackendError::Other(err.to_string()))?;
        self.backend_post_json(&url, Some(&body))
    }
}

impl BackendArchiveClient for BackendHttpClient {
    fn mark_archived(
        &self,
        change_id: &str,
    ) -> Result<ArchiveResult, ito_domain::backend::BackendError> {
        let url = format!(
            "{}/changes/{change_id}/archive",
            self.inner.runtime.project_api_prefix()
        );
        self.backend_post_json(&url, Some("{}"))
    }
}

fn read_response_body(response: ureq::http::Response<ureq::Body>) -> DomainResult<String> {
    let body = response
        .into_body()
        .read_to_string()
        .map_err(|err| DomainError::io("reading backend response", IoError::other(err)))?;
    Ok(body)
}

fn parse_task_response<T: DeserializeOwned>(
    response: ureq::http::Response<ureq::Body>,
) -> TaskMutationServiceResult<T> {
    let status = response.status().as_u16();
    let body = response
        .into_body()
        .read_to_string()
        .map_err(|err| TaskMutationError::io("reading backend response", IoError::other(err)))?;
    if !(200..300).contains(&status) {
        return Err(map_status_to_task_error(status, &body));
    }
    serde_json::from_str(&body)
        .map_err(|err| TaskMutationError::other(format!("Failed to parse backend response: {err}")))
}

fn parse_backend_response<T: DeserializeOwned>(
    response: ureq::http::Response<ureq::Body>,
) -> Result<T, ito_domain::backend::BackendError> {
    let status = response.status().as_u16();
    let body = response
        .into_body()
        .read_to_string()
        .map_err(|err| ito_domain::backend::BackendError::Other(err.to_string()))?;
    if !(200..300).contains(&status) {
        return Err(map_status_to_backend_error(status, &body));
    }
    serde_json::from_str(&body)
        .map_err(|err| ito_domain::backend::BackendError::Other(err.to_string()))
}

fn map_status_to_domain_error(
    status: u16,
    entity: &'static str,
    id: Option<&str>,
    body: &str,
) -> DomainError {
    if status == 404 {
        return DomainError::not_found(entity, id.unwrap_or("unknown"));
    }

    let kind = if status == 401 || status == 403 {
        ErrorKind::PermissionDenied
    } else if status >= 500 {
        ErrorKind::Other
    } else {
        ErrorKind::InvalidData
    };

    let msg = if body.trim().is_empty() {
        format!("backend returned HTTP {status}")
    } else {
        format!("backend returned HTTP {status}: {body}")
    };
    DomainError::io("backend request", IoError::new(kind, msg))
}

fn map_status_to_task_error(status: u16, body: &str) -> TaskMutationError {
    if let Ok(api_error) = serde_json::from_str::<ApiErrorBody>(body) {
        return match api_error.code.as_str() {
            "not_found" => TaskMutationError::not_found(api_error.error),
            "bad_request" => TaskMutationError::validation(api_error.error),
            _ => TaskMutationError::other(api_error.error),
        };
    }

    let message = if body.trim().is_empty() {
        format!("backend returned HTTP {status}")
    } else {
        format!("backend returned HTTP {status}: {body}")
    };
    match status {
        404 => TaskMutationError::not_found(message),
        400..=499 => TaskMutationError::validation(message),
        _ => TaskMutationError::other(message),
    }
}

fn map_status_to_backend_error(status: u16, body: &str) -> ito_domain::backend::BackendError {
    let message = if body.trim().is_empty() {
        format!("backend returned HTTP {status}")
    } else {
        body.to_string()
    };
    match status {
        401 | 403 => ito_domain::backend::BackendError::Unauthorized(message),
        404 => ito_domain::backend::BackendError::NotFound(message),
        409 => ito_domain::backend::BackendError::Other(message),
        500..=599 => ito_domain::backend::BackendError::Unavailable(message),
        _ => ito_domain::backend::BackendError::Other(message),
    }
}

fn task_error_from_domain(err: DomainError) -> TaskMutationError {
    match err {
        DomainError::Io { context, source } => TaskMutationError::io(context, source),
        DomainError::NotFound { entity, id } => {
            TaskMutationError::not_found(format!("{entity} not found: {id}"))
        }
        DomainError::AmbiguousTarget {
            entity,
            input,
            matches,
        } => TaskMutationError::validation(format!(
            "Ambiguous {entity} target '{input}'. Matches: {matches}"
        )),
    }
}

fn backend_error_from_domain(err: DomainError) -> ito_domain::backend::BackendError {
    match err {
        DomainError::Io { source, .. } => {
            ito_domain::backend::BackendError::Other(source.to_string())
        }
        DomainError::NotFound { entity, id } => {
            ito_domain::backend::BackendError::NotFound(format!("{entity} not found: {id}"))
        }
        DomainError::AmbiguousTarget {
            entity,
            input,
            matches,
        } => ito_domain::backend::BackendError::Other(format!(
            "Ambiguous {entity} target '{input}'. Matches: {matches}"
        )),
    }
}

fn parse_timestamp(raw: &str) -> DomainResult<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(raw)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|err| DomainError::io("parsing backend timestamp", IoError::other(err)))
}

fn sleep_backoff(attempt: u32) {
    let delay_ms = 150u64.saturating_mul(attempt as u64);
    std::thread::sleep(Duration::from_millis(delay_ms));
}

fn task_list_to_parse_result(list: ApiTaskList) -> TasksParseResult {
    let format = match list.format.as_str() {
        "enhanced" => TasksFormat::Enhanced,
        _ => TasksFormat::Checkbox,
    };

    let mut tasks = Vec::with_capacity(list.tasks.len());
    let mut missing_dependencies = false;
    for item in list.tasks {
        let status = TaskStatus::from_enhanced_label(&item.status).unwrap_or(TaskStatus::Pending);
        let dependencies = match item.dependencies {
            Some(deps) => deps,
            None => {
                missing_dependencies = true;
                Vec::new()
            }
        };
        tasks.push(TaskItem {
            id: item.id,
            name: item.name,
            wave: item.wave,
            status,
            updated_at: None,
            dependencies,
            files: Vec::new(),
            action: String::new(),
            verify: None,
            done_when: None,
            kind: TaskKind::Normal,
            header_line_index: 0,
        });
    }

    let progress = ProgressInfo {
        total: list.progress.total,
        complete: list.progress.complete,
        shelved: list.progress.shelved,
        in_progress: list.progress.in_progress,
        pending: list.progress.pending,
        remaining: list.progress.remaining,
    };

    let waves = if format == TasksFormat::Enhanced {
        let mut unique = BTreeSet::new();
        for task in &tasks {
            if let Some(wave) = task.wave {
                unique.insert(wave);
            }
        }
        unique
            .into_iter()
            .map(|wave| WaveInfo {
                wave,
                depends_on: Vec::new(),
                header_line_index: 0,
                depends_on_line_index: None,
            })
            .collect()
    } else {
        Vec::new()
    };

    let mut diagnostics = Vec::new();
    if missing_dependencies && format == TasksFormat::Enhanced {
        diagnostics.push(TaskDiagnostic {
            level: DiagnosticLevel::Warning,
            message: "Backend task payload missing dependencies; readiness may be inaccurate"
                .to_string(),
            task_id: None,
            line: None,
        });
    }

    TasksParseResult {
        format,
        tasks,
        waves,
        diagnostics,
        progress,
    }
}

fn tasks_from_progress(progress: &ApiProgress) -> TasksParseResult {
    TasksParseResult {
        format: TasksFormat::Checkbox,
        tasks: Vec::new(),
        waves: Vec::new(),
        diagnostics: Vec::new(),
        progress: ProgressInfo {
            total: progress.total,
            complete: progress.complete,
            shelved: progress.shelved,
            in_progress: progress.in_progress,
            pending: progress.pending,
            remaining: progress.remaining,
        },
    }
}

fn task_mutation_from_api(response: ApiTaskMutationEnvelope) -> TaskMutationResult {
    TaskMutationResult {
        change_id: response.change_id,
        task: TaskItem {
            id: response.task.id,
            name: response.task.name,
            wave: response.task.wave,
            status: TaskStatus::from_enhanced_label(&response.task.status)
                .unwrap_or(TaskStatus::Pending),
            updated_at: response.task.updated_at,
            dependencies: response.task.dependencies,
            files: response.task.files,
            action: response.task.action,
            verify: response.task.verify,
            done_when: response.task.done_when,
            kind: match response.task.kind.as_str() {
                "checkpoint" => TaskKind::Checkpoint,
                _ => TaskKind::Normal,
            },
            header_line_index: response.task.header_line_index,
        },
        revision: response.revision,
    }
}

#[derive(Debug, Deserialize)]
struct ApiChangeSummary {
    id: String,
    module_id: Option<String>,
    completed_tasks: u32,
    shelved_tasks: u32,
    in_progress_tasks: u32,
    pending_tasks: u32,
    total_tasks: u32,
    has_proposal: bool,
    has_design: bool,
    has_specs: bool,
    has_tasks: bool,
    #[allow(dead_code)]
    work_status: String,
    last_modified: String,
}

#[derive(Debug, Deserialize)]
struct ApiChange {
    id: String,
    module_id: Option<String>,
    proposal: Option<String>,
    design: Option<String>,
    specs: Vec<ApiSpec>,
    #[allow(dead_code)]
    progress: ApiProgress,
    last_modified: String,
}

#[derive(Debug, Deserialize)]
struct ApiSpec {
    name: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ApiProgress {
    total: usize,
    complete: usize,
    shelved: usize,
    in_progress: usize,
    pending: usize,
    remaining: usize,
}

#[derive(Debug, Deserialize)]
struct ApiTaskList {
    #[allow(dead_code)]
    change_id: String,
    tasks: Vec<ApiTaskItem>,
    progress: ApiProgress,
    format: String,
}

#[derive(Debug, Deserialize)]
struct ApiTaskItem {
    id: String,
    name: String,
    wave: Option<u32>,
    status: String,
    #[serde(default)]
    dependencies: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct ApiTaskMarkdown {
    #[allow(dead_code)]
    change_id: String,
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiTaskInitResult {
    change_id: String,
    path: Option<String>,
    existed: bool,
    revision: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiTaskMutationEnvelope {
    change_id: String,
    task: ApiTaskDetail,
    revision: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiTaskDetail {
    id: String,
    name: String,
    wave: Option<u32>,
    status: String,
    updated_at: Option<String>,
    dependencies: Vec<String>,
    files: Vec<String>,
    action: String,
    verify: Option<String>,
    done_when: Option<String>,
    kind: String,
    header_line_index: usize,
}

#[derive(Debug, Deserialize)]
struct ApiSpecSummary {
    id: String,
    path: String,
    last_modified: String,
}

#[derive(Debug, Deserialize)]
struct ApiSpecDocument {
    id: String,
    path: String,
    markdown: String,
    last_modified: String,
}

#[derive(Debug, Deserialize)]
struct ApiModuleSummary {
    id: String,
    name: String,
    change_count: u32,
}

#[derive(Debug, Deserialize)]
struct ApiModule {
    id: String,
    name: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiErrorBody {
    error: String,
    code: String,
}
