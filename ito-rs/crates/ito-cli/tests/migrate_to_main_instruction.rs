#[path = "support/mod.rs"]
mod fixtures;

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ito_core::coordination::{COORDINATION_DIRS, create_dir_link, gitignore_entries};
use ito_test_support::{TreeSnapshotEntry, reset_dir, run_rust_candidate, snapshot_tree_manifest};

struct LegacyFixture {
    _temp: tempfile::TempDir,
    project: PathBuf,
    home: PathBuf,
    source: PathBuf,
}

fn git(source: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(source)
        .env("GIT_AUTHOR_DATE", "2000-01-01T00:00:00Z")
        .env("GIT_COMMITTER_DATE", "2000-01-01T00:00:00Z")
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .output()
        .expect("run git");
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn remove_managed_link(path: &Path) {
    #[cfg(unix)]
    fs::remove_file(path).expect("remove managed link");

    #[cfg(windows)]
    fs::remove_dir(path).expect("remove managed junction");
}

fn managed_manifest(root: &Path) -> BTreeMap<PathBuf, TreeSnapshotEntry> {
    COORDINATION_DIRS
        .iter()
        .flat_map(|name| {
            snapshot_tree_manifest(&root.join(name))
                .expect("managed tree manifest")
                .into_iter()
                .map(move |(path, entry)| (PathBuf::from(name).join(path), entry))
        })
        .collect()
}

fn linked_legacy_fixture() -> LegacyFixture {
    let temp = tempfile::tempdir().expect("fixture");
    let project = temp.path().join("project");
    let home = temp.path().join("home");
    let source = temp.path().join("coordination");
    fs::create_dir_all(project.join(".ito")).expect("Ito root");
    fs::create_dir_all(source.join(".ito")).expect("coordination Ito root");
    fs::create_dir_all(&home).expect("home");

    for name in COORDINATION_DIRS {
        let source_dir = source.join(".ito").join(name);
        fs::create_dir_all(&source_dir).expect("managed source directory");
        fixtures::write(
            source_dir.join(".migration-proof"),
            &format!("stable source bytes for {name}\n"),
        );
        create_dir_link(&source_dir, &project.join(".ito").join(name)).expect("managed link");
    }
    let nested = source.join(".ito/audit/nested");
    fs::create_dir_all(&nested).expect("nested source directory");
    fixtures::write(nested.join("tool.sh"), "#!/bin/sh\necho migration-proof\n");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(nested.join("tool.sh"), fs::Permissions::from_mode(0o755))
            .expect("source executable mode");
        std::os::unix::fs::symlink("tool.sh", nested.join("tool-link"))
            .expect("nested source symlink");
    }

    let config = serde_json::json!({
        "changes": {
            "coordination_branch": {
                "enabled": true,
                "name": "ito/internal/test-changes",
                "storage": "worktree",
                "worktree_path": source,
            },
            "archive": {
                "main_integration_mode": "pull_request"
            }
        }
    });
    fixtures::write(
        project.join(".ito/config.json"),
        &serde_json::to_string_pretty(&config).expect("config"),
    );
    fixtures::write(
        project.join(".gitignore"),
        &format!(
            "# unrelated\n.ito/session.json\n# Ito coordination worktree symlinks\n{}\n",
            gitignore_entries().join("\n")
        ),
    );

    git(
        &source,
        &["init", "--initial-branch=ito/internal/test-changes"],
    );
    git(
        &source,
        &["config", "user.email", "ito-tests@example.invalid"],
    );
    git(&source, &["config", "user.name", "Ito Tests"]);
    git(&source, &["add", ".ito"]);
    git(&source, &["commit", "-m", "coordination source"]);

    fixtures::write(project.join("README.md"), "# Migration fixture\n");
    git(&project, &["init", "--initial-branch=main"]);
    git(
        &project,
        &["config", "user.email", "ito-tests@example.invalid"],
    );
    git(&project, &["config", "user.name", "Ito Tests"]);
    git(&project, &["add", "."]);
    git(&project, &["commit", "-m", "project main"]);

    LegacyFixture {
        _temp: temp,
        project,
        home,
        source,
    }
}

fn migration_fixture() -> (tempfile::TempDir, std::path::PathBuf, std::path::PathBuf) {
    let temp = tempfile::tempdir().expect("fixture");
    let project = temp.path().join("project");
    let home = temp.path().join("home");
    let coordination = temp.path().join("coordination");
    fs::create_dir_all(project.join(".ito")).expect("Ito root");
    fs::create_dir_all(&home).expect("home");
    let config = serde_json::json!({
        "changes": {
            "coordination_branch": {
                "enabled": true,
                "name": "ito/internal/test-changes",
                "storage": "worktree",
                "worktree_path": coordination,
            },
            "archive": {
                "main_integration_mode": "pull_request"
            }
        }
    });
    fixtures::write(
        project.join(".ito/config.json"),
        &serde_json::to_string_pretty(&config).expect("config"),
    );
    (temp, project, home)
}

#[test]
fn migrate_to_main_instruction_renders_without_coordination_mutation() {
    let (_temp, project, home) = migration_fixture();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "migrate-to-main"],
        &project,
        &home,
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stderr.is_empty(), "stderr={}", out.stderr);
    assert!(
        out.stdout
            .contains("# Migrate Ito coordination state to main")
    );
    assert!(out.stdout.contains(&project.to_string_lossy().to_string()));
    assert!(out.stdout.contains("ito/internal/test-changes"));
    assert!(out.stdout.contains("\"kind\": \"legacy\""));
    assert!(out.stdout.contains("pull_request"));
}

#[test]
fn migrate_to_main_json_uses_stable_artifact_identity() {
    let (_temp, project, home) = migration_fixture();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "migrate-to-main", "--json"],
        &project,
        &home,
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let value: serde_json::Value = serde_json::from_str(&out.stdout).expect("instruction JSON");
    assert_eq!(value["artifactId"], "migrate-to-main");
    assert!(
        value["instruction"]
            .as_str()
            .unwrap_or_default()
            .contains("Never delete or rewrite the source coordination worktree")
    );
}

#[test]
fn migrate_to_main_still_renders_when_evidence_cannot_be_decoded() {
    let (_temp, project, home) = migration_fixture();
    fs::write(project.join(".gitignore"), [0xff, 0xfe]).expect("invalid UTF-8 evidence");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "migrate-to-main"],
        &project,
        &home,
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("\"kind\": \"inspection_error\""));
    assert!(
        out.stdout
            .contains("cannot inspect legacy coordination marker")
    );
}

#[test]
fn migrate_to_main_still_renders_when_coordination_config_is_invalid() {
    let (_temp, project, home) = migration_fixture();
    fixtures::write(
        project.join(".ito/config.json"),
        r#"{"changes":{"coordination_branch":{"storage":42}}}"#,
    );
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "migrate-to-main"],
        &project,
        &home,
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("\"kind\": \"inspection_error\""));
    assert!(out.stdout.contains("resolved Ito configuration is invalid"));
}

#[test]
fn reversible_fixture_materialization_preserves_source_and_hashes() {
    let fixture = linked_legacy_fixture();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let source_ito = fixture.source.join(".ito");
    let source_before = managed_manifest(&source_ito);
    let source_commit_before = git(&fixture.source, &["rev-parse", "HEAD"]);
    let project_main_before = git(&fixture.project, &["rev-parse", "main"]);
    git(
        &fixture.project,
        &["checkout", "-b", "ito/migrate-coordination-to-main"],
    );

    let before = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "migrate-to-main"],
        &fixture.project,
        &fixture.home,
    );
    assert_eq!(before.code, 0, "stderr={}", before.stderr);
    assert!(before.stdout.contains("\"kind\": \"legacy\""));
    println!("before classification: legacy");
    println!("source commit: {source_commit_before}");
    for (path, entry) in &source_before {
        if let TreeSnapshotEntry::File { sha256, .. } = entry {
            println!("source {} {sha256}", path.display());
        }
    }

    for name in COORDINATION_DIRS {
        remove_managed_link(&fixture.project.join(".ito").join(name));
        reset_dir(
            &fixture.project.join(".ito").join(name),
            &source_ito.join(name),
        )
        .expect("materialize managed directory");
    }
    fixtures::write(
        fixture.project.join(".ito/config.json"),
        &serde_json::to_string_pretty(&serde_json::json!({
            "changes": {
                "coordination_branch": {
                    "enabled": false,
                    "name": "ito/internal/test-changes",
                    "storage": "embedded",
                    "worktree_path": fixture.source,
                },
                "archive": {
                    "main_integration_mode": "pull_request"
                }
            }
        }))
        .expect("embedded config"),
    );
    fixtures::write(
        fixture.project.join(".gitignore"),
        "# unrelated\n.ito/session.json\n",
    );

    let destination_after = managed_manifest(&fixture.project.join(".ito"));
    assert_eq!(destination_after, source_before);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let source_mode = fs::metadata(source_ito.join("audit/nested/tool.sh"))
            .expect("source mode")
            .permissions()
            .mode()
            & 0o777;
        let destination_mode = fs::metadata(fixture.project.join(".ito/audit/nested/tool.sh"))
            .expect("destination mode")
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(destination_mode, source_mode);
        println!("executable mode preserved: {destination_mode:o}");
    }
    assert_eq!(managed_manifest(&source_ito), source_before);
    assert_eq!(
        git(&fixture.source, &["rev-parse", "HEAD"]),
        source_commit_before
    );
    assert!(git(&fixture.source, &["status", "--porcelain"]).is_empty());

    let after = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "migrate-to-main"],
        &fixture.project,
        &fixture.home,
    );
    assert_eq!(after.code, 0, "stderr={}", after.stderr);
    assert!(after.stdout.contains("\"kind\": \"embedded\""));
    println!("after classification: embedded");
    for (path, entry) in &destination_after {
        if let TreeSnapshotEntry::File { sha256, .. } = entry {
            println!("destination {} {sha256}", path.display());
        }
    }
    println!(
        "source commit after: {}",
        git(&fixture.source, &["rev-parse", "HEAD"])
    );

    let validation = run_rust_candidate(
        rust_path,
        &["validate", "--all", "--strict"],
        &fixture.project,
        &fixture.home,
    );
    assert_eq!(validation.code, 0, "stderr={}", validation.stderr);

    git(&fixture.project, &["add", ".ito", ".gitignore"]);
    let staged = git(&fixture.project, &["diff", "--cached", "--name-status"]);
    assert!(staged.contains(".ito/config.json"));
    for name in COORDINATION_DIRS {
        assert!(
            staged.contains(&format!(".ito/{name}/.migration-proof")),
            "missing managed path in review diff: {name}\n{staged}"
        );
    }
    git(
        &fixture.project,
        &["commit", "-m", "migrate coordination state to main"],
    );
    assert_eq!(
        git(&fixture.project, &["rev-parse", "main"]),
        project_main_before
    );
    assert_eq!(
        git(&fixture.project, &["branch", "--show-current"]),
        "ito/migrate-coordination-to-main"
    );
    let review_diff = git(&fixture.project, &["diff", "--name-status", "main...HEAD"]);
    assert!(review_diff.contains(".ito/changes/.migration-proof"));
    let review_checkout = fixture._temp.path().join("review-checkout");
    let project_path = fixture.project.to_string_lossy().into_owned();
    let checkout_path = review_checkout.to_string_lossy().into_owned();
    git(
        fixture._temp.path(),
        &[
            "clone",
            "--quiet",
            "--branch",
            "ito/migrate-coordination-to-main",
            &project_path,
            &checkout_path,
        ],
    );
    assert_eq!(
        managed_manifest(&review_checkout.join(".ito")),
        source_before
    );
    println!("review branch: ito/migrate-coordination-to-main");
    println!("fresh review checkout manifest: exact match");
    println!("review diff:\n{review_diff}");
}

#[cfg(unix)]
#[test]
fn ambiguous_destination_is_reported_without_touching_conflicting_bytes() {
    let fixture = linked_legacy_fixture();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let conflict = fixture.project.join(".ito/changes");
    fs::remove_file(&conflict).expect("remove fixture link");
    fs::create_dir(&conflict).expect("conflict directory");
    fixtures::write(
        conflict.join(".migration-proof"),
        "different destination bytes\n",
    );
    let source_before = managed_manifest(&fixture.source.join(".ito"));

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "migrate-to-main"],
        &fixture.project,
        &fixture.home,
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("\"kind\": \"ambiguous\""));
    assert!(out.stdout.contains("stop and report the conflict"));
    println!("classification: ambiguous");
    println!("conflict action: stopped without mutation");
    assert_eq!(
        fs::read_to_string(conflict.join(".migration-proof")).expect("conflicting bytes"),
        "different destination bytes\n"
    );
    assert_eq!(
        managed_manifest(&fixture.source.join(".ito")),
        source_before
    );
    assert!(git(&fixture.source, &["status", "--porcelain"]).is_empty());
}
