use miette::{Result, miette};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RalphHistoryEntry {
    pub timestamp: i64,
    pub duration: i64,
    pub completion_promise_found: bool,
    pub file_changes_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RalphState {
    pub change_id: String,
    pub iteration: u32,
    pub history: Vec<RalphHistoryEntry>,
    pub context_file: String,
}

pub fn ralph_state_dir(ito_path: &Path, change_id: &str) -> PathBuf {
    ito_path.join(".state").join("ralph").join(change_id)
}

pub fn ralph_state_json_path(ito_path: &Path, change_id: &str) -> PathBuf {
    ralph_state_dir(ito_path, change_id).join("state.json")
}

pub fn ralph_context_path(ito_path: &Path, change_id: &str) -> PathBuf {
    ralph_state_dir(ito_path, change_id).join("context.md")
}

pub fn load_state(ito_path: &Path, change_id: &str) -> Result<Option<RalphState>> {
    let p = ralph_state_json_path(ito_path, change_id);
    if !p.exists() {
        return Ok(None);
    }
    let raw = ito_common::io::read_to_string(&p)?;
    let state = serde_json::from_str(&raw)
        .map_err(|e| miette!("JSON error parsing {p}: {e}", p = p.display()))?;
    Ok(Some(state))
}

pub fn save_state(ito_path: &Path, change_id: &str, state: &RalphState) -> Result<()> {
    let dir = ralph_state_dir(ito_path, change_id);
    ito_common::io::create_dir_all(&dir)?;
    let p = ralph_state_json_path(ito_path, change_id);
    let raw = serde_json::to_string_pretty(state)
        .map_err(|e| miette!("JSON error serializing state: {e}"))?;
    ito_common::io::write(&p, raw)?;
    Ok(())
}

pub fn load_context(ito_path: &Path, change_id: &str) -> Result<String> {
    let p = ralph_context_path(ito_path, change_id);
    if !p.exists() {
        return Ok(String::new());
    }
    ito_common::io::read_to_string(&p)
}

pub fn append_context(ito_path: &Path, change_id: &str, text: &str) -> Result<()> {
    let dir = ralph_state_dir(ito_path, change_id);
    ito_common::io::create_dir_all(&dir)?;
    let p = ralph_context_path(ito_path, change_id);
    let mut existing = ito_common::io::read_to_string_optional(&p)?.unwrap_or_default();

    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(());
    }

    if !existing.trim().is_empty() {
        existing.push_str("\n\n");
    }
    existing.push_str(trimmed);
    existing.push('\n');
    ito_common::io::write(&p, existing)?;
    Ok(())
}

pub fn clear_context(ito_path: &Path, change_id: &str) -> Result<()> {
    let dir = ralph_state_dir(ito_path, change_id);
    ito_common::io::create_dir_all(&dir)?;
    let p = ralph_context_path(ito_path, change_id);
    ito_common::io::write(&p, "")?;
    Ok(())
}
