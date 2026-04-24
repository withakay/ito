use super::{Cli, Commands};
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
