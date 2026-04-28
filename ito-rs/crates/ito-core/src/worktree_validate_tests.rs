use super::*;
use ito_config::types::WorktreeStrategy;

fn enabled_paths(worktrees_root: &str, main_root: &str) -> ResolvedWorktreePaths {
    ResolvedWorktreePaths {
        feature: WorktreeFeature::Enabled,
        strategy: WorktreeStrategy::BareControlSiblings,
        worktrees_root: Some(PathBuf::from(worktrees_root)),
        main_worktree_root: Some(PathBuf::from(main_root)),
    }
}

fn disabled_paths() -> ResolvedWorktreePaths {
    ResolvedWorktreePaths {
        feature: WorktreeFeature::Disabled,
        strategy: WorktreeStrategy::CheckoutSubdir,
        worktrees_root: None,
        main_worktree_root: None,
    }
}

#[test]
fn worktree_validate_disabled_reports_disabled_status() {
    let validation = validate_change_worktree(
        "012-07_guard-opencode-worktree-path",
        Path::new("/repo"),
        &disabled_paths(),
        None,
    );

    assert_eq!(validation.status, WorktreeValidationStatus::Disabled);
    assert!(validation.message.contains("disabled by configuration"));
}

#[test]
fn worktree_validate_rejects_main_checkout() {
    let validation = validate_change_worktree(
        "012-07_guard-opencode-worktree-path",
        Path::new("/repo/main"),
        &enabled_paths("/repo/ito-worktrees", "/repo/main"),
        Some("main"),
    );

    assert_eq!(validation.status, WorktreeValidationStatus::MainCheckout);
    assert!(validation.message.contains("main/control worktree"));
    assert!(
        validation
            .message
            .contains("/repo/ito-worktrees/012-07_guard-opencode-worktree-path")
    );
}

#[test]
fn worktree_validate_does_not_treat_checkout_subdir_worktree_as_main() {
    let validation = validate_change_worktree(
        "012-07_guard-opencode-worktree-path",
        Path::new("/repo/.ito-worktrees/012-07_guard-opencode-worktree-path-review"),
        &ResolvedWorktreePaths {
            feature: WorktreeFeature::Enabled,
            strategy: WorktreeStrategy::CheckoutSubdir,
            worktrees_root: Some(PathBuf::from("/repo/.ito-worktrees")),
            main_worktree_root: Some(PathBuf::from("/repo")),
        },
        Some("012-07_guard-opencode-worktree-path-review"),
    );

    assert_eq!(validation.status, WorktreeValidationStatus::Ok);
}

#[test]
fn worktree_validate_accepts_same_change_suffix_path() {
    let validation = validate_change_worktree(
        "012-07_guard-opencode-worktree-path",
        Path::new("/repo/ito-worktrees/012-07_guard-opencode-worktree-path-review"),
        &enabled_paths("/repo/ito-worktrees", "/repo/main"),
        Some("012-07_guard-opencode-worktree-path-review"),
    );

    assert_eq!(validation.status, WorktreeValidationStatus::Ok);
}

#[test]
fn worktree_validate_accepts_branch_match_when_path_differs() {
    let validation = validate_change_worktree(
        "012-07_guard-opencode-worktree-path",
        Path::new("/repo/ito-worktrees/review"),
        &enabled_paths("/repo/ito-worktrees", "/repo/main"),
        Some("012-07_guard-opencode-worktree-path-review"),
    );

    assert_eq!(validation.status, WorktreeValidationStatus::Ok);
}

#[test]
fn worktree_validate_reports_mismatch_outside_main_checkout() {
    let validation = validate_change_worktree(
        "012-07_guard-opencode-worktree-path",
        Path::new("/repo/ito-worktrees/other-change"),
        &enabled_paths("/repo/ito-worktrees", "/repo/main"),
        Some("other-change"),
    );

    assert_eq!(validation.status, WorktreeValidationStatus::Mismatch);
    assert!(
        validation
            .message
            .contains("should include the full change ID")
    );
}

#[test]
fn worktree_validate_rejects_superstring_false_positive() {
    let validation = validate_change_worktree(
        "012-07_guard-opencode-worktree-path",
        Path::new("/repo/ito-worktrees/foo-012-07_guard-opencode-worktree-pathology"),
        &enabled_paths("/repo/ito-worktrees", "/repo/main"),
        Some("foo-012-07_guard-opencode-worktree-pathology"),
    );

    assert_eq!(validation.status, WorktreeValidationStatus::Mismatch);
}
