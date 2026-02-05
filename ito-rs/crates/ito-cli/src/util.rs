use crate::cli_error::CliResult;
use crate::runtime::Runtime;
use ito_config::ito_dir::get_ito_path;
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
        env!("CARGO_PKG_VERSION"),
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

    result
}

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
        "create" | "new" | "plan" | "state" | "tasks" | "workflow" | "config" | "serve"
        | "agent-config" => {
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
        | "stats" | "ralph" | "loop" => {}
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
