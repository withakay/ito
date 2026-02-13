mod support;

use ito_test_support::run_rust_candidate;
use support::{make_repo_all_valid, reset_repo};

/// Verifies that top-level CLI aliases resolve to their intended commands and that each alias
/// produces the expected help text.
///
/// The test exercises the `ito` binary using common aliases (`ls`, `cr`, `st`, `sh`, `va`)
/// and asserts that each alias exits successfully and that the help output contains the
/// appropriate command description.
///
/// # Examples
///
/// ```
/// // Locate the test binary and run the alias help check.
/// let bin = assert_cmd::cargo::cargo_bin!("ito");
/// let out = run_rust_candidate(bin, &["ls", "--help"], repo.path(), home.path());
/// assert_eq!(out.code, 0);
/// assert!(out.stdout.contains("List changes"));
/// ```
#[test]
fn main_command_aliases_work() {
    let base = make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Test main command aliases with --help to verify resolution
    // ls -> list
    let out = run_rust_candidate(rust_path, &["ls", "--help"], repo.path(), home.path());
    assert_eq!(out.code, 0, "ls alias should work");
    assert!(
        out.stdout.contains("List changes"),
        "ls should resolve to list command"
    );

    // cr -> create
    let out = run_rust_candidate(rust_path, &["cr", "--help"], repo.path(), home.path());
    assert_eq!(out.code, 0, "cr alias should work");
    assert!(
        out.stdout.contains("Create a new module"),
        "cr should resolve to create command"
    );

    // st -> status
    let out = run_rust_candidate(rust_path, &["st", "--help"], repo.path(), home.path());
    assert_eq!(out.code, 0, "st alias should work");
    assert!(
        out.stdout.contains("Check completion status"),
        "st should resolve to status command"
    );

    // sh -> show
    let out = run_rust_candidate(rust_path, &["sh", "--help"], repo.path(), home.path());
    assert_eq!(out.code, 0, "sh alias should work");
    assert!(
        out.stdout.contains("Display details"),
        "sh should resolve to show command"
    );

    // va -> validate
    let out = run_rust_candidate(rust_path, &["va", "--help"], repo.path(), home.path());
    assert_eq!(out.code, 0, "va alias should work");
    assert!(
        out.stdout.contains("Check changes"),
        "va should resolve to validate command"
    );
}

/// Verifies that top-level command aliases execute and produce the expected JSON output.
///
/// Tests two alias executions:
/// - `ls --json` must succeed and return JSON containing a top-level `changes` field.
/// - `st --change <name> --json` must succeed and return JSON whose `changeName` equals the requested change.
///
/// # Examples
///
/// ```
/// // Example usage (test helpers assumed to be available in this crate):
/// let base = make_repo_all_valid();
/// let repo = tempfile::tempdir().unwrap();
/// let home = tempfile::tempdir().unwrap();
/// let bin = assert_cmd::cargo::cargo_bin!("ito");
/// reset_repo(repo.path(), base.path());
///
/// let out = run_rust_candidate(bin, &["ls", "--json"], repo.path(), home.path());
/// assert_eq!(out.code, 0);
///
/// let out = run_rust_candidate(
///     bin,
///     &["st", "--change", "000-01_test-change", "--json"],
///     repo.path(),
///     home.path(),
/// );
/// assert_eq!(out.code, 0);
/// ```
#[test]
fn main_command_aliases_execute() {
    let base = make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Test actual command execution with aliases
    // ls -> list
    let out = run_rust_candidate(rust_path, &["ls", "--json"], repo.path(), home.path());
    assert_eq!(out.code, 0, "ls --json should execute successfully");
    let v: serde_json::Value =
        serde_json::from_str(&out.stdout).expect("ls should return valid json");
    assert!(v.get("changes").is_some(), "ls should list changes");

    // st -> status
    let out = run_rust_candidate(
        rust_path,
        &["st", "--change", "000-01_test-change", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "st should execute successfully");
    let v: serde_json::Value =
        serde_json::from_str(&out.stdout).expect("st should return valid json");
    assert_eq!(
        v.get("changeName").and_then(|v| v.as_str()),
        Some("000-01_test-change"),
        "st should show status for change"
    );
}

/// Verifies that the `cr` subcommand aliases resolve to the intended `create` subcommands and show correct help text.
///
/// # Examples
///
/// ```
/// // This demonstrates the test invocation; running the test harness executes the assertions.
/// subcommand_aliases_work();
/// ```
#[test]
fn subcommand_aliases_work() {
    let base = make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Test create subcommand aliases
    // cr mo -> create module
    let out = run_rust_candidate(rust_path, &["cr", "mo", "--help"], repo.path(), home.path());
    assert_eq!(out.code, 0, "create module alias should work");
    assert!(
        out.stdout.contains("Create a module"),
        "mo should resolve to module"
    );

    // cr ch -> create change
    let out = run_rust_candidate(rust_path, &["cr", "ch", "--help"], repo.path(), home.path());
    assert_eq!(out.code, 0, "create change alias should work");
    assert!(
        out.stdout.contains("Create a change"),
        "ch should resolve to change"
    );
}

#[test]
fn short_flags_work() {
    let base = make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Test -c short flag for --change
    let out = run_rust_candidate(
        rust_path,
        &["status", "-c", "000-01_test-change", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "-c flag should work for status");
    let v: serde_json::Value =
        serde_json::from_str(&out.stdout).expect("status should return valid json");
    assert_eq!(
        v.get("changeName").and_then(|v| v.as_str()),
        Some("000-01_test-change"),
        "-c should work as alias for --change"
    );

    // Test combining alias and short flag
    let out = run_rust_candidate(
        rust_path,
        &["st", "-c", "000-01_test-change", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "st -c combination should work");
    let v: serde_json::Value =
        serde_json::from_str(&out.stdout).expect("status should return valid json");
    assert_eq!(
        v.get("changeName").and_then(|v| v.as_str()),
        Some("000-01_test-change"),
        "combining alias and short flag should work"
    );
}