use std::path::{Path, PathBuf};

use ito_core::change_repository::FsChangeRepository;
use ito_core::grep::{GrepInput, GrepScope, grep};
use ito_core::module_repository::FsModuleRepository;
use tempfile::TempDir;

fn write(path: impl AsRef<Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, contents).unwrap();
}

fn setup_repo() -> (TempDir, PathBuf) {
    let td = tempfile::tempdir().unwrap();
    let ito_path = td.path().join(".ito");

    // Minimal module definition for module-scoped grep.
    write(
        ito_path.join("modules/024_test/module.md"),
        "# Backend\n\n## Purpose\nThis purpose text is intentionally long enough to avoid strict-mode warnings.\n\n## Scope\n- *\n\n## Changes\n- [ ] 024-01_first-change\n- [ ] 024-02_second-change\n",
    );

    // Change 024-01
    write(
        ito_path.join("changes/024-01_first-change/proposal.md"),
        "# Proposal\n\nNeedle: alpha\n",
    );
    write(
        ito_path.join("changes/024-01_first-change/tasks.md"),
        "# Tasks\n- [ ] 1.1 Do thing\n",
    );

    // Change 024-02
    write(
        ito_path.join("changes/024-02_second-change/proposal.md"),
        "# Proposal\n\nNeedle: beta\n",
    );
    write(
        ito_path.join("changes/024-02_second-change/tasks.md"),
        "# Tasks\n- [ ] 1.1 Do thing\n",
    );

    // Change 025-01 (different module)
    write(
        ito_path.join("changes/025-01_third-change/proposal.md"),
        "# Proposal\n\nNeedle: gamma\n",
    );
    write(
        ito_path.join("changes/025-01_third-change/tasks.md"),
        "# Tasks\n- [ ] 1.1 Do thing\n",
    );

    (td, ito_path)
}

#[test]
fn grep_scope_change_only_searches_one_change() {
    let (_td, ito_path) = setup_repo();
    let change_repo = FsChangeRepository::new(&ito_path);
    let module_repo = FsModuleRepository::new(&ito_path);

    let out = grep(
        &ito_path,
        &GrepInput {
            pattern: "Needle:".to_string(),
            scope: GrepScope::Change("024-01_first-change".to_string()),
            limit: 0,
        },
        &change_repo,
        &module_repo,
    )
    .unwrap();

    assert_eq!(out.matches.len(), 1);
    let p = out.matches[0].path.to_string_lossy();
    assert!(p.contains("024-01_first-change"), "path: {p}");
    assert!(!p.contains("024-02_second-change"), "path: {p}");
    assert!(!p.contains("025-01_third-change"), "path: {p}");
}

#[test]
fn grep_scope_module_searches_all_changes_in_module() {
    let (_td, ito_path) = setup_repo();
    let change_repo = FsChangeRepository::new(&ito_path);
    let module_repo = FsModuleRepository::new(&ito_path);

    let out = grep(
        &ito_path,
        &GrepInput {
            pattern: "Needle:".to_string(),
            scope: GrepScope::Module("024".to_string()),
            limit: 0,
        },
        &change_repo,
        &module_repo,
    )
    .unwrap();

    let mut paths: Vec<String> = Vec::new();
    for m in &out.matches {
        paths.push(m.path.to_string_lossy().to_string());
    }

    assert!(paths.iter().any(|p| p.contains("024-01_first-change")));
    assert!(paths.iter().any(|p| p.contains("024-02_second-change")));
    assert!(
        !paths.iter().any(|p| p.contains("025-01_third-change")),
        "unexpected paths: {paths:?}"
    );
}

#[test]
fn grep_scope_all_searches_all_changes() {
    let (_td, ito_path) = setup_repo();
    let change_repo = FsChangeRepository::new(&ito_path);
    let module_repo = FsModuleRepository::new(&ito_path);

    let out = grep(
        &ito_path,
        &GrepInput {
            pattern: "Needle:".to_string(),
            scope: GrepScope::All,
            limit: 0,
        },
        &change_repo,
        &module_repo,
    )
    .unwrap();

    let mut paths: Vec<String> = Vec::new();
    for m in &out.matches {
        paths.push(m.path.to_string_lossy().to_string());
    }

    assert!(paths.iter().any(|p| p.contains("024-01_first-change")));
    assert!(paths.iter().any(|p| p.contains("024-02_second-change")));
    assert!(paths.iter().any(|p| p.contains("025-01_third-change")));
}

#[test]
fn grep_respects_limit_across_scopes() {
    let (_td, ito_path) = setup_repo();
    let change_repo = FsChangeRepository::new(&ito_path);
    let module_repo = FsModuleRepository::new(&ito_path);

    let out = grep(
        &ito_path,
        &GrepInput {
            pattern: "Needle:".to_string(),
            scope: GrepScope::All,
            limit: 2,
        },
        &change_repo,
        &module_repo,
    )
    .unwrap();

    assert_eq!(out.matches.len(), 2);
    assert!(out.truncated);
}
