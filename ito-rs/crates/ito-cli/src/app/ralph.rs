use crate::cli::RalphArgs;
use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use crate::util::parse_string_flag;
use ito_core::change_repository::FsChangeRepository;
use ito_core::harness::Harness;
use ito_core::harness::OpencodeHarness;
use ito_core::harness::stub::StubHarness;
use ito_core::module_repository::FsModuleRepository;
use ito_core::ralph as core_ralph;
use ito_core::task_repository::FsTaskRepository;

pub(crate) fn handle_loop(rt: &Runtime, args: &[String]) -> CliResult<()> {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        let loop_help = super::common::render_command_long_help(&["loop"], "ito loop");
        let ralph_help = super::common::render_command_long_help(&["ralph"], "ito ralph");
        println!("{loop_help}\n\n{ralph_help}");
        return Ok(());
    }
    // Match TS: loop is deprecated wrapper.
    eprintln!("Warning: `ito loop` is deprecated. Use `ito ralph` instead.");
    handle_ralph(rt, args)
}

pub(crate) fn handle_ralph(rt: &Runtime, args: &[String]) -> CliResult<()> {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "{}",
            super::common::render_command_long_help(&["ralph"], "ito ralph")
        );
        return Ok(());
    }

    fn parse_u32_flag(args: &[String], key: &str) -> Option<u32> {
        parse_string_flag(args, key).and_then(|v| v.parse::<u32>().ok())
    }

    fn collect_prompt(args: &[String]) -> String {
        // Collect positional args, skipping known flags + their values.
        let mut out: Vec<String> = Vec::new();
        let mut i = 0;
        while i < args.len() {
            let a = args[i].as_str();
            let takes_value = matches!(
                a,
                "--change"
                    | "--module"
                    | "--harness"
                    | "--model"
                    | "--min-iterations"
                    | "--max-iterations"
                    | "--completion-promise"
                    | "--validation-command"
                    | "--error-threshold"
                    | "--add-context"
                    | "--timeout"
                    | "--stub-script"
                    | "--file"
            );

            if takes_value {
                i += 2;
                continue;
            }

            if a.starts_with("--change=")
                || a.starts_with("--module=")
                || a.starts_with("--harness=")
                || a.starts_with("--model=")
                || a.starts_with("--min-iterations=")
                || a.starts_with("--max-iterations=")
                || a.starts_with("--completion-promise=")
                || a.starts_with("--validation-command=")
                || a.starts_with("--error-threshold=")
                || a.starts_with("--add-context=")
                || a.starts_with("--timeout=")
                || a.starts_with("--stub-script=")
                || a.starts_with("--file=")
            {
                i += 1;
                continue;
            }

            if a.starts_with('-') {
                i += 1;
                continue;
            }

            out.push(args[i].clone());
            i += 1;
        }
        out.join(" ")
    }

    let change_id = parse_string_flag(args, "--change");
    let module_id = parse_string_flag(args, "--module");

    let harness = parse_string_flag(args, "--harness").unwrap_or_else(|| "opencode".to_string());
    let model = parse_string_flag(args, "--model");

    let min_iterations = parse_u32_flag(args, "--min-iterations").unwrap_or(1);
    let max_iterations = parse_u32_flag(args, "--max-iterations");
    let completion_promise =
        parse_string_flag(args, "--completion-promise").unwrap_or_else(|| "COMPLETE".to_string());

    let skip_validation = args.iter().any(|a| a == "--skip-validation");
    let validation_command = parse_string_flag(args, "--validation-command");
    let exit_on_error = args.iter().any(|a| a == "--exit-on-error");
    let error_threshold =
        parse_u32_flag(args, "--error-threshold").unwrap_or(core_ralph::DEFAULT_ERROR_THRESHOLD);

    let allow_all = args.iter().any(|a| {
        matches!(
            a.as_str(),
            "--allow-all" | "--yolo" | "--dangerously-allow-all"
        )
    });
    let no_commit = args.iter().any(|a| a == "--no-commit");
    let status = args.iter().any(|a| a == "--status");
    let add_context = parse_string_flag(args, "--add-context");
    let clear_context = args.iter().any(|a| a == "--clear-context");
    let interactive = !args.iter().any(|a| a == "--no-interactive");
    let verbose = args.iter().any(|a| a == "--verbose" || a == "-v");
    let continue_module = args.iter().any(|a| a == "--continue-module");
    let continue_ready = args.iter().any(|a| a == "--continue-ready");

    let inactivity_timeout = if let Some(raw) = parse_string_flag(args, "--timeout") {
        match core_ralph::parse_duration(&raw) {
            Ok(d) => Some(d),
            Err(e) => {
                return fail(format!("Invalid --timeout '{raw}': {e}"));
            }
        }
    } else {
        None
    };

    let prompt_file = parse_string_flag(args, "--file");
    // Hidden testing flag.
    let stub_script = parse_string_flag(args, "--stub-script");

    if !interactive
        && change_id.is_none()
        && module_id.is_none()
        && !continue_ready
        && !status
        && add_context.is_none()
        && !clear_context
        && prompt_file.is_none()
    {
        return fail(
            "Either --change, --module, --continue-ready, --status, --add-context, --clear-context, or --file must be specified",
        );
    }

    if clear_context && change_id.is_none() {
        return fail("--change is required for --clear-context");
    }
    if add_context.is_some() && change_id.is_none() {
        return fail("--change is required for --add-context");
    }
    if status && change_id.is_none() && module_id.is_none() {
        return fail("--change is required for --status, or provide --module to auto-select");
    }

    let positional_prompt = collect_prompt(args);
    let prompt = if let Some(path) = prompt_file {
        let path_buf = std::path::PathBuf::from(&path);
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

    let ito_path = rt.ito_path();

    let mut harness_impl: Box<dyn Harness> = match harness.as_str() {
        "opencode" => Box::new(OpencodeHarness),
        "stub" => {
            let mut p = stub_script.map(std::path::PathBuf::from);
            if p.is_none() {
                // Prefer env var in CI, but allow missing.
                p = None;
            }
            match StubHarness::from_env_or_default(p) {
                Ok(h) => Box::new(h),
                Err(e) => return Err(to_cli_error(e)),
            }
        }
        _ => return fail(format!("Unknown harness: {h}", h = harness)),
    };

    let opts = core_ralph::RalphOptions {
        prompt,
        change_id,
        module_id,
        model,
        min_iterations,
        max_iterations,
        completion_promise,
        allow_all,
        no_commit,
        interactive,
        status,
        add_context,
        clear_context,
        verbose,
        continue_module,
        continue_ready,
        inactivity_timeout,
        skip_validation,
        validation_command,
        exit_on_error,
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

pub(crate) fn handle_ralph_clap(rt: &Runtime, args: &RalphArgs) -> CliResult<()> {
    let argv = ralph_args_to_argv(args);
    handle_ralph(rt, &argv)
}

pub(crate) fn handle_loop_clap(rt: &Runtime, args: &RalphArgs) -> CliResult<()> {
    let argv = ralph_args_to_argv(args);
    handle_loop(rt, &argv)
}

fn ralph_args_to_argv(args: &RalphArgs) -> Vec<String> {
    let mut argv: Vec<String> = Vec::new();
    if let Some(change) = &args.change {
        argv.push("--change".to_string());
        argv.push(change.clone());
    }
    if let Some(module) = &args.module {
        argv.push("--module".to_string());
        argv.push(module.clone());
    }
    if args.continue_module {
        argv.push("--continue-module".to_string());
    }
    if args.continue_ready {
        argv.push("--continue-ready".to_string());
    }
    argv.push("--harness".to_string());
    argv.push(args.harness.clone());
    if let Some(model) = &args.model {
        argv.push("--model".to_string());
        argv.push(model.clone());
    }
    argv.push("--min-iterations".to_string());
    argv.push(args.min_iterations.to_string());
    if let Some(max) = args.max_iterations {
        argv.push("--max-iterations".to_string());
        argv.push(max.to_string());
    }
    argv.push("--completion-promise".to_string());
    argv.push(args.completion_promise.clone());

    if args.skip_validation {
        argv.push("--skip-validation".to_string());
    }
    if args.exit_on_error {
        argv.push("--exit-on-error".to_string());
    }
    if let Some(threshold) = args.error_threshold {
        argv.push("--error-threshold".to_string());
        argv.push(threshold.to_string());
    }
    if let Some(cmd) = &args.validation_command {
        argv.push("--validation-command".to_string());
        argv.push(cmd.clone());
    }
    if args.allow_all {
        argv.push("--allow-all".to_string());
    }
    if args.no_commit {
        argv.push("--no-commit".to_string());
    }
    if args.status {
        argv.push("--status".to_string());
    }
    if let Some(add_context) = &args.add_context {
        argv.push("--add-context".to_string());
        argv.push(add_context.clone());
    }
    if args.clear_context {
        argv.push("--clear-context".to_string());
    }
    if args.no_interactive {
        argv.push("--no-interactive".to_string());
    }
    if args.verbose {
        argv.push("--verbose".to_string());
    }
    if let Some(stub_script) = &args.stub_script {
        argv.push("--stub-script".to_string());
        argv.push(stub_script.clone());
    }
    if let Some(timeout) = &args.timeout {
        argv.push("--timeout".to_string());
        argv.push(timeout.clone());
    }
    if let Some(file) = &args.file {
        argv.push("--file".to_string());
        argv.push(file.clone());
    }
    if !args.prompt.is_empty() {
        argv.extend(args.prompt.iter().cloned());
    }
    argv
}
