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
pub enum Outcome {
    Success,
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
pub struct Logger {
    file_path: PathBuf,
    ito_version: String,
    command_id: String,
    session_id: String,
    project_id: String,
    pid: u32,
}

impl Logger {
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

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn project_id(&self) -> &str {
        &self.project_id
    }

    pub fn write_start(&self) {
        self.write_event("command_start", None, None);
    }

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
            }) if !session_id.trim().is_empty() => {
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
    rand::thread_rng().fill_bytes(&mut out);
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
