use super::*;
use std::fs;
use tempfile::TempDir;

fn make_dirs() -> (TempDir, PathBuf, PathBuf) {
    let tmp = TempDir::new().expect("tempdir");
    let ito = tmp.path().join(".ito");
    let worktree_ito = tmp.path().join("worktree").join(".ito");
    fs::create_dir_all(&ito).unwrap();
    fs::create_dir_all(&worktree_ito).unwrap();
    (tmp, ito, worktree_ito)
}

#[test]
#[cfg(unix)]
fn create_dir_link_creates_symlink() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("real_dir");
    let dst = tmp.path().join("link");
    fs::create_dir_all(&src).unwrap();

    create_dir_link(&src, &dst).expect("symlink creation should succeed");

    assert!(dst.exists(), "link path should resolve");
    let target = fs::read_link(&dst).expect("should be a symlink");
    assert_eq!(target, src);
}

#[test]
#[cfg(unix)]
fn create_dir_link_fails_when_dst_exists() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("real_dir");
    let dst = tmp.path().join("existing");
    fs::create_dir_all(&src).unwrap();
    fs::create_dir_all(&dst).unwrap();

    let result = create_dir_link(&src, &dst);
    assert!(result.is_err(), "should fail when dst already exists");
}

#[test]
#[cfg(unix)]
fn wire_creates_symlinks_for_all_dirs() {
    let (_tmp, ito, worktree_ito) = make_dirs();

    wire_coordination_symlinks(&ito, &worktree_ito).expect("wire should succeed");

    for dir in COORDINATION_DIRS {
        let link = ito.join(dir);
        assert!(link.exists(), "link '{dir}' should exist");
        let target = fs::read_link(&link).expect("should be a symlink");
        assert_eq!(
            target,
            worktree_ito.join(dir),
            "link '{dir}' points at wrong target"
        );
    }
}

#[test]
#[cfg(unix)]
fn wire_is_idempotent() {
    let (_tmp, ito, worktree_ito) = make_dirs();

    wire_coordination_symlinks(&ito, &worktree_ito).expect("first wire");
    wire_coordination_symlinks(&ito, &worktree_ito).expect("second wire should be idempotent");

    for dir in COORDINATION_DIRS {
        let link = ito.join(dir);
        let target = fs::read_link(&link).expect("should still be a symlink");
        assert_eq!(target, worktree_ito.join(dir));
    }
}

#[test]
#[cfg(unix)]
fn wire_migrates_real_dir_content() {
    let (_tmp, ito, worktree_ito) = make_dirs();
    let changes_dir = ito.join("changes");
    fs::create_dir_all(&changes_dir).unwrap();
    let sentinel = changes_dir.join("sentinel.md");
    fs::write(&sentinel, "hello").unwrap();

    wire_coordination_symlinks(&ito, &worktree_ito).expect("wire should succeed");

    let link = ito.join("changes");
    assert!(fs::read_link(&link).is_ok(), "changes should be a symlink");
    let migrated = worktree_ito.join("changes").join("sentinel.md");
    assert!(migrated.exists(), "sentinel.md should be in the worktree");
    assert_eq!(fs::read_to_string(&migrated).unwrap(), "hello");
    let via_link = link.join("sentinel.md");
    assert!(
        via_link.exists(),
        "sentinel.md should be accessible via symlink"
    );
}

#[test]
#[cfg(unix)]
fn wire_handles_empty_real_dir() {
    let (_tmp, ito, worktree_ito) = make_dirs();
    let specs_dir = ito.join("specs");
    fs::create_dir_all(&specs_dir).unwrap();

    wire_coordination_symlinks(&ito, &worktree_ito).expect("wire should succeed");

    let link = ito.join("specs");
    assert!(fs::read_link(&link).is_ok(), "specs should be a symlink");
}

#[test]
fn gitignore_entries_added_when_missing() {
    let tmp = TempDir::new().unwrap();
    let project_root = tmp.path();

    update_gitignore_for_symlinks(project_root).expect("should succeed");

    let content = fs::read_to_string(project_root.join(".gitignore")).unwrap();
    for dir in COORDINATION_DIRS {
        assert!(content.contains(&format!(".ito/{dir}")));
    }
}

#[test]
fn gitignore_no_duplicates_on_second_call() {
    let tmp = TempDir::new().unwrap();
    let project_root = tmp.path();

    update_gitignore_for_symlinks(project_root).expect("first call");
    update_gitignore_for_symlinks(project_root).expect("second call");

    let content = fs::read_to_string(project_root.join(".gitignore")).unwrap();
    for dir in COORDINATION_DIRS {
        let entry = format!(".ito/{dir}");
        let count = content.lines().filter(|l| l.trim() == entry).count();
        assert_eq!(
            count, 1,
            ".ito/{dir} should appear exactly once, found {count}"
        );
    }
}

#[test]
fn gitignore_preserves_existing_content() {
    let tmp = TempDir::new().unwrap();
    let project_root = tmp.path();
    let gitignore_path = project_root.join(".gitignore");

    fs::write(&gitignore_path, "target/\n*.log\n").unwrap();

    update_gitignore_for_symlinks(project_root).expect("should succeed");

    let content = fs::read_to_string(&gitignore_path).unwrap();
    assert!(content.contains("target/"));
    assert!(content.contains("*.log"));
}

#[test]
fn gitignore_skips_already_present_entries() {
    let tmp = TempDir::new().unwrap();
    let project_root = tmp.path();
    let gitignore_path = project_root.join(".gitignore");

    fs::write(&gitignore_path, ".ito/changes\n.ito/specs\n").unwrap();

    update_gitignore_for_symlinks(project_root).expect("should succeed");

    let content = fs::read_to_string(&gitignore_path).unwrap();
    let changes_count = content
        .lines()
        .filter(|l| l.trim() == ".ito/changes")
        .count();
    let specs_count = content.lines().filter(|l| l.trim() == ".ito/specs").count();
    assert_eq!(changes_count, 1);
    assert_eq!(specs_count, 1);
    assert!(content.contains(".ito/modules"));
    assert!(content.contains(".ito/workflows"));
    assert!(content.contains(".ito/audit"));
}

#[test]
fn gitignore_created_when_absent() {
    let tmp = TempDir::new().unwrap();
    let project_root = tmp.path();

    assert!(!project_root.join(".gitignore").exists());
    update_gitignore_for_symlinks(project_root).expect("should succeed");
    assert!(project_root.join(".gitignore").exists());
}

#[test]
#[cfg(unix)]
fn remove_restores_real_dirs_with_content() {
    let (_tmp, ito, worktree_ito) = make_dirs();

    wire_coordination_symlinks(&ito, &worktree_ito).expect("wire");
    let via_link = ito.join("changes").join("task.md");
    fs::write(&via_link, "task content").unwrap();

    remove_coordination_symlinks(&ito, &worktree_ito).expect("remove");

    let changes = ito.join("changes");
    assert!(changes.is_dir(), "changes should be a real directory");
    assert!(
        fs::read_link(&changes).is_err(),
        "changes should not be a symlink"
    );
    let restored = changes.join("task.md");
    assert!(restored.exists(), "task.md should be restored");
    assert_eq!(fs::read_to_string(&restored).unwrap(), "task content");
}

#[test]
#[cfg(unix)]
fn remove_is_noop_for_real_dirs() {
    let (_tmp, ito, worktree_ito) = make_dirs();
    for dir in COORDINATION_DIRS {
        fs::create_dir_all(ito.join(dir)).unwrap();
    }

    remove_coordination_symlinks(&ito, &worktree_ito).expect("remove");

    for dir in COORDINATION_DIRS {
        let path = ito.join(dir);
        assert!(path.is_dir(), "{dir} should still be a real directory");
        assert!(
            fs::read_link(&path).is_err(),
            "{dir} should not be a symlink"
        );
    }
}

#[test]
#[cfg(unix)]
fn remove_is_noop_when_dirs_absent() {
    let (_tmp, ito, worktree_ito) = make_dirs();
    remove_coordination_symlinks(&ito, &worktree_ito).expect("remove on empty ito dir");
}

#[test]
fn health_embedded_returns_embedded() {
    let tmp = TempDir::new().unwrap();
    let ito = tmp.path().join(".ito");
    let worktree_ito = tmp.path().join("worktree").join(".ito");
    fs::create_dir_all(&ito).unwrap();

    let status = check_coordination_health(&ito, &worktree_ito, &CoordinationStorage::Embedded);
    assert_eq!(status, CoordinationHealthStatus::Embedded);
}

#[test]
fn health_worktree_missing_when_dir_absent() {
    let tmp = TempDir::new().unwrap();
    let ito = tmp.path().join(".ito");
    let worktree_ito = tmp.path().join("nonexistent").join(".ito");
    fs::create_dir_all(&ito).unwrap();

    let status = check_coordination_health(&ito, &worktree_ito, &CoordinationStorage::Worktree);
    assert_eq!(
        status,
        CoordinationHealthStatus::WorktreeMissing {
            expected_path: worktree_ito.clone()
        }
    );
}

#[test]
#[cfg(unix)]
fn health_healthy_when_all_symlinks_correct() {
    let (_tmp, ito, worktree_ito) = make_dirs();

    wire_coordination_symlinks(&ito, &worktree_ito).expect("wire");

    let status = check_coordination_health(&ito, &worktree_ito, &CoordinationStorage::Worktree);
    assert_eq!(status, CoordinationHealthStatus::Healthy);
}

#[test]
fn health_missing_link_is_not_wired() {
    let (_tmp, ito, worktree_ito) = make_dirs();

    let status = check_coordination_health(&ito, &worktree_ito, &CoordinationStorage::Worktree);

    let CoordinationHealthStatus::NotWired { dirs } = status else {
        panic!("expected NotWired, got {status:?}");
    };
    assert!(dirs.contains(&ito.join("changes")));
}

#[test]
#[cfg(unix)]
fn health_broken_symlinks_when_target_missing() {
    let (_tmp, ito, worktree_ito) = make_dirs();

    wire_coordination_symlinks(&ito, &worktree_ito).expect("wire");
    let target = worktree_ito.join("changes");
    fs::remove_dir_all(&target).unwrap();

    let status = check_coordination_health(&ito, &worktree_ito, &CoordinationStorage::Worktree);

    let CoordinationHealthStatus::BrokenSymlinks { broken } = status else {
        panic!("expected BrokenSymlinks, got {status:?}");
    };
    assert_eq!(broken.len(), 1);
    assert_eq!(broken[0].0, ito.join("changes"));
}

#[test]
#[cfg(unix)]
fn health_wrong_target_when_symlink_points_elsewhere() {
    let (_tmp, ito, worktree_ito) = make_dirs();
    let wrong_root = ito.parent().unwrap().join("other-worktree").join(".ito");
    fs::create_dir_all(&wrong_root).unwrap();

    wire_coordination_symlinks(&ito, &wrong_root).expect("wire");

    let status = check_coordination_health(&ito, &worktree_ito, &CoordinationStorage::Worktree);

    let CoordinationHealthStatus::WrongTargets { mismatched } = status else {
        panic!("expected WrongTargets, got {status:?}");
    };
    assert_eq!(mismatched.len(), COORDINATION_DIRS.len());
    assert_eq!(mismatched[0].0, ito.join("changes"));
}

#[test]
#[cfg(unix)]
fn health_not_wired_when_real_dirs_present() {
    let (_tmp, ito, worktree_ito) = make_dirs();
    for dir in COORDINATION_DIRS {
        fs::create_dir_all(ito.join(dir)).unwrap();
    }

    let status = check_coordination_health(&ito, &worktree_ito, &CoordinationStorage::Worktree);

    let CoordinationHealthStatus::NotWired { dirs } = status else {
        panic!("expected NotWired, got {status:?}");
    };
    assert_eq!(dirs.len(), COORDINATION_DIRS.len());
}

#[test]
fn format_message_healthy_is_none() {
    assert!(format_health_message(&CoordinationHealthStatus::Healthy).is_none());
}

#[test]
fn format_message_embedded_is_none() {
    assert!(format_health_message(&CoordinationHealthStatus::Embedded).is_none());
}

#[test]
fn format_message_worktree_missing_contains_path_and_hint() {
    let path = PathBuf::from("/some/path/.ito");
    let msg = format_health_message(&CoordinationHealthStatus::WorktreeMissing {
        expected_path: path.clone(),
    })
    .expect("should produce a message");

    assert!(msg.contains(&path.display().to_string()));
    assert!(msg.contains("ito init"));
}

#[test]
fn format_message_broken_symlinks_contains_paths_and_hint() {
    let link = PathBuf::from("/project/.ito/changes");
    let target = PathBuf::from("../worktree/.ito/changes");
    let msg = format_health_message(&CoordinationHealthStatus::BrokenSymlinks {
        broken: vec![(link.clone(), target.clone())],
    })
    .expect("should produce a message");

    assert!(msg.contains(&link.display().to_string()));
    assert!(msg.contains(&target.display().to_string()));
    assert!(msg.contains("ito init"));
}

#[test]
fn format_message_not_wired_contains_dir_and_hint() {
    let dir = PathBuf::from("/project/.ito/specs");
    let msg = format_health_message(&CoordinationHealthStatus::NotWired {
        dirs: vec![dir.clone()],
    })
    .expect("should produce a message");

    assert!(msg.contains(&dir.display().to_string()));
    assert!(msg.contains("ito init"));
}

#[test]
fn format_message_wrong_target_contains_paths_and_hint() {
    let link = PathBuf::from("/project/.ito/changes");
    let actual = PathBuf::from("/other/.ito/changes");
    let expected = PathBuf::from("/coord/.ito/changes");
    let msg = format_health_message(&CoordinationHealthStatus::WrongTargets {
        mismatched: vec![(link.clone(), actual.clone(), expected.clone())],
    })
    .expect("should produce a message");

    assert!(msg.contains(&link.display().to_string()));
    assert!(msg.contains(&actual.display().to_string()));
    assert!(msg.contains(&expected.display().to_string()));
    assert!(msg.contains("ito init"));
}
