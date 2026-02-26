//! Harness context inference helpers.
//!
//! These helpers infer an Ito target (change or module) from local signals so
//! harness hooks/plugins can recover context after compaction.
//!
//! Inference priority:
//!
//! 1. Current working directory path segments.
//! 2. Current git branch name (if available).
//!
//! When multiple change-id-like path segments are present, the last match wins.

use crate::errors::CoreResult;
use ito_common::id::{parse_change_id, parse_module_id};
use std::path::{Component, Path};

/// Kind of Ito target inferred for a harness session.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InferredItoTargetKind {
    /// A change id (e.g. `023-07_harness-context-inference`).
    Change,
    /// A module id (e.g. `023`).
    Module,
}

/// Inferred Ito target (change or module).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct InferredItoTarget {
    /// Target kind.
    pub kind: InferredItoTargetKind,
    /// Canonical identifier string.
    pub id: String,
}

/// Inference result suitable for harness hooks and plugins.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct HarnessContextInference {
    /// Inferred Ito target (or none when inference is inconclusive).
    pub target: Option<InferredItoTarget>,
    /// Continuation guidance appropriate to the inferred target.
    pub nudge: String,
}

/// Infer the current Ito target for a harness session.
///
/// This is a best-effort, deterministic inference based on local signals:
///
/// 1. Current working directory path segments.
/// 2. Current git branch name (if `git` is available).
pub fn infer_context_from_cwd(cwd: &Path) -> CoreResult<HarnessContextInference> {
    let target = infer_target_from_path(cwd).or_else(|| infer_target_from_git_branch(cwd));
    let nudge = nudge_for_target(target.as_ref());
    Ok(HarnessContextInference { target, nudge })
}

fn nudge_for_target(target: Option<&InferredItoTarget>) -> String {
    let Some(target) = target else {
        return "No Ito change/module inferred. Re-establish the target (try: `ito list --ready`, then `ito tasks next <change-id>`).".to_string();
    };

    match target.kind {
        InferredItoTargetKind::Change => format!(
            "If you are still implementing change {id}, continue now: `ito tasks next {id}` (then `ito tasks start/complete` as you progress).",
            id = target.id
        ),
        InferredItoTargetKind::Module => format!(
            "If you are still working in module {id}, continue now: `ito show module {id}` then run `ito tasks next <change-id>` for the next ready change.",
            id = target.id
        ),
    }
}

fn infer_target_from_path(cwd: &Path) -> Option<InferredItoTarget> {
    let mut change: Option<String> = None;
    let mut module: Option<String> = None;

    let mut last_was_ito_dir = false;
    let mut expect_module_dir = false;

    for component in cwd.components() {
        let Component::Normal(seg) = component else {
            continue;
        };
        let Some(seg) = seg.to_str() else {
            last_was_ito_dir = false;
            expect_module_dir = false;
            continue;
        };

        if expect_module_dir {
            if let Ok(parsed) = parse_module_id(seg) {
                module = Some(parsed.module_id.as_str().to_string());
            }
            expect_module_dir = false;
        }

        if let Ok(parsed) = parse_change_id(seg) {
            change = Some(parsed.canonical.as_str().to_string());
        }

        if last_was_ito_dir && seg == "modules" {
            expect_module_dir = true;
        }

        last_was_ito_dir = seg == ".ito";
    }

    if let Some(id) = change {
        return Some(InferredItoTarget {
            kind: InferredItoTargetKind::Change,
            id,
        });
    }

    let id = module?;
    Some(InferredItoTarget {
        kind: InferredItoTargetKind::Module,
        id,
    })
}

fn infer_target_from_git_branch(cwd: &Path) -> Option<InferredItoTarget> {
    let branch = git_output(cwd, &["branch", "--show-current"])?
        .trim()
        .to_string();
    if branch.is_empty() {
        return None;
    }

    if let Ok(parsed) = parse_change_id(&branch) {
        return Some(InferredItoTarget {
            kind: InferredItoTargetKind::Change,
            id: parsed.canonical.as_str().to_string(),
        });
    }

    if let Ok(parsed) = parse_module_id(&branch) {
        return Some(InferredItoTarget {
            kind: InferredItoTargetKind::Module,
            id: parsed.module_id.as_str().to_string(),
        });
    }

    None
}

fn git_output(cwd: &Path, args: &[&str]) -> Option<String> {
    let mut command = std::process::Command::new("git");
    command.args(args).current_dir(cwd);

    // Ignore injected git environment variables to avoid surprising repository selection.
    for (k, _) in std::env::vars_os() {
        let k = k.to_string_lossy();
        if k.starts_with("GIT_") {
            command.env_remove(k.as_ref());
        }
    }

    let output = command.output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).to_string())
}
