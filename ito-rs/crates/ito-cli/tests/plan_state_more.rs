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
fn plan_status_fails_without_roadmap() {
    let ctx = setup();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let out = run_rust_candidate(
        rust_path,
        &["plan", "status"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("ROADMAP.md not found"));
}

#[test]
fn state_show_fails_without_state() {
    let ctx = setup();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let out = run_rust_candidate(
        rust_path,
        &["state", "show"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("STATE.md not found"));
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
    assert!(out.stderr.contains("Planning structure initialized"));
    assert!(out.stdout.contains(".ito/planning/STATE.md"));
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
    assert!(out.stdout.contains("Current Progress"));
    assert!(out.stdout.contains("Phases"));
}

#[test]
fn state_note_writes_to_state_after_init() {
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
        &["state", "note", "hello"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stderr.contains("✔ Note recorded"));

    let out = run_rust_candidate(
        rust_path,
        &["state", "show"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("hello"));
}

#[test]
fn state_other_mutations_succeed_after_init() {
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
        &["state", "decision", "ship"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stderr.contains("✔ Decision recorded"));

    let out = run_rust_candidate(
        rust_path,
        &["state", "blocker", "waiting"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stderr.contains("✔ Blocker recorded"));

    let out = run_rust_candidate(
        rust_path,
        &["state", "question", "why"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stderr.contains("✔ Question added"));

    let out = run_rust_candidate(
        rust_path,
        &["state", "focus", "now"],
        ctx.repo.path(),
        ctx.home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stderr.contains("✔ Focus updated"));
}
