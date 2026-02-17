#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

#[test]
fn config_set_get_supports_coordination_branch_keys() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(home.path().join(".config/ito")).expect("create config dir");

    let out = run_rust_candidate(
        rust_path,
        &[
            "config",
            "set",
            "changes.coordination_branch.enabled",
            "false",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let out = run_rust_candidate(
        rust_path,
        &[
            "config",
            "set",
            "changes.coordination_branch.name",
            "ito/internal/custom-changes",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let out = run_rust_candidate(
        rust_path,
        &["config", "get", "changes.coordination_branch.enabled"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert_eq!(out.stdout.trim(), "false");

    let out = run_rust_candidate(
        rust_path,
        &["config", "get", "changes.coordination_branch.name"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert_eq!(out.stdout.trim(), "ito/internal/custom-changes");
}

#[test]
fn config_set_rejects_invalid_coordination_branch_name() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(home.path().join(".config/ito")).expect("create config dir");

    let out = run_rust_candidate(
        rust_path,
        &[
            "config",
            "set",
            "changes.coordination_branch.name",
            "bad..branch",
        ],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Invalid value 'bad..branch'"));
    assert!(out.stderr.contains("changes.coordination_branch.name"));
}

#[test]
fn config_help_path_list_unset_and_schema_smoke() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(home.path().join(".config/ito")).expect("create config dir");

    // No args prints help.
    let out = run_rust_candidate(rust_path, &["config"], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("ito config"));

    // Path.
    let out = run_rust_candidate(rust_path, &["config", "path"], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let path = out.stdout.trim();
    assert!(!path.is_empty());

    // Set + list.
    let out = run_rust_candidate(
        rust_path,
        &[
            "config",
            "set",
            "changes.coordination_branch.enabled",
            "true",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let out = run_rust_candidate(rust_path, &["config", "list"], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("json");
    assert_eq!(v["changes"]["coordination_branch"]["enabled"], true);

    // Unset + get should error.
    let out = run_rust_candidate(
        rust_path,
        &["config", "unset", "changes.coordination_branch.enabled"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let out = run_rust_candidate(
        rust_path,
        &["config", "get", "changes.coordination_branch.enabled"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Key not found"));

    // Schema stdout + schema output.
    let out = run_rust_candidate(rust_path, &["config", "schema"], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let schema: serde_json::Value = serde_json::from_str(&out.stdout).expect("schema json");
    assert!(schema.get("properties").is_some());

    let out_path = repo.path().join("schema.json");
    let out = run_rust_candidate(
        rust_path,
        &[
            "config",
            "schema",
            "--output",
            out_path.to_string_lossy().as_ref(),
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let written = std::fs::read_to_string(&out_path).expect("read schema file");
    let _: serde_json::Value = serde_json::from_str(&written).expect("schema json");
}

#[test]
fn config_unknown_subcommand_errors() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(home.path().join(".config/ito")).expect("create config dir");

    let out = run_rust_candidate(rust_path, &["config", "nope"], repo.path(), home.path());
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Unknown config subcommand"));
}
