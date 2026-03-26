//! Telemetry logging for Ito.
//!
//! This crate records low-volume execution events to a JSONL file under the
//! user's config directory. The output is designed to be append-only and
//! resilient: failures to read/write telemetry should never break the main
//! command flow.
//!
//! The logger intentionally stores only coarse metadata:
//! - a stable-but-anonymized `project_id` derived from a per-user salt
//! - a `session_id` persisted under `.ito/session.json` when available

#![warn(missing_docs)]

use chrono::{SecondsFormat, Utc};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

const EVENT_VERSION: u32 = 1;
const SALT_FILE_NAME: &str = "telemetry_salt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// High-level command outcome used for telemetry.
pub enum Outcome {
    /// Command finished successfully.
    Success,
    /// Command finished with an error.
    Error,
}

impl Outcome {
    fn as_str(self) -> &'static str {
        match self {
            Outcome::Success => "success",
            Outcome::Error => "error",
        }
    }
}

#[derive(Debug, Clone)]
/// Append-only telemetry logger.
///
/// Construct with [`Logger::new`]. When logging is disabled or cannot be
/// initialized, `new` returns `None`.
pub struct Logger {
    file_path: PathBuf,
    ito_version: String,
    command_id: String,
    session_id: String,
    project_id: String,
    pid: u32,
}

impl Logger {
    /// Create a logger if telemetry is enabled.
    ///
    /// Returns `None` when:
    /// - telemetry is disabled (`ITO_DISABLE_LOGGING`)
    /// - the config directory is not available
    /// - the telemetry salt cannot be read/created
    pub fn new(
        config_dir: Option<PathBuf>,
        project_root: &Path,
        ito_path: Option<&Path>,
        command_id: &str,
        ito_version: &str,
    ) -> Option<Self> {
        if logging_disabled() {
            log::debug!("telemetry: disabled by ITO_DISABLE_LOGGING");
            return None;
        }

        let config_dir = config_dir?;
        let salt_path = config_dir.join(SALT_FILE_NAME);
        let salt = load_or_create_salt(&salt_path)?;
        let project_id = compute_project_id(&salt, project_root);
        let session_id = resolve_session_id(ito_path);
        let file_path = log_file_path(&config_dir, &project_id, &session_id);

        if let Some(parent) = file_path.parent()
            && let Err(e) = std::fs::create_dir_all(parent)
        {
            log::debug!("telemetry: create_dir_all failed: {e}");
        }

        Some(Self {
            file_path,
            ito_version: ito_version.to_string(),
            command_id: command_id.to_string(),
            session_id,
            project_id,
            pid: std::process::id(),
        })
    }

    /// Session identifier for this execution.
    ///
    /// When an `.ito/` directory exists, this is persisted in
    /// `.ito/session.json` to allow grouping commands across runs.
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Stable anonymized project identifier.
    ///
    /// This is derived from `project_root` using a per-user random salt.
    pub fn project_id(&self) -> &str {
        &self.project_id
    }

    /// Write a `command_start` event.
    pub fn write_start(&self) {
        self.write_event("command_start", None, None);
    }

    /// Write a `command_end` event.
    pub fn write_end(&self, outcome: Outcome, duration: Duration) {
        let duration_ms = duration.as_millis();
        let duration_ms = u64::try_from(duration_ms).unwrap_or(u64::MAX);
        self.write_event("command_end", Some(outcome), Some(duration_ms));
    }

    fn write_event(
        &self,
        event_type: &'static str,
        outcome: Option<Outcome>,
        duration_ms: Option<u64>,
    ) {
        #[derive(Serialize)]
        struct Event {
            event_version: u32,
            event_id: String,
            timestamp: String,
            event_type: &'static str,
            ito_version: String,
            command_id: String,
            session_id: String,
            project_id: String,
            pid: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            outcome: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            duration_ms: Option<u64>,
        }

        let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
        let event = Event {
            event_version: EVENT_VERSION,
            event_id: uuid::Uuid::new_v4().to_string(),
            timestamp,
            event_type,
            ito_version: self.ito_version.clone(),
            command_id: self.command_id.clone(),
            session_id: self.session_id.clone(),
            project_id: self.project_id.clone(),
            pid: self.pid,
            outcome: outcome.map(|o| o.as_str().to_string()),
            duration_ms,
        };

        let Ok(line) = serde_json::to_string(&event) else {
            log::debug!("telemetry: failed to serialize event");
            return;
        };

        let Ok(mut f) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
        else {
            log::debug!("telemetry: failed to open log file");
            return;
        };
        if let Err(e) = writeln!(f, "{line}") {
            log::debug!("telemetry: failed to append log line: {e}");
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SessionState {
    session_id: String,
    created_at: String,
}

fn resolve_session_id(ito_path: Option<&Path>) -> String {
    let session_id = new_session_id();

    let Some(ito_path) = ito_path else {
        return session_id;
    };
    if !ito_path.is_dir() {
        return session_id;
    }

    let path = ito_path.join("session.json");
    if let Ok(contents) = std::fs::read_to_string(&path) {
        match serde_json::from_str::<SessionState>(&contents) {
            Ok(SessionState {
                session_id,
                created_at: _,
            }) if is_safe_session_id(&session_id) => {
                return session_id;
            }
            Ok(_) => {}
            Err(e) => {
                log::debug!("telemetry: failed to parse session.json: {e}");
            }
        }
    }

    let created_at = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
    let state = SessionState {
        session_id: session_id.clone(),
        created_at,
    };
    if let Ok(contents) = serde_json::to_string(&state)
        && let Err(e) = std::fs::write(&path, contents)
    {
        log::debug!("telemetry: failed to write session.json: {e}");
    }

    session_id
}

fn new_session_id() -> String {
    let ts = Utc::now().timestamp();
    let rand = uuid::Uuid::new_v4().simple().to_string();
    format!("{ts}-{rand}")
}

fn is_safe_session_id(session_id: &str) -> bool {
    let session_id = session_id.trim();
    if session_id.is_empty() {
        return false;
    }
    if session_id.len() > 128 {
        return false;
    }
    if session_id.contains('/') || session_id.contains('\\') || session_id.contains("..") {
        return false;
    }

    for c in session_id.chars() {
        if c.is_ascii_alphanumeric() || c == '-' {
            continue;
        }
        return false;
    }

    true
}

fn log_file_path(config_dir: &Path, project_id: &str, session_id: &str) -> PathBuf {
    config_dir
        .join("logs")
        .join("execution")
        .join("v1")
        .join("projects")
        .join(project_id)
        .join("sessions")
        .join(format!("{session_id}.jsonl"))
}

fn canonicalize_best_effort(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

fn compute_project_id(salt: &[u8; 32], project_root: &Path) -> String {
    let root = canonicalize_best_effort(project_root);
    let root = root.to_string_lossy();

    let mut hasher = sha2::Sha256::new();
    hasher.update(salt);
    hasher.update([0u8]);
    hasher.update(root.as_bytes());
    let digest = hasher.finalize();

    hex::encode(digest)
}

fn load_or_create_salt(path: &Path) -> Option<[u8; 32]> {
    if let Ok(bytes) = std::fs::read(path)
        && bytes.len() == 32
    {
        let mut out = [0u8; 32];
        out.copy_from_slice(&bytes);
        return Some(out);
    }

    if path.exists() {
        log::debug!("telemetry: telemetry_salt had unexpected length");
    }

    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let mut out = [0u8; 32];
    rand::rng().fill_bytes(&mut out);
    if let Err(e) = std::fs::write(path, out) {
        log::debug!("telemetry: failed to write telemetry_salt: {e}");
        return None;
    }

    Some(out)
}

#[allow(clippy::match_like_matches_macro)]
fn logging_disabled() -> bool {
    let Some(v) = std::env::var_os("ITO_DISABLE_LOGGING") else {
        return false;
    };
    let v = v.to_string_lossy();
    let v = v.trim().to_ascii_lowercase();
    match v.as_str() {
        "1" | "true" | "yes" => true,
        _ => false,
    }
}

// ── Invalid command logging ────────────────────────────────────────

/// Logger for invalid or unrecognized CLI commands.
///
/// Writes JSONL entries to
/// `~/.config/ito/logs/invalid_commands/v1/projects/{project_id}/{session_id}.jsonl`.
/// Each entry captures the full command that was attempted and the error
/// message, enabling downstream analysis of how agents invoke Ito incorrectly.
#[derive(Debug, Clone)]
pub struct InvalidCommandLogger {
    file_path: PathBuf,
    ito_version: String,
    session_id: String,
    project_id: String,
    pid: u32,
}

impl InvalidCommandLogger {
    /// Create a logger for invalid commands.
    ///
    /// Returns `None` when telemetry is disabled or the config directory
    /// is unavailable.
    pub fn new(
        config_dir: Option<PathBuf>,
        project_root: &Path,
        ito_path: Option<&Path>,
        ito_version: &str,
    ) -> Option<Self> {
        if logging_disabled() {
            return None;
        }

        let config_dir = config_dir?;
        let salt_path = config_dir.join(SALT_FILE_NAME);
        let salt = load_or_create_salt(&salt_path)?;
        let project_id = compute_project_id(&salt, project_root);
        let session_id = resolve_session_id(ito_path);
        let file_path = invalid_command_log_file_path(&config_dir, &project_id, &session_id);

        if let Some(parent) = file_path.parent()
            && let Err(e) = std::fs::create_dir_all(parent)
        {
            log::debug!("invalid_command_log: create_dir_all failed: {e}");
        }

        Some(Self {
            file_path,
            ito_version: ito_version.to_string(),
            session_id,
            project_id,
            pid: std::process::id(),
        })
    }

    /// Log an invalid command invocation.
    ///
    /// `raw_args` is the full list of arguments as passed to the CLI.
    /// `error_message` is the user-facing error text.
    pub fn log_invalid_command(&self, raw_args: &[String], error_message: &str) {
        #[derive(Serialize)]
        struct Entry<'a> {
            event_version: u32,
            event_id: String,
            timestamp: String,
            event_type: &'static str,
            ito_version: &'a str,
            session_id: &'a str,
            project_id: &'a str,
            pid: u32,
            raw_command: String,
            error_message: &'a str,
        }

        let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
        let entry = Entry {
            event_version: EVENT_VERSION,
            event_id: uuid::Uuid::new_v4().to_string(),
            timestamp,
            event_type: "invalid_command",
            ito_version: &self.ito_version,
            session_id: &self.session_id,
            project_id: &self.project_id,
            pid: self.pid,
            raw_command: format!("ito {}", raw_args.join(" ")),
            error_message,
        };

        let Ok(line) = serde_json::to_string(&entry) else {
            log::debug!("invalid_command_log: failed to serialize entry");
            return;
        };

        let Ok(mut f) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
        else {
            log::debug!("invalid_command_log: failed to open log file");
            return;
        };
        if let Err(e) = writeln!(f, "{line}") {
            log::debug!("invalid_command_log: failed to append log line: {e}");
        }
    }
}

fn invalid_command_log_file_path(config_dir: &Path, project_id: &str, session_id: &str) -> PathBuf {
    config_dir
        .join("logs")
        .join("invalid_commands")
        .join("v1")
        .join("projects")
        .join(project_id)
        .join(format!("{session_id}.jsonl"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_command_logger_writes_jsonl_entry() {
        let dir = tempfile::tempdir().unwrap();
        let config_dir = dir.path().join("config");
        std::fs::create_dir_all(&config_dir).unwrap();

        let project_root = dir.path().join("project");
        std::fs::create_dir_all(&project_root).unwrap();

        let logger =
            InvalidCommandLogger::new(Some(config_dir.clone()), &project_root, None, "0.0.0-test")
                .expect("logger should be created");

        logger.log_invalid_command(
            &[
                "agent".to_string(),
                "instruction".to_string(),
                "nonexistent".to_string(),
            ],
            "Unknown artifact 'nonexistent'",
        );

        // Find the written log file.
        let logs_dir = config_dir
            .join("logs")
            .join("invalid_commands")
            .join("v1")
            .join("projects");
        assert!(logs_dir.exists(), "logs directory should exist");

        let mut found = false;
        for entry in std::fs::read_dir(logs_dir).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                for file in std::fs::read_dir(entry.path()).unwrap() {
                    let file = file.unwrap();
                    let contents = std::fs::read_to_string(file.path()).unwrap();
                    assert!(contents.contains("\"event_type\":\"invalid_command\""));
                    assert!(contents.contains("ito agent instruction nonexistent"));
                    assert!(contents.contains("Unknown artifact"));
                    found = true;
                }
            }
        }
        assert!(found, "should have written at least one log entry");
    }

    #[test]
    fn unsafe_session_ids_are_rejected() {
        assert!(!is_safe_session_id(""));
        assert!(!is_safe_session_id("../escape"));
        assert!(!is_safe_session_id("a/b"));
        assert!(!is_safe_session_id("abc def"));
        assert!(is_safe_session_id(
            "1739330000-550e8400e29b41d4a716446655440000"
        ));
    }
}
