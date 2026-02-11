#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;
use ito_test_support::rust_candidate_command;

// PTY-based interactive tests are skipped on Windows due to platform differences
// in terminal handling that can cause hangs.
#[cfg(unix)]
use ito_test_support::pty::run_pty;

#[test]
fn plan_status_errors_when_roadmap_missing() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(repo.path().join(".ito")).unwrap();

    let out = run_rust_candidate(rust_path, &["plan", "status"], repo.path(), home.path());
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("ROADMAP.md not found"));
}

#[test]
fn status_missing_change_flag_lists_available_changes() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(rust_path, &["status"], repo.path(), home.path());
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Missing required option --change"));
    assert!(out.stderr.contains("Available changes"));
}

#[test]
fn status_schema_not_found_includes_available_schemas() {
    let base = fixtures::make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "status",
            "--change",
            "000-01_test-change",
            "--schema",
            "does-not-exist",
        ],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Schema 'does-not-exist' not found"));
    assert!(out.stderr.contains("Available schemas"));
}

#[test]
fn status_change_flag_supports_shorthand_and_partial_match() {
    let base = fixtures::make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["status", "--change", "0-1", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("status json");
    assert_eq!(v["changeName"], "000-01_test-change");

    let out = run_rust_candidate(
        rust_path,
        &["status", "--change", "000-01_test", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("status json");
    assert_eq!(v["changeName"], "000-01_test-change");
}

#[test]
fn status_change_flag_reports_ambiguous_target() {
    let base = fixtures::make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path()
            .join(".ito/changes/000-01_test-alternate/proposal.md"),
        "## Why\nAmbiguous fixture\n\n## What Changes\n- None\n\n## Impact\n- None\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &["status", "--change", "0-1"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("is ambiguous"));
    assert!(out.stderr.contains("000-01_test-change"));
    assert!(out.stderr.contains("000-01_test-alternate"));
}

#[test]
fn status_change_flag_supports_module_scoped_slug_query() {
    let base = fixtures::make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path()
            .join(".ito/changes/001-12_setup-wizard/proposal.md"),
        "## Why\nTest fixture\n\n## What Changes\n- None\n\n## Impact\n- None\n",
    );
    fixtures::write(
        repo.path()
            .join(".ito/changes/002-12_setup-wizard/proposal.md"),
        "## Why\nTest fixture\n\n## What Changes\n- None\n\n## Impact\n- None\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &["status", "--change", "1:setup wizard", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("status json");
    assert_eq!(v["changeName"], "001-12_setup-wizard");
}

#[test]
fn status_change_flag_not_found_shows_suggestions() {
    let base = fixtures::make_repo_all_valid();
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
        &["status", "--change", "setpu"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("not found"));
    assert!(out.stderr.contains("Did you mean"));
    assert!(out.stderr.contains("001-12_setup-wizard"));
}

#[test]
fn list_errors_when_ito_changes_dir_missing() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(rust_path, &["list"], repo.path(), home.path());
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("No Ito changes directory found"));
}

#[test]
fn list_modules_empty_prints_hint() {
    let base = fixtures::make_repo_changes_dir_but_empty();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(rust_path, &["list", "--modules"], repo.path(), home.path());
    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("No modules found"));
    assert!(out.stdout.contains("ito create module"));
}

#[test]
fn commands_run_from_nested_dir_use_git_worktree_root() {
    let base = fixtures::make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    let nested = repo.path().join("sub/dir");
    std::fs::create_dir_all(&nested).expect("create nested directory");

    let init = std::process::Command::new("git")
        .args(["init"])
        .current_dir(repo.path())
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git init should run");
    assert!(init.status.success(), "git init failed: {:?}", init);

    let out = run_rust_candidate(rust_path, &["list"], &nested, home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("000-01_test-change"));
}

#[test]
fn git_env_vars_do_not_override_runtime_root_detection() {
    let base = fixtures::make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    let nested = repo.path().join("sub/dir");
    std::fs::create_dir_all(&nested).expect("create nested directory");

    let init = std::process::Command::new("git")
        .args(["init"])
        .current_dir(repo.path())
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git init should run");
    assert!(init.status.success(), "git init failed: {:?}", init);

    let git_dir = std::env::current_dir()
        .expect("cwd")
        .join(".git")
        .to_string_lossy()
        .to_string();
    let git_work_tree = std::env::current_dir()
        .expect("cwd")
        .to_string_lossy()
        .to_string();

    let mut cmd = rust_candidate_command(rust_path);
    cmd.args(["list"]);
    cmd.current_dir(&nested);
    cmd.env("CI", "1");
    cmd.env("NO_COLOR", "1");
    cmd.env("ITO_INTERACTIVE", "0");
    cmd.env("TERM", "dumb");
    cmd.env("HOME", home.path());
    cmd.env("XDG_DATA_HOME", home.path());
    cmd.env("GIT_DIR", git_dir);
    cmd.env("GIT_WORK_TREE", git_work_tree);

    let out = cmd.output().expect("run ito list");
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();

    assert!(out.status.success(), "stderr={stderr}");
    assert!(stdout.contains("000-01_test-change"));
}

#[test]
fn list_specs_empty_prints_sentence_even_for_json() {
    let base = fixtures::make_repo_changes_dir_but_empty();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(rust_path, &["list", "--specs"], repo.path(), home.path());
    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("No specs found"));

    let out = run_rust_candidate(
        rust_path,
        &["list", "--specs", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("No specs found"));
}

#[test]
fn show_spec_json_filters_and_requirement_index_errors() {
    let base = fixtures::make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["show", "alpha", "--json", "--requirements"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("show spec json");
    let reqs = v.get("requirements").unwrap().as_array().unwrap();
    assert!(!reqs.is_empty());
    assert_eq!(
        reqs[0].get("scenarios").unwrap().as_array().unwrap().len(),
        0
    );

    let out = run_rust_candidate(
        rust_path,
        &["show", "alpha", "--json", "--requirement", "1"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("show spec json");
    assert_eq!(v.get("requirementCount").unwrap().as_u64(), Some(1));

    let out = run_rust_candidate(
        rust_path,
        &["show", "alpha", "--json", "--requirement", "99"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Requirement index out of range"));

    let out = run_rust_candidate(
        rust_path,
        &[
            "show",
            "alpha",
            "--json",
            "--requirements",
            "--requirement",
            "1",
        ],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Cannot use --requirement"));
}

#[test]
fn show_unknown_item_offers_suggestions() {
    let base = fixtures::make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(rust_path, &["show", "alpa"], repo.path(), home.path());
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Did you mean"));
    assert!(out.stderr.contains("alpha"));
}

#[test]
fn show_module_errors_and_json_not_implemented() {
    let base = fixtures::make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["show", "module", "000_ungrouped"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("# Ungrouped"));

    let out = run_rust_candidate(
        rust_path,
        &["show", "module", "000_ungrouped", "--json"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("not implemented"));

    let out = run_rust_candidate(
        rust_path,
        &["show", "module", "999"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("not found"));
}

#[test]
#[cfg(unix)]
#[ignore = "PTY interactive test — can hang in CI; run with --ignored locally"]
fn archive_prompts_on_incomplete_tasks_and_proceeds_when_confirmed() {
    let base = fixtures::make_repo_all_valid();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    // Make tasks incomplete.
    fixtures::write(
        repo.path().join(".ito/changes/000-01_test-change/tasks.md"),
        "## 1. Implementation\n- [ ] 1.1 Not done\n",
    );

    let out = run_pty(
        rust_path,
        &["archive", "000-01_test-change", "--skip-specs"],
        repo.path(),
        home.path(),
        "y\n",
    );
    assert_eq!(out.code, 0);
    let archive_root = repo.path().join(".ito/changes/archive");
    assert!(archive_root.exists());
}
