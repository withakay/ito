use crate::errors::{CoreError, CoreResult};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A single pending task resolved from an external Ralph task source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RalphTaskSource {
    /// A pending markdown checkbox task.
    Markdown {
        /// Source markdown file path.
        path: PathBuf,
        /// Zero-based line index of the checkbox entry.
        line_index: usize,
        /// Human-readable task text.
        task: String,
    },
    /// A pending YAML task entry.
    Yaml {
        /// Source YAML file path.
        path: PathBuf,
        /// Zero-based task index within the parsed `tasks` list.
        index: usize,
        /// Human-readable task title.
        task: String,
        /// Optional parallel grouping key.
        parallel_group: u32,
    },
    /// A pending GitHub issue task.
    Github {
        /// Repository in `owner/repo` form.
        repo: String,
        /// GitHub issue number.
        issue_number: u64,
        /// Human-readable task title.
        task: String,
    },
}

impl RalphTaskSource {
    /// Build a harness prompt block for this external task source.
    pub fn build_prompt(&self, base_prompt: &str) -> String {
        let task_block = match self {
            Self::Markdown { path, task, .. } => format!(
                "## External Task Source\n- Type: markdown\n- Path: {}\n\n## Pending Task\n{}",
                path.display(),
                task
            ),
            Self::Yaml { path, task, .. } => format!(
                "## External Task Source\n- Type: yaml\n- Path: {}\n\n## Pending Task\n{}",
                path.display(),
                task
            ),
            Self::Github {
                repo,
                issue_number,
                task,
            } => format!(
                "## External Task Source\n- Type: github\n- Repo: {}\n- Issue: #{}\n\n## Pending Task\n{}",
                repo, issue_number, task
            ),
        };

        if base_prompt.trim().is_empty() {
            return task_block;
        }

        format!("{task_block}\n\n---\n\n{base_prompt}")
    }
}

/// Resolve unchecked markdown checkbox tasks from a PRD-style markdown file.
pub fn resolve_markdown_task_sources(path: &Path) -> CoreResult<Vec<RalphTaskSource>> {
    let contents = std::fs::read_to_string(path).map_err(|err| {
        CoreError::io(
            format!("Failed to read markdown task file {}", path.display()),
            err,
        )
    })?;
    let mut tasks = Vec::new();

    for (line_index, line) in contents.lines().enumerate() {
        let Some(task) = pending_markdown_task(line) else {
            continue;
        };
        tasks.push(RalphTaskSource::Markdown {
            path: path.to_path_buf(),
            line_index,
            task,
        });
    }

    Ok(tasks)
}

/// Resolve incomplete YAML tasks from a task list file.
pub fn resolve_yaml_task_sources(path: &Path) -> CoreResult<Vec<RalphTaskSource>> {
    let contents = std::fs::read_to_string(path).map_err(|err| {
        CoreError::io(
            format!("Failed to read YAML task file {}", path.display()),
            err,
        )
    })?;
    let parsed: RalphYamlTasks = serde_yaml::from_str(&contents).map_err(|err| {
        CoreError::serde(
            format!("Failed to parse YAML task file {}", path.display()),
            err.to_string(),
        )
    })?;

    let mut tasks = Vec::new();
    for (index, task) in parsed.tasks.into_iter().enumerate() {
        if task.completed.unwrap_or(false) {
            continue;
        }
        tasks.push(RalphTaskSource::Yaml {
            path: path.to_path_buf(),
            index,
            task: task.title,
            parallel_group: task.parallel_group.unwrap_or(0),
        });
    }

    Ok(tasks)
}

/// Resolve open GitHub issues into Ralph task sources.
pub fn resolve_github_task_sources(
    repo: &str,
    label: Option<&str>,
) -> CoreResult<Vec<RalphTaskSource>> {
    let mut command = Command::new("gh");
    command
        .arg("issue")
        .arg("list")
        .arg("--repo")
        .arg(repo)
        .arg("--state")
        .arg("open")
        .arg("--json")
        .arg("number,title");
    if let Some(label) = label {
        command.arg("--label").arg(label);
    }

    let output = command.output().map_err(|err| {
        CoreError::process(format!(
            "Failed to run `gh issue list` for repository {repo}: {err}"
        ))
    })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(CoreError::process(format!(
            "`gh issue list` failed for repository {repo}: {stderr}"
        )));
    }

    let issues: Vec<GitHubIssue> = serde_json::from_slice(&output.stdout).map_err(|err| {
        CoreError::serde(
            format!("Failed to parse `gh issue list` output for repository {repo}"),
            err.to_string(),
        )
    })?;

    Ok(issues
        .into_iter()
        .map(|issue| RalphTaskSource::Github {
            repo: repo.to_string(),
            issue_number: issue.number,
            task: issue.title,
        })
        .collect())
}

fn pending_markdown_task(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let task = trimmed
        .strip_prefix("- [ ] ")
        .or_else(|| trimmed.strip_prefix("* [ ] "))?;
    let task = task.trim();
    if task.is_empty() {
        return None;
    }
    Some(task.to_string())
}

#[derive(Debug, Deserialize)]
struct RalphYamlTasks {
    tasks: Vec<RalphYamlTask>,
}

#[derive(Debug, Deserialize)]
struct RalphYamlTask {
    title: String,
    completed: Option<bool>,
    parallel_group: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct GitHubIssue {
    number: u64,
    title: String,
}
