use crate::cli::{Cli, Commands};
use crate::cli_error::{fail, CliResult};
use crate::runtime::Runtime;
use crate::{commands, util};
use clap::error::ErrorKind;
use clap::Parser;

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
    argv.extend(args.iter().cloned());

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
                return fail(e.to_string());
            }
        },
    };

    if cli.help_all {
        return commands::handle_help_all_flags(false);
    }

    let rt = Runtime::new();

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
        Some(Commands::Plan(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_plan_clap(&rt, args),
            );
        }
        Some(Commands::State(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_state_clap(&rt, args),
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
        Some(Commands::Workflow(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_workflow_clap(&rt, args),
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

        Some(Commands::Serve(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || commands::handle_serve_clap(&rt, args),
            );
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
        Some(Commands::Ralph(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || super::ralph::handle_ralph_clap(&rt, args),
            );
        }
        Some(Commands::Loop(args)) => {
            return util::with_logging(
                &rt,
                &command_id,
                &project_root,
                &ito_path_for_logging,
                || super::ralph::handle_loop_clap(&rt, args),
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
