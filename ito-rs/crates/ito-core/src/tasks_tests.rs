use std::path::Path;

use crate::change_repository::FsChangeRepository;

use super::list_ready_tasks_across_changes;

fn write(path: impl AsRef<Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("parent dirs should exist");
    }
    std::fs::write(path, contents).expect("test fixture should write");
}

fn make_ready_change(root: &Path, id: &str) {
    write(
        root.join(".ito/changes").join(id).join("proposal.md"),
        "## Why\nfixture\n\n## What Changes\n- fixture\n\n## Impact\n- fixture\n",
    );
    write(
        root.join(".ito/changes")
            .join(id)
            .join("specs")
            .join("alpha")
            .join("spec.md"),
        "## ADDED Requirements\n\n### Requirement: Fixture\nFixture requirement.\n\n#### Scenario: Works\n- **WHEN** fixture runs\n- **THEN** it is ready\n",
    );
    write(
        root.join(".ito/changes").join(id).join("tasks.md"),
        "## 1. Implementation\n- [ ] 1.1 pending\n",
    );
}

fn make_complete_change(root: &Path, id: &str) {
    write(
        root.join(".ito/changes").join(id).join("proposal.md"),
        "## Why\nfixture\n\n## What Changes\n- fixture\n\n## Impact\n- fixture\n",
    );
    write(
        root.join(".ito/changes")
            .join(id)
            .join("specs")
            .join("alpha")
            .join("spec.md"),
        "## ADDED Requirements\n\n### Requirement: Fixture\nFixture requirement.\n\n#### Scenario: Works\n- **WHEN** fixture runs\n- **THEN** it is ready\n",
    );
    write(
        root.join(".ito/changes").join(id).join("tasks.md"),
        "## 1. Implementation\n- [x] 1.1 done\n",
    );
}

#[test]
fn returns_ready_tasks_for_ready_changes() {
    let repo = tempfile::tempdir().expect("repo tempdir");
    let ito_path = repo.path().join(".ito");
    make_ready_change(repo.path(), "000-01_alpha");
    make_complete_change(repo.path(), "000-02_beta");

    let change_repo = FsChangeRepository::new(&ito_path);
    let ready =
        list_ready_tasks_across_changes(&change_repo, &ito_path).expect("ready task listing");

    assert_eq!(ready.len(), 1);
    assert_eq!(ready[0].change_id, "000-01_alpha");
    assert_eq!(ready[0].ready_tasks.len(), 1);
    assert_eq!(ready[0].ready_tasks[0].id, "1.1");
}

#[test]
fn returns_empty_when_no_ready_tasks_exist() {
    let repo = tempfile::tempdir().expect("repo tempdir");
    let ito_path = repo.path().join(".ito");
    make_complete_change(repo.path(), "000-01_alpha");

    let change_repo = FsChangeRepository::new(&ito_path);
    let ready =
        list_ready_tasks_across_changes(&change_repo, &ito_path).expect("ready task listing");

    assert!(ready.is_empty());
}

#[test]
fn read_tasks_markdown_returns_contents_for_existing_file() {
    let repo = tempfile::tempdir().expect("repo tempdir");
    let ito_path = repo.path().join(".ito");
    let change_id = "000-01_alpha";
    let tasks_content = "## 1. Implementation\n- [ ] 1.1 pending\n";
    write(
        ito_path.join("changes").join(change_id).join("tasks.md"),
        tasks_content,
    );

    let result = super::read_tasks_markdown(&ito_path, change_id).expect("should read tasks.md");
    assert_eq!(result, tasks_content);
}

#[test]
fn read_tasks_markdown_returns_error_for_missing_file() {
    let repo = tempfile::tempdir().expect("repo tempdir");
    let ito_path = repo.path().join(".ito");

    let result = super::read_tasks_markdown(&ito_path, "nonexistent-change");
    assert!(result.is_err(), "should fail for missing tasks.md");
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("tasks.md"),
        "error should mention tasks.md, got: {msg}"
    );
}

#[test]
fn read_tasks_markdown_rejects_traversal_like_change_id() {
    let repo = tempfile::tempdir().expect("repo tempdir");
    let ito_path = repo.path().join(".ito");

    let result = super::read_tasks_markdown(&ito_path, "../escape");
    assert!(result.is_err(), "traversal-like ids should fail");
}
