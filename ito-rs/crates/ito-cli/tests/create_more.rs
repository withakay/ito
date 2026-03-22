#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

#[test]
fn create_module_and_change_error_paths_and_outputs() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    // Missing module name.
    let out = run_rust_candidate(rust_path, &["create", "module"], repo.path(), home.path());
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Missing required argument"));

    // Create a module.
    let out = run_rust_candidate(
        rust_path,
        &["create", "module", "demo", "--scope", "*"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Created module"));

    // Create a module with explicit description.
    let out = run_rust_candidate(
        rust_path,
        &[
            "create",
            "module",
            "demo-described",
            "--scope",
            "*",
            "--description",
            "Demo module description",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let modules_dir = repo.path().join(".ito/modules");
    let module_dir = std::fs::read_dir(&modules_dir)
        .expect("read modules dir")
        .filter_map(|entry| entry.ok())
        .find_map(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with("_demo-described") {
                Some(entry.path())
            } else {
                None
            }
        })
        .expect("expected demo-described module directory");
    let module_md = module_dir.join("module.md");
    let module_md_content =
        std::fs::read_to_string(&module_md).expect("expected module.md for described module");
    assert!(
        module_md_content.contains("Demo module description"),
        "module.md should include provided description, got: {module_md_content}"
    );

    // Creating it again should hit the already-exists path.
    let out = run_rust_candidate(
        rust_path,
        &["create", "module", "demo", "--scope", "*"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("already exists"));

    // Missing change name.
    let out = run_rust_candidate(rust_path, &["create", "change"], repo.path(), home.path());
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Missing required argument"));

    // Invalid change name.
    let out = run_rust_candidate(
        rust_path,
        &["create", "change", "BadName"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);

    // Valid change with description prints summary to stderr.
    let out = run_rust_candidate(
        rust_path,
        &[
            "create",
            "change",
            "add-thing",
            "--schema",
            "spec-driven",
            "--description",
            "desc",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stderr.contains("✔ Created change"));
    assert!(out.stderr.contains("Next steps"));
}

// ── Task 3.2: --sub-module flag mutual exclusivity ────────────────────────────

#[test]
fn create_change_sub_module_and_module_are_mutually_exclusive() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    // Passing both --module and --sub-module should fail.
    let out = run_rust_candidate(
        rust_path,
        &[
            "create",
            "change",
            "my-change",
            "--module",
            "001",
            "--sub-module",
            "001.01",
        ],
        repo.path(),
        home.path(),
    );
    assert_ne!(
        out.code, 0,
        "should fail when both --module and --sub-module are given"
    );
    // Clap enforces conflicts_with, so the error comes from clap.
    let combined = format!("{}{}", out.stdout, out.stderr);
    assert!(
        combined.contains("cannot be used with") || combined.contains("mutually exclusive"),
        "error should mention mutual exclusivity; got: {combined}"
    );
}

// ── Task 3.4 + 3.5: --sub-module creates NNN.SS-NN_name and writes sub module.md ──

#[test]
fn create_change_with_sub_module_flag_creates_composite_id_change() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    // Set up parent module 024 and sub-module 01_auth.
    fixtures::write(
        repo.path().join(".ito/modules/024_backend/module.md"),
        "# Backend\n\n## Purpose\nBackend module\n\n## Scope\n- *\n\n## Changes\n<!-- Changes will be listed here as they are created -->\n",
    );
    fixtures::write(
        repo.path()
            .join(".ito/modules/024_backend/sub/01_auth/module.md"),
        "# Auth\n\n## Purpose\nAuth sub-module\n\n## Scope\n- *\n\n## Changes\n<!-- Changes will be listed here as they are created -->\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &["create", "change", "add-jwt", "--sub-module", "024.01"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stdout={} stderr={}", out.stdout, out.stderr);
    assert!(
        out.stderr.contains("024.01-01_add-jwt"),
        "output should mention the composite change id; got: {}",
        out.stderr
    );

    // The change directory should exist with the composite id.
    let change_dir = repo.path().join(".ito/changes/024.01-01_add-jwt");
    assert!(change_dir.exists(), "change directory should exist");
    assert!(
        change_dir.join(".ito.yaml").exists(),
        ".ito.yaml should exist"
    );

    // The sub-module's module.md should contain the new change entry.
    let sub_md = std::fs::read_to_string(
        repo.path()
            .join(".ito/modules/024_backend/sub/01_auth/module.md"),
    )
    .expect("read sub module.md");
    assert!(
        sub_md.contains("024.01-01_add-jwt"),
        "sub-module module.md should contain the new change; got:\n{sub_md}"
    );

    // The parent module.md should NOT contain the sub-module change.
    let parent_md = std::fs::read_to_string(repo.path().join(".ito/modules/024_backend/module.md"))
        .expect("read parent module.md");
    assert!(
        !parent_md.contains("024.01-01_add-jwt"),
        "parent module.md should not contain sub-module change; got:\n{parent_md}"
    );
}

// ── Task 3.7: Remote-mode guard ───────────────────────────────────────────────

#[test]
fn create_change_sub_module_rejects_remote_persistence_mode() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    // Write a backend config that enables remote persistence.
    fixtures::write(
        repo.path().join(".ito/config.json"),
        r#"{
  "backend": {
    "enabled": true,
    "url": "http://127.0.0.1:19999",
    "token": "test-token",
    "project": {
      "org": "acme",
      "repo": "widgets"
    }
  }
}"#,
    );

    let out = run_rust_candidate(
        rust_path,
        &["create", "change", "my-change", "--sub-module", "024.01"],
        repo.path(),
        home.path(),
    );

    // Should fail with a non-zero exit code.
    assert_ne!(
        out.code, 0,
        "should fail in remote persistence mode; stdout={} stderr={}",
        out.stdout, out.stderr
    );

    let combined = format!("{}{}", out.stdout, out.stderr);
    assert!(
        combined.contains("local-only") || combined.contains("remote persistence"),
        "error should mention local-only restriction; got: {combined}"
    );
}
