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

    #[test]
    fn save_and_load_state_round_trip() {
        let td = tempfile::tempdir().unwrap();
        let ito = td.path().join(".ito");
        let change_id = "001-01_test";
        let state = RalphState {
            change_id: change_id.to_string(),
            iteration: 5,
            history: vec![RalphHistoryEntry {
                timestamp: 1234567890,
                duration: 5000,
                completion_promise_found: true,
                file_changes_count: 3,
            }],
            context_file: ".ito/.state/ralph/001-01_test/context.md".to_string(),
        };
        save_state(&ito, change_id, &state).unwrap();
        let loaded = load_state(&ito, change_id).unwrap();
        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.change_id, state.change_id);
        assert_eq!(loaded.iteration, state.iteration);
        assert_eq!(loaded.history.len(), state.history.len());
        assert_eq!(loaded.history[0].timestamp, state.history[0].timestamp);
        assert_eq!(loaded.history[0].duration, state.history[0].duration);
        assert_eq!(
            loaded.history[0].completion_promise_found,
            state.history[0].completion_promise_found
        );
        assert_eq!(
            loaded.history[0].file_changes_count,
            state.history[0].file_changes_count
        );
        assert_eq!(loaded.context_file, state.context_file);
    }

    #[test]
    fn load_state_returns_none_when_missing() {
        let td = tempfile::tempdir().unwrap();
        let ito = td.path().join(".ito");
        let result = load_state(&ito, "nonexistent").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn is_safe_change_id_segment_rejects_empty() {
        let ito = tempfile::tempdir().unwrap();
        let ito_path = ito.path().join(".ito");
        let path = ralph_state_dir(&ito_path, "");
        assert!(path.ends_with(".state/ralph/invalid-change-id"));
    }

    #[test]
    fn is_safe_change_id_segment_rejects_too_long() {
        let ito = tempfile::tempdir().unwrap();
        let ito_path = ito.path().join(".ito");
        let long_id = "a".repeat(257);
        let path = ralph_state_dir(&ito_path, &long_id);
        assert!(path.ends_with(".state/ralph/invalid-change-id"));
    }

    #[test]
    fn is_safe_change_id_segment_rejects_backslash() {
        let ito = tempfile::tempdir().unwrap();
        let ito_path = ito.path().join(".ito");
        let path = ralph_state_dir(&ito_path, "foo\\bar");
        assert!(path.ends_with(".state/ralph/invalid-change-id"));
    }

    #[test]
    fn is_safe_change_id_segment_accepts_valid() {
        let ito = tempfile::tempdir().unwrap();
        let ito_path = ito.path().join(".ito");
        let path = ralph_state_dir(&ito_path, "003-05_my-change");
        assert!(path.ends_with("003-05_my-change"));
    }

    #[test]
    fn append_context_no_op_on_whitespace() {
        let td = tempfile::tempdir().unwrap();
        let ito = td.path().join(".ito");
        let change_id = "001-01_test";
        append_context(&ito, change_id, "   \n  ").unwrap();
        let context_path = ralph_context_path(&ito, change_id);
        if context_path.exists() {
            let content = ito_common::io::read_to_string_std(&context_path).unwrap();
            assert!(content.is_empty());
        }
    }

    #[test]
    fn load_context_returns_empty_when_missing() {
        let td = tempfile::tempdir().unwrap();
        let ito = td.path().join(".ito");
        let result = load_context(&ito, "nonexistent").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn ralph_state_json_path_correct() {
        let ito = std::path::Path::new("/tmp/repo/.ito");
        let path = ralph_state_json_path(ito, "001-01_test");
        assert!(path.ends_with("state.json"));
    }

    #[test]
    fn ralph_context_path_correct() {
        let ito = std::path::Path::new("/tmp/repo/.ito");
        let path = ralph_context_path(ito, "001-01_test");
        assert!(path.ends_with("context.md"));
    }
}
