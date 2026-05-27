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
