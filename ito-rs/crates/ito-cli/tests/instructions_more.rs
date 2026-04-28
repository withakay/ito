#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;
use serde_json::json;

#[test]
fn agent_instruction_proposal_without_change_prints_new_proposal_guide() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "proposal"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("Create a New Proposal"));
    assert!(out.stdout.contains("ito sync"));
    assert!(out.stdout.contains("ito create change"));
    assert!(out.stdout.contains("Known modules at instruction time"));
    let lower = out.stdout.to_lowercase();
    assert!(lower.contains("| 000 |"));
}

#[test]
fn agent_instruction_proposal_without_change_supports_json_output() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "proposal", "--json"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("valid json");
    assert_eq!(v["artifactId"], "new-proposal");
    assert!(
        v["instruction"]
            .as_str()
            .unwrap_or_default()
            .contains("Create a New Proposal")
    );
}

#[test]
fn agent_instruction_manifesto_uses_default_variant_and_profile() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "manifesto"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Ito Manifesto: Execution Contract"));
    assert!(out.stdout.contains("Manifesto variant: `light`"));
    assert!(out.stdout.contains("Capability profile: `full`"));
}

#[test]
fn agent_instruction_manifesto_json_includes_resolved_defaults() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "manifesto", "--json"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("valid json");
    assert_eq!(v["artifact"], "manifesto");
    assert_eq!(v["variant"], "light");
    assert_eq!(v["profile"], "full");
    assert_eq!(v["state"], "no-change-selected");
    assert!(
        v["supported_variants"]
            .as_array()
            .is_some_and(|items| items.iter().any(|item| item == "full"))
    );
    assert!(
        v["supported_profiles"]
            .as_array()
            .is_some_and(|items| items.iter().any(|item| item == "proposal-only"))
    );
}

#[test]
fn agent_instruction_manifesto_rejects_operation_for_light_variant() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "manifesto",
            "--variant",
            "light",
            "--operation",
            "apply",
        ],
        repo.path(),
        home.path(),
    );

    assert_ne!(out.code, 0);
    assert!(
        out.stderr
            .contains("The --operation selector is only supported with --variant full"),
        "stderr={}",
        out.stderr
    );
}

#[test]
fn agent_instruction_manifesto_rejects_operation_without_change() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "manifesto",
            "--variant",
            "full",
            "--operation",
            "apply",
        ],
        repo.path(),
        home.path(),
    );

    assert_ne!(out.code, 0);
    assert!(
        out.stderr
            .contains("The --operation selector requires --change"),
        "stderr={}",
        out.stderr
    );
}

#[test]
fn agent_instruction_manifesto_change_scope_includes_change_state() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "manifesto",
            "--change",
            "000-01_test-change",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Change ID: `000-01_test-change`"));
    assert!(out.stdout.contains("\"change_id\": \"000-01_test-change\""));
    assert!(out.stdout.contains("proposal-drafting"));
}

#[test]
fn agent_instruction_manifesto_change_scope_json_reports_state() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "manifesto",
            "--change",
            "000-01_test-change",
            "--json",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("valid json");
    assert_eq!(v["artifact"], "manifesto");
    assert_eq!(v["state"], "proposal-drafting");
    assert_eq!(v["variant"], "light");
    assert_eq!(v["profile"], "full");
}

#[test]
fn agent_instruction_manifesto_full_variant_renders_full_section() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "manifesto", "--variant", "full"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Manifesto variant: `full`"));
    assert!(out.stdout.contains("## Rendered Ito Instructions"));
    assert!(
        out.stdout
            .contains("No rendered operation instructions were embedded.")
    );
}

#[test]
fn agent_instruction_manifesto_redacts_explicit_coordination_path() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path().join(".ito/config.json"),
        "{\"changes\":{\"coordination_branch\":{\"storage\":\"worktree\",\"worktree_path\":\"/tmp/manifesto-sensitive\"}}}\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "manifesto"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("<redacted-path>"));
    assert!(!out.stdout.contains("/tmp/manifesto-sensitive"));
}

#[test]
fn agent_instruction_manifesto_full_variant_embeds_requested_proposal_instruction() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "manifesto",
            "--change",
            "000-01_test-change",
            "--variant",
            "full",
            "--operation",
            "proposal",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("### `proposal`"));
    assert!(out.stdout.contains("<artifact id=\"proposal\""));
    assert!(out.stdout.contains("Write to:"));
}

#[test]
fn agent_instruction_manifesto_full_variant_embeds_allowed_default_set() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "manifesto",
            "--change",
            "000-01_test-change",
            "--variant",
            "full",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("### `proposal`"));
    assert!(out.stdout.contains("### `review`"));
}

#[test]
fn agent_instruction_manifesto_full_variant_rejects_incompatible_operation() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "manifesto",
            "--change",
            "000-01_test-change",
            "--variant",
            "full",
            "--operation",
            "apply",
        ],
        repo.path(),
        home.path(),
    );

    assert_ne!(out.code, 0);
    assert!(
        out.stderr
            .contains("Requested operation 'apply' is not allowed"),
        "stderr={}",
        out.stderr
    );
}

#[test]
fn agent_instruction_manifesto_full_variant_supports_finish_for_archived_change() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(repo.path().join(".ito/changes/archive")).expect("archive dir");
    std::fs::rename(
        repo.path().join(".ito/changes/000-01_test-change"),
        repo.path()
            .join(".ito/changes/archive/2026-04-27-000-01_test-change"),
    )
    .expect("archive move");

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "manifesto",
            "--change",
            "000-01_test-change",
            "--variant",
            "full",
            "--operation",
            "finish",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("### `finish`"));
    assert!(out.stdout.contains("Refresh archive and specs"));
}

#[test]
fn agent_instruction_manifesto_planning_profile_is_advisory() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "manifesto", "--profile", "planning"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Capability profile: `planning`"));
    assert!(
        out.stdout
            .contains("Inspect and advise only. Do not mutate files.")
    );
}

#[test]
fn agent_instruction_manifesto_planning_profile_embeds_no_mutating_artifacts() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "manifesto",
            "--change",
            "000-01_test-change",
            "--variant",
            "full",
            "--profile",
            "planning",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Capability profile: `planning`"));
    assert!(
        out.stdout
            .contains("No rendered operation instructions were embedded.")
    );
    assert!(!out.stdout.contains("### `proposal`"));
}

#[test]
fn agent_instruction_manifesto_memory_config_embeds_operation_instructions() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path().join(".ito/config.json"),
        &(serde_json::to_string_pretty(&json!({
            "memory": {
                "capture": {
                    "kind": "command",
                    "command": "brv curate {context}"
                }
            }
        }))
        .unwrap()
            + "\n"),
    );

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "manifesto"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Memory is configured."));
    assert!(out.stdout.contains("Capture instruction:"));
    assert!(out.stdout.contains("brv curate"));
    assert!(!out.stdout.contains("\"capture_instruction\": null"));
}

#[test]
fn agent_instruction_manifesto_change_scope_reports_apply_ready_state() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path()
            .join(".ito/changes/000-01_test-change/design.md"),
        "## Context\nReady fixture\n",
    );
    fixtures::write(
        repo.path().join(".ito/changes/000-01_test-change/tasks.md"),
        "## 1. Implementation\n- [ ] 1.1 Do a thing\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "manifesto",
            "--change",
            "000-01_test-change",
            "--json",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("valid json");
    assert_eq!(v["state"], "apply-ready");
}

#[test]
fn agent_instruction_manifesto_change_scope_reports_applying_state() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path()
            .join(".ito/changes/000-01_test-change/design.md"),
        "## Context\nApplying fixture\n",
    );
    fixtures::write(
        repo.path().join(".ito/changes/000-01_test-change/tasks.md"),
        "## 1. Implementation\n- [x] 1.1 Done\n- [ ] 1.2 Remaining\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "manifesto",
            "--change",
            "000-01_test-change",
            "--json",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("valid json");
    assert_eq!(v["state"], "applying");
}

#[test]
fn agent_instruction_text_output_renders_artifact_envelope() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "proposal",
            "--change",
            "000-01_test-change",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("<artifact id=\"proposal\""));
    assert!(out.stdout.contains("Write to:"));
    assert!(out.stdout.contains("<testing_policy>"));
    assert!(out.stdout.contains("- tdd.workflow: red-green-refactor"));
    assert!(out.stdout.contains("RED -> GREEN -> REFACTOR"));
    assert!(out.stdout.contains("- coverage.target_percent: 80"));
}

#[test]
fn agent_instruction_proposal_honors_testing_policy_override() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path().join(".ito/config.json"),
        "{\"defaults\":{\"testing\":{\"coverage\":{\"target_percent\":93}}}}\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "proposal",
            "--change",
            "000-01_test-change",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("- coverage.target_percent: 93"));
}

#[test]
fn agent_instruction_change_flag_supports_shorthand() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "proposal", "--change", "0-1"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("<artifact id=\"proposal\""));
}

#[test]
fn agent_instruction_change_flag_reports_ambiguous_target() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path().join(".ito/changes/000-01_test-alt/proposal.md"),
        "## Why\nAmbiguous fixture\n\n## What Changes\n- None\n\n## Impact\n- None\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "proposal", "--change", "0-1"],
        repo.path(),
        home.path(),
    );

    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("is ambiguous"));
    assert!(out.stderr.contains("000-01_test-change"));
    assert!(out.stderr.contains("000-01_test-alt"));
}

#[test]
fn agent_instruction_change_flag_supports_slug_query() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path()
            .join(".ito/changes/001-12_setup-wizard/proposal.md"),
        "## Why\nTest fixture\n\n## What Changes\n- None\n\n## Impact\n- None\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "proposal",
            "--change",
            "setup wizard",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("<artifact id=\"proposal\""));
}

#[test]
fn agent_instruction_review_requires_change_flag() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "review"],
        repo.path(),
        home.path(),
    );

    assert_ne!(out.code, 0);
    assert!(
        out.stderr
            .contains("review instruction requires --change <id>"),
        "stderr was: {}",
        out.stderr
    );
}

#[test]
fn agent_instruction_review_renders_review_template() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "review",
            "--change",
            "000-01_test-change",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Peer Review"));
    assert!(out.stdout.contains("000-01_test-change"));
    assert!(out.stdout.contains("## Testing Policy"));
    assert!(out.stdout.contains("## Output Format"));
    assert!(out.stdout.contains("Verdict: needs-discussion"));
}

#[test]
fn agent_instruction_archive_without_change_prints_generic_guidance() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "archive"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("ito archive"), "stdout={}", out.stdout);
    assert!(
        out.stdout.contains("ito audit reconcile"),
        "stdout={}",
        out.stdout
    );
    assert!(out.stdout.contains("ito sync"), "stdout={}", out.stdout);
    assert!(
        out.stdout.contains("000-01_test-change"),
        "expected available change hint in generic mode; stdout={}",
        out.stdout
    );
}

#[test]
fn agent_instruction_archive_with_change_prints_targeted_instruction() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "archive",
            "--change",
            "000-01_test-change",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(
        out.stdout.contains("000-01_test-change"),
        "stdout={}",
        out.stdout
    );
    assert!(
        out.stdout.contains("ito archive 000-01_test-change --yes"),
        "stdout={}",
        out.stdout
    );
    assert!(out.stdout.contains("ito sync"), "stdout={}", out.stdout);
    assert!(
        out.stdout
            .contains("create an integration branch from `main`")
            || out
                .stdout
                .contains("create an integration branch from main"),
        "stdout={}",
        out.stdout
    );
}

#[test]
fn agent_instruction_archive_with_invalid_change_fails() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "archive",
            "--change",
            "999-99_does-not-exist",
        ],
        repo.path(),
        home.path(),
    );

    assert_ne!(out.code, 0, "should fail for unknown change");
    assert!(
        out.stderr.contains("999-99_does-not-exist"),
        "stderr should mention the invalid change id; stderr={}",
        out.stderr
    );
}

#[test]
fn agent_instruction_finish_with_change_prompts_for_archive() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "finish",
            "--change",
            "000-01_test-change",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(
        out.stdout
            .contains("Do you want to archive this change now?")
    );
    assert!(
        out.stdout
            .contains("ito agent instruction archive --change '000-01_test-change'")
    );
    assert!(out.stdout.contains("ito sync"), "stdout={}", out.stdout);
}

#[test]
fn agent_instruction_apply_text_is_compact_and_has_trailing_newline() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "apply",
            "--change",
            "000-01_test-change",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("## Apply: 000-01_test-change"));
    assert!(out.stdout.contains("ito sync"), "stdout={}", out.stdout);
    assert!(out.stdout.contains("### Testing Policy"));
    assert!(!out.stdout.contains("\n\n\n"), "stdout={}", out.stdout);
    assert!(out.stdout.ends_with('\n'), "stdout={}", out.stdout);
}
