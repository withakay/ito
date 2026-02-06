//! Task-oriented orchestration use-cases for adapters.

use std::path::Path;

use miette::Result;

use crate::change_repository::FsChangeRepository;
use crate::error_bridge::IntoCoreMiette;
use ito_domain::tasks::{
    TaskItem, compute_ready_and_blocked, parse_tasks_tracking_file, tasks_path,
};

/// Ready task list for a single change.
#[derive(Debug, Clone)]
pub struct ReadyTasksForChange {
    /// Canonical change id.
    pub change_id: String,
    /// Ready tasks from `tasks.md` after dependency computation.
    pub ready_tasks: Vec<TaskItem>,
}

/// Collect ready tasks across all currently ready changes.
///
/// This use-case keeps repository traversal and task orchestration in core,
/// while adapters remain focused on argument parsing and presentation.
pub fn list_ready_tasks_across_changes(ito_path: &Path) -> Result<Vec<ReadyTasksForChange>> {
    let change_repo = FsChangeRepository::new(ito_path);
    let summaries = change_repo.list().into_core_miette()?;

    let mut results: Vec<ReadyTasksForChange> = Vec::new();
    for summary in &summaries {
        if !summary.is_ready() {
            continue;
        }

        let path = tasks_path(ito_path, &summary.id);
        let Ok(contents) = ito_common::io::read_to_string(&path) else {
            continue;
        };

        let parsed = parse_tasks_tracking_file(&contents);
        if parsed
            .diagnostics
            .iter()
            .any(|d| d.level == ito_domain::tasks::DiagnosticLevel::Error)
        {
            continue;
        }

        let (ready, _blocked) = compute_ready_and_blocked(&parsed);
        if ready.is_empty() {
            continue;
        }

        results.push(ReadyTasksForChange {
            change_id: summary.id.clone(),
            ready_tasks: ready,
        });
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

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

        let ready = list_ready_tasks_across_changes(&ito_path).expect("ready task listing");

        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].change_id, "000-01_alpha");
        assert_eq!(ready[0].ready_tasks.len(), 1);
        assert_eq!(ready[0].ready_tasks[0].id, "1");
    }

    #[test]
    fn returns_empty_when_no_ready_tasks_exist() {
        let repo = tempfile::tempdir().expect("repo tempdir");
        let ito_path = repo.path().join(".ito");
        make_complete_change(repo.path(), "000-01_alpha");

        let ready = list_ready_tasks_across_changes(&ito_path).expect("ready task listing");

        assert!(ready.is_empty());
    }
}
