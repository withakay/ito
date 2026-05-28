use super::*;

fn write(path: impl AsRef<Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("parent dirs should exist");
    }
    std::fs::write(path, contents).expect("test fixture should write");
}

/// Create a minimal change fixture under the repository root for the given change id.
///
/// This writes three files for a change named `id` beneath `root/.ito/changes/<id>`:
/// - `proposal.md` with a simple proposal template,
/// - `tasks.md` containing the provided `tasks` text,
/// - `specs/alpha/spec.md` containing a small example requirement.
///
/// # Parameters
///
/// - `root`: Path to the repository root where the `.ito` directory will be created.
/// - `id`: Folder name for the change (used as the change identifier directory).
/// - `tasks`: Text to write into `tasks.md`.
///
/// # Examples
///
/// ```
/// let tmp = tempfile::tempdir().unwrap();
/// make_change(tmp.path(), "000-01_alpha", "- [ ] task1");
/// assert!(tmp.path().join(".ito/changes/000-01_alpha/tasks.md").exists());
/// ```
fn make_change(root: &Path, id: &str, tasks: &str) {
    write(
        root.join(".ito/changes").join(id).join("proposal.md"),
        "## Why\nfixture\n\n## What Changes\n- fixture\n\n## Impact\n- fixture\n",
    );
    write(root.join(".ito/changes").join(id).join("tasks.md"), tasks);
    write(
        root.join(".ito/changes")
            .join(id)
            .join("specs")
            .join("alpha")
            .join("spec.md"),
        "## ADDED Requirements\n\n### Requirement: Fixture\nFixture requirement.\n\n#### Scenario: Works\n- **WHEN** fixture runs\n- **THEN** it is ready\n",
    );
}

/// Recursively sets the filesystem modification time for a directory and all entries within it.
///
/// Traverses `dir`, sets the mtime of `dir` itself and every file and subdirectory it contains to `time`.
///
/// # Examples
///
/// ```
/// use std::fs::{create_dir_all, File};
/// use tempfile::tempdir;
/// use filetime::FileTime;
///
/// let td = tempdir().unwrap();
/// let nested = td.path().join("a/b");
/// create_dir_all(&nested).unwrap();
/// File::create(nested.join("f.txt")).unwrap();
/// let ft = FileTime::from_unix_time(1_600_000_000, 0);
/// set_mtime_recursive(td.path(), ft);
/// ```
fn set_mtime_recursive(dir: &Path, time: filetime::FileTime) {
    filetime::set_file_mtime(dir, time).expect("set dir mtime");
    for entry in std::fs::read_dir(dir).expect("read dir") {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        filetime::set_file_mtime(&path, time).expect("set entry mtime");
        if path.is_dir() {
            set_mtime_recursive(&path, time);
        }
    }
}

#[test]
fn counts_requirements_from_headings() {
    let md = r#"
# Title

## Purpose
blah

## Requirements

### Requirement: One
foo

### Requirement: Two
bar
"#;
    assert_eq!(count_requirements_in_spec_markdown(md), 2);
}

#[test]
fn iso_millis_matches_expected_shape() {
    let dt = DateTime::parse_from_rfc3339("2026-01-26T00:00:00.123Z")
        .unwrap()
        .with_timezone(&Utc);
    assert_eq!(to_iso_millis(dt), "2026-01-26T00:00:00.123Z");
}

#[test]
fn parse_modular_change_module_id_allows_overflow_change_numbers() {
    assert_eq!(parse_modular_change_module_id("001-02_foo"), Some("001"));
    assert_eq!(parse_modular_change_module_id("001-100_foo"), Some("001"));
    assert_eq!(parse_modular_change_module_id("001-1234_foo"), Some("001"));
}

#[test]
fn list_changes_filters_by_progress_status() {
    let repo = tempfile::tempdir().expect("repo tempdir");
    let ito_path = repo.path().join(".ito");
    make_change(
        repo.path(),
        "000-01_pending",
        "## 1. Implementation\n- [ ] 1.1 todo\n",
    );
    make_change(
        repo.path(),
        "000-02_partial",
        "## 1. Implementation\n- [x] 1.1 done\n- [ ] 1.2 todo\n",
    );
    make_change(
        repo.path(),
        "000-03_completed",
        "## 1. Implementation\n- [x] 1.1 done\n",
    );

    let change_repo = crate::change_repository::FsChangeRepository::new(&ito_path);

    let ready = list_changes(
        &change_repo,
        ListChangesInput {
            progress_filter: ChangeProgressFilter::Ready,
            sort: ChangeSortOrder::Name,
        },
    )
    .expect("ready list should succeed");
    assert_eq!(ready.len(), 2);
    assert_eq!(ready[0].name, "000-01_pending");
    assert_eq!(ready[1].name, "000-02_partial");

    let pending = list_changes(
        &change_repo,
        ListChangesInput {
            progress_filter: ChangeProgressFilter::Pending,
            sort: ChangeSortOrder::Name,
        },
    )
    .expect("pending list should succeed");
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].name, "000-01_pending");

    let partial = list_changes(
        &change_repo,
        ListChangesInput {
            progress_filter: ChangeProgressFilter::Partial,
            sort: ChangeSortOrder::Name,
        },
    )
    .expect("partial list should succeed");
    assert_eq!(partial.len(), 1);
    assert_eq!(partial[0].name, "000-02_partial");

    let completed = list_changes(
        &change_repo,
        ListChangesInput {
            progress_filter: ChangeProgressFilter::Completed,
            sort: ChangeSortOrder::Name,
        },
    )
    .expect("completed list should succeed");
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0].name, "000-03_completed");
    assert!(completed[0].completed);
}

#[test]
fn list_changes_sorts_by_name_and_recent() {
    let repo = tempfile::tempdir().expect("repo tempdir");
    let ito_path = repo.path().join(".ito");
    make_change(
        repo.path(),
        "000-01_alpha",
        "## 1. Implementation\n- [ ] 1.1 todo\n",
    );
    make_change(
        repo.path(),
        "000-02_beta",
        "## 1. Implementation\n- [ ] 1.1 todo\n",
    );
    // Set explicit mtimes recursively so sort-by-recent is deterministic without sleeping.
    // last_modified_recursive() walks all files, so we must set mtime on every entry.
    let alpha_dir = repo.path().join(".ito/changes/000-01_alpha");
    let beta_dir = repo.path().join(".ito/changes/000-02_beta");
    let earlier = filetime::FileTime::from_unix_time(1_000_000, 0);
    let later = filetime::FileTime::from_unix_time(2_000_000, 0);
    set_mtime_recursive(&alpha_dir, earlier);
    set_mtime_recursive(&beta_dir, later);

    let change_repo = crate::change_repository::FsChangeRepository::new(&ito_path);

    let by_name = list_changes(
        &change_repo,
        ListChangesInput {
            progress_filter: ChangeProgressFilter::All,
            sort: ChangeSortOrder::Name,
        },
    )
    .expect("name sort should succeed");
    assert_eq!(by_name[0].name, "000-01_alpha");
    assert_eq!(by_name[1].name, "000-02_beta");

    let by_recent = list_changes(
        &change_repo,
        ListChangesInput {
            progress_filter: ChangeProgressFilter::All,
            sort: ChangeSortOrder::Recent,
        },
    )
    .expect("recent sort should succeed");
    assert_eq!(by_recent[0].name, "000-02_beta");
    assert_eq!(by_recent[1].name, "000-01_alpha");
}
