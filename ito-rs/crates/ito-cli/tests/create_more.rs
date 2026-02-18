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
