use std::fs;
use std::path::Path;

use tempfile::TempDir;

use super::TaskRepository;
use ito_domain::tasks::TaskRepository as DomainTaskRepository;

fn setup_test_change(ito_dir: &Path, change_id: &str, tasks_content: &str) {
    let change_dir = ito_dir.join("changes").join(change_id);
    fs::create_dir_all(&change_dir).unwrap();
    fs::write(change_dir.join("tasks.md"), tasks_content).unwrap();
}

#[test]
fn load_tasks_uses_schema_apply_tracks_when_set() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    let ito_path = root.join(".ito");
    fs::create_dir_all(&ito_path).unwrap();

    // Override the project schema to point tracking at todo.md.
    let schema_dir = root
        .join(".ito")
        .join("templates")
        .join("schemas")
        .join("spec-driven");
    fs::create_dir_all(&schema_dir).unwrap();
    fs::write(
        schema_dir.join("schema.yaml"),
        "name: spec-driven\nversion: 1\nartifacts: []\napply:\n  tracks: todo.md\n",
    )
    .unwrap();

    let change_id = "001-03_tracks";
    let change_dir = ito_path.join("changes").join(change_id);
    fs::create_dir_all(&change_dir).unwrap();
    fs::write(change_dir.join(".ito.yaml"), "schema: spec-driven\n").unwrap();
    fs::write(
        change_dir.join("todo.md"),
        "## Tasks\n- [x] one\n- [ ] two\n",
    )
    .unwrap();

    let repo = TaskRepository::new(&ito_path);
    let (completed, total) = repo.get_task_counts(change_id).unwrap();

    assert_eq!(completed, 1);
    assert_eq!(total, 2);
}

#[test]
fn test_get_task_counts_checkbox_format() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    fs::create_dir_all(&ito_path).unwrap();

    setup_test_change(
        &ito_path,
        "001-01_test",
        r#"# Tasks

- [x] Task 1
- [x] Task 2
- [ ] Task 3
- [ ] Task 4
"#,
    );

    let repo = TaskRepository::new(&ito_path);
    let (completed, total) = repo.get_task_counts("001-01_test").unwrap();

    assert_eq!(completed, 2);
    assert_eq!(total, 4);
}

#[test]
fn test_get_task_counts_enhanced_format() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    fs::create_dir_all(&ito_path).unwrap();

    setup_test_change(
        &ito_path,
        "001-02_enhanced",
        r#"# Tasks

## Wave 1
- **Depends On**: None

### Task 1.1: First task
- **Status**: [x] complete
- **Updated At**: 2024-01-01

### Task 1.2: Second task
- **Status**: [ ] pending
- **Updated At**: 2024-01-01

### Task 1.3: Third task
- **Status**: [x] complete
- **Updated At**: 2024-01-01
"#,
    );

    let repo = TaskRepository::new(&ito_path);
    let (completed, total) = repo.get_task_counts("001-02_enhanced").unwrap();

    assert_eq!(completed, 2);
    assert_eq!(total, 3);
}

#[test]
fn test_missing_tasks_file_returns_zero() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    fs::create_dir_all(&ito_path).unwrap();

    let repo = TaskRepository::new(&ito_path);
    let (completed, total) = repo.get_task_counts("nonexistent").unwrap();

    assert_eq!(completed, 0);
    assert_eq!(total, 0);
}

#[test]
fn test_has_tasks() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    fs::create_dir_all(&ito_path).unwrap();

    setup_test_change(&ito_path, "001-01_with-tasks", "# Tasks\n- [ ] Task 1\n");
    setup_test_change(&ito_path, "001-02_no-tasks", "# Tasks\n\nNo tasks yet.\n");

    let repo = TaskRepository::new(&ito_path);

    assert!(repo.has_tasks("001-01_with-tasks").unwrap());
    assert!(!repo.has_tasks("001-02_no-tasks").unwrap());
    assert!(!repo.has_tasks("nonexistent").unwrap());
}
