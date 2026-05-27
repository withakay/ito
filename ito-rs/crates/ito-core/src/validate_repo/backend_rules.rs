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
#[path = "backend_rules_tests.rs"]
mod backend_rules_tests;
