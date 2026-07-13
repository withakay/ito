use crate::cli::{Cli, Commands};
use crate::cli_error::{CliError, CliResult, fail};
use crate::runtime::Runtime;
use crate::{commands, util};
use clap::Parser;
use clap::error::ErrorKind;
use ito_config::ConfigContext;
use ito_core::capabilities::CapabilityPreflight;

/// Parse CLI arguments, initialize the runtime and logging context, and dispatch the selected subcommand.
///
/// Handles global behavior (such as recognizing a global `--no-color` flag and producing help/version
/// output), constructs an argv suitable for clap, initializes `Runtime` and logging identifiers,
/// and routes execution to the chosen subcommand handler. If no command is provided or a command
/// is not implemented, prints the long help fallback.
///
/// # Examples
///
/// ```
/// let args = vec!["--version".to_string()];
/// let _ = ito_cli::run(&args);
/// ```
pub(super) fn run(args: &[String]) -> CliResult<()> {
    // Match TS behavior: `--no-color` sets NO_COLOR=1 globally before command execution.
    if args.iter().any(|a| a == "--no-color") {
        // Rust 1.93+ marks `set_var` unsafe due to potential UB when racing with
        // other threads reading the environment. We do this before any command
        // execution or thread spawning.
        unsafe {
            std::env::set_var("NO_COLOR", "1");
        }
    }

    let mut argv: Vec<String> = Vec::with_capacity(args.len() + 1);
    argv.push("ito".to_string());
    for a in args {
        // Unify help output: clap prints a short help for `-h` by default.
        // Rewrite `-h` to `--help` so `-h` and `--help` produce identical output.
        if a == "-h" {
            argv.push("--help".to_string());
        } else {
            argv.push(a.clone());
        }
    }

    let cli = match Cli::try_parse_from(argv) {
        Ok(v) => v,
        Err(e) => match e.kind() {
            ErrorKind::DisplayHelp => {
                print!("{e}");
                return Ok(());
            }
            ErrorKind::DisplayVersion => {
                // Match Commander.js behavior: `ito --version` prints the version only.
                let v = option_env!("ITO_WORKSPACE_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));
                // For debug builds, show git info for easier debugging
                #[cfg(debug_assertions)]
                {
                    let sha = option_env!("VERGEN_GIT_SHA").unwrap_or("unknown");
                    let dirty = option_env!("VERGEN_GIT_DIRTY")
                        .map(|d| if d == "true" { "-dirty" } else { "" })
                        .unwrap_or("");
                    println!("{v} ({}{dirty})", &sha[..7.min(sha.len())]);
                }
                #[cfg(not(debug_assertions))]
                println!("{v}");
                return Ok(());
            }
            _ => {
                let ctx = ConfigContext::from_process_env();
                util::maybe_log_invalid_command_early(&ctx, args, &e.to_string());
                return fail(e.to_string());
            }
        },
    };

    if cli.help_all {
        return commands::handle_help_all_flags(false);
    }

    let rt = Runtime::new();

    let preflight_mode = if is_recovery_safe_invocation(args) {
        CapabilityPreflight::Recovery
    } else {
        CapabilityPreflight::Stateful
    };
    if let Err(error) = rt.preflight(preflight_mode) {
        let error = CliError::from_core(error);
        if args.iter().any(|arg| arg == "--json")
            && let Some(value) = error.feature_unavailable_json()
        {
            println!(
                "{}",
                serde_json::to_string_pretty(&value).expect("JSON value serializes")
            );
            return Err(CliError::silent());
        }
        return Err(error);
    }

    let command_id = util::command_id_from_args(args);
    let project_root = util::project_root_for_logging(&rt, args);
    let ito_path_for_logging = util::ito_path_for_logging(&project_root, &rt);

    match &cli.command {
        Some(Commands::Help(args)) => {
            return commands::handle_help_clap(args);
        }
        Some(Commands::Completions(args)) => {
            return commands::handle_completions(args.shell);
        }
        Some(Commands::Create(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_create_clap(&rt, args),
            );
        }
        Some(Commands::New(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_new_clap(&rt, args),
            );
        }
        Some(Commands::Init(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || super::init::handle_init_clap(&rt, args),
            );
        }
        Some(Commands::Update(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || super::update::handle_update_clap(&rt, args),
            );
        }
        Some(Commands::List(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || super::list::handle_list_clap(&rt, args),
            );
        }
        Some(Commands::ListArchive(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || super::list::handle_list_archive(&rt, args.json),
            );
        }
        Some(Commands::Plan(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_plan_clap(&rt, args),
            );
        }
        Some(Commands::Grep(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || super::grep::handle_grep_clap(&rt, args),
            );
        }
        Some(Commands::Tasks(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_tasks_clap(&rt, args),
            );
        }
        Some(Commands::Patch(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_patch_clap(&rt, args),
            );
        }
        Some(Commands::Write(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_write_clap(&rt, args),
            );
        }
        Some(Commands::Templates(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_templates_clap(&rt, args),
            );
        }
        Some(Commands::Status(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || super::status::handle_status_clap(&rt, args),
            );
        }
        Some(Commands::Stats(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_stats_clap(&rt, args),
            );
        }
        Some(Commands::Config(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_config_clap(&rt, args),
            );
        }
        Some(Commands::Path(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_path_clap(&rt, args),
            );
        }
        Some(Commands::Worktree(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_worktree_clap(&rt, args),
            );
        }
        Some(Commands::View(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_view_clap(&rt, args),
            );
        }

        #[cfg(feature = "web")]
        Some(Commands::Serve(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_serve_clap(&rt, args),
            );
        }

        #[cfg(feature = "backend")]
        Some(Commands::Backend(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_backend_clap(&rt, args),
            );
        }

        #[cfg(not(feature = "backend"))]
        Some(Commands::Backend(args)) => {
            return unavailable_backend_command(args);
        }

        #[cfg(feature = "backend")]
        Some(Commands::ServeApiRemoved(args)) => {
            return fail(format!(
                "The top-level `ito serve-api` command has been removed.\nUse `{}` instead.",
                removed_serve_api_replacement(args)
            ));
        }

        #[cfg(not(feature = "backend"))]
        Some(Commands::ServeApiRemoved(_)) => {
            return Err(CliError::feature_unavailable(
                "backend",
                "ito serve-api",
                "use `ito backend serve` from an experimental build with the backend feature",
            ));
        }

        Some(Commands::Agent(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || super::instructions::handle_agent_clap(&rt, args),
            );
        }
        Some(Commands::Show(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || super::show::handle_show_clap(&rt, args),
            );
        }
        Some(Commands::Validate(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || super::validate::handle_validate_clap(&rt, args),
            );
        }
        Some(Commands::Ralph(ralph_args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_ralph_clap(&rt, ralph_args, args),
            );
        }
        Some(Commands::Loop(ralph_args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_loop_clap(&rt, ralph_args, args),
            );
        }
        Some(Commands::Util(args)) => {
            return commands::handle_util_clap(args);
        }
        Some(Commands::Trace(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || super::trace::handle_trace_clap(&rt, args),
            );
        }
        Some(Commands::Archive(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || super::archive::handle_archive_clap(&rt, args),
            );
        }
        Some(Commands::Sync(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_sync_clap(&rt, args),
            );
        }
        Some(Commands::Audit(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_audit_clap(&rt, args),
            );
        }
        Some(Commands::Dashboard(_)) => {
            return fail("dashboard is not implemented in ito-cli yet");
        }
        Some(Commands::Split(_)) => {
            return fail("split is not implemented in ito-cli yet");
        }
        None => {}
    }

    util::with_logging(
        &rt,
        &command_id,
        &project_root,
        &ito_path_for_logging,
        || {
            // Temporary fallback for unimplemented commands.
            println!("{}", super::common::render_command_long_help(&[], "ito"));
            Ok(())
        },
    )
}

fn is_recovery_safe_invocation(args: &[String]) -> bool {
    let positional = args
        .iter()
        .filter(|arg| !arg.starts_with('-'))
        .map(String::as_str)
        .collect::<Vec<_>>();
    match positional.as_slice() {
        ["help", ..]
        | ["completions", ..]
        | ["config", ..]
        | ["init", ..]
        | ["update", ..]
        | ["backend", ..]
        | ["serve-api", ..]
        | ["sync", ..] => true,
        ["tasks", operation, ..]
            if matches!(*operation, "claim" | "release" | "allocate" | "sync") =>
        {
            true
        }
        ["agent", "instruction", "migrate-to-main", ..] => true,
        _ => false,
    }
}

#[cfg(not(feature = "backend"))]
fn unavailable_backend_command(args: &crate::cli::BackendArgs) -> CliResult<()> {
    let error = CliError::feature_unavailable(
        "backend",
        "ito backend",
        "install an experimental build with the backend feature, or disable backend.enabled",
    );
    let wants_json = matches!(
        args.action,
        crate::cli::BackendAction::Status { json: true }
    );
    if wants_json {
        let value = error
            .feature_unavailable_json()
            .expect("feature-unavailable errors have JSON details");
        println!(
            "{}",
            serde_json::to_string_pretty(&value).expect("JSON value serializes")
        );
        return Err(CliError::silent());
    }
    Err(error)
}

#[cfg(feature = "backend")]
fn removed_serve_api_replacement(args: &crate::cli::RemovedServeApiArgs) -> String {
    let mut replacement = vec![
        "ito".to_string(),
        "backend".to_string(),
        "serve".to_string(),
    ];
    replacement.extend(args.args.iter().cloned());
    replacement.join(" ")
}

#[cfg(all(test, feature = "backend"))]
#[path = "run_tests.rs"]
mod run_tests;
