mod support;

use assert_cmd::Command;
use predicates::str::contains;
use support::{make_repo_all_valid, write};

#[test]
fn write_change_proposal_replaces_contents() {
    let repo = make_repo_all_valid();
    let mut command = Command::cargo_bin("ito").unwrap();
    command.current_dir(repo.path());
    command.write_stdin(
        "## Why\nUpdated\n\n## What Changes\n- Replace via CLI\n\n## Impact\n- Tests\n",
    );
    command.args(["write", "change", "000-01_test-change", "proposal"]);

    command.assert().success().stdout(contains(
        "Successfully updated artifact '000-01_test-change:proposal'",
    ));

    let proposal = std::fs::read_to_string(
        repo.path()
            .join(".ito/changes/000-01_test-change/proposal.md"),
    )
    .unwrap();
    assert!(proposal.contains("Updated"), "proposal={proposal}");
}

#[test]
fn patch_change_proposal_applies_unified_diff() {
    let repo = make_repo_all_valid();
    let mut command = Command::cargo_bin("ito").unwrap();
    command.current_dir(repo.path());
    command.write_stdin("--- proposal\n+++ proposal\n@@ -1,5 +1,5 @@\n ## Why\n-Test fixture\n+Patched fixture\n \n ## What Changes\n - Adds a small delta\n");
    command.args(["patch", "change", "000-01_test-change", "proposal"]);

    command.assert().success().stdout(contains(
        "Successfully patched artifact '000-01_test-change:proposal'",
    ));

    let proposal = std::fs::read_to_string(
        repo.path()
            .join(".ito/changes/000-01_test-change/proposal.md"),
    )
    .unwrap();
    assert!(proposal.contains("Patched fixture"), "proposal={proposal}");
}

#[test]
fn write_change_spec_delta_creates_missing_capability_file() {
    let repo = make_repo_all_valid();
    write(
        repo.path().join(".ito/modules/000_ungrouped/module.md"),
        "# Ungrouped\n\n## Purpose\nModule for ad-hoc changes. This purpose is long enough.\n\n## Scope\n- *\n\n## Changes\n- [ ] 000-01_test-change\n",
    );

    let mut command = Command::cargo_bin("ito").unwrap();
    command.current_dir(repo.path());
    command.write_stdin("## ADDED Requirements\n\n### Requirement: Beta Delta\nThe system SHALL add beta.\n\n#### Scenario: Beta works\n- **WHEN** beta is requested\n- **THEN** beta is present\n");
    command.args(["write", "change", "000-01_test-change", "spec", "beta"]);

    command.assert().success().stdout(contains(
        "Successfully created artifact '000-01_test-change:spec:beta'",
    ));

    let spec_delta = std::fs::read_to_string(
        repo.path()
            .join(".ito/changes/000-01_test-change/specs/beta/spec.md"),
    )
    .unwrap();
    assert!(spec_delta.contains("Requirement: Beta Delta"));
}
