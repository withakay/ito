use std::fs;
use std::path::Path;

use ito_config::types::{CoordinationBranchConfig, CoordinationBranchEnabled, CoordinationStorage};
use tempfile::TempDir;

use super::{LegacyCoordinationClass, ManagedPathKind, inspect_legacy_coordination};
use crate::coordination::{COORDINATION_DIRS, create_dir_link};

fn config(enabled: bool, storage: CoordinationStorage) -> CoordinationBranchConfig {
    CoordinationBranchConfig {
        enabled: CoordinationBranchEnabled(enabled),
        storage,
        ..CoordinationBranchConfig::default()
    }
}

fn roots() -> (TempDir, std::path::PathBuf, std::path::PathBuf) {
    let temp = TempDir::new().expect("tempdir");
    let project = temp.path().join("project");
    let ito = project.join(".ito");
    fs::create_dir_all(&ito).expect("ito root");
    (temp, project, ito)
}

fn create_embedded_dirs(ito: &Path) {
    for name in COORDINATION_DIRS {
        fs::create_dir_all(ito.join(name)).expect("managed directory");
    }
}

#[test]
fn configured_worktree_storage_is_legacy_even_without_links() {
    let (_temp, project, ito) = roots();
    let report = inspect_legacy_coordination(
        &project,
        &ito,
        &config(true, CoordinationStorage::Worktree),
        None,
    )
    .expect("inspection");

    assert_eq!(report.classification, LegacyCoordinationClass::Legacy);
    assert!(report.config.enabled);
    assert_eq!(report.config.storage, "worktree");
}

#[test]
fn real_directories_with_embedded_storage_are_main_compatible() {
    let (_temp, project, ito) = roots();
    create_embedded_dirs(&ito);

    let report = inspect_legacy_coordination(
        &project,
        &ito,
        &config(true, CoordinationStorage::Embedded),
        None,
    )
    .expect("inspection");

    assert_eq!(report.classification, LegacyCoordinationClass::Embedded);
    assert_eq!(report.managed_paths.len(), COORDINATION_DIRS.len());
    assert!(
        report
            .managed_paths
            .iter()
            .all(|evidence| matches!(evidence.kind, ManagedPathKind::Directory { empty: true }))
    );
}

#[test]
fn disabled_storage_with_no_managed_paths_is_absent() {
    let (_temp, project, ito) = roots();

    let report = inspect_legacy_coordination(
        &project,
        &ito,
        &config(false, CoordinationStorage::Worktree),
        None,
    )
    .expect("inspection");

    assert_eq!(report.classification, LegacyCoordinationClass::Absent);
}

#[test]
fn expected_coordination_links_are_legacy() {
    let (_temp, project, ito) = roots();
    let coordination_ito = project.join("coordination").join(".ito");
    create_embedded_dirs(&coordination_ito);
    for name in COORDINATION_DIRS {
        create_dir_link(&coordination_ito.join(name), &ito.join(name)).expect("link");
    }

    let report = inspect_legacy_coordination(
        &project,
        &ito,
        &config(true, CoordinationStorage::Worktree),
        Some(&coordination_ito),
    )
    .expect("inspection");

    assert_eq!(report.classification, LegacyCoordinationClass::Legacy);
    assert!(report.managed_paths.iter().all(|evidence| matches!(
        evidence.kind,
        ManagedPathKind::Link {
            matches_expected: Some(true),
            target_exists: true,
            ..
        }
    )));
}

#[cfg(unix)]
#[test]
fn broken_expected_link_is_still_legacy_evidence() {
    let (_temp, project, ito) = roots();
    let coordination_ito = project.join("coordination").join(".ito");
    let expected = coordination_ito.join("changes");
    create_dir_link(&expected, &ito.join("changes")).expect("broken link");

    let report = inspect_legacy_coordination(
        &project,
        &ito,
        &config(true, CoordinationStorage::Worktree),
        Some(&coordination_ito),
    )
    .expect("inspection");

    assert_eq!(report.classification, LegacyCoordinationClass::Legacy);
    assert!(matches!(
        report.managed_paths[0].kind,
        ManagedPathKind::Link {
            matches_expected: Some(true),
            target_exists: false,
            ..
        }
    ));
}

#[cfg(unix)]
#[test]
fn wrong_link_target_is_ambiguous() {
    let (_temp, project, ito) = roots();
    let coordination_ito = project.join("coordination").join(".ito");
    let wrong = project.join("wrong");
    fs::create_dir_all(&wrong).expect("wrong target");
    create_dir_link(&wrong, &ito.join("changes")).expect("wrong link");

    let report = inspect_legacy_coordination(
        &project,
        &ito,
        &config(true, CoordinationStorage::Worktree),
        Some(&coordination_ito),
    )
    .expect("inspection");

    assert_eq!(report.classification, LegacyCoordinationClass::Ambiguous);
}

#[cfg(unix)]
#[test]
fn inconsistent_link_roots_are_ambiguous_without_an_expected_root() {
    let (_temp, project, ito) = roots();
    let first_root = project.join("first/.ito");
    let second_root = project.join("second/.ito");
    fs::create_dir_all(first_root.join("changes")).expect("first target");
    fs::create_dir_all(second_root.join("specs")).expect("second target");
    create_dir_link(&first_root.join("changes"), &ito.join("changes")).expect("first link");
    create_dir_link(&second_root.join("specs"), &ito.join("specs")).expect("second link");

    let report = inspect_legacy_coordination(
        &project,
        &ito,
        &config(true, CoordinationStorage::Worktree),
        None,
    )
    .expect("inspection");

    assert_eq!(report.classification, LegacyCoordinationClass::Ambiguous);
}

#[test]
fn mixed_link_and_non_empty_real_directory_is_ambiguous() {
    let (_temp, project, ito) = roots();
    let coordination_ito = project.join("coordination").join(".ito");
    create_embedded_dirs(&coordination_ito);
    create_dir_link(&coordination_ito.join("changes"), &ito.join("changes")).expect("link");
    fs::create_dir_all(ito.join("specs")).expect("specs");
    fs::write(ito.join("specs/spec.md"), "conflicting").expect("conflict");

    let report = inspect_legacy_coordination(
        &project,
        &ito,
        &config(true, CoordinationStorage::Worktree),
        Some(&coordination_ito),
    )
    .expect("inspection");

    assert_eq!(report.classification, LegacyCoordinationClass::Ambiguous);
}

#[test]
fn residual_managed_gitignore_marker_is_ambiguous_after_materialization() {
    let (_temp, project, ito) = roots();
    create_embedded_dirs(&ito);
    fs::write(
        project.join(".gitignore"),
        "# Ito coordination worktree symlinks\n.ito/changes\n.ito/specs\n.ito/modules\n.ito/workflows\n.ito/audit\n",
    )
    .expect("gitignore");

    let report = inspect_legacy_coordination(
        &project,
        &ito,
        &config(true, CoordinationStorage::Embedded),
        None,
    )
    .expect("inspection");

    assert_eq!(report.classification, LegacyCoordinationClass::Ambiguous);
    assert!(report.gitignore.marker_present);
    assert_eq!(report.gitignore.matching_entries.len(), 5);
}

#[test]
fn partial_real_directory_materialization_is_ambiguous() {
    let (_temp, project, ito) = roots();
    fs::create_dir_all(ito.join("changes")).expect("partial materialization");

    let report = inspect_legacy_coordination(
        &project,
        &ito,
        &config(false, CoordinationStorage::Embedded),
        None,
    )
    .expect("inspection");

    assert_eq!(report.classification, LegacyCoordinationClass::Ambiguous);
}

#[test]
fn partial_coordination_gitignore_entries_are_ambiguous() {
    let (_temp, project, ito) = roots();
    create_embedded_dirs(&ito);
    fs::write(project.join(".gitignore"), ".ito/changes\n.ito/specs\n")
        .expect("partial legacy ignores");

    let report = inspect_legacy_coordination(
        &project,
        &ito,
        &config(false, CoordinationStorage::Embedded),
        None,
    )
    .expect("inspection");

    assert_eq!(report.classification, LegacyCoordinationClass::Ambiguous);
    assert!(!report.gitignore.marker_present);
    assert_eq!(report.gitignore.matching_entries.len(), 2);
}

#[test]
fn worktree_config_with_materialized_directories_is_ambiguous() {
    let (_temp, project, ito) = roots();
    create_embedded_dirs(&ito);

    let report = inspect_legacy_coordination(
        &project,
        &ito,
        &config(true, CoordinationStorage::Worktree),
        None,
    )
    .expect("inspection");

    assert_eq!(report.classification, LegacyCoordinationClass::Ambiguous);
}

#[test]
fn inspection_does_not_change_files_or_links() {
    let (_temp, project, ito) = roots();
    let coordination_ito = project.join("coordination").join(".ito");
    create_embedded_dirs(&coordination_ito);
    create_dir_link(&coordination_ito.join("changes"), &ito.join("changes")).expect("link");
    let gitignore = "# Ito coordination worktree symlinks\n.ito/changes\n";
    fs::write(project.join(".gitignore"), gitignore).expect("gitignore");
    let before_target = fs::read_link(ito.join("changes")).expect("target before");

    let _report = inspect_legacy_coordination(
        &project,
        &ito,
        &config(true, CoordinationStorage::Worktree),
        Some(&coordination_ito),
    )
    .expect("inspection");

    assert_eq!(
        fs::read_link(ito.join("changes")).expect("target after"),
        before_target
    );
    assert_eq!(
        fs::read_to_string(project.join(".gitignore")).expect("gitignore after"),
        gitignore
    );
}

#[test]
fn classification_serialization_has_one_stable_tagged_shape() {
    let cases = [
        (
            LegacyCoordinationClass::Absent,
            serde_json::json!({"kind": "absent"}),
        ),
        (
            LegacyCoordinationClass::Embedded,
            serde_json::json!({"kind": "embedded"}),
        ),
        (
            LegacyCoordinationClass::Legacy,
            serde_json::json!({"kind": "legacy"}),
        ),
        (
            LegacyCoordinationClass::Ambiguous,
            serde_json::json!({"kind": "ambiguous"}),
        ),
    ];

    for (classification, expected) in cases {
        assert_eq!(
            serde_json::to_value(classification).expect("serialize classification"),
            expected
        );
    }
}
