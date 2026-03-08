//! HTTP client for backend repository reads.
//!
//! Provides a shared backend client that implements repository reader ports
//! using the backend REST API.

use std::collections::BTreeSet;
use std::io::{Error as IoError, ErrorKind};
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::backend_client::{BackendRuntime, is_retriable_status};
use ito_domain::backend::{BackendChangeReader, BackendModuleReader};
use ito_domain::changes::{Change, ChangeSummary, Spec};
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::modules::{Module, ModuleSummary};
use ito_domain::tasks::{
    DiagnosticLevel, ProgressInfo, TaskDiagnostic, TaskItem, TaskKind, TaskStatus, TasksFormat,
    TasksParseResult, WaveInfo,
};

/// Backend HTTP client shared across repository adapters.
#[derive(Debug, Clone)]
pub(crate) struct BackendHttpClient {
    inner: Arc<BackendHttpClientInner>,
}

#[derive(Debug)]
struct BackendHttpClientInner {
    runtime: BackendRuntime,
    agent: ureq::Agent,
}

impl BackendHttpClient {
    /// Create a backend HTTP client from a resolved runtime.
    pub(crate) fn new(runtime: BackendRuntime) -> Self {
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
        let response = self.request_with_retry(url)?;
        let status = response.status().as_u16();
        let body = read_response_body(response)?;
        if status != 200 {
            return Err(map_status_to_domain_error(status, entity, id, &body));
        }
        serde_json::from_str(&body)
            .map_err(|err| DomainError::io("parsing backend response", IoError::other(err)))
    }

    fn request_with_retry(&self, url: &str) -> DomainResult<ureq::http::Response<ureq::Body>> {
        let max_retries = self.inner.runtime.max_retries;
        let mut attempt = 0u32;
        loop {
            let response = self
                .inner
                .agent
                .get(url)
                .header(
                    "Authorization",
                    &format!("Bearer {}", self.inner.runtime.token),
                )
                .call();

            match response {
                Ok(resp) => {
                    let status = resp.status().as_u16();
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
    fn list_changes(&self) -> DomainResult<Vec<ChangeSummary>> {
        let url = format!("{}/changes", self.inner.runtime.project_api_prefix());
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

    fn get_change(&self, change_id: &str) -> DomainResult<Change> {
        let url = format!(
            "{}/changes/{change_id}",
            self.inner.runtime.project_api_prefix()
        );
        let change: ApiChange = self.get_json(&url, "change", Some(change_id))?;
        let tasks = self.load_tasks_parse_result(change_id)?;
        let last_modified = parse_timestamp(&change.last_modified)?;
        Ok(Change {
            id: change.id,
            module_id: change.module_id,
            path: std::path::PathBuf::new(),
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
            path: std::path::PathBuf::new(),
        })
    }
}

fn read_response_body(response: ureq::http::Response<ureq::Body>) -> DomainResult<String> {
    let body = response
        .into_body()
        .read_to_string()
        .map_err(|err| DomainError::io("reading backend response", IoError::other(err)))?;
    Ok(body)
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
