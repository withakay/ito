use super::*;
use std::fs;
use tempfile::TempDir;

fn setup_test_ito(tmp: &TempDir) -> std::path::PathBuf {
    let ito_path = tmp.path().join(".ito");
    fs::create_dir_all(ito_path.join("changes")).unwrap();
    ito_path
}

fn create_change(ito_path: &Path, id: &str, with_tasks: bool) {
    let change_dir = ito_path.join("changes").join(id);
    fs::create_dir_all(&change_dir).unwrap();
    fs::write(change_dir.join("proposal.md"), "# Proposal\n").unwrap();
    fs::write(change_dir.join("design.md"), "# Design\n").unwrap();

    let specs_dir = change_dir.join("specs").join("test-spec");
    fs::create_dir_all(&specs_dir).unwrap();
    fs::write(specs_dir.join("spec.md"), "## Requirements\n").unwrap();

    if with_tasks {
        fs::write(
            change_dir.join("tasks.md"),
            "# Tasks\n- [x] Task 1\n- [ ] Task 2\n",
        )
        .unwrap();
    }
}

fn create_archived_change(ito_path: &Path, id: &str) {
    let archive_dir = ito_path.join("changes").join("archive").join(id);
    fs::create_dir_all(&archive_dir).unwrap();
    fs::write(archive_dir.join("proposal.md"), "# Archived\n").unwrap();
}

#[test]
fn exists_and_get_work() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_test_ito(&tmp);
    create_change(&ito_path, "005-01_test", true);

    let repo = FsChangeRepository::new(&ito_path);
    assert!(repo.exists("005-01_test"));
    assert!(!repo.exists("999-99_missing"));

    let change = repo.get("005-01_test").unwrap();
    assert_eq!(change.id, "005-01_test");
    assert_eq!(change.task_progress(), (1, 2));
    assert!(change.proposal.is_some());
    assert!(change.design.is_some());
    assert_eq!(change.specs.len(), 1);
}

#[test]
fn list_skips_archive_dir() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_test_ito(&tmp);
    create_change(&ito_path, "005-01_first", true);
    create_archived_change(&ito_path, "005-99_old");

    let repo = FsChangeRepository::new(&ito_path);
    let changes = repo.list().unwrap();

    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].id, "005-01_first");
}

#[test]
fn resolve_target_reports_ambiguity() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_test_ito(&tmp);
    create_change(&ito_path, "001-12_first-change", false);
    create_change(&ito_path, "001-12_follow-up", false);

    let repo = FsChangeRepository::new(&ito_path);
    assert_eq!(
        repo.resolve_target("1-12"),
        ChangeTargetResolution::Ambiguous(vec![
            "001-12_first-change".to_string(),
            "001-12_follow-up".to_string(),
        ])
    );
}

#[test]
fn resolve_target_module_scoped_query() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_test_ito(&tmp);
    create_change(&ito_path, "001-12_setup-wizard", false);
    create_change(&ito_path, "002-12_setup-wizard", false);

    let repo = FsChangeRepository::new(&ito_path);
    assert_eq!(
        repo.resolve_target("1:setup"),
        ChangeTargetResolution::Unique("001-12_setup-wizard".to_string())
    );
    assert_eq!(
        repo.resolve_target("2:setup"),
        ChangeTargetResolution::Unique("002-12_setup-wizard".to_string())
    );
}

#[test]
fn resolve_target_includes_archive_when_requested() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_test_ito(&tmp);
    create_archived_change(&ito_path, "001-12_setup-wizard");

    let repo = FsChangeRepository::new(&ito_path);
    assert_eq!(
        repo.resolve_target("1-12"),
        ChangeTargetResolution::NotFound
    );

    assert_eq!(
        repo.resolve_target_with_options(
            "1-12",
            ResolveTargetOptions {
                lifecycle: ChangeLifecycleFilter::All,
            }
        ),
        ChangeTargetResolution::Unique("001-12_setup-wizard".to_string())
    );
}

#[test]
fn suggest_targets_prioritizes_slug_matches() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_test_ito(&tmp);
    create_change(&ito_path, "001-12_setup-wizard", false);
    create_change(&ito_path, "001-13_setup-service", false);
    create_change(&ito_path, "002-01_other-work", false);

    let repo = FsChangeRepository::new(&ito_path);
    let suggestions = repo.suggest_targets("setup", 2);
    assert_eq!(
        suggestions,
        vec!["001-12_setup-wizard", "001-13_setup-service"]
    );
}
