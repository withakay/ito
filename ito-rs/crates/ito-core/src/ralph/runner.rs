use crate::error_bridge::IntoCoreResult;
use crate::errors::{CoreError, CoreResult};
use crate::harness::types::MAX_RETRIABLE_RETRIES;
use crate::harness::{Harness, HarnessName};
use crate::process::{ProcessRequest, ProcessRunner, SystemProcessRunner};
use crate::ralph::duration::format_duration;
use crate::ralph::prompt::{BuildPromptOptions, build_ralph_prompt};
use crate::ralph::state::{
    RalphHistoryEntry, RalphState, append_context, clear_context, load_context, load_state,
    save_state,
};
use crate::ralph::validation;
use ito_domain::changes::{
    ChangeRepository as DomainChangeRepository, ChangeSummary, ChangeTargetResolution,
    ChangeWorkStatus,
};
use ito_domain::modules::ModuleRepository as DomainModuleRepository;
use ito_domain::tasks::TaskRepository as DomainTaskRepository;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
/// Runtime options for a single Ralph loop invocation.
pub struct RalphOptions {
    /// Base prompt content appended after any change/module context.
    pub prompt: String,

    /// Optional change id to scope the loop to.
    pub change_id: Option<String>,

    /// Optional module id to scope the loop to.
    pub module_id: Option<String>,

    /// Optional model override passed through to the harness.
    pub model: Option<String>,

    /// Minimum number of iterations required before a completion promise is honored.
    pub min_iterations: u32,

    /// Optional maximum iteration count.
    pub max_iterations: Option<u32>,

    /// Completion token that signals the loop is done (e.g. `COMPLETE`).
    pub completion_promise: String,

    /// Auto-approve all harness prompts and actions.
    pub allow_all: bool,

    /// Skip creating a git commit after each iteration.
    pub no_commit: bool,

    /// Enable interactive mode when supported by the harness.
    pub interactive: bool,

    /// Print the current saved state without running a new iteration.
    pub status: bool,

    /// Append additional markdown to the saved Ralph context and exit.
    pub add_context: Option<String>,

    /// Clear any saved Ralph context and exit.
    pub clear_context: bool,

    /// Print the full prompt sent to the harness.
    pub verbose: bool,

    /// When targeting a module, continue through ready changes until module work is complete.
    pub continue_module: bool,

    /// When set, continuously process eligible changes across the repo.
    ///
    /// Eligible changes are those whose derived work status is `Ready` or `InProgress`.
    pub continue_ready: bool,

    /// Inactivity timeout - restart iteration if no output for this duration.
    pub inactivity_timeout: Option<Duration>,

    /// Skip all completion validation.
    ///
    /// When set, the loop trusts the completion promise and exits immediately.
    pub skip_validation: bool,

    /// Additional validation command to run when a completion promise is detected.
    ///
    /// This runs after the project validation steps.
    pub validation_command: Option<String>,

    /// Exit immediately when the harness process returns non-zero.
    ///
    /// When false, Ralph captures the failure output and continues iterating.
    pub exit_on_error: bool,

    /// Maximum number of non-zero harness exits allowed before failing.
    ///
    /// Applies only when `exit_on_error` is false.
    pub error_threshold: u32,
}

/// Default maximum number of non-zero harness exits Ralph tolerates.
pub const DEFAULT_ERROR_THRESHOLD: u32 = 10;

/// Run the Ralph loop for a change (or repository/module sequence) until the configured completion promise is detected.
///
/// Persists lightweight per-change state under `.ito/.state/ralph/<change>/` so iteration history and context are available for inspection.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
///
/// // Prepare repositories, options and a harness implementing the required traits,
/// // then invoke run_ralph with the workspace path:
/// // let ito = Path::new(".");
/// // run_ralph(ito, &change_repo, &task_repo, &module_repo, opts, &mut harness)?;
/// ```
pub fn run_ralph(
    ito_path: &Path,
    change_repo: &impl DomainChangeRepository,
    task_repo: &impl DomainTaskRepository,
    module_repo: &impl DomainModuleRepository,
    opts: RalphOptions,
    harness: &mut dyn Harness,
) -> CoreResult<()> {
    let process_runner = SystemProcessRunner;

    if opts.continue_ready {
        if opts.continue_module {
            return Err(CoreError::Validation(
                "--continue-ready cannot be used with --continue-module".into(),
            ));
        }
        if opts.change_id.is_some() || opts.module_id.is_some() {
            return Err(CoreError::Validation(
                "--continue-ready cannot be used with --change or --module".into(),
            ));
        }
        if opts.status || opts.add_context.is_some() || opts.clear_context {
            return Err(CoreError::Validation(
                "--continue-ready cannot be combined with --status, --add-context, or --clear-context".into(),
            ));
        }

        loop {
            let current_changes = repo_changes(change_repo)?;
            let eligible_changes = repo_eligible_change_ids(&current_changes);
            print_eligible_changes(&eligible_changes);

            if eligible_changes.is_empty() {
                let incomplete = repo_incomplete_change_ids(&current_changes);
                if incomplete.is_empty() {
                    println!("\nAll changes are complete.");
                    return Ok(());
                }

                return Err(CoreError::Validation(format!(
                    "Repository has no eligible changes. Remaining non-complete changes: {}",
                    incomplete.join(", ")
                )));
            }

            let mut next_change = eligible_changes[0].clone();

            let preflight_changes = repo_changes(change_repo)?;
            let preflight_eligible = repo_eligible_change_ids(&preflight_changes);
            if preflight_eligible.is_empty() {
                let incomplete = repo_incomplete_change_ids(&preflight_changes);
                if incomplete.is_empty() {
                    println!("\nAll changes are complete.");
                    return Ok(());
                }
                return Err(CoreError::Validation(format!(
                    "Repository changed during selection and now has no eligible changes. Remaining non-complete changes: {}",
                    incomplete.join(", ")
                )));
            }
            let preflight_first = preflight_eligible[0].clone();
            if preflight_first != next_change {
                println!(
                    "\nRepository state shifted before start; reorienting from {from} to {to}.",
                    from = next_change,
                    to = preflight_first
                );
                next_change = preflight_first;
            }

            println!(
                "\nStarting change {change} (lowest eligible change id).",
                change = next_change
            );

            let mut single_opts = opts.clone();
            single_opts.continue_ready = false;
            single_opts.change_id = Some(next_change);

            run_ralph(
                ito_path,
                change_repo,
                task_repo,
                module_repo,
                single_opts,
                harness,
            )?;
        }
    }

    if opts.continue_module {
        if opts.change_id.is_some() {
            return Err(CoreError::Validation(
                "--continue-module cannot be used with --change. Use --module only.".into(),
            ));
        }
        let Some(module_id) = opts.module_id.clone() else {
            return Err(CoreError::Validation(
                "--continue-module requires --module".into(),
            ));
        };
        if opts.status || opts.add_context.is_some() || opts.clear_context {
            return Err(CoreError::Validation(
                "--continue-module cannot be combined with --status, --add-context, or --clear-context".into()
            ));
        }

        loop {
            let current_changes = module_changes(change_repo, &module_id)?;
            let ready_changes = module_ready_change_ids(&current_changes);
            print_ready_changes(&module_id, &ready_changes);

            if ready_changes.is_empty() {
                let incomplete = module_incomplete_change_ids(&current_changes);

                if incomplete.is_empty() {
                    println!("\nModule {module} is complete.", module = module_id);
                    return Ok(());
                }

                return Err(CoreError::Validation(format!(
                    "Module {module} has no ready changes. Remaining non-complete changes: {}",
                    incomplete.join(", "),
                    module = module_id
                )));
            }

            let mut next_change = ready_changes[0].clone();

            let preflight_changes = module_changes(change_repo, &module_id)?;
            let preflight_ready = module_ready_change_ids(&preflight_changes);
            if preflight_ready.is_empty() {
                let incomplete = module_incomplete_change_ids(&preflight_changes);
                if incomplete.is_empty() {
                    println!("\nModule {module} is complete.", module = module_id);
                    return Ok(());
                }
                return Err(CoreError::Validation(format!(
                    "Module {module} changed during selection and now has no ready changes. Remaining non-complete changes: {}",
                    incomplete.join(", "),
                    module = module_id
                )));
            }
            let preflight_first = preflight_ready[0].clone();
            if preflight_first != next_change {
                println!(
                    "\nModule state shifted before start; reorienting from {from} to {to}.",
                    from = next_change,
                    to = preflight_first
                );
                next_change = preflight_first;
            }

            println!(
                "\nStarting module change {change} (lowest ready change id).",
                change = next_change
            );

            let mut single_opts = opts.clone();
            single_opts.continue_module = false;
            single_opts.continue_ready = false;
            single_opts.change_id = Some(next_change);

            run_ralph(
                ito_path,
                change_repo,
                task_repo,
                module_repo,
                single_opts,
                harness,
            )?;

            let post_changes = module_changes(change_repo, &module_id)?;
            let post_ready = module_ready_change_ids(&post_changes);
            print_ready_changes(&module_id, &post_ready);
        }
    }

    if opts.change_id.is_none()
        && let Some(module_id) = opts.module_id.as_deref()
        && !opts.status
        && opts.add_context.is_none()
        && !opts.clear_context
    {
        let module_changes = module_changes(change_repo, module_id)?;
        let ready_changes = module_ready_change_ids(&module_changes);
        print_ready_changes(module_id, &ready_changes);
    }

    let unscoped_target = opts.change_id.is_none() && opts.module_id.is_none();
    let (change_id, module_id) = if unscoped_target {
        ("unscoped".to_string(), "unscoped".to_string())
    } else {
        resolve_target(
            change_repo,
            opts.change_id,
            opts.module_id,
            opts.interactive,
        )?
    };

    if opts.status {
        let state = load_state(ito_path, &change_id)?;
        if let Some(state) = state {
            println!("\n=== Ralph Status for {id} ===\n", id = state.change_id);
            println!("Iteration: {iter}", iter = state.iteration);
            println!("History entries: {n}", n = state.history.len());
            if !state.history.is_empty() {
                println!("\nRecent iterations:");
                let n = state.history.len();
                let start = n.saturating_sub(5);
                for (i, h) in state.history.iter().enumerate().skip(start) {
                    println!(
                        "  {idx}: duration={dur}ms, changes={chg}, promise={p}",
                        idx = i + 1,
                        dur = h.duration,
                        chg = h.file_changes_count,
                        p = h.completion_promise_found
                    );
                }
            }
        } else {
            println!("\n=== Ralph Status for {id} ===\n", id = change_id);
            println!("No state found");
        }
        return Ok(());
    }

    if let Some(text) = opts.add_context.as_deref() {
        append_context(ito_path, &change_id, text)?;
        println!("Added context to {id}", id = change_id);
        return Ok(());
    }
    if opts.clear_context {
        clear_context(ito_path, &change_id)?;
        println!("Cleared Ralph context for {id}", id = change_id);
        return Ok(());
    }

    let ito_dir_name = ito_path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| ".ito".to_string());
    let context_file = format!(
        "{ito_dir}/.state/ralph/{change}/context.md",
        ito_dir = ito_dir_name,
        change = change_id
    );

    let mut state = load_state(ito_path, &change_id)?.unwrap_or(RalphState {
        change_id: change_id.clone(),
        iteration: 0,
        history: vec![],
        context_file,
    });

    let max_iters = opts.max_iterations.unwrap_or(u32::MAX);
    if max_iters == 0 {
        return Err(CoreError::Validation(
            "--max-iterations must be >= 1".into(),
        ));
    }
    if opts.error_threshold == 0 {
        return Err(CoreError::Validation(
            "--error-threshold must be >= 1".into(),
        ));
    }

    // Print startup message so user knows something is happening
    println!(
        "\n=== Starting Ralph for {change} (harness: {harness}) ===",
        change = change_id,
        harness = harness.name()
    );
    if let Some(model) = &opts.model {
        println!("Model: {model}");
    }
    if let Some(max) = opts.max_iterations {
        println!("Max iterations: {max}");
    }
    if opts.allow_all {
        println!("Mode: --yolo (auto-approve all)");
    }
    if let Some(timeout) = opts.inactivity_timeout {
        println!("Inactivity timeout: {}", format_duration(timeout));
    }
    println!();

    let mut last_validation_failure: Option<String> = None;
    let mut harness_error_count: u32 = 0;
    let mut retriable_retry_count: u32 = 0;

    for _ in 0..max_iters {
        let iteration = state.iteration.saturating_add(1);

        println!("\n=== Ralph Loop Iteration {i} ===\n", i = iteration);

        let context_content = load_context(ito_path, &change_id)?;
        let prompt = build_ralph_prompt(
            ito_path,
            change_repo,
            module_repo,
            &opts.prompt,
            BuildPromptOptions {
                change_id: if unscoped_target {
                    None
                } else {
                    Some(change_id.clone())
                },
                module_id: if unscoped_target {
                    None
                } else {
                    Some(module_id.clone())
                },
                iteration: Some(iteration),
                max_iterations: opts.max_iterations,
                min_iterations: opts.min_iterations,
                completion_promise: opts.completion_promise.clone(),
                context_content: Some(context_content),
                validation_failure: last_validation_failure.clone(),
            },
        )?;

        if opts.verbose {
            println!("--- Prompt sent to harness ---");
            println!("{}", prompt);
            println!("--- End of prompt ---\n");
        }

        let started = std::time::Instant::now();
        let run = harness
            .run(&crate::harness::HarnessRunConfig {
                prompt,
                model: opts.model.clone(),
                cwd: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
                env: std::collections::BTreeMap::new(),
                interactive: opts.interactive && !opts.allow_all,
                allow_all: opts.allow_all,
                inactivity_timeout: opts.inactivity_timeout,
            })
            .map_err(|e| CoreError::Process(format!("Harness execution failed: {e}")))?;

        // Pass through output if harness didn't already stream it
        if !harness.streams_output() {
            if !run.stdout.is_empty() {
                print!("{}", run.stdout);
            }
            if !run.stderr.is_empty() {
                eprint!("{}", run.stderr);
            }
        }

        // Mirror TS: completion promise is detected from stdout (not stderr).
        let completion_found = completion_promise_found(&run.stdout, &opts.completion_promise);

        let file_changes_count = if harness.name() != HarnessName::Stub {
            count_git_changes(&process_runner)? as u32
        } else {
            0
        };

        // Handle timeout - log and continue to next iteration
        if run.timed_out {
            println!("\n=== Inactivity timeout reached. Restarting iteration... ===\n");
            // Don't update state for timed out iterations, just retry
            continue;
        }

        if run.exit_code != 0 {
            if run.is_retriable() {
                retriable_retry_count = retriable_retry_count.saturating_add(1);
                if retriable_retry_count > MAX_RETRIABLE_RETRIES {
                    return Err(CoreError::Process(format!(
                        "Harness '{name}' crashed {count} consecutive times (exit code {code}); giving up",
                        name = harness.name(),
                        count = retriable_retry_count,
                        code = run.exit_code
                    )));
                }
                println!(
                    "\n=== Harness process crashed (exit code {code}, attempt {count}/{max}). Retrying... ===\n",
                    code = run.exit_code,
                    count = retriable_retry_count,
                    max = MAX_RETRIABLE_RETRIES
                );
                continue;
            }

            // Non-retriable non-zero exit: reset the consecutive crash counter.
            retriable_retry_count = 0;

            if opts.exit_on_error {
                return Err(CoreError::Process(format!(
                    "Harness '{name}' exited with code {code}",
                    name = harness.name(),
                    code = run.exit_code
                )));
            }

            harness_error_count = harness_error_count.saturating_add(1);
            if harness_error_count >= opts.error_threshold {
                return Err(CoreError::Process(format!(
                    "Harness '{name}' exceeded non-zero exit threshold ({count}/{threshold}); last exit code {code}",
                    name = harness.name(),
                    count = harness_error_count,
                    threshold = opts.error_threshold,
                    code = run.exit_code
                )));
            }

            last_validation_failure = Some(render_harness_failure(
                harness.name().as_str(),
                run.exit_code,
                &run.stdout,
                &run.stderr,
            ));
            println!(
                "\n=== Harness exited with code {code} ({count}/{threshold}). Continuing to let Ralph fix it... ===\n",
                code = run.exit_code,
                count = harness_error_count,
                threshold = opts.error_threshold
            );
            continue;
        }

        // Successful exit: reset both counters.
        retriable_retry_count = 0;

        if !opts.no_commit {
            commit_iteration(&process_runner, iteration)?;
        }

        let timestamp = now_ms()?;
        let duration = started.elapsed().as_millis() as i64;
        state.history.push(RalphHistoryEntry {
            timestamp,
            duration,
            completion_promise_found: completion_found,
            file_changes_count,
        });
        state.iteration = iteration;
        save_state(ito_path, &change_id, &state)?;

        if completion_found && iteration >= opts.min_iterations {
            if opts.skip_validation {
                println!("\n=== Warning: --skip-validation set. Completion is not verified. ===\n");
                println!(
                    "\n=== Completion promise \"{p}\" detected. Loop complete. ===\n",
                    p = opts.completion_promise
                );
                return Ok(());
            }

            let report = validate_completion(
                ito_path,
                task_repo,
                if unscoped_target {
                    None
                } else {
                    Some(change_id.as_str())
                },
                opts.validation_command.as_deref(),
            )?;
            if report.passed {
                println!(
                    "\n=== Completion promise \"{p}\" detected (validated). Loop complete. ===\n",
                    p = opts.completion_promise
                );
                return Ok(());
            }

            last_validation_failure = Some(report.context_markdown);
            println!(
                "\n=== Completion promise detected, but validation failed. Continuing... ===\n"
            );
        }
    }

    Ok(())
}

fn module_changes(
    change_repo: &impl DomainChangeRepository,
    module_id: &str,
) -> CoreResult<Vec<ChangeSummary>> {
    let changes = change_repo.list_by_module(module_id).into_core()?;
    if changes.is_empty() {
        return Err(CoreError::NotFound(format!(
            "No changes found for module {module}",
            module = module_id
        )));
    }
    Ok(changes)
}

fn module_ready_change_ids(changes: &[ChangeSummary]) -> Vec<String> {
    let mut ready_change_ids = Vec::new();
    for change in changes {
        if change.is_ready() {
            ready_change_ids.push(change.id.clone());
        }
    }
    ready_change_ids
}

fn repo_changes(change_repo: &impl DomainChangeRepository) -> CoreResult<Vec<ChangeSummary>> {
    change_repo.list().into_core()
}

fn repo_eligible_change_ids(changes: &[ChangeSummary]) -> Vec<String> {
    let mut eligible_change_ids = Vec::new();
    for change in changes {
        let work_status = change.work_status();
        if work_status == ChangeWorkStatus::Ready || work_status == ChangeWorkStatus::InProgress {
            eligible_change_ids.push(change.id.clone());
        }
    }
    eligible_change_ids.sort();
    eligible_change_ids
}

fn repo_incomplete_change_ids(changes: &[ChangeSummary]) -> Vec<String> {
    let mut incomplete_change_ids = Vec::new();
    for change in changes {
        if change.work_status() != ChangeWorkStatus::Complete {
            incomplete_change_ids.push(change.id.clone());
        }
    }
    incomplete_change_ids.sort();
    incomplete_change_ids
}

fn print_eligible_changes(eligible_changes: &[String]) {
    println!("\nEligible changes (ready or in-progress):");
    if eligible_changes.is_empty() {
        println!("  (none)");
        return;
    }

    for (idx, change_id) in eligible_changes.iter().enumerate() {
        if idx == 0 {
            println!("  - {change} (selected first)", change = change_id);
            continue;
        }
        println!("  - {change}", change = change_id);
    }
}

fn module_incomplete_change_ids(changes: &[ChangeSummary]) -> Vec<String> {
    let mut incomplete_change_ids = Vec::new();
    for change in changes {
        if change.work_status() != ChangeWorkStatus::Complete {
            incomplete_change_ids.push(change.id.clone());
        }
    }
    incomplete_change_ids
}

fn print_ready_changes(module_id: &str, ready_changes: &[String]) {
    println!("\nReady changes for module {module}:", module = module_id);
    if ready_changes.is_empty() {
        println!("  (none)");
        return;
    }

    for (idx, change_id) in ready_changes.iter().enumerate() {
        if idx == 0 {
            println!("  - {change} (selected first)", change = change_id);
            continue;
        }
        println!("  - {change}", change = change_id);
    }
}

#[derive(Debug)]
struct CompletionValidationReport {
    passed: bool,
    context_markdown: String,
}

fn validate_completion(
    ito_path: &Path,
    task_repo: &impl DomainTaskRepository,
    change_id: Option<&str>,
    extra_command: Option<&str>,
) -> CoreResult<CompletionValidationReport> {
    let mut passed = true;
    let mut sections: Vec<String> = Vec::new();

    if let Some(change_id) = change_id {
        let task = validation::check_task_completion(task_repo, change_id)?;
        sections.push(render_validation_result("Ito task status", &task));
        if !task.success {
            passed = false;
        }

        // Audit consistency check (warning only, does not fail validation)
        let audit_report = crate::audit::run_reconcile(ito_path, Some(change_id), false);
        if !audit_report.drifts.is_empty() {
            let drift_lines: Vec<String> = audit_report
                .drifts
                .iter()
                .map(|d| format!("  - {d}"))
                .collect();
            sections.push(format!(
                "### Audit consistency\n\n- Result: WARN\n- Summary: {} drift items detected between audit log and file state\n\n{}",
                audit_report.drifts.len(),
                drift_lines.join("\n")
            ));
        }
    } else {
        sections.push(
            "### Ito task status\n\n- Result: SKIP\n- Summary: No change selected; skipped task validation"
                .to_string(),
        );
    }

    let timeout = Duration::from_secs(5 * 60);
    let project = validation::run_project_validation(ito_path, timeout)?;
    sections.push(render_validation_result("Project validation", &project));
    if !project.success {
        passed = false;
    }

    if let Some(cmd) = extra_command {
        let project_root = ito_path.parent().unwrap_or_else(|| Path::new("."));
        let extra = validation::run_extra_validation(project_root, cmd, timeout)?;
        sections.push(render_validation_result("Extra validation", &extra));
        if !extra.success {
            passed = false;
        }
    }

    Ok(CompletionValidationReport {
        passed,
        context_markdown: sections.join("\n\n"),
    })
}

fn render_validation_result(title: &str, r: &validation::ValidationResult) -> String {
    let mut md = String::new();
    md.push_str(&format!("### {title}\n\n"));
    md.push_str(&format!(
        "- Result: {}\n",
        if r.success { "PASS" } else { "FAIL" }
    ));
    md.push_str(&format!("- Summary: {}\n", r.message.trim()));
    if let Some(out) = r.output.as_deref() {
        let out = out.trim();
        if !out.is_empty() {
            md.push_str("\nOutput:\n\n```text\n");
            md.push_str(out);
            md.push_str("\n```\n");
        }
    }
    md
}

fn render_harness_failure(name: &str, exit_code: i32, stdout: &str, stderr: &str) -> String {
    let mut md = String::new();
    md.push_str("### Harness execution\n\n");
    md.push_str("- Result: FAIL\n");
    md.push_str(&format!("- Harness: {name}\n"));
    md.push_str(&format!("- Exit code: {code}\n", code = exit_code));

    let stdout = stdout.trim();
    if !stdout.is_empty() {
        md.push_str("\nStdout:\n\n```text\n");
        md.push_str(stdout);
        md.push_str("\n```\n");
    }

    let stderr = stderr.trim();
    if !stderr.is_empty() {
        md.push_str("\nStderr:\n\n```text\n");
        md.push_str(stderr);
        md.push_str("\n```\n");
    }

    md
}

fn completion_promise_found(stdout: &str, token: &str) -> bool {
    let mut rest = stdout;
    loop {
        let Some(start) = rest.find("<promise>") else {
            return false;
        };
        let after_start = &rest[start + "<promise>".len()..];
        let Some(end) = after_start.find("</promise>") else {
            return false;
        };
        let inner = &after_start[..end];
        if inner.trim() == token {
            return true;
        }

        rest = &after_start[end + "</promise>".len()..];
    }
}

fn resolve_target(
    change_repo: &impl DomainChangeRepository,
    change_id: Option<String>,
    module_id: Option<String>,
    interactive: bool,
) -> CoreResult<(String, String)> {
    // If change is provided, resolve canonical ID and infer module.
    if let Some(change) = change_id {
        let change = match change_repo.resolve_target(&change) {
            ChangeTargetResolution::Unique(id) => id,
            ChangeTargetResolution::Ambiguous(matches) => {
                return Err(CoreError::Validation(format!(
                    "Change '{change}' is ambiguous. Matches: {}",
                    matches.join(", ")
                )));
            }
            ChangeTargetResolution::NotFound => {
                return Err(CoreError::NotFound(format!("Change '{change}' not found")));
            }
        };
        let module = infer_module_from_change(&change)?;
        return Ok((change, module));
    }

    if let Some(module) = module_id {
        let changes = change_repo.list_by_module(&module).into_core()?;
        if changes.is_empty() {
            return Err(CoreError::NotFound(format!(
                "No changes found for module {module}",
                module = module
            )));
        }

        let ready_changes = module_ready_change_ids(&changes);
        if let Some(change_id) = ready_changes.first() {
            return Ok((change_id.clone(), infer_module_from_change(change_id)?));
        }

        let incomplete = module_incomplete_change_ids(&changes);

        if incomplete.is_empty() {
            return Err(CoreError::Validation(format!(
                "Module {module} has no ready changes because all changes are complete",
                module = module
            )));
        }

        return Err(CoreError::Validation(format!(
            "Module {module} has no ready changes. Remaining non-complete changes: {}",
            incomplete.join(", "),
            module = module
        )));
    }

    if !interactive {
        return Err(CoreError::Validation(
            "Change selection requires interactive mode. Use --change to specify or run in interactive mode.".into()
        ));
    }

    Err(CoreError::Validation(
        "Interactive selection is not yet implemented in Rust. Use --change to specify.".into(),
    ))
}

fn infer_module_from_change(change_id: &str) -> CoreResult<String> {
    let Some((module, _rest)) = change_id.split_once('-') else {
        return Err(CoreError::Validation(format!(
            "Invalid change ID format: {id}",
            id = change_id
        )));
    };
    Ok(module.to_string())
}

fn now_ms() -> CoreResult<i64> {
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| CoreError::Process(format!("Clock error: {e}")))?;
    Ok(dur.as_millis() as i64)
}

fn count_git_changes(runner: &dyn ProcessRunner) -> CoreResult<usize> {
    let request = ProcessRequest::new("git").args(["status", "--porcelain"]);
    let out = runner
        .run(&request)
        .map_err(|e| CoreError::Process(format!("Failed to run git status: {e}")))?;
    if !out.success {
        // Match TS behavior: the git error output is visible to the user.
        let err = out.stderr;
        if !err.is_empty() {
            eprint!("{}", err);
        }
        return Ok(0);
    }
    let s = out.stdout;
    let mut line_count = 0;
    for line in s.lines() {
        if !line.trim().is_empty() {
            line_count += 1;
        }
    }
    Ok(line_count)
}

fn commit_iteration(runner: &dyn ProcessRunner, iteration: u32) -> CoreResult<()> {
    let add_request = ProcessRequest::new("git").args(["add", "-A"]);
    let add = runner
        .run(&add_request)
        .map_err(|e| CoreError::Process(format!("Failed to run git add: {e}")))?;
    if !add.success {
        let stdout = add.stdout.trim().to_string();
        let stderr = add.stderr.trim().to_string();
        let mut msg = String::from("git add failed");
        if !stdout.is_empty() {
            msg.push_str("\nstdout:\n");
            msg.push_str(&stdout);
        }
        if !stderr.is_empty() {
            msg.push_str("\nstderr:\n");
            msg.push_str(&stderr);
        }
        return Err(CoreError::Process(msg));
    }

    let msg = format!("Ralph loop iteration {iteration}");
    let commit_request = ProcessRequest::new("git").args(["commit", "-m", &msg]);
    let commit = runner
        .run(&commit_request)
        .map_err(|e| CoreError::Process(format!("Failed to run git commit: {e}")))?;
    if !commit.success {
        let stdout = commit.stdout.trim().to_string();
        let stderr = commit.stderr.trim().to_string();
        let mut msg = format!("git commit failed for iteration {iteration}");
        if !stdout.is_empty() {
            msg.push_str("\nstdout:\n");
            msg.push_str(&stdout);
        }
        if !stderr.is_empty() {
            msg.push_str("\nstderr:\n");
            msg.push_str(&stderr);
        }
        return Err(CoreError::Process(msg));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // completion_promise_found tests
    // ========================================================================

    /// Single promise tag with matching token returns true.
    #[test]
    fn test_completion_promise_found_single_match() {
        let stdout = "Some output\n<promise>TOKEN</promise>\nMore output";
        let result = completion_promise_found(stdout, "TOKEN");
        assert!(result);
    }

    /// No promise tags returns false.
    #[test]
    fn test_completion_promise_found_no_tags() {
        let stdout = "Some output without any promise tags";
        let result = completion_promise_found(stdout, "TOKEN");
        assert!(!result);
    }

    /// Multiple promise tags, first doesn't match but second does returns true.
    #[test]
    fn test_completion_promise_found_second_match() {
        let stdout = "<promise>WRONG</promise>\n<promise>TOKEN</promise>";
        let result = completion_promise_found(stdout, "TOKEN");
        assert!(result);
    }

    /// Empty token with empty promise tags returns true.
    #[test]
    fn test_completion_promise_found_empty_token() {
        let stdout = "<promise></promise>";
        let result = completion_promise_found(stdout, "");
        assert!(result);
    }

    /// Token only in stderr (empty stdout) returns false.
    #[test]
    fn test_completion_promise_found_empty_stdout() {
        let stdout = "";
        let result = completion_promise_found(stdout, "TOKEN");
        assert!(!result);
    }

    /// Whitespace around token is trimmed and matches.
    #[test]
    fn test_completion_promise_found_whitespace_trimmed() {
        let stdout = "<promise>  TOKEN  </promise>";
        let result = completion_promise_found(stdout, "TOKEN");
        assert!(result);
    }

    /// Nested promise tags - first closing tag is matched, inner content includes tag.
    #[test]
    fn test_completion_promise_found_nested_tags() {
        let stdout = "<promise><promise>TOKEN</promise></promise>";
        let result = completion_promise_found(stdout, "TOKEN");
        // The function finds first <promise>, then first </promise>,
        // so inner content is "<promise>TOKEN" which doesn't match "TOKEN"
        assert!(!result);
    }

    /// Incomplete tag (no closing) returns false.
    #[test]
    fn test_completion_promise_found_incomplete_tag() {
        let stdout = "<promise>TOKEN";
        let result = completion_promise_found(stdout, "TOKEN");
        assert!(!result);
    }

    // ========================================================================
    // infer_module_from_change tests
    // ========================================================================

    /// Valid change ID with underscore returns module prefix.
    #[test]
    fn test_infer_module_from_change_valid_with_underscore() {
        let result = infer_module_from_change("003-05_foo");
        assert!(result.is_ok());
        let module = result.unwrap();
        assert_eq!(module, "003");
    }

    /// Valid change ID without underscore returns module prefix.
    #[test]
    fn test_infer_module_from_change_valid_without_underscore() {
        let result = infer_module_from_change("003-05");
        assert!(result.is_ok());
        let module = result.unwrap();
        assert_eq!(module, "003");
    }

    /// No hyphen in change ID returns validation error.
    #[test]
    fn test_infer_module_from_change_no_hyphen() {
        let result = infer_module_from_change("nohyphen");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let matches_validation = match err {
            CoreError::Validation(msg) => msg.contains("Invalid change ID format"),
            _ => false,
        };
        assert!(matches_validation);
    }

    // ========================================================================
    // render_validation_result tests
    // ========================================================================

    /// Passing validation result contains PASS and message.
    #[test]
    fn test_render_validation_result_passing() {
        let result = validation::ValidationResult {
            success: true,
            message: "All checks passed".to_string(),
            output: None,
        };
        let rendered = render_validation_result("Test Validation", &result);
        assert!(rendered.contains("### Test Validation"));
        assert!(rendered.contains("PASS"));
        assert!(rendered.contains("All checks passed"));
    }

    /// Failing validation result with output contains FAIL and code block.
    #[test]
    fn test_render_validation_result_failing_with_output() {
        let result = validation::ValidationResult {
            success: false,
            message: "Check failed".to_string(),
            output: Some("Error details here".to_string()),
        };
        let rendered = render_validation_result("Test Validation", &result);
        assert!(rendered.contains("### Test Validation"));
        assert!(rendered.contains("FAIL"));
        assert!(rendered.contains("Check failed"));
        assert!(rendered.contains("```text"));
        assert!(rendered.contains("Error details here"));
    }

    /// Result with no output does not include code block.
    #[test]
    fn test_render_validation_result_no_output() {
        let result = validation::ValidationResult {
            success: true,
            message: "Success".to_string(),
            output: None,
        };
        let rendered = render_validation_result("Test Validation", &result);
        assert!(!rendered.contains("```"));
    }

    /// Result with whitespace-only output does not include code block.
    #[test]
    fn test_render_validation_result_whitespace_only_output() {
        let result = validation::ValidationResult {
            success: true,
            message: "Success".to_string(),
            output: Some("   \n\t  ".to_string()),
        };
        let rendered = render_validation_result("Test Validation", &result);
        assert!(!rendered.contains("```"));
    }

    // ========================================================================
    // render_harness_failure tests
    // ========================================================================

    /// Harness failure with both stdout and stderr includes both code blocks.
    #[test]
    fn test_render_harness_failure_both_outputs() {
        let rendered =
            render_harness_failure("test-harness", 1, "stdout content", "stderr content");
        assert!(rendered.contains("### Harness execution"));
        assert!(rendered.contains("FAIL"));
        assert!(rendered.contains("test-harness"));
        assert!(rendered.contains("Exit code: 1"));
        assert!(rendered.contains("Stdout:"));
        assert!(rendered.contains("stdout content"));
        assert!(rendered.contains("Stderr:"));
        assert!(rendered.contains("stderr content"));
    }

    /// Harness failure with empty stdout does not include stdout block.
    #[test]
    fn test_render_harness_failure_empty_stdout() {
        let rendered = render_harness_failure("test-harness", 1, "", "stderr content");
        assert!(!rendered.contains("Stdout:"));
        assert!(rendered.contains("Stderr:"));
        assert!(rendered.contains("stderr content"));
    }

    /// Harness failure with empty stderr does not include stderr block.
    #[test]
    fn test_render_harness_failure_empty_stderr() {
        let rendered = render_harness_failure("test-harness", 1, "stdout content", "");
        assert!(rendered.contains("Stdout:"));
        assert!(rendered.contains("stdout content"));
        assert!(!rendered.contains("Stderr:"));
    }

    /// Harness failure with both empty includes only header section.
    #[test]
    fn test_render_harness_failure_both_empty() {
        let rendered = render_harness_failure("test-harness", 1, "", "");
        assert!(rendered.contains("### Harness execution"));
        assert!(rendered.contains("FAIL"));
        assert!(rendered.contains("test-harness"));
        assert!(rendered.contains("Exit code: 1"));
        assert!(!rendered.contains("Stdout:"));
        assert!(!rendered.contains("Stderr:"));
    }
}
