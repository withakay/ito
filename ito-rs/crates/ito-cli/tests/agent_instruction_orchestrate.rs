#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

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
