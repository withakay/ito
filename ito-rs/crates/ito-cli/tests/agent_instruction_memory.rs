//! Integration tests for the three memory-* agent instruction artifacts.
//!
//! Covers all nine cells of the {operation: capture, search, query} ×
//! {branch: command, skill, not-configured} matrix. Each test sets up a
//! minimal Ito project root, writes a `.ito/config.json` with the
//! configuration under test, and asserts on the rendered output.

#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;
use serde_json::json;

fn setup_project_with_memory(memory: serde_json::Value) -> (tempfile::TempDir, tempfile::TempDir) {
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");

    std::fs::write(repo.path().join("README.md"), "temp\n").expect("write");
    fixtures::git_init_with_initial_commit(repo.path());

    let cfg = json!({ "memory": memory });
    fixtures::write(
        repo.path().join(".ito/config.json"),
        &(serde_json::to_string_pretty(&cfg).unwrap() + "\n"),
    );
    (repo, home)
}

fn setup_project_without_memory() -> (tempfile::TempDir, tempfile::TempDir) {
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");

    std::fs::write(repo.path().join("README.md"), "temp\n").expect("write");
    fixtures::git_init_with_initial_commit(repo.path());
    fixtures::write(repo.path().join(".ito/config.json"), "{}\n");

    (repo, home)
}

// ---- memory-capture ---------------------------------------------------------

#[test]
fn memory_capture_command_branch_renders_executable_command_line() {
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let (repo, home) = setup_project_with_memory(json!({
        "capture": {
            "kind": "command",
            "command": "brv curate {context} {files} {folders}"
        }
    }));

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "memory-capture",
            "--context",
            "decision X",
            "--file",
            "a.md",
            "--file",
            "b.md",
            "--folder",
            "docs/",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Capture this memory"), "stdout={}", out.stdout);
    assert!(
        out.stdout
            .contains("brv curate 'decision X' --file 'a.md' --file 'b.md' --folder 'docs/'"),
        "stdout={}",
        out.stdout
    );
}

#[test]
fn memory_capture_skill_branch_emits_structured_inputs() {
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let (repo, home) = setup_project_with_memory(json!({
        "capture": {
            "kind": "skill",
            "skill": "ito-memory-markdown",
            "options": { "root": ".ito/memories" }
        }
    }));

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "memory-capture",
            "--context",
            "note",
            "--file",
            "x.md",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Skill: `ito-memory-markdown`"));
    assert!(out.stdout.contains("`context` = `\"note\"`"));
    assert!(out.stdout.contains("`files` = `[\"x.md\"]`"));
    assert!(out.stdout.contains("\"root\""));
}

#[test]
fn memory_capture_not_configured_branch_renders_setup_guidance() {
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let (repo, home) = setup_project_without_memory();

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "memory-capture", "--context", "x"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Memory `capture` is not configured"));
    assert!(out.stdout.contains("\"kind\": \"command\""));
    assert!(out.stdout.contains("\"kind\": \"skill\""));
    assert!(out.stdout.contains("There is no default provider"));
}

// ---- memory-search ----------------------------------------------------------

#[test]
fn memory_search_command_branch_substitutes_query_and_default_limit() {
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let (repo, home) = setup_project_with_memory(json!({
        "search": {
            "kind": "command",
            "command": "brv search {query} --limit {limit}"
        }
    }));

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "memory-search",
            "--query",
            "coordination",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("brv search 'coordination' --limit 10"));
}

#[test]
fn memory_search_command_branch_overrides_limit_when_supplied() {
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let (repo, home) = setup_project_with_memory(json!({
        "search": {
            "kind": "command",
            "command": "brv search {query} --limit {limit}"
        }
    }));

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "memory-search",
            "--query",
            "x",
            "--limit",
            "3",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("brv search 'x' --limit 3"));
}

#[test]
fn memory_search_skill_branch_emits_structured_inputs() {
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let (repo, home) = setup_project_with_memory(json!({
        "search": {
            "kind": "skill",
            "skill": "byterover-explore"
        }
    }));

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "memory-search",
            "--query",
            "auth",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Skill: `byterover-explore`"));
    assert!(out.stdout.contains("`query` = `\"auth\"`"));
    assert!(out.stdout.contains("`limit` = `10`"));
}

#[test]
fn memory_search_not_configured_branch_renders_setup_guidance() {
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let (repo, home) = setup_project_without_memory();

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "memory-search",
            "--query",
            "x",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Memory `search` is not configured"));
}

#[test]
fn memory_search_requires_query_flag() {
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let (repo, home) = setup_project_without_memory();

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "memory-search"],
        repo.path(),
        home.path(),
    );

    assert_ne!(out.code, 0, "expected failure, stdout={}", out.stdout);
    let combined = format!("{}{}", out.stdout, out.stderr);
    assert!(
        combined.contains("--query"),
        "expected --query mention, combined={combined}"
    );
}

// ---- memory-query -----------------------------------------------------------

#[test]
fn memory_query_command_branch_substitutes_query() {
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let (repo, home) = setup_project_with_memory(json!({
        "query": {
            "kind": "command",
            "command": "brv query {query}"
        }
    }));

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "memory-query",
            "--query",
            "How does coordination work?",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out
        .stdout
        .contains("brv query 'How does coordination work?'"));
}

#[test]
fn memory_query_skill_branch_emits_structured_inputs() {
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let (repo, home) = setup_project_with_memory(json!({
        "query": {
            "kind": "skill",
            "skill": "byterover-explore"
        }
    }));

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "memory-query", "--query", "auth"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Skill: `byterover-explore`"));
    assert!(out.stdout.contains("`query` = `\"auth\"`"));
}

#[test]
fn memory_query_not_configured_branch_renders_setup_guidance() {
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let (repo, home) = setup_project_without_memory();

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "memory-query", "--query", "x"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Memory `query` is not configured"));
}

// ---- mixed configurations ---------------------------------------------------

#[test]
fn memory_capture_renders_skill_when_only_capture_configured() {
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let (repo, home) = setup_project_with_memory(json!({
        "capture": {
            "kind": "skill",
            "skill": "byterover-explore"
        }
    }));

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "memory-capture", "--context", "x"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Skill: `byterover-explore`"));
}

#[test]
fn memory_query_renders_not_configured_when_only_capture_set() {
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let (repo, home) = setup_project_with_memory(json!({
        "capture": {
            "kind": "command",
            "command": "brv curate {context}"
        }
    }));

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "memory-query", "--query", "x"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Memory `query` is not configured"));
}
