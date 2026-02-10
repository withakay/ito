//! Completion validation for the Ralph loop.
//!
//! These helpers are invoked when a completion promise is detected. They verify:
//! - Ito task status (all tasks complete or shelved)
//! - Project validation commands (build/tests/lints)
//! - Optional extra validation command provided via CLI

use crate::error_bridge::IntoCoreResult;
use crate::errors::{CoreError, CoreResult};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use ito_domain::tasks::{DiagnosticLevel, TaskRepository as DomainTaskRepository};

use crate::process::{ProcessRequest, ProcessRunner, SystemProcessRunner};

/// Result of one validation step.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the step succeeded.
    pub success: bool,
    /// Human-readable message summary.
    pub message: String,
    /// Optional verbose output (stdout/stderr, details).
    pub output: Option<String>,
}

/// Which validation step produced a [`ValidationResult`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationStep {
    /// Ito task status validation.
    TaskStatus,
    /// Project validation commands.
    ProjectCheck,
    /// Extra command provided via `--validation-command`.
    ExtraCommand,
}

/// Check that all tasks for `change_id` are complete or shelved.
///
/// Missing tasks file is treated as success.
pub fn check_task_completion(
    task_repo: &impl DomainTaskRepository,
    change_id: &str,
) -> CoreResult<ValidationResult> {
    let repo = task_repo;
    let parsed = repo.load_tasks(change_id).into_core()?;

    if parsed.progress.total == 0 {
        return Ok(ValidationResult {
            success: true,
            message: "No tasks configured; skipping task status validation".to_string(),
            output: None,
        });
    }

    let mut parse_errors: usize = 0;
    for diagnostic in &parsed.diagnostics {
        if diagnostic.level == DiagnosticLevel::Error {
            parse_errors += 1;
        }
    }

    let remaining = parsed.progress.remaining;
    let success = remaining == 0 && parse_errors == 0;

    let mut lines: Vec<String> = Vec::new();
    lines.push(format!("Total: {}", parsed.progress.total));
    lines.push(format!("Complete: {}", parsed.progress.complete));
    lines.push(format!("Shelved: {}", parsed.progress.shelved));
    lines.push(format!("In-progress: {}", parsed.progress.in_progress));
    lines.push(format!("Pending: {}", parsed.progress.pending));
    lines.push(format!("Remaining: {}", parsed.progress.remaining));

    if parse_errors > 0 {
        lines.push(format!("Parse errors: {parse_errors}"));
    }

    if !success {
        lines.push(String::new());
        lines.push("Incomplete tasks:".to_string());
        for t in &parsed.tasks {
            if t.status.is_done() {
                continue;
            }
            lines.push(format!(
                "- {id} ({status}) {name}",
                id = t.id,
                status = t.status.as_enhanced_label(),
                name = t.name
            ));
        }
    }

    let output = Some(lines.join("\n"));

    let message = if success {
        "All tasks are complete or shelved".to_string()
    } else {
        "Tasks remain pending or in-progress".to_string()
    };

    Ok(ValidationResult {
        success,
        message,
        output,
    })
}

/// Run project validation commands discovered from configuration sources.
///
/// If no validation is configured, returns success with a warning message.
pub fn run_project_validation(ito_path: &Path, timeout: Duration) -> CoreResult<ValidationResult> {
    let project_root = ito_path.parent().unwrap_or_else(|| Path::new("."));
    let commands = discover_project_validation_commands(project_root, ito_path)?;

    if commands.is_empty() {
        return Ok(ValidationResult {
            success: true,
            message: "Warning: no project validation configured; skipping".to_string(),
            output: None,
        });
    }

    let mut combined: Vec<String> = Vec::new();
    for cmd in commands {
        let out = run_shell_with_timeout(project_root, &cmd, timeout)?;
        combined.push(out.render());
        if !out.success {
            return Ok(ValidationResult {
                success: false,
                message: format!("Project validation failed: `{cmd}`"),
                output: Some(combined.join("\n\n")),
            });
        }
    }

    Ok(ValidationResult {
        success: true,
        message: "Project validation passed".to_string(),
        output: Some(combined.join("\n\n")),
    })
}

/// Run an extra validation command provided explicitly by the user.
pub fn run_extra_validation(
    project_root: &Path,
    command: &str,
    timeout: Duration,
) -> CoreResult<ValidationResult> {
    let out = run_shell_with_timeout(project_root, command, timeout)?;
    Ok(ValidationResult {
        success: out.success,
        message: if out.success {
            format!("Extra validation passed: `{command}`")
        } else {
            format!("Extra validation failed: `{command}`")
        },
        output: Some(out.render()),
    })
}

fn discover_project_validation_commands(
    project_root: &Path,
    ito_path: &Path,
) -> CoreResult<Vec<String>> {
    let candidates: Vec<(ProjectSource, PathBuf)> = vec![
        (ProjectSource::RepoJson, project_root.join("ito.json")),
        (ProjectSource::ItoConfigJson, ito_path.join("config.json")),
        (ProjectSource::AgentsMd, project_root.join("AGENTS.md")),
        (ProjectSource::ClaudeMd, project_root.join("CLAUDE.md")),
    ];

    for (source, path) in candidates {
        if !path.exists() {
            continue;
        }
        let contents = fs::read_to_string(&path)
            .map_err(|e| CoreError::io(format!("Failed to read {}", path.display()), e))?;
        let commands = match source {
            ProjectSource::RepoJson | ProjectSource::ItoConfigJson => {
                extract_commands_from_json_str(&contents)
            }
            ProjectSource::AgentsMd | ProjectSource::ClaudeMd => {
                extract_commands_from_markdown(&contents)
            }
        };
        if !commands.is_empty() {
            return Ok(commands);
        }
    }

    Ok(Vec::new())
}

#[derive(Debug, Clone, Copy)]
enum ProjectSource {
    RepoJson,
    ItoConfigJson,
    AgentsMd,
    ClaudeMd,
}

fn extract_commands_from_json_str(contents: &str) -> Vec<String> {
    let v: Value = match serde_json::from_str(contents) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    extract_commands_from_json_value(&v)
}

fn extract_commands_from_json_value(v: &Value) -> Vec<String> {
    let pointers = [
        "/ralph/validationCommands",
        "/ralph/validationCommand",
        "/ralph/validation/commands",
        "/ralph/validation/command",
        "/validationCommands",
        "/validationCommand",
        "/project/validationCommands",
        "/project/validationCommand",
        "/project/validation/commands",
        "/project/validation/command",
    ];

    for p in pointers {
        if let Some(v) = v.pointer(p) {
            let commands = normalize_commands_value(v);
            if !commands.is_empty() {
                return commands;
            }
        }
    }

    Vec::new()
}

fn normalize_commands_value(v: &Value) -> Vec<String> {
    match v {
        Value::String(s) => {
            let s = s.trim();
            if s.is_empty() {
                Vec::new()
            } else {
                vec![s.to_string()]
            }
        }
        Value::Array(items) => {
            let mut out: Vec<String> = Vec::new();
            for item in items {
                if let Value::String(s) = item {
                    let s = s.trim();
                    if !s.is_empty() {
                        out.push(s.to_string());
                    }
                }
            }
            out
        }
        Value::Null => Vec::new(),
        Value::Bool(_b) => Vec::new(),
        Value::Number(_n) => Vec::new(),
        Value::Object(_obj) => Vec::new(),
    }
}

fn extract_commands_from_markdown(contents: &str) -> Vec<String> {
    // Heuristic: accept `make check` / `make test` lines anywhere.
    let mut out: Vec<String> = Vec::new();
    for line in contents.lines() {
        let l = line.trim();
        if l == "make check" || l == "make test" {
            out.push(l.to_string());
        }
    }
    out.dedup();
    out
}

#[derive(Debug)]
struct ShellRunOutput {
    command: String,
    success: bool,
    exit_code: i32,
    timed_out: bool,
    stdout: String,
    stderr: String,
}

impl ShellRunOutput {
    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("Command: {}\n", self.command));
        if self.timed_out {
            s.push_str("Result: TIMEOUT\n");
        } else if self.success {
            s.push_str("Result: PASS\n");
        } else {
            s.push_str(&format!("Result: FAIL (exit {})\n", self.exit_code));
        }
        if !self.stdout.trim().is_empty() {
            s.push_str("\nStdout:\n");
            s.push_str(&truncate_for_context(&self.stdout, 12_000));
            s.push('\n');
        }
        if !self.stderr.trim().is_empty() {
            s.push_str("\nStderr:\n");
            s.push_str(&truncate_for_context(&self.stderr, 12_000));
            s.push('\n');
        }
        s
    }
}

fn run_shell_with_timeout(cwd: &Path, cmd: &str, timeout: Duration) -> CoreResult<ShellRunOutput> {
    let runner = SystemProcessRunner;
    let request = ProcessRequest::new("sh")
        .args(["-lc", cmd])
        .current_dir(cwd.to_path_buf());
    let output = runner.run_with_timeout(&request, timeout).map_err(|e| {
        CoreError::Process(format!("Failed to run validation command '{cmd}': {e}"))
    })?;

    Ok(ShellRunOutput {
        command: cmd.to_string(),
        success: output.success,
        exit_code: output.exit_code,
        timed_out: output.timed_out,
        stdout: output.stdout,
        stderr: output.stderr,
    })
}

fn truncate_for_context(s: &str, max_bytes: usize) -> String {
    if s.len() <= max_bytes {
        return s.to_string();
    }
    let mut out = s[..max_bytes].to_string();
    out.push_str("\n... (truncated) ...");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write(path: &Path, contents: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, contents).unwrap();
    }

    #[test]
    fn task_completion_passes_when_no_tasks() {
        let td = tempfile::tempdir().unwrap();
        let ito = td.path().join(".ito");
        fs::create_dir_all(&ito).unwrap();
        let task_repo = crate::task_repository::FsTaskRepository::new(&ito);
        let r = check_task_completion(&task_repo, "001-01_missing").unwrap();
        assert!(r.success);
    }

    #[test]
    fn task_completion_fails_when_remaining() {
        let td = tempfile::tempdir().unwrap();
        let ito = td.path().join(".ito");
        fs::create_dir_all(ito.join("changes/001-01_test")).unwrap();
        write(
            &ito.join("changes/001-01_test/tasks.md"),
            "# Tasks\n\n- [x] done\n- [ ] todo\n",
        );
        let task_repo = crate::task_repository::FsTaskRepository::new(&ito);
        let r = check_task_completion(&task_repo, "001-01_test").unwrap();
        assert!(!r.success);
    }

    #[test]
    fn project_validation_discovers_commands_from_repo_json() {
        let td = tempfile::tempdir().unwrap();
        let project_root = td.path();
        let ito = project_root.join(".ito");
        fs::create_dir_all(&ito).unwrap();
        write(
            &project_root.join("ito.json"),
            r#"{ "ralph": { "validationCommands": ["true"] } }"#,
        );
        let cmds = discover_project_validation_commands(project_root, &ito).unwrap();
        assert_eq!(cmds, vec!["true".to_string()]);
    }

    #[test]
    fn shell_timeout_is_failure() {
        let td = tempfile::tempdir().unwrap();
        let out =
            run_shell_with_timeout(td.path(), "sleep 0.1", Duration::from_millis(50)).unwrap();
        assert!(out.timed_out);
        assert!(!out.success);
    }
}
