#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

#[test]
fn validate_ambiguous_item_is_an_error() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    let out = run_rust_candidate(rust_path, &["validate", "alpha"], repo.path(), home.path());
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Ambiguous item"));
}

#[test]
fn validate_type_module_special_cases_to_spec_by_id() {
    let base = fixtures::make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["validate", "--type", "module", "alpha"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("Specification 'alpha' is valid"));
}

#[test]
fn validate_unknown_spec_offers_suggestions() {
    let base = fixtures::make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["validate", "--type", "spec", "alpa"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Did you mean"));
    assert!(out.stderr.contains("alpha"));
}

#[test]
fn validate_all_prints_failure_report_in_text_mode() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["validate", "--all", "--strict"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Validation failed"));
    assert!(out.stderr.contains("spec beta"));
}

#[test]
fn validate_all_json_success_has_summary_and_by_type() {
    let base = fixtures::make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["validate", "--all", "--strict", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("validate all json");
    assert!(v.get("items").is_some());
    assert!(v.get("summary").is_some());
    assert!(v.get("summary").and_then(|s| s.get("byType")).is_some());
}

#[test]
fn validate_module_routes_and_error_paths() {
    let base = fixtures::make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(rust_path, &["validate", "module"], repo.path(), home.path());
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("ito validate module"));

    // Invalid module reports issues.
    fixtures::write(
        repo.path().join(".ito/modules/001_demo/module.md"),
        "# Demo\n\n## Purpose\nshort\n\n## Scope\n\n## Changes\n- [ ] 001-01_bad\n",
    );
    let out = run_rust_candidate(
        rust_path,
        &["validate", "module", "001_demo"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("has issues"));

    // Valid module.
    let out = run_rust_candidate(
        rust_path,
        &["validate", "module", "000_ungrouped"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("is valid"));
}
