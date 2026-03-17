//! Audit event storage abstractions.

use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use std::sync::OnceLock;

use ito_config::{ConfigContext, load_cascading_project_config, resolve_audit_mirror_settings};
use ito_domain::audit::event::AuditEvent;
use ito_domain::audit::writer::AuditWriter;
use ito_domain::backend::{BackendEventIngestClient, EventBatch};

use crate::backend_client::idempotency_key;
use crate::backend_http::BackendHttpClient;
use crate::process::{ProcessRequest, ProcessRunner, SystemProcessRunner};
use crate::repository_runtime::{PersistenceMode, resolve_repository_runtime};

use super::mirror::{
    InternalBranchLogRead, append_jsonl_to_internal_branch, read_internal_branch_log,
};
use super::writer::{
    append_event_to_file, audit_log_path, parse_events_from_jsonl, read_events_from_path,
};

/// Storage location descriptor for audit events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditStorageLocation {
    /// Filesystem-backed storage at the given path.
    Filesystem(PathBuf),
    /// Non-filesystem or abstract storage identified by a short label.
    Other(&'static str),
}

/// Combined read/write abstraction for audit event storage.
pub trait AuditEventStore: AuditWriter + Send + Sync {
    /// Read all available events from the underlying storage.
    fn read_all(&self) -> Vec<AuditEvent>;

    /// Describe the underlying storage location for diagnostics and routing.
    fn location(&self) -> AuditStorageLocation;
}

/// Build a stable deduplication key for an audit storage location.
pub fn audit_storage_location_key(location: &AuditStorageLocation) -> String {
    match location {
        AuditStorageLocation::Filesystem(path) => format!("fs:{}", path.display()),
        AuditStorageLocation::Other(label) => format!("other:{label}"),
    }
}

struct BackendAuditStore {
    client: BackendHttpClient,
}

impl BackendAuditStore {
    fn new(client: BackendHttpClient) -> Self {
        Self { client }
    }
}

impl AuditWriter for BackendAuditStore {
    fn append(&self, event: &AuditEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let batch = EventBatch {
            events: vec![event.clone()],
            idempotency_key: idempotency_key("audit-write"),
        };

        if let Err(err) = self.client.ingest(&batch) {
            tracing::warn!("backend audit write failed: {err}");
        }

        Ok(())
    }
}

impl AuditEventStore for BackendAuditStore {
    fn read_all(&self) -> Vec<AuditEvent> {
        match self.client.list_audit_events() {
            Ok(events) => events,
            Err(err) => {
                tracing::warn!("backend audit read failed: {err}");
                Vec::new()
            }
        }
    }

    fn location(&self) -> AuditStorageLocation {
        AuditStorageLocation::Other("backend")
    }
}

struct LocalAuditStore {
    ito_path: PathBuf,
    branch: String,
    fallback_path: PathBuf,
    legacy_migration_done: OnceLock<()>,
}

impl LocalAuditStore {
    fn new(ito_path: &Path, branch: String, fallback_path: PathBuf) -> Self {
        Self {
            ito_path: ito_path.to_path_buf(),
            branch,
            fallback_path,
            legacy_migration_done: OnceLock::new(),
        }
    }

    fn repo_root(&self) -> Option<&Path> {
        self.ito_path.parent()
    }

    fn append_to_branch(&self, event: &AuditEvent) -> Result<(), String> {
        let Some(repo_root) = self.repo_root() else {
            return Err("unable to resolve project root for internal audit branch".to_string());
        };
        let json = serde_json::to_string(event)
            .map_err(|err| format!("failed to serialize audit event: {err}"))?;
        append_jsonl_to_internal_branch(repo_root, &self.branch, &format!("{json}\n"))
            .map_err(|err| err.to_string())
    }

    fn read_from_branch(&self) -> Result<InternalBranchRead, String> {
        let Some(repo_root) = self.repo_root() else {
            return Err("unable to resolve project root for internal audit branch".to_string());
        };
        let branch_read =
            read_internal_branch_log(repo_root, &self.branch).map_err(|err| err.to_string())?;
        Ok(match branch_read {
            InternalBranchLogRead::BranchMissing => InternalBranchRead::BranchMissing,
            InternalBranchLogRead::LogMissing => InternalBranchRead::LogMissing,
            InternalBranchLogRead::Contents(contents) => {
                InternalBranchRead::Events(parse_events_from_jsonl(&contents))
            }
        })
    }

    fn append_to_fallback(&self, event: &AuditEvent) {
        if let Err(err) = append_event_to_file(&self.fallback_path, event) {
            tracing::warn!("fallback audit write failed: {err}");
        }
    }

    fn migrate_legacy_worktree_log(&self) {
        let legacy_path = audit_log_path(&self.ito_path);
        let Ok(contents) = std::fs::read_to_string(&legacy_path) else {
            return;
        };
        if contents.trim().is_empty() {
            return;
        }

        if let Some(repo_root) = self.repo_root() {
            match append_jsonl_to_internal_branch(repo_root, &self.branch, &contents) {
                Ok(()) => return,
                Err(err) => {
                    tracing::warn!("legacy tracked audit log import failed: {err}");
                    eprintln!(
                        "Warning: durable internal audit storage unavailable; migrating legacy tracked audit log into local fallback store '{}': {err}",
                        self.fallback_path.display()
                    );
                }
            }
        }

        if let Err(err) = merge_jsonl_file(&self.fallback_path, &contents) {
            tracing::warn!("legacy tracked audit fallback import failed: {err}");
        }
    }

    fn ensure_legacy_worktree_log_migrated(&self) {
        self.legacy_migration_done.get_or_init(|| {
            self.migrate_legacy_worktree_log();
        });
    }

    fn warn_and_fallback(&self, err: &str, event: &AuditEvent) {
        tracing::warn!("internal audit branch unavailable: {err}");
        eprintln!(
            "Warning: durable internal audit storage unavailable; using local fallback store '{}': {err}",
            self.fallback_path.display()
        );
        self.append_to_fallback(event);
    }
}

impl AuditWriter for LocalAuditStore {
    fn append(&self, event: &AuditEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.ensure_legacy_worktree_log_migrated();
        if let Err(err) = self.append_to_branch(event) {
            self.warn_and_fallback(&err, event);
        }
        Ok(())
    }
}

impl AuditEventStore for LocalAuditStore {
    fn read_all(&self) -> Vec<AuditEvent> {
        self.ensure_legacy_worktree_log_migrated();
        match self.read_from_branch() {
            Ok(InternalBranchRead::Events(events)) => events,
            Ok(InternalBranchRead::BranchMissing) => read_events_from_path(&self.fallback_path),
            Ok(InternalBranchRead::LogMissing) => {
                tracing::warn!(
                    branch = %self.branch,
                    "internal audit branch exists but has no audit log yet; treating as empty history"
                );
                Vec::new()
            }
            Err(err) => {
                tracing::warn!("internal audit branch read failed: {err}");
                read_events_from_path(&self.fallback_path)
            }
        }
    }

    fn location(&self) -> AuditStorageLocation {
        if self.repo_root().is_some() {
            AuditStorageLocation::Other("internal-branch")
        } else {
            AuditStorageLocation::Filesystem(self.fallback_path.clone())
        }
    }
}

enum InternalBranchRead {
    BranchMissing,
    LogMissing,
    Events(Vec<AuditEvent>),
}

/// Resolve the default audit store for the current project.
///
/// Today this is filesystem-backed. Later tasks can route this to internal-branch
/// or backend-managed storage without changing reader call sites again.
pub fn default_audit_store(ito_path: &Path) -> Box<dyn AuditEventStore> {
    let ctx = ConfigContext::from_process_env();
    if let Ok(runtime) = resolve_repository_runtime(ito_path, &ctx)
        && runtime.mode() == PersistenceMode::Remote
        && let Some(backend_runtime) = runtime.backend_runtime().cloned()
    {
        return Box::new(BackendAuditStore::new(BackendHttpClient::new(
            backend_runtime,
        )));
    }

    let branch = resolve_internal_audit_branch(ito_path, &ctx);
    let fallback_path = fallback_audit_log_path(ito_path);
    Box::new(LocalAuditStore::new(ito_path, branch, fallback_path))
}

fn resolve_internal_audit_branch(ito_path: &Path, ctx: &ConfigContext) -> String {
    let Some(project_root) = ito_path.parent() else {
        return "ito/internal/audit".to_string();
    };
    let resolved = load_cascading_project_config(project_root, ito_path, ctx);
    let (_, branch) = resolve_audit_mirror_settings(&resolved.merged);
    branch
}

fn fallback_audit_log_path(ito_path: &Path) -> PathBuf {
    let runner = SystemProcessRunner;
    if let Some(project_root) = ito_path.parent()
        && let Some(git_dir) = git_dir_path(&runner, project_root)
    {
        return git_dir.join("ito").join("audit").join("events.jsonl");
    }

    ito_path
        .join(".state-local")
        .join("audit")
        .join("events.jsonl")
}

fn git_dir_path(runner: &dyn ProcessRunner, project_root: &Path) -> Option<PathBuf> {
    let out = runner
        .run(
            &ProcessRequest::new("git")
                .args(["rev-parse", "--absolute-git-dir"])
                .current_dir(project_root),
        )
        .ok()?;
    if !out.success {
        return None;
    }

    let path = out.stdout.trim();
    if path.is_empty() {
        return None;
    }
    Some(PathBuf::from(path))
}

fn merge_jsonl_file(path: &Path, incoming: &str) -> std::io::Result<()> {
    let existing = std::fs::read_to_string(path).unwrap_or_default();
    let merged = merge_jsonl_contents(&existing, incoming);
    if merged == existing {
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, merged)
}

fn merge_jsonl_contents(existing: &str, incoming: &str) -> String {
    let mut merged = Vec::new();
    let mut seen = HashSet::new();

    for line in existing.lines().chain(incoming.lines()) {
        let line = line.trim();
        if line.is_empty() || !seen.insert(line.to_string()) {
            continue;
        }
        merged.push(line.to_string());
    }

    if merged.is_empty() {
        String::new()
    } else {
        format!("{}\n", merged.join("\n"))
    }
}
