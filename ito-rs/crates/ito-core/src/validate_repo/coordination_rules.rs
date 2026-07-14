//! Rules under the `coordination/*` namespace.
//!
//! These rules are gated on `changes.coordination_branch.storage` and
//! validate the coordination-worktree wiring used by Ito to share state
//! across change worktrees.
//!
//! Four rules live here:
//!
//! - `coordination/symlinks-wired` (ERROR) — every directory listed in
//!   [`crate::coordination::COORDINATION_DIRS`] under `.ito/` MUST be a
//!   symlink resolving into the coordination worktree.
//! - `coordination/gitignore-entries` (WARNING) — `.gitignore` MUST contain
//!   each canonical [`crate::coordination::gitignore_entries`] line.
//! - `coordination/staged-symlinked-paths` (ERROR) — staged paths under
//!   any coordination directory belong to the coordination branch, not the
//!   working branch.
//! - `coordination/branch-name-set` (WARNING) — the coordination branch
//!   name must be non-empty and SHOULD live under `ito/internal/`.

use std::path::Path;

use ito_config::types::{CoordinationStorage, ItoConfig};

use crate::coordination::{
    COORDINATION_DIRS, check_coordination_health, format_health_message, gitignore_entries,
};
use crate::coordination_worktree::resolved_coordination_worktree_path;
use crate::errors::CoreError;
use crate::validate::{ValidationIssue, error, warning, with_metadata, with_rule_id};

use super::rule::{Rule, RuleContext, RuleId, RuleSeverity};

const SYMLINKS_WIRED_ID: RuleId = RuleId::new("coordination/symlinks-wired");
const GITIGNORE_ENTRIES_ID: RuleId = RuleId::new("coordination/gitignore-entries");
const STAGED_SYMLINKED_PATHS_ID: RuleId = RuleId::new("coordination/staged-symlinked-paths");
const BRANCH_NAME_SET_ID: RuleId = RuleId::new("coordination/branch-name-set");

const COORD_BRANCH_PREFIX: &str = "ito/internal/";

/// True when coordination storage requires worktree wiring.
fn storage_is_worktree(config: &ItoConfig) -> bool {
    match config.changes.coordination_branch.storage {
        CoordinationStorage::Worktree => true,
        CoordinationStorage::Embedded => false,
    }
}

// ── coordination/symlinks-wired ──────────────────────────────────────────

/// `coordination/symlinks-wired` — wraps [`check_coordination_health`].
pub(crate) struct SymlinksWiredRule;

impl Rule for SymlinksWiredRule {
    fn id(&self) -> RuleId {
        SYMLINKS_WIRED_ID
    }

    fn severity(&self) -> RuleSeverity {
        RuleSeverity::Error
    }

    fn description(&self) -> &'static str {
        "Coordination directories under .ito/ are wired as symlinks into the coordination worktree."
    }

    fn gate(&self) -> Option<&'static str> {
        Some("changes.coordination_branch.storage == worktree")
    }

    fn is_active(&self, config: &ItoConfig) -> bool {
        storage_is_worktree(config)
    }

    fn check(&self, ctx: &RuleContext<'_>) -> Result<Vec<ValidationIssue>, CoreError> {
        let coord = &ctx.config.changes.coordination_branch;
        let ito_path = ctx.project_root.join(".ito");

        // `allow_local_fallback = false`: when the project has no `origin`
        // remote and no `backend.project` config, surface that as a
        // targeted error rather than silently validating a phantom hash-
        // derived path. The `sync` code path uses the same setting for
        // exactly this reason.
        let worktree_path = match resolved_coordination_worktree_path(
            ctx.project_root,
            &ito_path,
            ctx.config,
            false,
        ) {
            Ok(p) => p,
            Err(err) => {
                let issue = error(
                    ".ito",
                    format!(
                        "Cannot resolve the coordination worktree path. \
                         Why: coordination storage is `worktree` but neither `origin` \
                         remote nor `backend.project` (org/repo) is configured. \
                         Underlying error: {err}",
                    ),
                );
                let issue = with_rule_id(issue, SYMLINKS_WIRED_ID.as_str());
                let issue = with_metadata(
                    issue,
                    serde_json::json!({
                        "fix": "Add an `origin` remote (`git remote add origin <url>`) or set \
                                `backend.project.org` and `backend.project.repo` in \
                                `.ito/config.json`.",
                    }),
                );
                return Ok(vec![issue]);
            }
        };

        let worktree_ito_path = worktree_path.join(".ito");
        let status = check_coordination_health(&ito_path, &worktree_ito_path, &coord.storage);

        let Some(message) = format_health_message(&status) else {
            // Healthy or Embedded — nothing to report.
            return Ok(Vec::new());
        };

        // Wrap the underlying health-check message with explicit Why/Fix
        // framing so the rule output satisfies `ito-rs/AGENTS.md` error
        // quality rules even when `format_health_message` is reused
        // elsewhere with a leaner format.
        let wrapped = format!(
            "Coordination symlinks under `.ito/` are not wired. \
             Why: storage mode `worktree` requires each coordination directory \
             to be a symlink resolving into the coordination worktree. \
             Details: {message}",
        );

        let issue = error(".ito", wrapped);
        let issue = with_rule_id(issue, SYMLINKS_WIRED_ID.as_str());
        let issue = with_metadata(
            issue,
            serde_json::json!({
                "fix": "Run `ito sync` to repair coordination symlinks, then rerun `ito validate repo`.",
                "expected_worktree_ito_path": worktree_ito_path.to_string_lossy(),
            }),
        );
        Ok(vec![issue])
    }
}

// ── coordination/gitignore-entries ───────────────────────────────────────

/// `coordination/gitignore-entries` — every canonical `.ito/<dir>` line is
/// present in `.gitignore`.
pub(crate) struct GitignoreEntriesRule;

impl Rule for GitignoreEntriesRule {
    fn id(&self) -> RuleId {
        GITIGNORE_ENTRIES_ID
    }

    fn severity(&self) -> RuleSeverity {
        RuleSeverity::Warning
    }

    fn description(&self) -> &'static str {
        "Canonical `.ito/<dir>` entries are listed in `.gitignore`."
    }

    fn gate(&self) -> Option<&'static str> {
        Some("changes.coordination_branch.storage == worktree")
    }

    fn is_active(&self, config: &ItoConfig) -> bool {
        storage_is_worktree(config)
    }

    fn check(&self, ctx: &RuleContext<'_>) -> Result<Vec<ValidationIssue>, CoreError> {
        let gitignore_path = ctx.project_root.join(".gitignore");
        let existing = read_gitignore(&gitignore_path)?;

        let mut issues = Vec::new();
        for entry in gitignore_entries() {
            if existing.lines().any(|l| l.trim() == *entry) {
                continue;
            }
            let issue = warning(
                ".gitignore",
                format!("Canonical coordination entry `{entry}` is missing from `.gitignore`."),
            );
            let issue = with_rule_id(issue, GITIGNORE_ENTRIES_ID.as_str());
            let issue = with_metadata(
                issue,
                serde_json::json!({
                    "fix": format!("Append `{entry}` to `.gitignore`."),
                    "entry": entry,
                }),
            );
            issues.push(issue);
        }

        Ok(issues)
    }
}

fn read_gitignore(path: &Path) -> Result<String, CoreError> {
    if !path.exists() {
        return Ok(String::new());
    }
    std::fs::read_to_string(path).map_err(|e| {
        CoreError::io(
            format!(
                "Cannot read `{p}` for `coordination/gitignore-entries`.\n\
                 Why: filesystem error.\n\
                 Fix: confirm read permissions on `{p}`.",
                p = path.display(),
            ),
            e,
        )
    })
}

// ── coordination/staged-symlinked-paths ──────────────────────────────────

/// `coordination/staged-symlinked-paths` — staged paths under any
/// coordination directory belong to the coordination branch, not the
/// working branch.
pub(crate) struct StagedSymlinkedPathsRule;

impl Rule for StagedSymlinkedPathsRule {
    fn id(&self) -> RuleId {
        STAGED_SYMLINKED_PATHS_ID
    }

    fn severity(&self) -> RuleSeverity {
        RuleSeverity::Error
    }

    fn description(&self) -> &'static str {
        "No staged paths under `.ito/{changes,specs,modules,workflows,audit}` (those belong to the coordination branch)."
    }

    fn gate(&self) -> Option<&'static str> {
        Some("changes.coordination_branch.storage == worktree && staged context present")
    }

    fn is_active(&self, config: &ItoConfig) -> bool {
        storage_is_worktree(config)
    }

    fn check(&self, ctx: &RuleContext<'_>) -> Result<Vec<ValidationIssue>, CoreError> {
        if ctx.staged.is_empty() {
            return Ok(Vec::new());
        }

        let mut issues = Vec::new();
        for staged in ctx.staged.iter() {
            // `Path::strip_prefix` does component-wise comparison, so
            // ".ito-extra/foo" does not match ".ito" and is correctly
            // skipped here.
            let Some(rest) = staged.strip_prefix(".ito").ok() else {
                continue;
            };
            // `staged == ".ito"` itself: the strip leaves an empty path
            // with no components — not a coordination-dir violation, so
            // skip silently.
            let Some(first) = rest.components().next() else {
                continue;
            };
            let segment = match first {
                std::path::Component::Normal(s) => s.to_string_lossy(),
                std::path::Component::Prefix(_)
                | std::path::Component::RootDir
                | std::path::Component::CurDir
                | std::path::Component::ParentDir => continue,
            };
            if !COORDINATION_DIRS.iter().any(|dir| *dir == segment.as_ref()) {
                continue;
            }

            let path_display = staged.to_string_lossy().into_owned();
            let issue = error(
                path_display.clone(),
                format!(
                    "Staged path `{path_display}` lives under a coordination directory. \
                     Coordination paths belong to the coordination branch, not the working branch.",
                ),
            );
            let issue = with_rule_id(issue, STAGED_SYMLINKED_PATHS_ID.as_str());
            let issue = with_metadata(
                issue,
                serde_json::json!({
                    "fix": "Unstage the path; coordination edits flow through `ito sync` and the \
                            coordination worktree, not direct commits to the working branch.",
                    "coordination_dir": segment.as_ref(),
                }),
            );
            issues.push(issue);
        }

        Ok(issues)
    }
}

// ── coordination/branch-name-set ─────────────────────────────────────────

/// `coordination/branch-name-set` — the coordination branch name is non-empty
/// and follows the `ito/internal/` convention.
pub(crate) struct BranchNameSetRule;

impl Rule for BranchNameSetRule {
    fn id(&self) -> RuleId {
        BRANCH_NAME_SET_ID
    }

    fn severity(&self) -> RuleSeverity {
        RuleSeverity::Warning
    }

    fn description(&self) -> &'static str {
        "Coordination branch name is non-empty and follows the `ito/internal/` convention."
    }

    fn gate(&self) -> Option<&'static str> {
        // Always active — even with embedded storage, a misconfigured
        // branch name can cause confusion when a project flips to worktree
        // storage later.
        None
    }

    fn is_active(&self, _config: &ItoConfig) -> bool {
        true
    }

    fn check(&self, ctx: &RuleContext<'_>) -> Result<Vec<ValidationIssue>, CoreError> {
        let name = ctx.config.changes.coordination_branch.name.trim();
        let mut issues = Vec::new();

        if name.is_empty() {
            let issue = warning(
                ".ito/config.json",
                "`changes.coordination_branch.name` is empty.",
            );
            let issue = with_rule_id(issue, BRANCH_NAME_SET_ID.as_str());
            issues.push(with_metadata(
                issue,
                serde_json::json!({
                    "fix": format!(
                        "Set `changes.coordination_branch.name` to a value under `{COORD_BRANCH_PREFIX}` (e.g. `ito/internal/changes`)."
                    ),
                }),
            ));
            return Ok(issues);
        }

        if !name.starts_with(COORD_BRANCH_PREFIX) {
            let issue = warning(
                ".ito/config.json",
                format!(
                    "`changes.coordination_branch.name = \"{name}\"` does not follow the \
                     `{COORD_BRANCH_PREFIX}*` convention.",
                ),
            );
            let issue = with_rule_id(issue, BRANCH_NAME_SET_ID.as_str());
            issues.push(with_metadata(
                issue,
                serde_json::json!({
                    "fix": format!(
                        "Rename the branch under `{COORD_BRANCH_PREFIX}` (e.g. \
                         `{COORD_BRANCH_PREFIX}changes`) to keep coordination branches in the \
                         workspace's internal namespace.",
                    ),
                    "current": name,
                }),
            ));
        }

        Ok(issues)
    }
}

#[cfg(test)]
#[path = "coordination_rules_tests.rs"]
mod coordination_rules_tests;
