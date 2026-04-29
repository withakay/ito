//! Rules under the `repository/*` namespace.
//!
//! Both rules gate on `repository.mode == sqlite`. When the project uses
//! the filesystem-backed repository, neither rule fires.
//!
//! - `repository/sqlite-db-path-set` — the configured `db_path` must
//!   resolve inside the project root and have an existing parent
//!   directory.
//! - `repository/sqlite-db-not-committed` — the database file MUST NOT
//!   be tracked by git AND SHOULD be covered by a `.gitignore` entry.

use std::path::{Path, PathBuf};

use ito_config::types::{ItoConfig, RepositoryPersistenceMode};

use crate::errors::CoreError;
use crate::process::{ProcessRequest, ProcessRunner};
use crate::validate::{ValidationIssue, error, warning, with_metadata, with_rule_id};

use super::rule::{Rule, RuleContext, RuleId, RuleSeverity};

const SQLITE_DB_PATH_SET_ID: RuleId = RuleId::new("repository/sqlite-db-path-set");
const SQLITE_DB_NOT_COMMITTED_ID: RuleId = RuleId::new("repository/sqlite-db-not-committed");

/// True when the repository persistence mode is sqlite.
fn mode_is_sqlite(config: &ItoConfig) -> bool {
    match config.repository.mode {
        RepositoryPersistenceMode::Sqlite => true,
        RepositoryPersistenceMode::Filesystem => false,
    }
}

/// Return the configured sqlite db path resolved against `project_root`.
///
/// Returns `None` when the path is unset or empty.
fn resolve_db_path(config: &ItoConfig, project_root: &Path) -> Option<PathBuf> {
    let raw = config.repository.sqlite.db_path.as_ref()?.trim();
    if raw.is_empty() {
        return None;
    }
    let path = PathBuf::from(raw);
    if path.is_absolute() {
        Some(path)
    } else {
        Some(project_root.join(path))
    }
}

/// True when `path` is inside `root` after both lexical normalization
/// (collapsing `..` and `.` components) AND, when possible, OS-level
/// canonicalization (resolving symlinks).
///
/// `Path::starts_with` is component-based and does NOT resolve `..`, so a
/// fallback that uses it directly would let `<root>/.ito/state/../../etc/passwd`
/// claim to be "inside the root". The lexical normalisation below
/// closes that hole when the file does not yet exist (so
/// `Path::canonicalize` is unavailable).
fn path_inside_root(path: &Path, root: &Path) -> bool {
    let canonical_root = root.canonicalize().ok();

    // Best case: canonicalize both. Resolves symlinks too.
    if let Some(canonical_root) = canonical_root.as_ref()
        && let Ok(canonical_path) = path.canonicalize()
    {
        return canonical_path.starts_with(canonical_root);
    }

    // File doesn't exist yet (or root cannot be canonicalised). Lexically
    // normalise `path`, then compare against the canonicalised root (when
    // available) AND the original root — handles macOS where `/var` →
    // `/private/var` so a path under `/var/...` would not literally
    // start with `/private/var`.
    let normalized = lexical_normalize(path);
    if let Some(canonical_root) = canonical_root.as_ref()
        && normalized.starts_with(canonical_root)
    {
        return true;
    }
    normalized.starts_with(root)
}

/// Collapse `.` and `..` components without touching the filesystem.
///
/// Unlike `Path::canonicalize`, this is purely lexical — `..` after a
/// regular component pops the previous component; `..` after `..` or at
/// the start is kept (we cannot know what the parent of an unknown root
/// is).
fn lexical_normalize(path: &Path) -> PathBuf {
    use std::path::Component;
    let mut out: Vec<Component<'_>> = Vec::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                let pops_prior_normal = match out.last() {
                    Some(Component::Normal(_)) => true,
                    Some(_) | None => false,
                };
                if pops_prior_normal {
                    out.pop();
                } else {
                    out.push(component);
                }
            }
            Component::CurDir => {}
            Component::Prefix(_) | Component::RootDir | Component::Normal(_) => {
                out.push(component);
            }
        }
    }
    out.iter().collect()
}

// ── repository/sqlite-db-path-set ────────────────────────────────────────

/// `repository/sqlite-db-path-set` — sqlite db_path must be set and
/// resolvable inside the project root.
pub(crate) struct SqliteDbPathSetRule;

impl Rule for SqliteDbPathSetRule {
    fn id(&self) -> RuleId {
        SQLITE_DB_PATH_SET_ID
    }

    fn severity(&self) -> RuleSeverity {
        RuleSeverity::Error
    }

    fn description(&self) -> &'static str {
        "SQLite db_path is set, resolves inside the project root, and has an existing parent."
    }

    fn gate(&self) -> Option<&'static str> {
        Some("repository.mode == sqlite")
    }

    fn is_active(&self, config: &ItoConfig) -> bool {
        mode_is_sqlite(config)
    }

    fn check(&self, ctx: &RuleContext<'_>) -> Result<Vec<ValidationIssue>, CoreError> {
        let Some(db_path) = resolve_db_path(ctx.config, ctx.project_root) else {
            let issue = error(
                ".ito/config.json",
                "`repository.sqlite.db_path` is empty or unset while `repository.mode = \"sqlite\"`. \
                 The runtime cannot open a database without a path.",
            );
            let issue = with_rule_id(issue, SQLITE_DB_PATH_SET_ID.as_str());
            let issue = with_metadata(
                issue,
                serde_json::json!({
                    "fix": "Set `repository.sqlite.db_path` to a project-relative path under \
                            `.ito/state/`, e.g. `.ito/state/ito.db`.",
                    "config_key": "repository.sqlite.db_path",
                }),
            );
            return Ok(vec![issue]);
        };

        if !path_inside_root(&db_path, ctx.project_root) {
            let issue = error(
                ".ito/config.json",
                format!(
                    "`repository.sqlite.db_path = \"{path}\"` resolves outside the project root \
                     (`{root}`). The repository runtime confines persistence to the project tree.",
                    path = db_path.display(),
                    root = ctx.project_root.display(),
                ),
            );
            let issue = with_rule_id(issue, SQLITE_DB_PATH_SET_ID.as_str());
            let issue = with_metadata(
                issue,
                serde_json::json!({
                    "fix": "Use a project-relative path (e.g. `.ito/state/ito.db`).",
                    "resolved": db_path.to_string_lossy(),
                    "project_root": ctx.project_root.to_string_lossy(),
                }),
            );
            return Ok(vec![issue]);
        }

        let mut issues = Vec::new();
        if let Some(parent) = db_path.parent()
            && !parent.exists()
        {
            let issue = warning(
                ".ito/config.json",
                format!(
                    "`repository.sqlite.db_path` parent directory does not exist: `{parent}`. \
                     SQLite cannot create parent directories on its own; the database open \
                     will fail at runtime if the directory is still missing.",
                    parent = parent.display(),
                ),
            );
            let issue = with_rule_id(issue, SQLITE_DB_PATH_SET_ID.as_str());
            issues.push(with_metadata(
                issue,
                serde_json::json!({
                    "fix": format!("Create the directory: `mkdir -p {}`", parent.display()),
                    "missing_parent": parent.to_string_lossy(),
                }),
            ));
        }

        Ok(issues)
    }
}

// ── repository/sqlite-db-not-committed ───────────────────────────────────

/// `repository/sqlite-db-not-committed` — db file must not be tracked by
/// git AND should be covered by `.gitignore`.
pub(crate) struct SqliteDbNotCommittedRule;

impl Rule for SqliteDbNotCommittedRule {
    fn id(&self) -> RuleId {
        SQLITE_DB_NOT_COMMITTED_ID
    }

    fn severity(&self) -> RuleSeverity {
        RuleSeverity::Error
    }

    fn description(&self) -> &'static str {
        "SQLite database file is not tracked by git and is covered by `.gitignore`."
    }

    fn gate(&self) -> Option<&'static str> {
        Some("repository.mode == sqlite")
    }

    fn is_active(&self, config: &ItoConfig) -> bool {
        mode_is_sqlite(config)
    }

    fn check(&self, ctx: &RuleContext<'_>) -> Result<Vec<ValidationIssue>, CoreError> {
        let Some(db_path) = resolve_db_path(ctx.config, ctx.project_root) else {
            // Companion rule `sqlite-db-path-set` reports the missing path.
            return Ok(Vec::new());
        };

        let rel_str = match db_path.strip_prefix(ctx.project_root) {
            Ok(rel) => rel.to_string_lossy().into_owned(),
            Err(_) => db_path.to_string_lossy().into_owned(),
        };

        if git_tracks_path(ctx.runner, ctx.project_root, &rel_str)? {
            let issue = error(
                rel_str.clone(),
                format!(
                    "SQLite database `{rel_str}` is currently tracked by git. \
                     Committing the live database leaks every entry through history \
                     and corrupts the file across machine clones.",
                ),
            );
            let issue = with_rule_id(issue, SQLITE_DB_NOT_COMMITTED_ID.as_str());
            let issue = with_metadata(
                issue,
                serde_json::json!({
                    "fix": format!(
                        "Untrack: `git rm --cached {rel_str}`. \
                         Then ensure `.gitignore` covers it (e.g. `{rel_str}` or \
                         `{parent}/*.db`).",
                        parent = std::path::Path::new(&rel_str)
                            .parent()
                            .map(std::path::Path::to_string_lossy)
                            .unwrap_or_default(),
                    ),
                    "untrack_command": format!("git rm --cached {rel_str}"),
                    "path": rel_str,
                }),
            );
            return Ok(vec![issue]);
        }

        if !git_check_ignore(ctx.runner, ctx.project_root, &rel_str)? {
            let issue = warning(
                ".gitignore",
                format!(
                    "SQLite database `{rel_str}` is not currently tracked by git but is \
                     also not covered by `.gitignore`. A future `git add .` would \
                     accidentally commit it.",
                ),
            );
            let issue = with_rule_id(issue, SQLITE_DB_NOT_COMMITTED_ID.as_str());
            return Ok(vec![with_metadata(
                issue,
                serde_json::json!({
                    "fix": format!("Append `{rel_str}` (or a glob like `*.db`) to `.gitignore`."),
                    "path": rel_str,
                }),
            )]);
        }

        Ok(Vec::new())
    }
}

/// Return `true` when `rel_path` is currently tracked by git.
///
/// Implementation: `git ls-files --error-unmatch <path>` exits 0 only when
/// the path is in the index; non-zero means untracked or missing.
fn git_tracks_path(
    runner: &dyn ProcessRunner,
    project_root: &Path,
    rel_path: &str,
) -> Result<bool, CoreError> {
    let request = ProcessRequest::new("git")
        .args(["ls-files", "--error-unmatch", "--", rel_path])
        .current_dir(project_root);

    let output = runner.run(&request).map_err(|err| {
        CoreError::process(format!(
            "Cannot run `git ls-files --error-unmatch {rel_path}`.\n\
             Why: {err}\n\
             Fix: ensure git is installed and `{root}` is a git repository.",
            root = project_root.display(),
        ))
    })?;
    Ok(output.success)
}

/// Return `true` when `rel_path` is matched by a `.gitignore` rule.
///
/// Implementation: `git check-ignore -q <path>` exits 0 when the path is
/// ignored, 1 when it is not, and 128+ on error. We treat error exits as
/// "unknown" and conservatively return `false` so the rule warns rather
/// than silently passes.
fn git_check_ignore(
    runner: &dyn ProcessRunner,
    project_root: &Path,
    rel_path: &str,
) -> Result<bool, CoreError> {
    let request = ProcessRequest::new("git")
        .args(["check-ignore", "-q", "--", rel_path])
        .current_dir(project_root);

    let output = runner.run(&request).map_err(|err| {
        CoreError::process(format!(
            "Cannot run `git check-ignore {rel_path}`.\n\
             Why: {err}\n\
             Fix: ensure git is installed and `{root}` is a git repository.",
            root = project_root.display(),
        ))
    })?;
    Ok(output.success)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process::{ProcessExecutionError, ProcessOutput};
    use crate::validate_repo::staged::StagedFiles;
    use ito_config::types::{ItoConfig, RepositoryRuntimeConfig, RepositorySqliteConfig};
    use std::cell::RefCell;
    use std::time::Duration;
    use tempfile::TempDir;

    /// Test runner that returns canned exit codes per `(program, args[0..N])` prefix.
    ///
    /// The first matching rule wins; falls back to a default if no rule matches.
    struct ScriptedRunner {
        rules: Vec<(Vec<&'static str>, ProcessOutput)>,
        default: ProcessOutput,
        seen: RefCell<Vec<ProcessRequest>>,
    }

    fn ok_output(success: bool, exit_code: i32) -> ProcessOutput {
        ProcessOutput {
            exit_code,
            success,
            stdout: String::new(),
            stderr: String::new(),
            timed_out: false,
        }
    }

    impl ScriptedRunner {
        fn new(default: ProcessOutput) -> Self {
            Self {
                rules: Vec::new(),
                default,
                seen: RefCell::new(Vec::new()),
            }
        }

        fn with_rule(mut self, args_prefix: &[&'static str], output: ProcessOutput) -> Self {
            self.rules.push((args_prefix.to_vec(), output));
            self
        }
    }

    impl ProcessRunner for ScriptedRunner {
        fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
            self.seen.borrow_mut().push(request.clone());
            for (prefix, output) in &self.rules {
                if request.args.len() >= prefix.len()
                    && request.args.iter().zip(prefix.iter()).all(|(a, b)| a == b)
                {
                    return Ok(output.clone());
                }
            }
            Ok(self.default.clone())
        }
        fn run_with_timeout(
            &self,
            request: &ProcessRequest,
            _timeout: Duration,
        ) -> Result<ProcessOutput, ProcessExecutionError> {
            self.run(request)
        }
    }

    fn config(mode: RepositoryPersistenceMode, db_path: Option<&str>) -> ItoConfig {
        ItoConfig {
            repository: RepositoryRuntimeConfig {
                mode,
                sqlite: RepositorySqliteConfig {
                    db_path: db_path.map(str::to_string),
                },
            },
            ..ItoConfig::default()
        }
    }

    // ── activation ───────────────────────────────────────────────────────

    #[test]
    fn rules_inactive_in_filesystem_mode() {
        let cfg = config(
            RepositoryPersistenceMode::Filesystem,
            Some(".ito/state/ito.db"),
        );
        assert!(!SqliteDbPathSetRule.is_active(&cfg));
        assert!(!SqliteDbNotCommittedRule.is_active(&cfg));
    }

    #[test]
    fn rules_active_in_sqlite_mode() {
        let cfg = config(RepositoryPersistenceMode::Sqlite, Some(".ito/state/ito.db"));
        assert!(SqliteDbPathSetRule.is_active(&cfg));
        assert!(SqliteDbNotCommittedRule.is_active(&cfg));
    }

    // ── repository/sqlite-db-path-set ────────────────────────────────────

    #[test]
    fn sqlite_db_path_set_errors_when_path_unset() {
        let cfg = config(RepositoryPersistenceMode::Sqlite, None);
        let tmp = TempDir::new().unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

        let issues = SqliteDbPathSetRule.check(&ctx).unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].level, "ERROR");
        assert!(issues[0].message.contains("empty or unset"));
    }

    #[test]
    fn sqlite_db_path_set_errors_when_path_outside_project_root() {
        let cfg = config(RepositoryPersistenceMode::Sqlite, Some("/var/tmp/ito.db"));
        let tmp = TempDir::new().unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

        let issues = SqliteDbPathSetRule.check(&ctx).unwrap();
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("outside the project root"));
    }

    #[test]
    fn sqlite_db_path_set_errors_when_path_escapes_root_via_dotdot() {
        // Regression test for a `..` traversal bypass in path_inside_root:
        // `Path::starts_with` is component-based and does not resolve `..`,
        // so a path of `.ito/state/../../../etc/passwd` could appear to
        // "start with" the project root while actually escaping it.
        let cfg = config(
            RepositoryPersistenceMode::Sqlite,
            Some(".ito/state/../../../etc/passwd"),
        );
        let tmp = TempDir::new().unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

        let issues = SqliteDbPathSetRule.check(&ctx).unwrap();
        assert_eq!(
            issues.len(),
            1,
            "expected one error for `..` traversal escape; got {issues:?}",
        );
        assert!(
            issues[0].message.contains("outside the project root"),
            "expected outside-root message; got: {}",
            issues[0].message,
        );
    }

    #[test]
    fn sqlite_db_path_set_warns_when_parent_directory_missing() {
        let cfg = config(RepositoryPersistenceMode::Sqlite, Some(".ito/state/ito.db"));
        let tmp = TempDir::new().unwrap();
        // Don't create `.ito/state/`.
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

        let issues = SqliteDbPathSetRule.check(&ctx).unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].level, "WARNING");
        assert!(issues[0].message.contains("parent directory"));
    }

    #[test]
    fn sqlite_db_path_set_passes_when_parent_exists() {
        let cfg = config(RepositoryPersistenceMode::Sqlite, Some(".ito/state/ito.db"));
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join(".ito/state")).unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

        let issues = SqliteDbPathSetRule.check(&ctx).unwrap();
        assert!(issues.is_empty(), "expected no issues; got {issues:?}");
    }

    // ── repository/sqlite-db-not-committed ───────────────────────────────

    #[test]
    fn sqlite_db_not_committed_errors_when_file_is_tracked() {
        let cfg = config(RepositoryPersistenceMode::Sqlite, Some(".ito/state/ito.db"));
        let tmp = TempDir::new().unwrap();
        // ls-files --error-unmatch returns success → tracked.
        let runner = ScriptedRunner::new(ok_output(false, 1))
            .with_rule(&["ls-files", "--error-unmatch"], ok_output(true, 0));
        let staged = StagedFiles::empty();
        let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

        let issues = SqliteDbNotCommittedRule.check(&ctx).unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].level, "ERROR");
        assert!(issues[0].message.contains("tracked by git"));
        let metadata = issues[0].metadata.as_ref().expect("metadata");
        let untrack = metadata
            .get("untrack_command")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        assert!(untrack.starts_with("git rm --cached"));
    }

    #[test]
    fn sqlite_db_not_committed_warns_when_untracked_and_not_ignored() {
        let cfg = config(RepositoryPersistenceMode::Sqlite, Some(".ito/state/ito.db"));
        let tmp = TempDir::new().unwrap();
        // ls-files --error-unmatch fails (untracked); check-ignore fails (not ignored).
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

        let issues = SqliteDbNotCommittedRule.check(&ctx).unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].level, "WARNING");
        assert!(issues[0].message.contains("not covered by `.gitignore`"));
    }

    #[test]
    fn sqlite_db_not_committed_passes_when_untracked_and_ignored() {
        let cfg = config(RepositoryPersistenceMode::Sqlite, Some(".ito/state/ito.db"));
        let tmp = TempDir::new().unwrap();
        // ls-files fails (untracked); check-ignore succeeds (ignored).
        let runner = ScriptedRunner::new(ok_output(false, 1))
            .with_rule(&["check-ignore"], ok_output(true, 0));
        let staged = StagedFiles::empty();
        let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

        let issues = SqliteDbNotCommittedRule.check(&ctx).unwrap();
        assert!(issues.is_empty());
    }

    #[test]
    fn sqlite_db_not_committed_silent_when_path_unset() {
        // Companion rule `sqlite-db-path-set` reports the missing path.
        let cfg = config(RepositoryPersistenceMode::Sqlite, None);
        let tmp = TempDir::new().unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

        let issues = SqliteDbNotCommittedRule.check(&ctx).unwrap();
        assert!(issues.is_empty());
    }
}
