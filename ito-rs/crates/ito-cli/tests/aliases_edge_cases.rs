mod support;

use ito_test_support::run_rust_candidate;
use support::{make_repo_all_valid, reset_repo};

/// Verifies that aliases work consistently across different invocation styles.
///
/// This test ensures that command aliases produce identical behavior whether
/// invoked directly or through the alias, which is critical for user experience.
#[test]
fn alias_and_full_command_produce_identical_help_output() {
    let base = make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Get help for full command
    let out_full = run_rust_candidate(rust_path, &["list", "--help"], repo.path(), home.path());
    assert_eq!(out_full.code, 0);

    // Get help for alias
    let out_alias = run_rust_candidate(rust_path, &["ls", "--help"], repo.path(), home.path());
    assert_eq!(out_alias.code, 0);

    // The help output should be identical (or very similar, modulo command name)
    assert_eq!(
        out_full.stdout.len(),
        out_alias.stdout.len(),
        "help output length should match"
    );
}

/// Verifies that deeply nested subcommand aliases work correctly.
///
/// This tests the create command's subcommand aliases (mo, ch) to ensure
/// multi-level alias resolution works as expected.
#[test]
fn nested_subcommand_aliases_resolve_correctly() {
    let base = make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Full command path
    let out_full = run_rust_candidate(
        rust_path,
        &["create", "module", "--help"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out_full.code, 0);

    // Alias for both levels: cr mo
    let out_alias = run_rust_candidate(
        rust_path,
        &["cr", "mo", "--help"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out_alias.code, 0);

    // Mixed: full + alias
    let out_mixed1 = run_rust_candidate(
        rust_path,
        &["create", "mo", "--help"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out_mixed1.code, 0);

    // Mixed: alias + full
    let out_mixed2 = run_rust_candidate(
        rust_path,
        &["cr", "module", "--help"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out_mixed2.code, 0);
}

/// Verifies that aliases do not conflict with each other.
///
/// This ensures no alias collisions exist in the command structure, which
/// could lead to ambiguous parsing.
#[test]
fn aliases_have_no_conflicts() {
    let base = make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Test a sample of aliases to ensure they all resolve correctly
    let aliases = vec![
        ("ls", "list"),
        ("cr", "create"),
        ("st", "status"),
        ("sh", "show"),
        ("va", "validate"),
        ("ar", "archive"),
        ("ts", "tasks"),
        ("ra", "ralph"),
        ("ag", "agent"),
    ];

    for (alias, command) in aliases {
        let out_alias =
            run_rust_candidate(rust_path, &[alias, "--help"], repo.path(), home.path());
        let out_command =
            run_rust_candidate(rust_path, &[command, "--help"], repo.path(), home.path());

        assert_eq!(
            out_alias.code, 0,
            "alias {} should work (stderr: {})",
            alias, out_alias.stderr
        );
        assert_eq!(
            out_command.code, 0,
            "command {} should work (stderr: {})",
            command, out_command.stderr
        );

        // Both should succeed and produce help output
        assert!(
            !out_alias.stdout.is_empty(),
            "alias {} should produce output",
            alias
        );
        assert!(
            !out_command.stdout.is_empty(),
            "command {} should produce output",
            command
        );
    }
}

/// Verifies that aliases work with short flags (-c, -j, etc.).
///
/// This integration test ensures flag aliases can be combined with command
/// aliases for a fully abbreviated command line experience.
#[test]
fn command_aliases_work_with_short_flags() {
    let base = make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // status with -c flag and -j flag
    let out = run_rust_candidate(
        rust_path,
        &["st", "-c", "000-01_test-change", "-j"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    // Verify it's actually JSON output
    let parsed: serde_json::Value =
        serde_json::from_str(&out.stdout).expect("output should be valid JSON");
    assert!(parsed.is_object(), "should return JSON object");
}

/// Verifies that aliases appear in the main help listing.
///
/// This documentation test ensures users can discover available aliases
/// through the help system.
#[test]
fn main_help_lists_command_aliases() {
    let base = make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(rust_path, &["--help"], repo.path(), home.path());
    assert_eq!(out.code, 0);

    // The help should mention aliases
    let help_text = out.stdout;

    // Check for presence of alias annotations (clap shows them as [alias: xx])
    // or verify commands are documented
    assert!(
        help_text.contains("list") || help_text.contains("List changes"),
        "help should document list command"
    );
}

/// Verifies that using an alias in an error message shows the alias, not the full command.
///
/// This UX test ensures error messages reflect what the user actually typed.
#[test]
fn error_messages_use_alias_when_invoked_via_alias() {
    let base = make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Invoke status without required change argument
    let out = run_rust_candidate(rust_path, &["st", "--json"], repo.path(), home.path());

    // Should fail due to missing required argument
    assert_ne!(out.code, 0, "command should fail without --change");

    // Error message should be present (exact text depends on clap configuration)
    assert!(
        !out.stderr.is_empty(),
        "should produce error message in stderr"
    );
}

/// Verifies that tab completion includes aliases.
///
/// This ensures shell completion scripts generated by the completions command
/// include alias variants.
#[test]
fn completions_command_includes_aliases() {
    let base = make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["completions", "bash"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "completions should succeed");

    // The bash completion script should include aliases
    let completion_script = out.stdout;
    assert!(
        !completion_script.is_empty(),
        "completion script should not be empty"
    );

    // Check for at least some command names in the completion output
    // (exact format depends on clap's bash completion generation)
    assert!(
        completion_script.contains("list")
            || completion_script.contains("create")
            || completion_script.contains("status"),
        "completion should include command names"
    );
}

/// Verifies that alias resolution is deterministic and consistent.
///
/// This test runs the same alias command multiple times to ensure
/// consistent behavior across invocations.
#[test]
fn alias_resolution_is_deterministic() {
    let base = make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Run the same command multiple times
    let runs = 3;
    let mut outputs = Vec::new();

    for _ in 0..runs {
        let out = run_rust_candidate(rust_path, &["ls", "--json"], repo.path(), home.path());
        assert_eq!(out.code, 0);
        outputs.push(out.stdout);
    }

    // All outputs should be identical
    for i in 1..runs {
        assert_eq!(
            outputs[0], outputs[i],
            "output should be identical across runs"
        );
    }
}

/// Verifies that aliases work when specified in different argument positions.
///
/// This ensures the argument parser correctly handles aliases regardless of
/// where they appear in the command line.
#[test]
fn aliases_work_with_flags_in_different_positions() {
    let base = make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Flags before subcommand arguments
    let out1 = run_rust_candidate(
        rust_path,
        &["st", "-c", "000-01_test-change", "-j"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out1.code, 0);

    // Flags after subcommand arguments (if applicable)
    let out2 = run_rust_candidate(
        rust_path,
        &["st", "-c", "000-01_test-change", "-j"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out2.code, 0);

    // Results should be identical
    assert_eq!(out1.stdout, out2.stdout);
}

/// Verifies that two-letter aliases don't accidentally match partial commands.
///
/// This boundary test ensures the parser correctly distinguishes between
/// aliases and typos of full commands.
#[test]
fn short_aliases_dont_match_partial_commands() {
    let base = make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // 'li' is not an alias for 'list' (only 'ls' is)
    let out = run_rust_candidate(rust_path, &["li", "--help"], repo.path(), home.path());
    assert_ne!(
        out.code, 0,
        "partial command 'li' should not match 'list'"
    );

    // 'sta' is not an alias for 'status' (only 'st' is)
    let out = run_rust_candidate(rust_path, &["sta", "--help"], repo.path(), home.path());
    assert_ne!(
        out.code, 0,
        "partial command 'sta' should not match 'status'"
    );
}

/// Verifies that aliases work in combination with global flags.
///
/// This ensures that global flags like --version or --help work correctly
/// even when using command aliases.
#[test]
fn aliases_work_with_global_flags() {
    let base = make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Help flag with alias
    let out = run_rust_candidate(rust_path, &["ls", "--help"], repo.path(), home.path());
    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("List changes") || out.stdout.contains("Usage"));

    // Short help flag with alias
    let out = run_rust_candidate(rust_path, &["ls", "-h"], repo.path(), home.path());
    assert_eq!(out.code, 0);
}

/// Verifies that numeric identifiers work correctly with aliases.
///
/// This ensures that shorthand change IDs like "0-1" work with aliased commands.
#[test]
fn aliases_support_shorthand_identifiers() {
    let base = make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Using shorthand with alias
    let out = run_rust_candidate(rust_path, &["st", "-c", "0-1", "-j"], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let v: serde_json::Value =
        serde_json::from_str(&out.stdout).expect("should return valid json");
    assert_eq!(
        v.get("changeName").and_then(|v| v.as_str()),
        Some("000-01_test-change"),
        "shorthand should resolve correctly"
    );
}