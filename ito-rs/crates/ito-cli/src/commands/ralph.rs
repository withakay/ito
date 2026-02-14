use crate::cli::RalphArgs;
use crate::cli_error::{fail, to_cli_error, CliResult};
use crate::runtime::Runtime;
use ito_core::change_repository::FsChangeRepository;
use ito_core::harness::stub::StubHarness;
use ito_core::harness::ClaudeCodeHarness;
use ito_core::harness::CodexHarness;
use ito_core::harness::GitHubCopilotHarness;
use ito_core::harness::Harness;
use ito_core::harness::OpencodeHarness;
use ito_core::module_repository::FsModuleRepository;
use ito_core::ralph as core_ralph;
use ito_core::task_repository::FsTaskRepository;

/// Handle the `ito loop` command (deprecated alias for `ito ralph`).
pub(crate) fn handle_loop_clap(rt: &Runtime, args: &RalphArgs) -> CliResult<()> {
    eprintln!("Warning: `ito loop` is deprecated. Use `ito ralph` instead.");
    handle_ralph_clap(rt, args)
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
/// handle_ralph_clap(&rt, &args).expect("ralph command failed");
/// ```
pub(crate) fn handle_ralph_clap(rt: &Runtime, args: &RalphArgs) -> CliResult<()> {
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
        return fail(
            "Either --change, --module, --continue-ready, --status, --add-context, --clear-context, or --file must be specified",
        );
    }

    if args.clear_context && args.change.is_none() {
        return fail("--change is required for --clear-context");
    }
    if args.add_context.is_some() && args.change.is_none() {
        return fail("--change is required for --add-context");
    }
    if args.status && args.change.is_none() && args.module.is_none() {
        return fail("--change is required for --status, or provide --module to auto-select");
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

    let mut harness_impl: Box<dyn Harness> = match args.harness.as_str() {
        "claude" => Box::new(ClaudeCodeHarness),
        "codex" => Box::new(CodexHarness),
        "github-copilot" | "copilot" => Box::new(GitHubCopilotHarness),
        "opencode" => Box::new(OpencodeHarness),
        "stub" => {
            let p = args.stub_script.as_ref().map(std::path::PathBuf::from);
            match StubHarness::from_env_or_default(p) {
                Ok(h) => Box::new(h),
                Err(e) => return Err(to_cli_error(e)),
            }
        }
        _ => {
            let known = ito_core::harness::HarnessName::user_facing()
                .map(|h| h.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            return fail(format!(
                "Unknown harness: {h} (known harnesses: {known})",
                h = args.harness,
            ));
        }
    };

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
    };

    let change_repo = FsChangeRepository::new(ito_path);
    let module_repo = FsModuleRepository::new(ito_path);
    let task_repo = FsTaskRepository::new(ito_path);

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
