#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

use crate::fixtures::{installed_specialist_asset_paths, obsolete_specialist_asset_paths};

const COORDINATOR_PATHS: &[&str] = &[
    ".opencode/agent/ito-orchestrator.md",
    ".claude/agents/ito-orchestrator.md",
    ".github/agents/ito-orchestrator.md",
    ".pi/agents/ito-orchestrator.md",
    ".agents/skills/ito-orchestrator/SKILL.md",
];

#[test]
fn init_update_with_tools_all_removes_obsolete_specialist_orchestrator_assets() {
    assert_specialist_cleanup(&["--update"]);
}

#[test]
fn init_force_with_tools_all_removes_obsolete_specialist_orchestrator_assets() {
    assert_specialist_cleanup(&["--force"]);
}

fn assert_specialist_cleanup(extra_args: &[&str]) {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let obsolete = obsolete_specialist_asset_paths();
    for rel in &obsolete {
        fixtures::write(repo.path().join(rel), "obsolete specialist asset\n");
    }

    let repo_path = repo.path().to_string_lossy();
    let mut argv = vec!["init", repo_path.as_ref(), "--tools", "all"];
    argv.extend_from_slice(extra_args);
    let out = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    for rel in &obsolete {
        assert!(
            !repo.path().join(rel).exists(),
            "expected obsolete specialist asset {rel} to be removed"
        );
    }

    for rel in installed_specialist_asset_paths() {
        assert!(repo.path().join(&rel).exists(), "expected {rel} to install");
    }

    for rel in COORDINATOR_PATHS {
        assert!(
            repo.path().join(rel).exists(),
            "expected coordinator asset {rel} to remain installed"
        );
    }
}
