use super::{print_archive_follow_up, requires_local_changes_dir};
use ito_config::types::ArchiveMainIntegrationMode;

#[test]
fn only_filesystem_mode_requires_local_changes_dir() {
    assert!(requires_local_changes_dir(
        ito_core::repository_runtime::PersistenceMode::Filesystem
    ));
    assert!(!requires_local_changes_dir(
        ito_core::repository_runtime::PersistenceMode::Sqlite
    ));
    assert!(!requires_local_changes_dir(
        ito_core::repository_runtime::PersistenceMode::Remote
    ));
}

#[test]
fn archive_follow_up_messages_cover_all_modes() {
    print_archive_follow_up(
        ArchiveMainIntegrationMode::DirectMerge,
        "025-09_add-worktree-sync-command",
    );
    print_archive_follow_up(
        ArchiveMainIntegrationMode::PullRequest,
        "025-09_add-worktree-sync-command",
    );
    print_archive_follow_up(
        ArchiveMainIntegrationMode::PullRequestAutoMerge,
        "025-09_add-worktree-sync-command",
    );
    print_archive_follow_up(
        ArchiveMainIntegrationMode::CoordinationOnly,
        "025-09_add-worktree-sync-command",
    );
}
