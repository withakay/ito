use super::{Cli, Commands, WorktreeCommand};
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
