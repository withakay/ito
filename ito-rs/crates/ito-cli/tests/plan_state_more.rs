#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

struct Ctx {
    _base: tempfile::TempDir,
    repo: tempfile::TempDir,
    home: tempfile::TempDir,
}

fn setup() -> Ctx {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(repo.path().join(".ito")).unwrap();

    Ctx {
        _base: base,
        repo,
        home,
    }
}

#[test]
fn plan_status_reports_missing_workspace_without_error() {
    let ctx = setup();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let out = run_rust_candidate(
        rust_path,
        &["plan", "status"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Planning Workspace"));
    assert!(out.stdout.contains("Planning: missing"));
    assert!(out.stdout.contains("Run `ito plan init`"));
}

#[test]
fn plan_status_reports_invalid_workspace_without_init_hint_loop() {
    let ctx = setup();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    std::fs::write(ctx.repo.path().join(".ito/planning"), "not a directory\n").unwrap();

    let out = run_rust_candidate(
        rust_path,
        &["plan", "status"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Planning: invalid"));
    assert!(out.stdout.contains("Planning path is not a directory"));
    assert!(out.stdout.contains("Rename or remove it"));
}

#[test]
fn plan_status_reports_invalid_research_workspace() {
    let ctx = setup();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    std::fs::create_dir_all(ctx.repo.path().join(".ito/planning")).unwrap();
    std::fs::write(ctx.repo.path().join(".ito/planning/topic.md"), "# Topic\n").unwrap();
    std::fs::write(ctx.repo.path().join(".ito/research"), "not a directory\n").unwrap();

    let out = run_rust_candidate(
        rust_path,
        &["plan", "status"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Research: invalid"));
    assert!(out.stdout.contains("Research path is not a directory"));
    assert!(out.stdout.contains("before storing deep-dive research"));
    assert!(out.stdout.contains("Planning Documents"));
    assert!(out.stdout.contains("topic.md"));
}

#[test]
fn plan_init_creates_structure() {
    let ctx = setup();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let out = run_rust_candidate(
        rust_path,
        &["plan", "init"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stderr.contains("Planning workspace available"));
    assert!(
        out.stdout
            .contains(&ctx.repo.path().join(".ito/planning").display().to_string())
    );
    assert!(
        out.stdout
            .contains(&ctx.repo.path().join(".ito/research").display().to_string())
    );
    assert!(ctx.repo.path().join(".ito/research").is_dir());
    assert!(!ctx.repo.path().join(".ito/planning/PROJECT.md").exists());
    assert!(!ctx.repo.path().join(".ito/planning/ROADMAP.md").exists());
    assert!(!ctx.repo.path().join(".ito/planning/STATE.md").exists());
}

#[test]
fn plan_init_prints_configured_workspace_paths() {
    let ctx = setup();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    std::fs::write(
        ctx.repo.path().join("ito.json"),
        r#"{"projectPath":".custom-ito"}"#,
    )
    .unwrap();
    std::fs::create_dir_all(ctx.repo.path().join(".custom-ito")).unwrap();

    let out = run_rust_candidate(
        rust_path,
        &["plan", "init"],
        ctx.repo.path(),
        ctx.home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(
        out.stdout.contains(
            &ctx.repo
                .path()
                .join(".custom-ito/planning")
                .display()
                .to_string()
        ),
        "stdout={}",
        out.stdout
    );
    assert!(
        out.stdout.contains(
            &ctx.repo
                .path()
                .join(".custom-ito/research")
                .display()
                .to_string()
        ),
        "stdout={}",
        out.stdout
    );
}

#[test]
fn plan_init_is_idempotent() {
    let ctx = setup();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let out = run_rust_candidate(
        rust_path,
        &["plan", "init"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let plan_dir = ctx.repo.path().join(".ito/planning");
    std::fs::write(plan_dir.join("topic.md"), "# Topic\n").unwrap();

    let out = run_rust_candidate(
        rust_path,
        &["plan", "init"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert_eq!(
        std::fs::read_to_string(plan_dir.join("topic.md")).unwrap(),
        "# Topic\n"
    );
    assert!(!plan_dir.join("PROJECT.md").exists());
    assert!(!plan_dir.join("ROADMAP.md").exists());
    assert!(!plan_dir.join("STATE.md").exists());
}

#[test]
fn plan_init_reports_conflicting_planning_file() {
    let ctx = setup();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    std::fs::write(ctx.repo.path().join(".ito/planning"), "not a directory\n").unwrap();

    let out = run_rust_candidate(
        rust_path,
        &["plan", "init"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Could not create planning workspace"));
    assert!(out.stderr.contains("exists but is not a directory"));
    assert!(out.stderr.contains("Rename or remove it"));
    assert!(
        !out.stderr
            .contains("Check directory permissions and disk space")
    );
}

#[test]
fn plan_init_warns_about_conflicting_research_file() {
    let ctx = setup();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    std::fs::write(ctx.repo.path().join(".ito/research"), "not a directory\n").unwrap();

    let out = run_rust_candidate(
        rust_path,
        &["plan", "init"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stderr.contains("Planning workspace available"));
    assert!(out.stderr.contains("exists but is not a directory"));
    assert!(out.stderr.contains("before storing deep-dive research"));
    assert!(out.stdout.contains("supporting research can live"));
    assert!(ctx.repo.path().join(".ito/planning").is_dir());
}

#[test]
fn plan_status_succeeds_after_init() {
    let ctx = setup();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let out = run_rust_candidate(
        rust_path,
        &["plan", "init"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let out = run_rust_candidate(
        rust_path,
        &["plan", "status"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Planning Workspace"));
    assert!(out.stdout.contains("Planning: available"));
    assert!(out.stdout.contains("No planning documents yet"));
    assert!(out.stdout.contains("ito-proposal"));
}

#[test]
fn plan_status_lists_markdown_documents() {
    let ctx = setup();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    std::fs::create_dir_all(ctx.repo.path().join(".ito/planning")).unwrap();
    std::fs::create_dir_all(ctx.repo.path().join(".ito/research")).unwrap();
    std::fs::write(ctx.repo.path().join(".ito/planning/topic.md"), "# Topic\n").unwrap();
    std::fs::write(
        ctx.repo.path().join(".ito/planning/notes.markdown"),
        "# Notes\n",
    )
    .unwrap();
    std::fs::write(ctx.repo.path().join(".ito/planning/notes.txt"), "ignore\n").unwrap();

    let out = run_rust_candidate(
        rust_path,
        &["plan", "status"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Research: available"));
    assert!(out.stdout.contains("topic.md"));
    assert!(out.stdout.contains("notes.markdown"));
    assert!(!out.stdout.contains("notes.txt"));
}
