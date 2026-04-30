//! Rules under the `backend/*` namespace.
//!
//! All three rules gate on `backend.enabled == true`.
//!
//! - `backend/token-not-committed` — a security-critical check: the
//!   `backend.token` MUST NOT be committed to a tracked configuration
//!   file. Severity is **`ERROR` regardless** of the engine's `--strict`
//!   flag.
//! - `backend/url-scheme-valid` — `backend.url` must be a parseable
//!   `http`/`https` URL.
//! - `backend/project-org-repo-set` — multi-tenant routing requires both
//!   `backend.project.org` and `backend.project.repo`.

use std::path::Path;

use ito_config::ConfigContext;
use ito_config::load_cascading_project_config;
use ito_config::types::ItoConfig;

use crate::errors::CoreError;
use crate::process::{ProcessRequest, ProcessRunner};
use crate::validate::{ValidationIssue, error, with_metadata, with_rule_id};

use super::rule::{Rule, RuleContext, RuleId, RuleSeverity};

const TOKEN_NOT_COMMITTED_ID: RuleId = RuleId::new("backend/token-not-committed");
const URL_SCHEME_VALID_ID: RuleId = RuleId::new("backend/url-scheme-valid");
const PROJECT_ORG_REPO_SET_ID: RuleId = RuleId::new("backend/project-org-repo-set");

// ── backend/token-not-committed ──────────────────────────────────────────

/// `backend/token-not-committed` — flag a `backend.token` set in any
/// tracked-by-git configuration layer.
pub(crate) struct TokenNotCommittedRule;

impl Rule for TokenNotCommittedRule {
    fn id(&self) -> RuleId {
        TOKEN_NOT_COMMITTED_ID
    }

    fn severity(&self) -> RuleSeverity {
        RuleSeverity::Error
    }

    fn description(&self) -> &'static str {
        "`backend.token` is not committed to a tracked configuration file."
    }

    fn gate(&self) -> Option<&'static str> {
        Some("backend.enabled == true")
    }

    fn is_active(&self, config: &ItoConfig) -> bool {
        config.backend.enabled
    }
    fn check(&self, ctx: &RuleContext<'_>) -> Result<Vec<ValidationIssue>, CoreError> {
        // We reload the cascading config here rather than receiving it via
        // `RuleContext` because the engine's merged `ItoConfig` discards
        // per-layer provenance and we need to know which layer set the
        // token. The extra disk I/O is acceptable for an infrequent
        // validation pass; the TOCTOU window between engine init and rule
        // execution is negligible in practice.
        //
        // `ConfigContext::from_process_env()` matches the CLI's view of
        // the layer set (XDG / HOME / PROJECT_DIR), so the rule reports
        // what an operator running `ito validate repo` actually sees.
        let cfg_ctx = ConfigContext::from_process_env();
        let ito_path = ctx.project_root.join(".ito");
        let cascading = load_cascading_project_config(ctx.project_root, &ito_path, &cfg_ctx);

        let mut issues = Vec::new();
        for layer in &cascading.layers {
            let Some(token) = layer
                .value
                .get("backend")
                .and_then(|b| b.get("token"))
                .and_then(|t| t.as_str())
            else {
                continue;
            };
            if token.trim().is_empty() {
                continue;
            }

            // Each layer is potentially a leak. Classify by path so the
            // operator can immediately tell whether to rotate or to
            // change the source.
            let path_str = layer.path.to_string_lossy();
            if !is_layer_tracked_in_git(ctx.runner, ctx.project_root, &layer.path)? {
                continue;
            }

            let issue = error(
                &path_str,
                format!(
                    "`backend.token` is set in `{path_str}`, which is tracked by git. \
                     Committing the token to history is a security incident; rotate it \
                     immediately and move the value to an environment variable, a \
                     gitignored config layer, or a system keychain.",
                ),
            );
            let issue = with_rule_id(issue, TOKEN_NOT_COMMITTED_ID.as_str());
            issues.push(with_metadata(
                issue,
                serde_json::json!({
                    "fix": format!(
                        "1) Rotate the token at the backend. \
                         2) Remove `backend.token` from `{path_str}`. \
                         3) Set the new token via the `ITO_BACKEND_TOKEN` env var, or \
                         add it to `.ito/config.local.json` (gitignored)."
                    ),
                    "tracked_path": path_str,
                }),
            ));
        }

        Ok(issues)
    }
}

/// True when `path` is currently tracked by git.
///
/// Mirrors `repository::sqlite-db-not-committed`'s helper but scoped to a
/// config layer path. Returns `false` on any git failure so the rule errs
/// on the side of NOT emitting a false positive (a false negative here is
/// less harmful than blocking commits with a phantom error).
fn is_layer_tracked_in_git(
    runner: &dyn ProcessRunner,
    project_root: &Path,
    path: &Path,
) -> Result<bool, CoreError> {
    let rel = match path.strip_prefix(project_root) {
        Ok(rel) => rel.to_path_buf(),
        // Layer is outside the project (e.g. ~/.config/ito/config.json) —
        // by definition not tracked in this repo.
        Err(_) => return Ok(false),
    };
    let rel_str = rel.to_string_lossy().into_owned();

    let request = ProcessRequest::new("git")
        .args(["ls-files", "--error-unmatch", "--", &rel_str])
        .current_dir(project_root);

    let output = runner.run(&request).map_err(|err| {
        CoreError::process(format!(
            "Cannot check whether `{rel_str}` is tracked by git.\n\
             Why: {err}\n\
             Fix: ensure git is installed and `{root}` is a git repository.",
            root = project_root.display(),
        ))
    })?;
    Ok(output.success)
}

// ── backend/url-scheme-valid ─────────────────────────────────────────────

/// `backend/url-scheme-valid` — `backend.url` must be a non-empty,
/// parseable URL with an `http` or `https` scheme.
pub(crate) struct UrlSchemeValidRule;

impl Rule for UrlSchemeValidRule {
    fn id(&self) -> RuleId {
        URL_SCHEME_VALID_ID
    }

    fn severity(&self) -> RuleSeverity {
        RuleSeverity::Error
    }

    fn description(&self) -> &'static str {
        "`backend.url` is a parseable http(s) URL."
    }

    fn gate(&self) -> Option<&'static str> {
        Some("backend.enabled == true")
    }

    fn is_active(&self, config: &ItoConfig) -> bool {
        config.backend.enabled
    }

    fn check(&self, ctx: &RuleContext<'_>) -> Result<Vec<ValidationIssue>, CoreError> {
        let url = ctx.config.backend.url.trim();
        if url.is_empty() {
            return Ok(vec![issue_for_url_problem(
                "`backend.url` is empty while `backend.enabled = true`. \
                 The backend client requires an addressable endpoint to connect to.",
                "Set `backend.url` to the backend endpoint, for example `https://api.example.com`.",
            )]);
        }

        // Lightweight scheme check: avoid a `url` crate dependency.
        // Spec only requires accept http/https; reject everything else.
        let Some((scheme, rest)) = url.split_once("://") else {
            return Ok(vec![issue_for_url_problem(
                format!("`backend.url = \"{url}\"` is not a parseable URL (missing scheme)."),
                "Use a full URL with an `http://` or `https://` prefix.",
            )]);
        };

        match scheme {
            "http" | "https" => {}
            // `scheme` is a free-form string from `split_once`; we cannot
            // exhaustively enumerate every possible value, so a catch-all
            // arm is the only option here. (See `.agents/skills/rust-style`
            // — wildcards are allowed when the matched type is open.)
            other => {
                return Ok(vec![issue_for_url_problem(
                    format!(
                        "`backend.url = \"{url}\"` uses unsupported scheme `{other}`. \
                         Only `http` and `https` are accepted."
                    ),
                    "Switch the URL to `http://` or `https://`.",
                )]);
            }
        }

        if rest.trim().is_empty() {
            return Ok(vec![issue_for_url_problem(
                format!("`backend.url = \"{url}\"` has no host."),
                "Use a full URL with a host, for example `https://api.example.com`.",
            )]);
        }

        Ok(Vec::new())
    }
}

fn issue_for_url_problem(message: impl Into<String>, fix: impl Into<String>) -> ValidationIssue {
    let issue = error(".ito/config.json", message);
    let issue = with_rule_id(issue, URL_SCHEME_VALID_ID.as_str());
    with_metadata(
        issue,
        serde_json::json!({
            "fix": fix.into(),
            "config_key": "backend.url",
        }),
    )
}

// ── backend/project-org-repo-set ─────────────────────────────────────────

/// `backend/project-org-repo-set` — both `backend.project.org` and
/// `backend.project.repo` must be set when the backend is enabled.
pub(crate) struct ProjectOrgRepoSetRule;

impl Rule for ProjectOrgRepoSetRule {
    fn id(&self) -> RuleId {
        PROJECT_ORG_REPO_SET_ID
    }

    fn severity(&self) -> RuleSeverity {
        RuleSeverity::Error
    }

    fn description(&self) -> &'static str {
        "`backend.project.org` and `backend.project.repo` are both set."
    }

    fn gate(&self) -> Option<&'static str> {
        Some("backend.enabled == true")
    }

    fn is_active(&self, config: &ItoConfig) -> bool {
        config.backend.enabled
    }

    fn check(&self, ctx: &RuleContext<'_>) -> Result<Vec<ValidationIssue>, CoreError> {
        let mut issues = Vec::new();
        let project = &ctx.config.backend.project;
        let org_set = project
            .org
            .as_ref()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false);
        let repo_set = project
            .repo
            .as_ref()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false);

        if !org_set {
            issues.push(missing_identifier_issue("backend.project.org"));
        }
        if !repo_set {
            issues.push(missing_identifier_issue("backend.project.repo"));
        }
        Ok(issues)
    }
}

fn missing_identifier_issue(key: &'static str) -> ValidationIssue {
    let issue = error(
        ".ito/config.json",
        format!(
            "`{key}` is empty or unset. Multi-tenant backend routing requires both \
             `backend.project.org` and `backend.project.repo`."
        ),
    );
    let issue = with_rule_id(issue, PROJECT_ORG_REPO_SET_ID.as_str());
    with_metadata(
        issue,
        serde_json::json!({
            "fix": format!("Set `{key}` in `.ito/config.json`."),
            "config_key": key,
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process::{ProcessExecutionError, ProcessOutput, ProcessRequest, ProcessRunner};
    use crate::validate_repo::staged::StagedFiles;
    use ito_config::types::{BackendApiConfig, BackendProjectConfig, ItoConfig};
    use std::cell::RefCell;
    use std::time::Duration;
    use tempfile::TempDir;

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

    fn config(enabled: bool, url: &str, org: Option<&str>, repo: Option<&str>) -> ItoConfig {
        ItoConfig {
            backend: BackendApiConfig {
                enabled,
                url: url.to_string(),
                project: BackendProjectConfig {
                    org: org.map(str::to_string),
                    repo: repo.map(str::to_string),
                },
                ..BackendApiConfig::default()
            },
            ..ItoConfig::default()
        }
    }

    fn ctx_for<'a>(
        cfg: &'a ItoConfig,
        tmp: &'a TempDir,
        runner: &'a dyn ProcessRunner,
        staged: &'a StagedFiles,
    ) -> RuleContext<'a> {
        RuleContext::new(cfg, tmp.path(), staged, runner)
    }

    // ── activation ───────────────────────────────────────────────────────

    #[test]
    fn rules_inactive_when_backend_disabled() {
        let cfg = config(false, "https://example.com", Some("a"), Some("b"));
        assert!(!TokenNotCommittedRule.is_active(&cfg));
        assert!(!UrlSchemeValidRule.is_active(&cfg));
        assert!(!ProjectOrgRepoSetRule.is_active(&cfg));
    }

    #[test]
    fn rules_active_when_backend_enabled() {
        let cfg = config(true, "https://example.com", Some("a"), Some("b"));
        assert!(TokenNotCommittedRule.is_active(&cfg));
        assert!(UrlSchemeValidRule.is_active(&cfg));
        assert!(ProjectOrgRepoSetRule.is_active(&cfg));
    }

    // ── backend/url-scheme-valid ─────────────────────────────────────────

    #[test]
    fn url_scheme_passes_for_https() {
        let cfg = config(true, "https://api.example.com", Some("a"), Some("b"));
        let tmp = TempDir::new().unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let issues = UrlSchemeValidRule
            .check(&ctx_for(&cfg, &tmp, &runner, &staged))
            .unwrap();
        assert!(issues.is_empty());
    }

    #[test]
    fn url_scheme_passes_for_http() {
        let cfg = config(true, "http://localhost:9010", Some("a"), Some("b"));
        let tmp = TempDir::new().unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let issues = UrlSchemeValidRule
            .check(&ctx_for(&cfg, &tmp, &runner, &staged))
            .unwrap();
        assert!(issues.is_empty());
    }

    #[test]
    fn url_scheme_fails_for_ftp() {
        let cfg = config(true, "ftp://files.example.com", Some("a"), Some("b"));
        let tmp = TempDir::new().unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let issues = UrlSchemeValidRule
            .check(&ctx_for(&cfg, &tmp, &runner, &staged))
            .unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].level, "ERROR");
        assert!(issues[0].message.contains("ftp"));
    }

    #[test]
    fn url_scheme_fails_for_unparseable() {
        let cfg = config(true, "not a url", Some("a"), Some("b"));
        let tmp = TempDir::new().unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let issues = UrlSchemeValidRule
            .check(&ctx_for(&cfg, &tmp, &runner, &staged))
            .unwrap();
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("not a parseable URL"));
    }

    #[test]
    fn url_scheme_fails_for_scheme_only_no_host() {
        // Edge case: scheme is valid but `://` is followed by nothing.
        let cfg = config(true, "https://", Some("a"), Some("b"));
        let tmp = TempDir::new().unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let issues = UrlSchemeValidRule
            .check(&ctx_for(&cfg, &tmp, &runner, &staged))
            .unwrap();
        assert_eq!(issues.len(), 1);
        assert!(
            issues[0].message.contains("no host"),
            "expected 'no host' message; got: {}",
            issues[0].message,
        );
    }

    #[test]
    fn url_scheme_fails_for_empty() {
        let cfg = config(true, "", Some("a"), Some("b"));
        let tmp = TempDir::new().unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let issues = UrlSchemeValidRule
            .check(&ctx_for(&cfg, &tmp, &runner, &staged))
            .unwrap();
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("empty"));
    }

    // ── backend/project-org-repo-set ─────────────────────────────────────

    #[test]
    fn project_org_repo_passes_when_both_set() {
        let cfg = config(true, "https://x", Some("withakay"), Some("ito"));
        let tmp = TempDir::new().unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let issues = ProjectOrgRepoSetRule
            .check(&ctx_for(&cfg, &tmp, &runner, &staged))
            .unwrap();
        assert!(issues.is_empty());
    }

    #[test]
    fn project_org_repo_fails_when_org_missing() {
        let cfg = config(true, "https://x", None, Some("ito"));
        let tmp = TempDir::new().unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let issues = ProjectOrgRepoSetRule
            .check(&ctx_for(&cfg, &tmp, &runner, &staged))
            .unwrap();
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("backend.project.org"));
    }

    #[test]
    fn project_org_repo_fails_when_repo_missing() {
        let cfg = config(true, "https://x", Some("withakay"), None);
        let tmp = TempDir::new().unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let issues = ProjectOrgRepoSetRule
            .check(&ctx_for(&cfg, &tmp, &runner, &staged))
            .unwrap();
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("backend.project.repo"));
    }

    #[test]
    fn project_org_repo_reports_both_when_both_missing() {
        let cfg = config(true, "https://x", None, None);
        let tmp = TempDir::new().unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let issues = ProjectOrgRepoSetRule
            .check(&ctx_for(&cfg, &tmp, &runner, &staged))
            .unwrap();
        assert_eq!(issues.len(), 2);
    }

    // ── backend/token-not-committed ──────────────────────────────────────

    #[test]
    fn token_not_committed_passes_when_no_layer_sets_token() {
        // A fresh tempdir has no config files, so `cascading.layers` is
        // empty and the rule is silent.
        let cfg = config(true, "https://x", Some("a"), Some("b"));
        let tmp = TempDir::new().unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let issues = TokenNotCommittedRule
            .check(&ctx_for(&cfg, &tmp, &runner, &staged))
            .unwrap();
        assert!(issues.is_empty());
    }

    #[test]
    fn token_not_committed_fails_when_token_in_tracked_config() {
        // Fixture: `.ito/config.json` (tracked) sets backend.token.
        // ScriptedRunner returns success for `git ls-files --error-unmatch`,
        // simulating a tracked file.
        let cfg = config(true, "https://x", Some("a"), Some("b"));
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join(".ito")).unwrap();
        std::fs::write(
            tmp.path().join(".ito/config.json"),
            r#"{"backend": {"token": "secret-leak"}}"#,
        )
        .unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1))
            .with_rule(&["ls-files", "--error-unmatch"], ok_output(true, 0));
        let staged = StagedFiles::empty();
        let issues = TokenNotCommittedRule
            .check(&ctx_for(&cfg, &tmp, &runner, &staged))
            .unwrap();
        assert_eq!(issues.len(), 1, "expected one error, got {issues:?}");
        assert_eq!(issues[0].level, "ERROR");
        assert!(issues[0].message.contains("tracked by git"));
    }

    #[test]
    fn token_not_committed_passes_when_token_in_local_config() {
        // Fixture: `.ito/config.local.json` sets the token but is NOT
        // tracked (git ls-files --error-unmatch returns failure).
        let cfg = config(true, "https://x", Some("a"), Some("b"));
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join(".ito")).unwrap();
        std::fs::write(
            tmp.path().join(".ito/config.local.json"),
            r#"{"backend": {"token": "secret-but-local"}}"#,
        )
        .unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1));
        let staged = StagedFiles::empty();
        let issues = TokenNotCommittedRule
            .check(&ctx_for(&cfg, &tmp, &runner, &staged))
            .unwrap();
        assert!(issues.is_empty());
    }

    #[test]
    fn token_not_committed_emits_error_severity_directly() {
        // The rule constructs ERROR-level issues via `error()` rather than
        // returning WARNINGs that strict mode would promote. This test
        // pins the severity contract so a future refactor can't silently
        // demote the rule to WARNING.
        let cfg = config(true, "https://x", Some("a"), Some("b"));
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join(".ito")).unwrap();
        std::fs::write(
            tmp.path().join(".ito/config.json"),
            r#"{"backend": {"token": "leak"}}"#,
        )
        .unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1))
            .with_rule(&["ls-files", "--error-unmatch"], ok_output(true, 0));
        let staged = StagedFiles::empty();
        let issues = TokenNotCommittedRule
            .check(&ctx_for(&cfg, &tmp, &runner, &staged))
            .unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].level, "ERROR");
        assert_eq!(TokenNotCommittedRule.severity(), RuleSeverity::Error);
    }

    #[test]
    fn token_not_committed_strict_flag_does_not_weaken_severity() {
        // End-to-end check: run the rule through the engine path with
        // both `strict = false` and `strict = true` and confirm the
        // emitted issue has level "ERROR" in both cases. The engine's
        // strict flag promotes WARNINGs to errors but never demotes an
        // existing ERROR.
        use crate::validate_repo::run_repo_validation;

        let mut cfg = config(true, "https://x", Some("a"), Some("b"));
        // Disable every other gated rule so only `backend/*` rules fire
        // under this fixture.
        cfg.changes.coordination_branch.storage = ito_config::types::CoordinationStorage::Embedded;
        cfg.worktrees.enabled = false;

        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join(".ito")).unwrap();
        std::fs::write(
            tmp.path().join(".ito/config.json"),
            r#"{"backend": {"token": "leak"}}"#,
        )
        .unwrap();
        let runner = ScriptedRunner::new(ok_output(false, 1))
            .with_rule(&["ls-files", "--error-unmatch"], ok_output(true, 0));
        let staged = StagedFiles::empty();

        for strict in [false, true] {
            let report = run_repo_validation(&cfg, tmp.path(), &staged, &runner, strict);
            let token_issues: Vec<_> = report
                .issues
                .iter()
                .filter(|i| i.rule_id.as_deref() == Some(TOKEN_NOT_COMMITTED_ID.as_str()))
                .collect();
            assert_eq!(
                token_issues.len(),
                1,
                "expected one token issue (strict={strict}); got {token_issues:?}",
            );
            assert_eq!(
                token_issues[0].level, "ERROR",
                "token-not-committed must always emit ERROR (strict={strict})",
            );
        }
    }
}
