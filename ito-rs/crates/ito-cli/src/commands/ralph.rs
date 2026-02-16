use crate::cli::{HarnessArg, RalphArgs};
use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use ito_core::change_repository::FsChangeRepository;
use ito_core::harness::ClaudeCodeHarness;
use ito_core::harness::CodexHarness;
use ito_core::harness::GitHubCopilotHarness;
use ito_core::harness::Harness;
use ito_core::harness::OpencodeHarness;
use ito_core::harness::stub::StubHarness;
use ito_core::module_repository::FsModuleRepository;
use ito_core::ralph as core_ralph;
use ito_core::task_repository::FsTaskRepository;
use std::io::IsTerminal;

/// Handle the `ito loop` command (deprecated alias for `ito ralph`).
pub(crate) fn handle_loop_clap(
    rt: &Runtime,
    args: &RalphArgs,
    raw_args: &[String],
) -> CliResult<()> {
    eprintln!("Warning: `ito loop` is deprecated. Use `ito ralph` instead.");
    handle_ralph_clap(rt, args, raw_args)
}

fn load_worktree_config(ito_path: &std::path::Path, rt: &Runtime) -> core_ralph::WorktreeConfig {
    let project_root = ito_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));
    let cfg = ito_config::load_cascading_project_config(project_root, ito_path, rt.ctx());
    let merged = cfg.merged;
    let enabled = merged
        .pointer("/worktrees/enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let dir_name = merged
        .pointer("/worktrees/layout/dir_name")
        .and_then(|v| v.as_str())
        .unwrap_or("ito-worktrees")
        .to_string();
    core_ralph::WorktreeConfig { enabled, dir_name }
}

/// Handle the `ito ralph` command using parsed `RalphArgs`.
///
/// Validates mutually dependent flags, composes the prompt from an optional
/// prompt file plus positional prompt tokens, parses the optional inactivity
/// timeout, selects and initializes the requested harness, assembles
/// `RalphOptions`, initializes filesystem-backed repositories, and invokes the
/// core Ralph runtime. Returns a CLI error when validations fail, when the
/// prompt file cannot be read, when the timeout is invalid, when a stub harness
/// cannot be constructed, or when the Ralph runtime returns an error.
///
/// # Returns
///
/// `Ok(())` on success, or a `CliResult` error describing the failure.
///
/// # Examples
///
/// ```no_run
/// use ito_cli::{handle_ralph_clap, RalphArgs};
/// use ito_core::Runtime;
///
/// // Construct a runtime and args appropriate for your environment.
/// let rt = Runtime::from_env();
/// let args = RalphArgs {
///     prompt: vec!["Describe the change".into()],
///     ..Default::default()
/// };
///
/// let raw_args: Vec<String> = Vec::new();
/// handle_ralph_clap(&rt, &args, &raw_args).expect("ralph command failed");
/// ```
pub(crate) fn handle_ralph_clap(
    rt: &Runtime,
    args: &RalphArgs,
    raw_args: &[String],
) -> CliResult<()> {
    let interactive = !args.no_interactive;

    if !interactive
        && args.change.is_none()
        && args.module.is_none()
        && !args.continue_ready
        && !args.status
        && args.add_context.is_none()
        && !args.clear_context
        && args.file.is_none()
    {
        return fail("When --no-interactive is set, you must provide --change (or --module).");
    }

    if !interactive {
        if args.clear_context && args.change.is_none() {
            return fail("--change is required for --clear-context when --no-interactive is set");
        }
        if args.add_context.is_some() && args.change.is_none() {
            return fail("--change is required for --add-context when --no-interactive is set");
        }
        if args.status && args.change.is_none() && args.module.is_none() {
            return fail("--change is required for --status when --no-interactive is set");
        }
    }

    let positional_prompt = args.prompt.join(" ");
    let prompt = if let Some(path) = &args.file {
        let path_buf = std::path::PathBuf::from(path);
        let file_prompt = ito_common::io::read_to_string_std(&path_buf)
            .map_err(|e| to_cli_error(miette::miette!("Failed to read prompt file {path}: {e}")))?;
        if positional_prompt.is_empty() {
            file_prompt
        } else {
            format!("{file_prompt}\n\n{positional_prompt}")
        }
    } else {
        positional_prompt
    };

    let inactivity_timeout = if let Some(raw) = &args.timeout {
        match core_ralph::parse_duration(raw) {
            Ok(d) => Some(d),
            Err(e) => {
                return fail(format!("Invalid --timeout '{raw}': {e}"));
            }
        }
    } else {
        None
    };

    let error_threshold = args
        .error_threshold
        .unwrap_or(core_ralph::DEFAULT_ERROR_THRESHOLD);

    let ito_path = rt.ito_path();

    let worktree_config = load_worktree_config(ito_path, rt);

    let change_repo = FsChangeRepository::new(ito_path);
    let module_repo = FsModuleRepository::new(ito_path);
    let task_repo = FsTaskRepository::new(ito_path);

    // Interactive target selection lives in the CLI layer.
    // When no explicit change is provided, prompt for one or more changes and
    // run Ralph sequentially for each selection.
    let needs_picker = interactive
        && args.change.is_none()
        && args.file.is_none()
        && !args.continue_ready
        && !args.continue_module;

    if needs_picker {
        let is_tty = std::io::stdin().is_terminal() && std::io::stdout().is_terminal();
        if !is_tty {
            return fail(
                "Interactive change selection requires a TTY. Use --no-interactive with --change or --module.",
            );
        }

        let single_target = args.status || args.add_context.is_some() || args.clear_context;
        let selected = pick_change_ids(&change_repo, args.module.as_deref(), single_target)?;

        let mut overrides = RalphWizardOverrides::from_args(args);
        if !single_target {
            overrides = prompt_missing_ralph_options(raw_args, overrides)?;
        }

        let mut harness_impl: Box<dyn Harness> = make_harness(overrides.harness, args)?;
        let base_opts = core_ralph::RalphOptions {
            prompt,
            change_id: None,
            module_id: None,
            model: overrides.model.clone(),
            min_iterations: overrides.min_iterations,
            max_iterations: overrides.max_iterations,
            completion_promise: args.completion_promise.clone(),
            allow_all: overrides.allow_all,
            no_commit: overrides.no_commit,
            interactive,
            status: args.status,
            add_context: args.add_context.clone(),
            clear_context: args.clear_context,
            verbose: args.verbose,
            continue_module: args.continue_module,
            continue_ready: args.continue_ready,
            inactivity_timeout,
            skip_validation: args.skip_validation,
            validation_command: args.validation_command.clone(),
            exit_on_error: overrides.exit_on_error,
            error_threshold,
            worktree: worktree_config,
        };

        for (idx, change_id) in selected.iter().enumerate() {
            if selected.len() > 1 {
                println!(
                    "\n=== Ralph Selection {i}/{n}: {change} ===\n",
                    i = idx + 1,
                    n = selected.len(),
                    change = change_id
                );
            }

            let mut per_change = base_opts.clone();
            per_change.change_id = Some(change_id.clone());

            core_ralph::run_ralph(
                ito_path,
                &change_repo,
                &task_repo,
                &module_repo,
                per_change,
                harness_impl.as_mut(),
            )
            .map_err(to_cli_error)?;
        }

        return Ok(());
    }

    let mut harness_impl: Box<dyn Harness> = make_harness(args.harness, args)?;
    let opts = core_ralph::RalphOptions {
        prompt,
        change_id: args.change.clone(),
        module_id: args.module.clone(),
        model: args.model.clone(),
        min_iterations: args.min_iterations,
        max_iterations: args.max_iterations,
        completion_promise: args.completion_promise.clone(),
        allow_all: args.allow_all,
        no_commit: args.no_commit,
        interactive,
        status: args.status,
        add_context: args.add_context.clone(),
        clear_context: args.clear_context,
        verbose: args.verbose,
        continue_module: args.continue_module,
        continue_ready: args.continue_ready,
        inactivity_timeout,
        skip_validation: args.skip_validation,
        validation_command: args.validation_command.clone(),
        exit_on_error: args.exit_on_error,
        error_threshold,
        worktree: worktree_config,
    };

    core_ralph::run_ralph(
        ito_path,
        &change_repo,
        &task_repo,
        &module_repo,
        opts,
        harness_impl.as_mut(),
    )
    .map_err(to_cli_error)?;

    Ok(())
}

#[derive(Debug, Clone)]
struct RalphWizardOverrides {
    harness: HarnessArg,
    model: Option<String>,
    min_iterations: u32,
    max_iterations: Option<u32>,
    no_commit: bool,
    allow_all: bool,
    exit_on_error: bool,
}

impl RalphWizardOverrides {
    fn from_args(args: &RalphArgs) -> Self {
        Self {
            harness: args.harness,
            model: args.model.clone(),
            min_iterations: args.min_iterations,
            max_iterations: args.max_iterations,
            no_commit: args.no_commit,
            allow_all: args.allow_all,
            exit_on_error: args.exit_on_error,
        }
    }
}

fn make_harness(selected: HarnessArg, args: &RalphArgs) -> CliResult<Box<dyn Harness>> {
    Ok(match selected {
        HarnessArg::Claude => Box::new(ClaudeCodeHarness),
        HarnessArg::Codex => Box::new(CodexHarness),
        HarnessArg::Copilot => Box::new(GitHubCopilotHarness),
        HarnessArg::Opencode => Box::new(OpencodeHarness),
        HarnessArg::Stub => {
            let p = args.stub_script.as_ref().map(std::path::PathBuf::from);
            let h = StubHarness::from_env_or_default(p).map_err(to_cli_error)?;
            Box::new(h)
        }
    })
}

fn prompt_missing_ralph_options(
    raw_args: &[String],
    mut overrides: RalphWizardOverrides,
) -> CliResult<RalphWizardOverrides> {
    let theme = dialoguer::theme::ColorfulTheme::default();

    println!("\n--- Ralph Options ---\n");

    if !argv_has_flag(raw_args, "--harness") {
        let items: &[(HarnessArg, &str)] = &[
            (HarnessArg::Opencode, "opencode"),
            (HarnessArg::Claude, "claude"),
            (HarnessArg::Codex, "codex"),
            (HarnessArg::Copilot, "copilot"),
        ];

        let default_idx = match overrides.harness {
            HarnessArg::Opencode => 0,
            HarnessArg::Claude => 1,
            HarnessArg::Codex => 2,
            HarnessArg::Copilot => 3,
            HarnessArg::Stub => 0,
        };

        let labels: Vec<&str> = items.iter().map(|(_, l)| *l).collect();

        let idx = match dialoguer::Select::with_theme(&theme)
            .with_prompt("Harness")
            .items(&labels)
            .default(default_idx)
            .interact_opt()
        {
            Ok(Some(v)) => v,
            Ok(None) => return fail("Selection cancelled"),
            Err(e) => return fail(format!("Failed to prompt for harness: {e}")),
        };

        overrides.harness = items[idx].0;
    }

    if !argv_has_flag(raw_args, "--model") {
        let initial = overrides.model.clone().unwrap_or_default();
        let v = match dialoguer::Input::<String>::with_theme(&theme)
            .with_prompt("Model (optional, blank = default)")
            .with_initial_text(initial)
            .allow_empty(true)
            .interact_text()
        {
            Ok(v) => v,
            Err(e) => return fail(format!("Failed to prompt for model: {e}")),
        };
        overrides.model = if v.trim().is_empty() { None } else { Some(v) };
    }

    if !argv_has_flag(raw_args, "--min-iterations") {
        let v = match dialoguer::Input::<u32>::with_theme(&theme)
            .with_prompt("Min iterations")
            .default(overrides.min_iterations)
            .validate_with(|v: &u32| -> Result<(), String> {
                if *v == 0 {
                    return Err("Must be >= 1".to_string());
                }
                Ok(())
            })
            .interact_text()
        {
            Ok(v) => v,
            Err(e) => return fail(format!("Failed to prompt for min iterations: {e}")),
        };

        overrides.min_iterations = v;
    }

    if !argv_has_flag(raw_args, "--max-iterations") {
        let default = overrides
            .max_iterations
            .map(|v| v.to_string())
            .unwrap_or_default();

        let v = match dialoguer::Input::<String>::with_theme(&theme)
            .with_prompt("Max iterations (blank = unlimited)")
            .default(default)
            .allow_empty(true)
            .validate_with(|s: &String| -> Result<(), String> {
                let s = s.trim();
                if s.is_empty() {
                    return Ok(());
                }
                let n: u32 = s
                    .parse()
                    .map_err(|_| "Must be a positive integer or blank".to_string())?;
                if n == 0 {
                    return Err("Must be >= 1".to_string());
                }
                Ok(())
            })
            .interact_text()
        {
            Ok(v) => v,
            Err(e) => return fail(format!("Failed to prompt for max iterations: {e}")),
        };

        overrides.max_iterations = match v.trim() {
            "" => None,
            v => Some(v.parse().unwrap()),
        };
    }

    if !argv_has_flag(raw_args, "--no-commit") {
        let v = match dialoguer::Confirm::with_theme(&theme)
            .with_prompt("Disable per-iteration git commits? (--no-commit)")
            .default(overrides.no_commit)
            .interact_opt()
        {
            Ok(Some(v)) => v,
            Ok(None) => return fail("Selection cancelled"),
            Err(e) => return fail(format!("Failed to prompt for --no-commit: {e}")),
        };
        overrides.no_commit = v;
    }

    if !argv_has_any_flag(
        raw_args,
        &["--allow-all", "--yolo", "--dangerously-allow-all"],
    ) {
        let v = match dialoguer::Confirm::with_theme(&theme)
            .with_prompt("Allow all tool actions? (--allow-all)")
            .default(overrides.allow_all)
            .interact_opt()
        {
            Ok(Some(v)) => v,
            Ok(None) => return fail("Selection cancelled"),
            Err(e) => return fail(format!("Failed to prompt for --allow-all: {e}")),
        };
        overrides.allow_all = v;
    }

    if !argv_has_flag(raw_args, "--exit-on-error") {
        let v = match dialoguer::Confirm::with_theme(&theme)
            .with_prompt("Exit immediately on harness error? (--exit-on-error)")
            .default(overrides.exit_on_error)
            .interact_opt()
        {
            Ok(Some(v)) => v,
            Ok(None) => return fail("Selection cancelled"),
            Err(e) => return fail(format!("Failed to prompt for --exit-on-error: {e}")),
        };
        overrides.exit_on_error = v;
    }

    if let Some(max) = overrides.max_iterations
        && max < overrides.min_iterations
    {
        return fail(format!(
            "max-iterations ({max}) must be >= min-iterations ({min})",
            min = overrides.min_iterations
        ));
    }

    Ok(overrides)
}

fn argv_has_flag(argv: &[String], flag: &str) -> bool {
    let eq_prefix = format!("{flag}=");
    for arg in argv {
        if arg == flag || arg.starts_with(&eq_prefix) {
            return true;
        }
    }
    false
}

fn argv_has_any_flag(argv: &[String], flags: &[&str]) -> bool {
    for f in flags {
        if argv_has_flag(argv, f) {
            return true;
        }
    }
    false
}

fn pick_change_ids(
    change_repo: &FsChangeRepository<'_>,
    module_id: Option<&str>,
    single: bool,
) -> CliResult<Vec<String>> {
    let changes = match module_id {
        Some(module_id) => change_repo.list_by_module(module_id),
        None => change_repo.list(),
    }
    .map_err(to_cli_error)?;

    if changes.is_empty() {
        return match module_id {
            Some(module_id) => fail(format!("No changes found for module {module_id}")),
            None => fail("No changes found in repository"),
        };
    }

    if changes.len() == 1 {
        return Ok(vec![changes[0].id.clone()]);
    }

    let ids: Vec<String> = changes.into_iter().map(|c| c.id).collect();
    let theme = dialoguer::theme::ColorfulTheme::default();

    if single {
        let prompt = match module_id {
            Some(module_id) => format!("Select a change in module {module_id}"),
            None => "Select a change".to_string(),
        };

        let idx = match dialoguer::Select::with_theme(&theme)
            .with_prompt(prompt)
            .items(&ids)
            .default(0)
            .interact_opt()
        {
            Ok(Some(v)) => v,
            Ok(None) => return fail("Selection cancelled"),
            Err(e) => return fail(format!("Failed to prompt for change selection: {e}")),
        };

        return Ok(vec![ids[idx].clone()]);
    }

    let prompt = match module_id {
        Some(module_id) => format!("Select change(s) in module {module_id}"),
        None => "Select change(s)".to_string(),
    };

    let mut indices = match dialoguer::MultiSelect::with_theme(&theme)
        .with_prompt(prompt)
        .items(&ids)
        .interact_opt()
    {
        Ok(Some(v)) => v,
        Ok(None) => return fail("Selection cancelled"),
        Err(e) => return fail(format!("Failed to prompt for change selection: {e}")),
    };

    if indices.is_empty() {
        return fail("No changes selected");
    }

    indices.sort();
    Ok(indices.into_iter().map(|i| ids[i].clone()).collect())
}
