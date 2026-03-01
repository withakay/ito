use crate::cli_error::CliResult;
use crate::runtime::Runtime;
use ito_config::ito_dir::get_ito_path;
use ito_config::load_cascading_project_config;
use ito_config::types::ItoConfig;
use ito_core::backend_client::resolve_backend_runtime;
use ito_core::event_forwarder::{ForwarderConfig, forward_events};
use ito_logging::{Logger as ExecLogger, Outcome as LogOutcome};
use std::path::{Path, PathBuf};

pub(crate) fn env_filter() -> tracing_subscriber::EnvFilter {
    if let Ok(v) = std::env::var("LOG_LEVEL") {
        let v = v.trim();
        if !v.is_empty() {
            let v = v.to_ascii_lowercase();
            let v = match v.as_str() {
                "0" | "off" | "none" => "off".to_string(),
                "1" => "info".to_string(),
                _ => v,
            };

            if let Ok(filter) = tracing_subscriber::EnvFilter::try_new(v) {
                return filter;
            }
        }
    }

    tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("off"))
}

pub(crate) fn with_logging<F>(
    rt: &Runtime,
    command_id: &str,
    project_root: &Path,
    ito_path_for_logging: &Path,
    f: F,
) -> CliResult<()>
where
    F: FnOnce() -> CliResult<()>,
{
    let config_dir = ito_config::ito_config_dir(rt.ctx());
    let logger = ExecLogger::new(
        config_dir,
        project_root,
        Some(ito_path_for_logging),
        command_id,
        option_env!("ITO_WORKSPACE_VERSION").unwrap_or(env!("CARGO_PKG_VERSION")),
    );
    let started = std::time::Instant::now();
    if let Some(l) = &logger {
        l.write_start();
    }

    let result = f();
    let outcome = match &result {
        Ok(()) => LogOutcome::Success,
        Err(_) => LogOutcome::Error,
    };
    if let Some(l) = logger {
        l.write_end(outcome, started.elapsed());
    }

    // Best-effort: forward locally produced audit events to the backend
    // when backend mode is enabled. Failures are logged as warnings but
    // never change the command outcome.
    forward_events_if_backend(rt);

    result
}

/// Builds a normalized command identifier from a list of command-line arguments.
///
/// The first non-flag argument is used as the primary command (defaults to `"ito"` when absent).
/// `"x-templates"` is normalized to `"templates"`. For certain commands a second non-flag
/// positional argument is appended (e.g., `create project` -> `ito.create.project`); the final
/// identifier is lowercased, hyphens are replaced with underscores, and prefixed with `"ito"`.
///
/// # Examples
///
/// ```
/// let id = command_id_from_args(&vec!["create".to_string(), "My-Project".to_string()]);
/// assert_eq!(id, "ito.create.my_project");
///
/// let id = command_id_from_args(&vec!["--verbose".to_string(), "agent".to_string(), "instruction".to_string()]);
/// assert_eq!(id, "ito.agent.instruction");
///
/// let id = command_id_from_args(&Vec::<String>::new());
/// assert_eq!(id, "ito");
/// ```
pub(crate) fn command_id_from_args(args: &[String]) -> String {
    let mut positional: Vec<&str> = Vec::new();
    for a in args {
        if a.starts_with('-') {
            continue;
        }
        positional.push(a.as_str());
    }

    let Some(cmd) = positional.first().copied() else {
        return "ito".to_string();
    };

    let cmd = if cmd == "x-templates" {
        "templates"
    } else {
        cmd
    };

    let mut parts: Vec<&str> = Vec::new();
    parts.push(cmd);

    match cmd {
        "create" | "new" | "plan" | "tasks" | "config" | "serve" | "agent-config" => {
            if let Some(sub) = positional.get(1).copied()
                && !sub.starts_with('-')
            {
                parts.push(sub);
            }
        }
        "show" | "validate" => {
            if let Some(kind) = positional.get(1).copied()
                && kind == "module"
            {
                parts.push(kind);
            }
        }
        "agent" => {
            if let Some(sub) = positional.get(1).copied()
                && sub == "instruction"
            {
                parts.push(sub);
            }
        }
        "templates" | "instructions" | "x-instructions" | "list" | "init" | "update" | "status"
        | "stats" | "ralph" | "loop" | "path" | "grep" => {}
        _ => {}
    }

    let mut out = String::from("ito");
    for p in parts {
        out.push('.');
        for ch in p.chars() {
            if ch == '-' {
                out.push('_');
                continue;
            }
            out.push(ch.to_ascii_lowercase());
        }
    }

    out
}

pub(crate) fn project_root_for_logging(rt: &Runtime, args: &[String]) -> PathBuf {
    let Some(cmd) = args.first().map(|s| s.as_str()) else {
        return PathBuf::from(".");
    };

    if cmd == "init" || cmd == "update" {
        for a in args.iter().skip(1) {
            if a.starts_with('-') {
                continue;
            }
            return PathBuf::from(a);
        }
        return PathBuf::from(".");
    }

    let ito_path = rt.ito_path();
    ito_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

pub(crate) fn ito_path_for_logging(project_root: &Path, rt: &Runtime) -> PathBuf {
    get_ito_path(project_root, rt.ctx())
}

pub(crate) fn parse_string_flag(args: &[String], key: &str) -> Option<String> {
    let mut iter = args.iter();
    while let Some(a) = iter.next() {
        if a == key {
            return iter.next().cloned();
        }
        if let Some(v) = a.strip_prefix(&format!("{key}=")) {
            return Some(v.to_string());
        }
    }
    None
}

pub(crate) fn split_csv(raw: &str) -> Vec<String> {
    raw.split(',').map(|s| s.trim().to_string()).collect()
}

// ── Event forwarding ───────────────────────────────────────────────

/// Best-effort forwarding of local audit events to the backend.
///
/// Called after every command completes. Returns silently when backend
/// mode is not enabled or if any step fails. Never affects command outcome.
fn forward_events_if_backend(rt: &Runtime) {
    let ito_path = rt.ito_path();
    let Some(project_root) = ito_path.parent() else {
        return;
    };

    let merged = load_cascading_project_config(project_root, ito_path, rt.ctx()).merged;
    let config: ItoConfig = match serde_json::from_value(merged) {
        Ok(config) => config,
        Err(e) => {
            tracing::warn!("Skipping backend event forwarding due to invalid config: {e}");
            return;
        }
    };

    if !config.backend.enabled {
        return;
    }

    let Ok(Some(runtime)) = resolve_backend_runtime(&config.backend) else {
        return;
    };

    let client = HttpEventIngestClient {
        base_url: runtime.base_url,
        token: runtime.token,
        timeout: runtime.timeout,
        org: runtime.org,
        repo: runtime.repo,
    };
    let forwarder_config = ForwarderConfig {
        max_retries: runtime.max_retries,
        ..ForwarderConfig::default()
    };

    match forward_events(&client, ito_path, &forwarder_config) {
        Ok(result) => {
            if result.failed_batches > 0 {
                eprintln!(
                    "Warning: {}/{} event forwarding batches failed. \
                     Events will be retried on the next command.",
                    result.failed_batches,
                    result.failed_batches
                        + (result.forwarded + result.duplicates + forwarder_config.batch_size - 1)
                            / forwarder_config.batch_size.max(1)
                );
            }
        }
        Err(e) => {
            tracing::warn!("event forwarding failed: {e}");
        }
    }
}

/// HTTP-based event ingest client that submits event batches to the backend.
struct HttpEventIngestClient {
    base_url: String,
    token: String,
    timeout: std::time::Duration,
    org: String,
    repo: String,
}

impl ito_core::BackendEventIngestClient for HttpEventIngestClient {
    fn ingest(
        &self,
        batch: &ito_core::EventBatch,
    ) -> Result<ito_core::EventIngestResult, ito_core::BackendError> {
        let url = format!(
            "{}/api/v1/projects/{}/{}/events",
            self.base_url, self.org, self.repo
        );
        let body = serde_json::to_string(batch)
            .map_err(|e| ito_core::BackendError::Other(format!("serialize batch: {e}")))?;

        // Use a blocking HTTP client (ureq) since CLI commands are sync.
        let config = ureq::Agent::config_builder()
            .timeout_global(Some(self.timeout))
            // ureq 3.x treats 4xx/5xx as errors by default; disable so we can
            // map status codes into BackendError variants.
            .http_status_as_error(false)
            .build();
        let agent: ureq::Agent = config.into();

        let result = agent
            .post(&url)
            .header("Authorization", &format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .send(&body);

        match result {
            Ok(mut resp) => {
                let status = resp.status().as_u16();
                let text = resp
                    .body_mut()
                    .read_to_string()
                    .unwrap_or_else(|_| String::new());
                if status == 200 {
                    let ingest_result: ito_core::EventIngestResult = serde_json::from_str(&text)
                        .map_err(|e| {
                            ito_core::BackendError::Other(format!("parse response: {e}"))
                        })?;
                    Ok(ingest_result)
                } else if status == 401 {
                    Err(ito_core::BackendError::Unauthorized(
                        "invalid or expired token".to_string(),
                    ))
                } else if status == 400 {
                    Err(ito_core::BackendError::Other(format!(
                        "validation error: {text}"
                    )))
                } else {
                    Err(ito_core::BackendError::Unavailable(format!(
                        "HTTP {status}"
                    )))
                }
            }
            Err(e) => Err(ito_core::BackendError::Unavailable(format!(
                "connection error: {e}"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_csv_trims_parts() {
        assert_eq!(split_csv("a, b ,c"), vec!["a", "b", "c"]);
    }

    #[test]
    fn command_id_uses_positional_args_and_normalizes_hyphens() {
        let args = vec![
            "agent".to_string(),
            "instruction".to_string(),
            "apply".to_string(),
        ];
        assert_eq!(command_id_from_args(&args), "ito.agent.instruction");

        let args = vec!["agent-config".to_string(), "summary".to_string()];
        assert_eq!(command_id_from_args(&args), "ito.agent_config.summary");
    }

    #[test]
    fn command_id_maps_x_templates_to_templates() {
        let args = vec!["x-templates".to_string(), "--json".to_string()];
        assert_eq!(command_id_from_args(&args), "ito.templates");
    }
}
