use clap::Parser;

use super::{CommandIntent, command_intent};
use crate::cli::Cli;

fn parse(args: &[&str]) -> crate::cli::Commands {
    Cli::try_parse_from(args)
        .unwrap_or_else(|error| panic!("failed to parse {args:?}: {error}"))
        .command
        .expect("parsed command")
}

#[test]
fn command_intent_keeps_diagnostic_and_artifact_reads_read_only() {
    for args in [
        &["ito", "list"][..],
        &["ito", "list-archive"][..],
        &["ito", "show", "031-01_example"][..],
        &[
            "ito",
            "change",
            "preflight",
            "031-01_example",
            "--for",
            "execute",
        ][..],
        &["ito", "validate", "031-01_example"][..],
        &["ito", "tasks", "status", "031-01_example"][..],
        &["ito", "tasks", "show", "031-01_example"][..],
        &["ito", "plan", "status"][..],
        &[
            "ito",
            "agent",
            "instruction",
            "apply",
            "--change",
            "031-01_example",
        ][..],
        &["ito", "config", "get", "defaults.schema"][..],
        &["ito", "path", "project-root"][..],
        &["ito", "worktree", "validate", "--change", "031-01_example"][..],
        &["ito", "audit", "log"][..],
        &["ito", "audit", "reconcile"][..],
        &["ito", "backend", "serve"][..],
        &["ito", "util", "parse-id", "031-01_example"][..],
        &["ito", "help"][..],
    ] {
        assert_eq!(
            command_intent(&parse(args)),
            CommandIntent::ReadOnly,
            "expected read-only intent for {args:?}"
        );
    }
}

#[test]
fn command_intent_marks_migrate_to_main_instruction_as_recovery() {
    let command = parse(&["ito", "agent", "instruction", "migrate-to-main"]);
    assert_eq!(command_intent(&command), CommandIntent::Recovery);
}

#[test]
fn command_intent_marks_nested_writes_and_instruction_sync_as_mutating() {
    for args in [
        &["ito", "tasks", "start", "031-01_example", "1.1"][..],
        &["ito", "plan", "init"][..],
        &[
            "ito",
            "agent",
            "instruction",
            "apply",
            "--change",
            "031-01_example",
            "--sync",
        ][..],
        &["ito", "config", "set", "defaults.schema", "minimalist"][..],
        &["ito", "worktree", "ensure", "--change", "031-01_example"][..],
        &["ito", "audit", "reconcile", "--fix"][..],
        &[
            "ito",
            "change",
            "preflight",
            "031-01_example",
            "--for",
            "execute",
            "--refresh",
        ][..],
    ] {
        assert_eq!(
            command_intent(&parse(args)),
            CommandIntent::Mutating,
            "expected mutating intent for {args:?}"
        );
    }
}

#[test]
fn command_intent_fails_closed_for_unknown_external_operations() {
    let command = parse(&["ito", "config", "future-operation"]);
    assert_eq!(command_intent(&command), CommandIntent::Mutating);
}
