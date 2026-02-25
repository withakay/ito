mod support;

use ito_test_support::run_rust_candidate;
use support::write;

fn make_repo_with_two_specs_and_one_delta() -> tempfile::TempDir {
    let td = tempfile::tempdir().expect("repo");
    write(td.path().join("README.md"), "# temp\n");

    write(
        td.path().join(".ito/specs/beta/spec.md"),
        "# Beta\n\n## Purpose\nBeta purpose long enough for strict mode.\n\n## Requirements\n\n### Requirement: Beta\nThe system SHALL beta.\n\n#### Scenario: Beta\n- **WHEN** beta\n- **THEN** beta\n",
    );
    write(
        td.path().join(".ito/specs/alpha/spec.md"),
        "# Alpha\n\n## Purpose\nAlpha purpose long enough for strict mode.\n\n## Requirements\n\n### Requirement: Alpha\nThe system SHALL alpha.\n\n#### Scenario: Alpha\n- **WHEN** alpha\n- **THEN** alpha\n",
    );

    // Delta spec that MUST NOT appear in `ito show specs` output.
    write(
        td.path()
            .join(".ito/changes/000-01_demo/specs/delta/spec.md"),
        "DELTA CONTENT MUST NOT APPEAR\n",
    );

    td
}

#[test]
fn show_specs_bundles_truth_specs_as_markdown_with_metadata() {
    let base = make_repo_with_two_specs_and_one_delta();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    support::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(rust_path, &["show", "specs"], repo.path(), home.path());
    assert_eq!(out.code, 0);

    let root = std::fs::canonicalize(repo.path()).unwrap_or_else(|_| repo.path().to_path_buf());
    let alpha_path = root
        .join(".ito/specs/alpha/spec.md")
        .to_string_lossy()
        .to_string();
    let beta_path = root
        .join(".ito/specs/beta/spec.md")
        .to_string_lossy()
        .to_string();

    assert!(
        out.stdout
            .contains(&format!("<!-- spec-id: alpha; source: {alpha_path} -->"))
    );
    assert!(
        out.stdout
            .contains(&format!("<!-- spec-id: beta; source: {beta_path} -->"))
    );
    assert!(out.stdout.contains("# Alpha"));
    assert!(out.stdout.contains("# Beta"));
    assert!(!out.stdout.contains("DELTA CONTENT MUST NOT APPEAR"));

    let alpha_idx = out.stdout.find("<!-- spec-id: alpha").unwrap();
    let beta_idx = out.stdout.find("<!-- spec-id: beta").unwrap();
    assert!(alpha_idx < beta_idx);
}

#[test]
fn show_specs_bundles_truth_specs_as_json_with_absolute_paths() {
    let base = make_repo_with_two_specs_and_one_delta();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    support::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["show", "specs", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);

    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("json");
    assert_eq!(v["specCount"].as_u64(), Some(2));

    let specs = v["specs"].as_array().expect("specs should be array");
    assert_eq!(specs.len(), 2);
    assert_eq!(specs[0]["id"].as_str(), Some("alpha"));
    assert_eq!(specs[1]["id"].as_str(), Some("beta"));

    let root = std::fs::canonicalize(repo.path()).unwrap_or_else(|_| repo.path().to_path_buf());
    let alpha_path = root.join(".ito/specs/alpha/spec.md");
    let beta_path = root.join(".ito/specs/beta/spec.md");
    assert_eq!(
        specs[0]["path"].as_str(),
        Some(alpha_path.to_string_lossy().as_ref())
    );
    assert_eq!(
        specs[1]["path"].as_str(),
        Some(beta_path.to_string_lossy().as_ref())
    );
    assert!(
        specs[0]["markdown"]
            .as_str()
            .unwrap_or_default()
            .contains("# Alpha"),
        "alpha markdown should be present"
    );
    assert!(
        specs[1]["markdown"]
            .as_str()
            .unwrap_or_default()
            .contains("# Beta"),
        "beta markdown should be present"
    );
}
