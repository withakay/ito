#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::{CmdOutput, run_rust_candidate, rust_candidate_command};
use std::path::Path;

#[test]
fn orchestrate_requires_orchestrate_md() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "orchestrate"],
        repo.path(),
        home.path(),
    );

    assert_ne!(out.code, 0);
    assert!(
        out.stderr.contains("orchestrate.md"),
        "stderr was: {}",
        out.stderr
    );
    assert!(
        out.stderr.contains("ito-orchestrate-setup"),
        "stderr was: {}",
        out.stderr
    );
}

#[test]
fn orchestrate_succeeds_when_orchestrate_md_exists() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path().join(".ito/user-prompts/orchestrate.md"),
        "---\npreset: generic\nmax_parallel: auto\n---\n\n## MUST\n- Run make check\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "orchestrate"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(
        out.stdout
            .contains("Orchestrate: Change Apply Coordination")
    );
    assert!(out.stdout.contains("ito-orchestrator-workflow"));
    assert!(out.stdout.contains("Coordinator agent"));
    assert!(out.stdout.contains("ito-orchestrator"));
    assert!(out.stdout.contains("ito-planner"));
    assert!(out.stdout.contains("ito-researcher"));
    assert!(out.stdout.contains("ito-worker"));
    assert!(out.stdout.contains("ito-reviewer"));
    assert!(out.stdout.contains("ito-test-runner"));
    assert!(out.stdout.contains("Source-of-Truth Precedence"));
    assert!(out.stdout.contains("Detected harness"));
    assert!(out.stdout.contains("Direct Coordinator Activation"));
    assert!(out.stdout.contains("Delegated Role Agents"));
    assert!(out.stdout.contains("Gate Planning"));
    assert!(out.stdout.contains("Run State"));
    assert!(out.stdout.contains("Failure and Remediation"));
    assert!(out.stdout.contains("Resume Behavior"));
    assert!(out.stdout.contains("depends_on"));
    assert!(out.stdout.contains("preferred_gates"));
    assert!(
        out.stdout
            .contains(".ito/.state/orchestrate/runs/<run-id>/")
    );
    assert!(out.stdout.contains("events.jsonl"));
    let plan = out.stdout.find("plan-worker").expect("plan role");
    let apply = out.stdout.find("apply-worker").expect("apply role");
    assert!(plan < apply, "expected plan role before apply role");
}

#[test]
fn orchestrate_includes_detected_opencode_harness_context() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path().join(".ito/user-prompts/orchestrate.md"),
        "---\npreset: generic\n---\n\n## MUST\n- Run tests\n",
    );

    let out = run_orchestrate_with_env(rust_path, repo.path(), home.path(), "OPENCODE_SESSION_ID");

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Detected harness**: `opencode`"));
    assert!(out.stdout.contains("Preset agent roles"));
    assert!(out.stdout.contains("ito-planner"));
}

#[test]
fn orchestrate_uses_canonical_harness_detection_order() {
    let cases = [
        ("ITO_HARNESS_SESSION_ID", "generic"),
        ("CLAUDE_SESSION_ID", "claude-code"),
        ("CODEX_SESSION_ID", "codex"),
        ("GITHUB_COPILOT_CHAT_SESSION_ID", "github-copilot"),
        ("PI_SESSION_ID", "pi"),
    ];

    for (env_key, expected) in cases {
        let base = fixtures::make_repo_with_spec_change_fixture();
        let repo = tempfile::tempdir().expect("work");
        let home = tempfile::tempdir().expect("home");
        let rust_path = assert_cmd::cargo::cargo_bin!("ito");

        fixtures::reset_repo(repo.path(), base.path());
        fixtures::write(
            repo.path().join(".ito/user-prompts/orchestrate.md"),
            "---\npreset: generic\n---\n\n## MUST\n- Run tests\n",
        );

        let out = run_orchestrate_with_env(rust_path, repo.path(), home.path(), env_key);

        assert_eq!(out.code, 0, "stderr={}", out.stderr);
        assert!(
            out.stdout
                .contains(&format!("Detected harness**: `{expected}`")),
            "expected {env_key} to render harness {expected}; stdout={}",
            out.stdout
        );
    }
}

#[test]
fn orchestrate_reports_unknown_harness_without_session_env() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path().join(".ito/user-prompts/orchestrate.md"),
        "---\npreset: generic\n---\n\n## MUST\n- Run tests\n",
    );

    let out = run_orchestrate_without_harness_env(rust_path, repo.path(), home.path());

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Detected harness**: `unknown`"));
}

fn run_orchestrate_with_env(program: &Path, cwd: &Path, home: &Path, env_key: &str) -> CmdOutput {
    let mut cmd = rust_candidate_command(program);
    configure_orchestrate_command(&mut cmd, cwd, home);
    cmd.env(env_key, "test-session");
    run_command(cmd)
}

fn run_orchestrate_without_harness_env(program: &Path, cwd: &Path, home: &Path) -> CmdOutput {
    let mut cmd = rust_candidate_command(program);
    configure_orchestrate_command(&mut cmd, cwd, home);
    run_command(cmd)
}

fn configure_orchestrate_command(cmd: &mut std::process::Command, cwd: &Path, home: &Path) {
    cmd.args(["agent", "instruction", "orchestrate"]);
    cmd.current_dir(cwd);
    cmd.env("CI", "1");
    cmd.env("NO_COLOR", "1");
    cmd.env("ITO_INTERACTIVE", "0");
    cmd.env("TERM", "dumb");
    cmd.env("HOME", home);
    cmd.env("XDG_CONFIG_HOME", home.join(".config"));
    cmd.env("XDG_DATA_HOME", home);
    for env_var in ito_common::harness::HARNESS_SESSION_ENV_VARS {
        cmd.env_remove(env_var);
    }
}

fn run_command(mut cmd: std::process::Command) -> CmdOutput {
    let out = cmd.output().expect("command should run");
    CmdOutput {
        code: out.status.code().unwrap_or(1),
        stdout: String::from_utf8_lossy(&out.stdout).to_string(),
        stderr: String::from_utf8_lossy(&out.stderr).to_string(),
    }
}

#[test]
fn orchestrate_policy_identifies_direct_and_delegated_surfaces() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path().join(".ito/user-prompts/orchestrate.md"),
        "---\npreset: generic\n---\n\n## MUST\n- Keep local policy additive\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "orchestrate"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(
        out.stdout
            .contains("Activate `ito-orchestrator` directly as the coordinator"),
        "stdout={}",
        out.stdout
    );
    assert!(
        out.stdout.contains(
            "Do not dispatch `ito-orchestrator`, `ito-general`, or `ito-thinking` as ordinary worker sub-agents"
        ),
        "stdout={}",
        out.stdout
    );
    assert!(
        out.stdout
            .contains("Dispatch delegated role agents for bounded work packets"),
        "stdout={}",
        out.stdout
    );
    assert!(
        out.stdout.contains("Keep local policy additive"),
        "stdout={}",
        out.stdout
    );
    assert!(
        out.stdout
            .contains("Project `orchestrate.md` guidance is additive local policy"),
        "stdout={}",
        out.stdout
    );
}

#[test]
fn orchestrate_surfaces_recommended_skills_from_preset() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path().join(".ito/user-prompts/orchestrate.md"),
        "---\npreset: rust\n---\n\n## MUST\n- Run cargo test\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "orchestrate"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(
        out.stdout.contains("Recommended skills"),
        "expected rendered instruction to list recommended skills; stdout was:\n{}",
        out.stdout
    );
    assert!(
        out.stdout.contains("rust-style"),
        "expected rust-style to be listed; stdout was:\n{}",
        out.stdout
    );
    assert!(
        out.stdout.contains("rust-engineer"),
        "expected rust preset agent roles to be listed; stdout was:\n{}",
        out.stdout
    );
    assert!(out.stdout.contains("ito-planner"));
    assert!(out.stdout.contains("ito-researcher"));
}

#[test]
fn orchestrate_tolerates_trailing_whitespace_in_front_matter_delimiter() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    // Closing `---` has trailing spaces before the newline — the parser must
    // still treat the next line as the body and leave the document intact.
    fixtures::write(
        repo.path().join(".ito/user-prompts/orchestrate.md"),
        "---\npreset: generic\n---   \n\n## MUST\n- No trailing-ws bugs\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "orchestrate"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(
        out.stdout.contains("No trailing-ws bugs"),
        "expected body content to be preserved despite trailing whitespace on the closing delimiter; stdout was:\n{}",
        out.stdout
    );
}

#[test]
fn orchestrate_json_output_has_correct_artifact_id() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path().join(".ito/user-prompts/orchestrate.md"),
        "# ok\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "orchestrate", "--json"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains(r#""artifactId": "orchestrate""#));
    assert!(out.stdout.contains(r#""instruction":"#));
}
