use super::{ChangeCommand, Cli, Commands, ReadinessPhaseArg, WorktreeCommand};
#[cfg(any(not(feature = "backend"), not(feature = "coordination-branch")))]
use clap::CommandFactory;
use clap::Parser;

#[test]
fn parses_top_level_sync_command() {
    let cli = Cli::parse_from(["ito", "sync"]);

    let Some(Commands::Sync(args)) = cli.command else {
        panic!("expected top-level sync command");
    };

    assert!(!args.force);
    assert!(!args.json);
}

#[test]
fn parses_top_level_sync_force_flag() {
    let cli = Cli::parse_from(["ito", "sync", "--force", "--json"]);

    let Some(Commands::Sync(args)) = cli.command else {
        panic!("expected top-level sync command");
    };

    assert!(args.force);
    assert!(args.json);
}

#[test]
fn parses_worktree_validate_with_json_flag() {
    let cli = Cli::parse_from([
        "ito",
        "worktree",
        "validate",
        "--change",
        "012-07_guard-opencode-worktree-path",
        "--json",
    ]);

    let Some(Commands::Worktree(args)) = cli.command else {
        panic!("expected worktree command");
    };

    let WorktreeCommand::Validate(validate_args) = args.command else {
        panic!("expected worktree validate subcommand");
    };

    assert_eq!(
        validate_args.change_args.change,
        "012-07_guard-opencode-worktree-path"
    );
    assert!(validate_args.json);
}

#[test]
fn parses_change_preflight_prepare_defaults() {
    let cli = Cli::parse_from([
        "ito",
        "change",
        "preflight",
        "031-02_enforce-main-first-implementation",
        "--for",
        "prepare",
    ]);

    let Some(Commands::Change(args)) = cli.command else {
        panic!("expected change command");
    };
    let ChangeCommand::Preflight(args) = args.command;
    assert_eq!(args.change_id, "031-02_enforce-main-first-implementation");
    assert_eq!(args.phase, ReadinessPhaseArg::Prepare);
    assert!(!args.refresh);
    assert!(!args.json);
}

#[test]
fn parses_change_preflight_execute_refresh_json() {
    let cli = Cli::parse_from([
        "ito",
        "change",
        "preflight",
        "031-02_enforce-main-first-implementation",
        "--for=execute",
        "--refresh",
        "--json",
    ]);

    let Some(Commands::Change(args)) = cli.command else {
        panic!("expected change command");
    };
    let ChangeCommand::Preflight(args) = args.command;
    assert_eq!(args.phase, ReadinessPhaseArg::Execute);
    assert!(args.refresh);
    assert!(args.json);
}

#[cfg(not(feature = "backend"))]
#[test]
fn default_build_parses_backend_compatibility_command() {
    let cli = Cli::parse_from(["ito", "backend", "status", "--json"]);
    let Some(Commands::Backend(_)) = cli.command else {
        panic!("expected hidden backend compatibility command");
    };
}

#[cfg(not(feature = "backend"))]
#[test]
fn default_help_hides_backend_command() {
    let help = Cli::command().render_long_help().to_string();
    assert!(!help.contains("ito backend status"));
}

#[cfg(not(feature = "coordination-branch"))]
#[test]
fn default_help_hides_coordination_sync_command() {
    let help = Cli::command().render_long_help().to_string();
    assert!(!help.contains("ito sync --force"));
}
