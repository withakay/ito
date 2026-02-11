use ito_config::ConfigContext;
use ito_config::ito_dir::get_ito_path;
use ito_core::audit::{
    AuditEvent, AuditWriter, EventContext, FsAuditWriter, resolve_context, resolve_user_identity,
};
use ito_core::repo_index::RepoIndex;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

fn resolve_runtime_root() -> PathBuf {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let fallback = || find_nearest_ito_root(&cwd).unwrap_or_else(|| cwd.clone());
    let mut command = std::process::Command::new("git");
    command
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(&cwd);
    for (key, _value) in std::env::vars_os() {
        let key = key.to_string_lossy();
        if key.starts_with("GIT_") {
            command.env_remove(key.as_ref());
        }
    }
    let output = command.output();

    let Ok(output) = output else {
        return fallback();
    };
    if !output.status.success() {
        return fallback();
    }

    let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if root.is_empty() {
        return fallback();
    }
    PathBuf::from(root)
}

fn find_nearest_ito_root(start: &Path) -> Option<PathBuf> {
    let mut cur = start.to_path_buf();
    loop {
        if cur.join(".ito").is_dir() {
            return Some(cur);
        }

        let parent = cur.parent().map(Path::to_path_buf)?;
        cur = parent;
    }
}

pub(crate) struct Runtime {
    ctx: ConfigContext,
    cwd: PathBuf,
    ito_path: OnceLock<PathBuf>,
    repo_index: OnceLock<RepoIndex>,
    audit_writer: OnceLock<FsAuditWriter>,
    event_context: OnceLock<EventContext>,
    user_identity: OnceLock<String>,
}

impl Runtime {
    pub(crate) fn new() -> Self {
        Self {
            ctx: ConfigContext::from_process_env(),
            cwd: resolve_runtime_root(),
            ito_path: OnceLock::new(),
            repo_index: OnceLock::new(),
            audit_writer: OnceLock::new(),
            event_context: OnceLock::new(),
            user_identity: OnceLock::new(),
        }
    }

    pub(crate) fn ctx(&self) -> &ConfigContext {
        &self.ctx
    }

    pub(crate) fn ito_path(&self) -> &Path {
        self.ito_path
            .get_or_init(|| get_ito_path(&self.cwd, &self.ctx))
            .as_path()
    }

    pub(crate) fn repo_index(&self) -> &RepoIndex {
        self.repo_index
            .get_or_init(|| RepoIndex::load(self.ito_path()).unwrap_or_default())
    }

    /// Returns the filesystem-backed audit writer, lazily initialized.
    pub(crate) fn audit_writer(&self) -> &FsAuditWriter {
        self.audit_writer
            .get_or_init(|| FsAuditWriter::new(self.ito_path()))
    }

    /// Returns the event context (session ID, git info), lazily initialized.
    pub(crate) fn event_context(&self) -> &EventContext {
        self.event_context
            .get_or_init(|| resolve_context(self.ito_path()))
    }

    /// Returns the user identity string (e.g., "@jack"), lazily initialized.
    pub(crate) fn user_identity(&self) -> &str {
        self.user_identity.get_or_init(resolve_user_identity)
    }

    /// Emit an audit event using the runtime's writer. Best-effort: never fails.
    pub(crate) fn emit_audit_event(&self, event: &AuditEvent) {
        let _ = self.audit_writer().append(event);
    }
}
