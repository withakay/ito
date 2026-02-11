//! Persistent Ralph loop state.
//!
//! The Ralph loop stores a small amount of JSON state on disk so users can:
//! - inspect iteration history (duration, whether completion was detected)
//! - add or clear additional context that is appended to future prompts
//!
//! State is stored under `.ito/.state/ralph/<change-id>/`.

use crate::errors::{CoreError, CoreResult};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
/// One historical record for a Ralph iteration.
pub struct RalphHistoryEntry {
    /// Wall clock time (ms since epoch) when the iteration finished.
    pub timestamp: i64,
    /// Duration (ms) the harness run took.
    pub duration: i64,
    /// Whether the completion promise token was observed in harness stdout.
    pub completion_promise_found: bool,
    /// Number of changed files in the git working tree after the iteration.
    pub file_changes_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
/// Saved state for a Ralph loop scoped to a specific change.
pub struct RalphState {
    /// Change id this state belongs to.
    pub change_id: String,
    /// Last completed iteration number.
    pub iteration: u32,
    /// History entries for completed iterations.
    pub history: Vec<RalphHistoryEntry>,
    /// Display path for the context file (used by some UIs).
    pub context_file: String,
}

/// Return the on-disk directory for Ralph state for `change_id`.
pub fn ralph_state_dir(ito_path: &Path, change_id: &str) -> PathBuf {
    if !is_safe_change_id_segment(change_id) {
        return ito_path
            .join(".state")
            .join("ralph")
            .join("invalid-change-id");
    }
    ito_path.join(".state").join("ralph").join(change_id)
}

/// Return the path to `state.json` for `change_id`.
pub fn ralph_state_json_path(ito_path: &Path, change_id: &str) -> PathBuf {
    ralph_state_dir(ito_path, change_id).join("state.json")
}

/// Return the path to `context.md` for `change_id`.
pub fn ralph_context_path(ito_path: &Path, change_id: &str) -> PathBuf {
    ralph_state_dir(ito_path, change_id).join("context.md")
}

/// Load saved state for `change_id`.
pub fn load_state(ito_path: &Path, change_id: &str) -> CoreResult<Option<RalphState>> {
    let p = ralph_state_json_path(ito_path, change_id);
    if !p.exists() {
        return Ok(None);
    }
    let raw = ito_common::io::read_to_string_std(&p)
        .map_err(|e| CoreError::io(format!("reading {}", p.display()), e))?;
    let state = serde_json::from_str(&raw)
        .map_err(|e| CoreError::Parse(format!("JSON error parsing {p}: {e}", p = p.display())))?;
    Ok(Some(state))
}

/// Persist `state` for `change_id`.
pub fn save_state(ito_path: &Path, change_id: &str, state: &RalphState) -> CoreResult<()> {
    let dir = ralph_state_dir(ito_path, change_id);
    ito_common::io::create_dir_all_std(&dir)
        .map_err(|e| CoreError::io(format!("creating directory {}", dir.display()), e))?;
    let p = ralph_state_json_path(ito_path, change_id);
    let raw = serde_json::to_string_pretty(state)
        .map_err(|e| CoreError::Parse(format!("JSON error serializing state: {e}")))?;
    ito_common::io::write_std(&p, raw)
        .map_err(|e| CoreError::io(format!("writing {}", p.display()), e))?;
    Ok(())
}

/// Load the saved context markdown for `change_id`.
///
/// Missing files return an empty string.
pub fn load_context(ito_path: &Path, change_id: &str) -> CoreResult<String> {
    let p = ralph_context_path(ito_path, change_id);
    if !p.exists() {
        return Ok(String::new());
    }
    ito_common::io::read_to_string_std(&p)
        .map_err(|e| CoreError::io(format!("reading {}", p.display()), e))
}

/// Append `text` to the saved context for `change_id`.
///
/// Empty/whitespace-only input is ignored.
pub fn append_context(ito_path: &Path, change_id: &str, text: &str) -> CoreResult<()> {
    let dir = ralph_state_dir(ito_path, change_id);
    ito_common::io::create_dir_all_std(&dir)
        .map_err(|e| CoreError::io(format!("creating directory {}", dir.display()), e))?;
    let p = ralph_context_path(ito_path, change_id);
    let existing_result = ito_common::io::read_to_string_std(&p);
    let mut existing = match existing_result {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(CoreError::io(format!("reading {}", p.display()), e)),
    };

    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(());
    }

    if !existing.trim().is_empty() {
        existing.push_str("\n\n");
    }
    existing.push_str(trimmed);
    existing.push('\n');
    ito_common::io::write_std(&p, existing)
        .map_err(|e| CoreError::io(format!("writing {}", p.display()), e))?;
    Ok(())
}

/// Clear the saved context for `change_id`.
pub fn clear_context(ito_path: &Path, change_id: &str) -> CoreResult<()> {
    let dir = ralph_state_dir(ito_path, change_id);
    ito_common::io::create_dir_all_std(&dir)
        .map_err(|e| CoreError::io(format!("creating directory {}", dir.display()), e))?;
    let p = ralph_context_path(ito_path, change_id);
    ito_common::io::write_std(&p, "")
        .map_err(|e| CoreError::io(format!("writing {}", p.display()), e))?;
    Ok(())
}

fn is_safe_change_id_segment(change_id: &str) -> bool {
    let change_id = change_id.trim();
    if change_id.is_empty() {
        return false;
    }
    if change_id.len() > 256 {
        return false;
    }
    if change_id.contains('/') || change_id.contains('\\') || change_id.contains("..") {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ralph_state_dir_uses_safe_fallback_for_invalid_change_ids() {
        let ito = std::path::Path::new("/tmp/repo/.ito");
        let path = ralph_state_dir(ito, "../escape");
        assert!(path.ends_with(".state/ralph/invalid-change-id"));
    }
}
