use crate::cli::{HarnessArg, RalphArgs};
use crate::cli_error::{CliResult, fail, to_cli_error};
use ito_core::ralph as core_ralph;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

pub(super) fn branch_label(
    args: &RalphArgs,
    source: Option<&RalphTaskSource>,
    prompt: &str,
) -> String {
    if let Some(change) = args.change.as_deref() {
        return change.to_string();
    }
    if let Some(module) = args.module.as_deref() {
        return format!("module-{module}");
    }
    if let Some(source) = source {
        return match source {
            RalphTaskSource::Markdown { task, .. } => task.clone(),
            RalphTaskSource::Yaml { task, .. } => task.clone(),
            RalphTaskSource::Github {
                issue_number, task, ..
            } => {
                format!("issue-{issue_number}-{task}")
            }
        };
    }
    let prompt = prompt.trim();
    if prompt.is_empty() {
        "ralph-task".to_string()
    } else {
        prompt.to_string()
    }
}

#[derive(Debug, Clone)]
pub(super) enum RalphTaskSource {
    Markdown {
        path: PathBuf,
        line_index: usize,
        task: String,
    },
    Yaml {
        path: PathBuf,
        index: usize,
        task: String,
        parallel_group: u32,
    },
    Github {
        repo: String,
        issue_number: u64,
        task: String,
    },
}

impl RalphTaskSource {
    pub(super) fn parallel_group(&self) -> Option<u32> {
        match self {
            RalphTaskSource::Yaml { parallel_group, .. } => Some(*parallel_group),
            _ => None,
        }
    }

    pub(super) fn build_prompt(&self, base_prompt: &str) -> String {
        let task_block = match self {
            RalphTaskSource::Markdown { path, task, .. } => format!(
                "## External Task Source\n- Type: markdown\n- Path: {}\n\n## Pending Task\n{}",
                path.display(),
                task
            ),
            RalphTaskSource::Yaml { path, task, .. } => format!(
                "## External Task Source\n- Type: yaml\n- Path: {}\n\n## Pending Task\n{}",
                path.display(),
                task
            ),
            RalphTaskSource::Github {
                repo,
                issue_number,
                task,
            } => format!(
                "## External Task Source\n- Type: github\n- Repo: {}\n- Issue: #{}\n\n## Pending Task\n{}",
                repo, issue_number, task
            ),
        };

        if base_prompt.trim().is_empty() {
            task_block
        } else {
            format!("{task_block}\n\n---\n\n{base_prompt}")
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct RalphYamlTasks {
    tasks: Vec<RalphYamlTask>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RalphYamlTask {
    title: String,
    completed: Option<bool>,
    parallel_group: Option<u32>,
}

pub(super) fn resolve_task_source(args: &RalphArgs) -> CliResult<Option<RalphTaskSource>> {
    let mut all = resolve_all_task_sources(args)?;
    Ok(all.drain(..).next())
}

pub(super) fn resolve_all_task_sources(args: &RalphArgs) -> CliResult<Vec<RalphTaskSource>> {
    let mut source_count = 0;
    if args.prd.is_some() {
        source_count += 1;
    }
    if args.yaml.is_some() {
        source_count += 1;
    }
    if args.github.is_some() {
        source_count += 1;
    }
    if source_count > 1 {
        return fail("Use only one external task source at a time (--prd, --yaml, or --github).");
    }
    if args.draft_pr && !args.create_pr {
        return fail("--draft-pr requires --create-pr");
    }
    if args.sync_issue.is_some() && args.prd.is_none() {
        return fail("--sync-issue requires --prd");
    }
    if source_count > 0 && (args.change.is_some() || args.module.is_some() || args.continue_ready) {
        return fail(
            "External task sources cannot be combined with --change, --module, or --continue-ready.",
        );
    }

    if let Some(path) = &args.prd {
        let path = PathBuf::from(path);
        let contents = ito_common::io::read_to_string_std(&path).map_err(|e| {
            to_cli_error(miette::miette!(
                "Failed to read PRD file {}: {e}",
                path.display()
            ))
        })?;
        let tasks = contents
            .lines()
            .enumerate()
            .filter_map(|(idx, line)| {
                parse_markdown_task_line(line).map(|s| RalphTaskSource::Markdown {
                    path: path.clone(),
                    line_index: idx,
                    task: s,
                })
            })
            .collect::<Vec<_>>();
        if tasks.is_empty() {
            return fail(format!(
                "No pending markdown tasks found in {}",
                path.display()
            ));
        }
        return Ok(tasks);
    }

    if let Some(path) = &args.yaml {
        let path = PathBuf::from(path);
        let contents = ito_common::io::read_to_string_std(&path).map_err(|e| {
            to_cli_error(miette::miette!(
                "Failed to read YAML task file {}: {e}",
                path.display()
            ))
        })?;
        let parsed: RalphYamlTasks = serde_yaml::from_str(&contents).map_err(|e| {
            to_cli_error(miette::miette!(
                "Failed to parse YAML task file {}: {e}",
                path.display()
            ))
        })?;
        let sources = parsed
            .tasks
            .iter()
            .enumerate()
            .filter(|(_, task)| task.completed != Some(true))
            .map(|(idx, task)| RalphTaskSource::Yaml {
                path: path.clone(),
                index: idx,
                task: task.title.clone(),
                parallel_group: task.parallel_group.unwrap_or(0),
            })
            .collect::<Vec<_>>();
        if sources.is_empty() {
            return fail(format!("No pending YAML tasks found in {}", path.display()));
        }
        return Ok(sources);
    }

    if let Some(repo) = &args.github {
        let mut cmd = Command::new("gh");
        cmd.arg("issue")
            .arg("list")
            .arg("--repo")
            .arg(repo)
            .arg("--state")
            .arg("open")
            .arg("--json")
            .arg("number,title");
        if let Some(label) = &args.github_label {
            cmd.arg("--label").arg(label);
        }
        let output = cmd
            .output()
            .map_err(|e| to_cli_error(miette::miette!("Failed to run gh issue list: {e}")))?;
        if !output.status.success() {
            return fail(format!(
                "gh issue list failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let value: serde_json::Value = serde_json::from_slice(&output.stdout).map_err(|e| {
            to_cli_error(miette::miette!("Failed to parse gh issue list output: {e}"))
        })?;
        let items = value.as_array().ok_or_else(|| {
            to_cli_error(miette::miette!("No open GitHub issues found for {repo}"))
        })?;
        if items.is_empty() {
            return fail(format!("No open GitHub issues found for {repo}"));
        }

        let mut sources = Vec::new();
        for issue in items {
            let issue_number = issue
                .get("number")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| {
                    to_cli_error(miette::miette!("Missing GitHub issue number in gh output"))
                })?;
            let task = issue
                .get("title")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    to_cli_error(miette::miette!("Missing GitHub issue title in gh output"))
                })?
                .to_string();
            sources.push(RalphTaskSource::Github {
                repo: repo.clone(),
                issue_number,
                task,
            });
        }
        return Ok(sources);
    }

    Ok(Vec::new())
}

pub(super) fn ralph_run_completed(ito_path: &Path, target: &str) -> CliResult<bool> {
    Ok(core_ralph::state::load_state(ito_path, target)
        .map_err(to_cli_error)?
        .and_then(|state| state.last_outcome)
        .is_some_and(|outcome| {
            outcome == "validated-complete" || outcome == "unvalidated-complete"
        }))
}

pub(super) fn sync_task_source(source: &RalphTaskSource) -> CliResult<()> {
    match source {
        RalphTaskSource::Markdown {
            path,
            line_index,
            task,
        } => {
            let contents = ito_common::io::read_to_string_std(path).map_err(|e| {
                to_cli_error(miette::miette!(
                    "Failed to read PRD file {}: {e}",
                    path.display()
                ))
            })?;
            let mut lines = contents.lines().map(|s| s.to_string()).collect::<Vec<_>>();
            if let Some(line) = lines.get_mut(*line_index) {
                *line = format!("- [x] {task}");
            }
            let updated = format!("{}\n", lines.join("\n"));
            ito_common::io::write_std(path, updated).map_err(|e| {
                to_cli_error(miette::miette!(
                    "Failed to update PRD file {}: {e}",
                    path.display()
                ))
            })?;
        }
        RalphTaskSource::Yaml { path, index, .. } => {
            let contents = ito_common::io::read_to_string_std(path).map_err(|e| {
                to_cli_error(miette::miette!(
                    "Failed to read YAML task file {}: {e}",
                    path.display()
                ))
            })?;
            let mut parsed: RalphYamlTasks = serde_yaml::from_str(&contents).map_err(|e| {
                to_cli_error(miette::miette!(
                    "Failed to parse YAML task file {}: {e}",
                    path.display()
                ))
            })?;
            if let Some(task) = parsed.tasks.get_mut(*index) {
                task.completed = Some(true);
            }
            let updated = serde_yaml::to_string(&parsed).map_err(|e| {
                to_cli_error(miette::miette!(
                    "Failed to serialize YAML task file {}: {e}",
                    path.display()
                ))
            })?;
            ito_common::io::write_std(path, updated).map_err(|e| {
                to_cli_error(miette::miette!(
                    "Failed to update YAML task file {}: {e}",
                    path.display()
                ))
            })?;
        }
        RalphTaskSource::Github {
            repo, issue_number, ..
        } => {
            let output = Command::new("gh")
                .arg("issue")
                .arg("close")
                .arg(issue_number.to_string())
                .arg("--repo")
                .arg(repo)
                .output()
                .map_err(|e| to_cli_error(miette::miette!("Failed to run gh issue close: {e}")))?;
            if !output.status.success() {
                return fail(format!(
                    "gh issue close failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
    }

    Ok(())
}

pub(super) fn sync_issue_body(
    repo_root: &Path,
    issue_number: u64,
    source: &RalphTaskSource,
) -> CliResult<()> {
    let RalphTaskSource::Markdown { path, .. } = source else {
        return Ok(());
    };
    let output = Command::new("gh")
        .arg("issue")
        .arg("edit")
        .arg(issue_number.to_string())
        .arg("--body-file")
        .arg(path)
        .current_dir(repo_root)
        .output()
        .map_err(|e| to_cli_error(miette::miette!("Failed to run gh issue edit: {e}")))?;
    if !output.status.success() {
        return fail(format!(
            "gh issue edit failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}

pub(super) fn run_parallel_task_sources(
    repo_root: &Path,
    sources: &[RalphTaskSource],
    base_prompt: &str,
    args: &RalphArgs,
) -> CliResult<()> {
    let exe = std::env::current_exe()
        .map_err(|e| to_cli_error(miette::miette!("Failed to resolve current executable: {e}")))?;
    let batches = batch_task_sources(sources, args.max_parallel.max(1));
    let mut failures: Vec<(String, String)> = Vec::new();

    for batch in batches {
        let mut workers: Vec<ParallelWorker> = Vec::new();
        for (idx, source) in batch.into_iter().enumerate() {
            let label = branch_label(args, Some(&source), base_prompt);
            let worker =
                spawn_parallel_worker(repo_root, &exe, args, source, label, idx, base_prompt)?;
            workers.push(worker);
        }

        for worker in workers {
            let label = worker.label.clone();
            match worker.wait_and_sync(repo_root, args) {
                Ok(Some((label, reason))) => failures.push((label, reason)),
                Ok(None) => {}
                Err(err) => failures.push((label, err.to_string())),
            }
        }
    }

    if failures.is_empty() {
        return Ok(());
    }

    let joined = failures
        .into_iter()
        .map(|(label, reason)| format!("{label}: {reason}"))
        .collect::<Vec<_>>()
        .join("; ");
    fail(format!(
        "Parallel Ralph run completed with failures: {joined}"
    ))
}

#[derive(Debug)]
struct ParallelWorker {
    source: RalphTaskSource,
    label: String,
    worktree_dir: PathBuf,
    worktree_ito: PathBuf,
    branch: String,
    keep_branch: bool,
    child: Child,
}

impl ParallelWorker {
    fn wait_and_sync(
        self,
        repo_root: &Path,
        args: &RalphArgs,
    ) -> CliResult<Option<(String, String)>> {
        let ParallelWorker {
            source,
            label,
            worktree_dir,
            worktree_ito,
            branch,
            keep_branch,
            child,
        } = self;
        let output = child
            .wait_with_output()
            .map_err(|e| to_cli_error(miette::miette!("Failed waiting for Ralph worker: {e}")))?;
        let success = match ralph_run_completed(&worktree_ito, "unscoped") {
            Ok(completed) => output.status.success() && completed,
            Err(err) => {
                cleanup_parallel_worktree(repo_root, &worktree_dir, &branch, keep_branch)?;
                return Ok(Some((label, err.to_string())));
            }
        };
        if success {
            let mut orchestration_error: Option<String> = None;
            if !keep_branch && let Err(err) = integrate_parallel_branch(repo_root, &branch) {
                orchestration_error = Some(err.to_string());
            }
            if orchestration_error.is_none()
                && args.create_pr
                && let Err(err) = create_pull_request(
                    repo_root,
                    &branch,
                    args.base_branch.as_deref(),
                    &label,
                    args.draft_pr,
                )
            {
                orchestration_error = Some(err.to_string());
            }
            if orchestration_error.is_none()
                && let Err(err) = sync_task_source(&source)
            {
                orchestration_error = Some(err.to_string());
            }
            cleanup_parallel_worktree(repo_root, &worktree_dir, &branch, keep_branch)?;
            if let Some(reason) = orchestration_error {
                return Ok(Some((label, reason)));
            }
            return Ok(None);
        }

        let reason = failure_message(&worktree_ito)?;
        cleanup_parallel_worktree(repo_root, &worktree_dir, &branch, keep_branch)?;
        Ok(Some((label, reason)))
    }
}

fn failure_message(worktree_ito: &Path) -> CliResult<String> {
    let state = core_ralph::state::load_state(worktree_ito, "unscoped").map_err(to_cli_error)?;
    if let Some(state) = state
        && let Some(failure) = state.last_failure
    {
        return Ok(failure
            .lines()
            .next()
            .unwrap_or("worker failed")
            .to_string());
    }
    Ok("worker failed".to_string())
}

fn parse_markdown_task_line(line: &str) -> Option<String> {
    let line = line.trim_start();
    let candidate = line
        .strip_prefix("- [ ] ")
        .or_else(|| line.strip_prefix("* [ ] "))?;
    let task = candidate.trim();
    if task.is_empty() {
        return None;
    }
    Some(task.to_string())
}

fn spawn_parallel_worker(
    repo_root: &Path,
    exe: &Path,
    args: &RalphArgs,
    source: RalphTaskSource,
    label: String,
    idx: usize,
    base_prompt: &str,
) -> CliResult<ParallelWorker> {
    let (worktree_dir, branch) =
        create_parallel_worktree(repo_root, &label, args.base_branch.as_deref(), idx)?;
    configure_parallel_worktree_excludes(&worktree_dir)?;
    let worktree_ito = worktree_dir.join(".ito");
    let prompt_dir = worktree_ito.join(".state/ralph-parallel");
    std::fs::create_dir_all(&prompt_dir).map_err(|e| {
        to_cli_error(miette::miette!(
            "Failed to create Ralph parallel state dir: {e}"
        ))
    })?;
    let prompt_file = prompt_dir.join(format!("task-{}.md", idx));
    std::fs::write(&prompt_file, source.build_prompt(base_prompt))
        .map_err(|e| to_cli_error(miette::miette!("Failed to write worker prompt file: {e}")))?;

    let mut cmd = Command::new(exe);
    cmd.arg("ralph")
        .arg("--file")
        .arg(&prompt_file)
        .arg("--harness")
        .arg(harness_name(args.harness))
        .arg("--no-interactive")
        .arg("--min-iterations")
        .arg(args.min_iterations.to_string())
        .arg("--completion-promise")
        .arg(&args.completion_promise);
    if let Some(max) = args.max_iterations {
        cmd.arg("--max-iterations").arg(max.to_string());
    }
    if let Some(model) = &args.model {
        cmd.arg("--model").arg(model);
    }
    if let Some(timeout) = &args.timeout {
        cmd.arg("--timeout").arg(timeout);
    }
    if let Some(validation_command) = &args.validation_command {
        cmd.arg("--validation-command").arg(validation_command);
    }
    if args.skip_validation {
        cmd.arg("--skip-validation");
    }
    if args.allow_all {
        cmd.arg("--allow-all");
    }
    if args.exit_on_error {
        cmd.arg("--exit-on-error");
    }
    if let Some(threshold) = args.error_threshold {
        cmd.arg("--error-threshold").arg(threshold.to_string());
    }
    if let Some(stub_script) = &args.stub_script {
        cmd.arg("--stub-script").arg(stub_script);
    }
    let child = cmd.current_dir(&worktree_dir).spawn().map_err(|e| {
        to_cli_error(miette::miette!(
            "Failed to spawn parallel Ralph worker: {e}"
        ))
    })?;

    Ok(ParallelWorker {
        source,
        label,
        worktree_dir,
        worktree_ito,
        branch,
        keep_branch: args.branch_per_task || args.create_pr,
        child,
    })
}

fn batch_task_sources(
    sources: &[RalphTaskSource],
    max_parallel: usize,
) -> Vec<Vec<RalphTaskSource>> {
    let mut batches = Vec::new();
    let mut idx = 0;
    while idx < sources.len() {
        let group = sources[idx].parallel_group();
        let mut chunk = Vec::new();
        while idx < sources.len()
            && sources[idx].parallel_group() == group
            && chunk.len() < max_parallel
        {
            chunk.push(sources[idx].clone());
            idx += 1;
        }
        batches.push(chunk);
    }
    batches
}

fn create_parallel_worktree(
    repo_root: &Path,
    label: &str,
    base_branch: Option<&str>,
    idx: usize,
) -> CliResult<(PathBuf, String)> {
    let current_branch = git_current_branch(repo_root)?;
    let base = base_branch.unwrap_or(current_branch.as_str()).to_string();
    let branch = format!("ralph/parallel-{}-{}", idx + 1, slugify(label));
    let dir = std::env::temp_dir().join(format!("ito-ralph-{}-{}", std::process::id(), idx + 1));
    if dir.exists() {
        let _ = Command::new("git")
            .args([
                "worktree",
                "remove",
                "--force",
                dir.to_string_lossy().as_ref(),
            ])
            .current_dir(repo_root)
            .output();
        let _ = std::fs::remove_dir_all(&dir);
    }
    let _ = Command::new("git")
        .args(["branch", "-D", &branch])
        .current_dir(repo_root)
        .output();
    run_git(
        repo_root,
        &[
            "worktree",
            "add",
            "-b",
            &branch,
            dir.to_string_lossy().as_ref(),
            &base,
        ],
    )?;
    Ok((dir, branch))
}

fn configure_parallel_worktree_excludes(worktree_dir: &Path) -> CliResult<()> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-path", "info/exclude"])
        .current_dir(worktree_dir)
        .output()
        .map_err(|e| {
            to_cli_error(miette::miette!(
                "Failed to resolve worktree exclude path: {e}"
            ))
        })?;
    if !output.status.success() {
        return fail(format!(
            "git rev-parse --git-path info/exclude failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let exclude_raw = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim());
    let exclude_path = if exclude_raw.is_absolute() {
        exclude_raw
    } else {
        worktree_dir.join(exclude_raw)
    };
    if let Some(parent) = exclude_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            to_cli_error(miette::miette!(
                "Failed to create worktree exclude directory: {e}"
            ))
        })?;
    }
    let mut existing = std::fs::read_to_string(&exclude_path).unwrap_or_default();
    for pattern in [".ito/session.json"] {
        if !existing.lines().any(|line| line.trim() == pattern) {
            if !existing.ends_with('\n') && !existing.is_empty() {
                existing.push('\n');
            }
            existing.push_str(pattern);
            existing.push('\n');
        }
    }
    std::fs::write(&exclude_path, existing).map_err(|e| {
        to_cli_error(miette::miette!(
            "Failed to update worktree exclude file: {e}"
        ))
    })?;
    Ok(())
}

fn cleanup_parallel_worktree(
    repo_root: &Path,
    worktree_dir: &Path,
    branch: &str,
    keep_branch: bool,
) -> CliResult<()> {
    let _ = Command::new("git")
        .args([
            "worktree",
            "remove",
            "--force",
            worktree_dir.to_string_lossy().as_ref(),
        ])
        .current_dir(repo_root)
        .output();
    let _ = Command::new("git")
        .args(["worktree", "prune"])
        .current_dir(repo_root)
        .output();
    if !keep_branch {
        let _ = Command::new("git")
            .args(["branch", "-D", branch])
            .current_dir(repo_root)
            .output();
    }
    Ok(())
}

fn integrate_parallel_branch(repo_root: &Path, branch: &str) -> CliResult<()> {
    let current_branch = git_current_branch(repo_root)?;
    let output = Command::new("git")
        .args([
            "rev-list",
            "--reverse",
            &format!("{current_branch}..{branch}"),
        ])
        .current_dir(repo_root)
        .output()
        .map_err(|e| {
            to_cli_error(miette::miette!(
                "Failed to inspect parallel branch commits: {e}"
            ))
        })?;
    if !output.status.success() {
        return fail(format!(
            "git rev-list failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let commits = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect::<Vec<_>>();

    for commit in commits {
        run_git(repo_root, &["cherry-pick", &commit])?;
    }

    Ok(())
}

pub(super) fn create_task_branch(
    repo_root: &Path,
    label: &str,
    base_branch: Option<&str>,
) -> CliResult<(String, String)> {
    if git_worktree_dirty(repo_root)? {
        return fail("--branch-per-task requires a clean working tree before switching branches.");
    }
    let current_branch = git_current_branch(repo_root)?;
    let base = base_branch.unwrap_or(current_branch.as_str()).to_string();
    let branch = format!("ralph/{}", slugify(label));

    if current_branch != base {
        run_git(repo_root, &["checkout", &base])?;
    }
    run_git(repo_root, &["checkout", "-B", &branch])?;
    Ok((branch, base))
}

fn git_worktree_dirty(repo_root: &Path) -> CliResult<bool> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(repo_root)
        .output()
        .map_err(|e| to_cli_error(miette::miette!("Failed to run git status: {e}")))?;
    if !output.status.success() {
        return fail(format!(
            "git status failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let dirty = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| {
            !matches!(
                *line,
                "?? .ito/session.json" | "?? .opencode/package-lock.json"
            )
        })
        .any(|_| true);
    Ok(dirty)
}

pub(super) fn create_pull_request(
    repo_root: &Path,
    branch: &str,
    base_branch: Option<&str>,
    title: &str,
    draft: bool,
) -> CliResult<()> {
    let base = base_branch
        .map(|s| s.to_string())
        .unwrap_or_else(|| git_current_branch(repo_root).unwrap_or_else(|_| "main".to_string()));
    run_git(repo_root, &["push", "-u", "origin", branch])?;

    let mut cmd = Command::new("gh");
    cmd.arg("pr")
        .arg("create")
        .arg("--base")
        .arg(&base)
        .arg("--head")
        .arg(branch)
        .arg("--title")
        .arg(title)
        .arg("--body")
        .arg("Automated Ralph task execution.");
    if draft {
        cmd.arg("--draft");
    }
    let output = cmd
        .current_dir(repo_root)
        .output()
        .map_err(|e| to_cli_error(miette::miette!("Failed to run gh pr create: {e}")))?;
    if !output.status.success() {
        return fail(format!(
            "gh pr create failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}

fn git_current_branch(repo_root: &Path) -> CliResult<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .current_dir(repo_root)
        .output()
        .map_err(|e| to_cli_error(miette::miette!("Failed to run git rev-parse: {e}")))?;
    if !output.status.success() {
        return fail(format!(
            "git rev-parse failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn run_git(repo_root: &Path, args: &[&str]) -> CliResult<()> {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .output()
        .map_err(|e| to_cli_error(miette::miette!("Failed to run git {}: {e}", args.join(" "))))?;
    if !output.status.success() {
        return fail(format!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}

fn slugify(input: &str) -> String {
    let mut out = String::new();
    let mut last_dash = false;
    for ch in input.chars().flat_map(|c| c.to_lowercase()) {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_dash = false;
        } else if !last_dash {
            out.push('-');
            last_dash = true;
        }
        if out.len() >= 50 {
            break;
        }
    }
    out.trim_matches('-').to_string()
}

pub(super) fn is_command_available(program: &str) -> bool {
    std::env::var_os("PATH")
        .map(|paths| {
            std::env::split_paths(&paths).any(|dir| {
                let candidate = dir.join(program);
                candidate.is_file()
            })
        })
        .unwrap_or(false)
}

pub(super) fn add_browser_guidance(mut prompt: String) -> String {
    prompt.push_str(
        "\n\n---\n\n## Browser Automation\nYou may use `agent-browser` for UI verification and browser-driven workflows. Useful commands include `agent-browser open <url>`, `agent-browser snapshot`, `agent-browser click @e1`, `agent-browser type @e1 \"text\"`, `agent-browser screenshot file.png`, and `agent-browser content`. Use it when the task benefits from browser automation.\n",
    );
    prompt
}

fn harness_name(harness: HarnessArg) -> &'static str {
    match harness {
        HarnessArg::Opencode => "opencode",
        HarnessArg::Claude => "claude",
        HarnessArg::Codex => "codex",
        HarnessArg::Copilot => "copilot",
        HarnessArg::Stub => "stub",
    }
}

pub(super) fn notify_run_result(error: Option<String>) {
    #[cfg(target_os = "macos")]
    {
        let body = error
            .as_deref()
            .unwrap_or("Ralph run completed successfully");
        let _ = Command::new("osascript")
            .arg("-e")
            .arg(format!(
                "display notification \"{}\" with title \"Ito Ralph\"",
                body.replace('"', "'")
            ))
            .output();
    }

    #[cfg(target_os = "linux")]
    {
        let summary = match error {
            Some(_) => "Ito Ralph failed",
            None => "Ito Ralph complete",
        };
        let body = error
            .as_deref()
            .unwrap_or("Ralph run completed successfully");
        let _ = Command::new("notify-send").arg(summary).arg(body).output();
    }
}
