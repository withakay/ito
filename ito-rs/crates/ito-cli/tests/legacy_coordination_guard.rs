#[path = "support/mod.rs"]
mod fixtures;

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use ito_core::legacy_coordination::MANAGED_STATE_DIRS as COORDINATION_DIRS;
use ito_test_support::run_rust_candidate;

#[derive(Debug, PartialEq, Eq)]
enum SnapshotEntry {
    Directory,
    File(Vec<u8>),
    Link(PathBuf),
}

struct LegacyFixture {
    temp: tempfile::TempDir,
    project: PathBuf,
    home: PathBuf,
}

fn create_managed_link(source: &Path, destination: &Path) {
    #[cfg(unix)]
    std::os::unix::fs::symlink(source, destination).expect("coordination link");

    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(source, destination).expect("coordination link");
}

impl LegacyFixture {
    fn linked() -> Self {
        let temp = tempfile::tempdir().expect("fixture root");
        let project = temp.path().join("project");
        let home = temp.path().join("home");
        let coordination = temp.path().join("coordination");
        let project_ito = project.join(".ito");
        let coordination_ito = coordination.join(".ito");
        fs::create_dir_all(&project_ito).expect("project Ito root");
        fs::create_dir_all(&home).expect("home");

        for name in COORDINATION_DIRS {
            let target = coordination_ito.join(name);
            fs::create_dir_all(&target).expect("coordination directory");
            create_managed_link(&target, &project_ito.join(name));
        }

        let config = serde_json::json!({
            "changes": {
                "coordination_branch": {
                    "enabled": true,
                    "storage": "worktree",
                    "worktree_path": coordination,
                }
            }
        });
        fixtures::write(
            project_ito.join("config.json"),
            &serde_json::to_string_pretty(&config).expect("config JSON"),
        );
        fixtures::write(project.join("README.md"), "# Guard fixture\n");
        fixtures::git_init_with_initial_commit(&project);

        Self {
            temp,
            project,
            home,
        }
    }

    fn ambiguous() -> Self {
        let fixture = Self::linked();
        let specs = fixture.project.join(".ito/specs");
        remove_link(&specs);
        fs::create_dir_all(&specs).expect("real specs directory");
        fixtures::write(specs.join("local.md"), "conflicting destination\n");
        fixture
    }

    fn snapshot(&self) -> BTreeMap<PathBuf, SnapshotEntry> {
        snapshot_tree(self.temp.path())
    }

    fn git_state(&self) -> (String, String) {
        (
            git_output(&self.project, &["rev-parse", "HEAD"]),
            git_output(&self.project, &["status", "--porcelain=v1"]),
        )
    }

    #[cfg(not(feature = "coordination-branch"))]
    fn update_config(&self, update: impl FnOnce(&mut serde_json::Value)) {
        let path = self.project.join(".ito/config.json");
        let mut config: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&path).expect("read config"))
                .expect("parse config");
        update(&mut config);
        fixtures::write(
            path,
            &serde_json::to_string_pretty(&config).expect("updated config"),
        );
    }
}

#[test]
#[cfg(not(feature = "coordination-branch"))]
fn legacy_read_warns_once_and_does_not_mutate_state() {
    let fixture = LegacyFixture::linked();
    let before = fixture.snapshot();
    let git_before = fixture.git_state();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["list", "--json"],
        &fixture.project,
        &fixture.home,
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert_eq!(
        out.stderr
            .matches("ito agent instruction migrate-to-main")
            .count(),
        1,
        "stderr={}",
        out.stderr
    );
    assert!(out.stderr.contains("read-only command allowed"));
    assert_eq!(fixture.snapshot(), before);
    assert_eq!(fixture.git_state(), git_before);
}

#[test]
#[cfg(not(feature = "coordination-branch"))]
fn legacy_mutation_is_blocked_before_any_state_change() {
    let fixture = LegacyFixture::linked();
    let before = fixture.snapshot();
    let git_before = fixture.git_state();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["create", "module", "must-not-exist"],
        &fixture.project,
        &fixture.home,
    );

    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("mutating command blocked"));
    assert!(out.stderr.contains("No mutation occurred"));
    assert!(out.stderr.contains("ito agent instruction migrate-to-main"));
    assert_eq!(fixture.snapshot(), before);
    assert_eq!(fixture.git_state(), git_before);
}

#[test]
#[cfg(not(feature = "coordination-branch"))]
fn legacy_config_init_and_update_mutations_are_blocked() {
    let cases: &[&[&str]] = &[
        &["config", "schema", "--output", ".ito/changes/schema.json"],
        &["init", ".", "--tools", "none", "--update"],
        &["update", "."],
    ];

    for args in cases {
        let fixture = LegacyFixture::linked();
        let before = fixture.snapshot();
        let git_before = fixture.git_state();
        let rust_path = assert_cmd::cargo::cargo_bin!("ito");

        let out = run_rust_candidate(rust_path, args, &fixture.project, &fixture.home);

        assert_ne!(out.code, 0, "args={args:?}");
        assert!(
            out.stderr.contains("mutating command blocked"),
            "args={args:?} stderr={}",
            out.stderr
        );
        assert_eq!(fixture.snapshot(), before, "args={args:?}");
        assert_eq!(fixture.git_state(), git_before, "args={args:?}");
    }
}

#[test]
fn ambiguous_legacy_state_also_fails_closed() {
    let fixture = LegacyFixture::ambiguous();
    let before = fixture.snapshot();
    let git_before = fixture.git_state();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["tasks", "init", "031-01_example"],
        &fixture.project,
        &fixture.home,
    );

    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("ambiguous"), "stderr={}", out.stderr);
    assert!(out.stderr.contains("mutating command blocked"));
    assert_eq!(fixture.snapshot(), before);
    assert_eq!(fixture.git_state(), git_before);
}

#[test]
#[cfg(all(feature = "backend", not(feature = "coordination-branch")))]
fn legacy_backend_grep_does_not_materialize_remote_cache() {
    let fixture = LegacyFixture::linked();
    let changes = std::fs::read_link(fixture.project.join(".ito/changes"))
        .expect("coordination changes link");
    fixtures::write(
        changes.join("000-01_probe/proposal.md"),
        "# Proposal\n\nneedle\n",
    );

    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("probe listener");
    listener.set_nonblocking(true).expect("nonblocking probe");
    let address = listener.local_addr().expect("probe address");
    let (connected_tx, connected_rx) = std::sync::mpsc::channel();
    let (stop_tx, stop_rx) = std::sync::mpsc::channel();
    let server = std::thread::spawn(move || {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while std::time::Instant::now() < deadline {
            if stop_rx.try_recv().is_ok() {
                return;
            }
            match listener.accept() {
                Ok((mut stream, _)) => {
                    use std::io::Write;
                    let _ = connected_tx.send(());
                    let _ = stream.write_all(
                        b"HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                    );
                    return;
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
                Err(error) => panic!("probe accept failed: {error}"),
            }
        }
    });
    fixture.update_config(|config| {
        config["backend"] = serde_json::json!({
            "enabled": true,
            "url": format!("http://{address}"),
            "project": { "org": "acme", "repo": "widgets" }
        });
    });
    let before = fixture.snapshot();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["grep", "000-01_probe", "needle"],
        &fixture.project,
        &fixture.home,
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stderr.contains("read-only command allowed"));
    let _ = stop_tx.send(());
    server.join().expect("probe server");
    assert!(
        connected_rx.try_recv().is_err(),
        "suppressed grep contacted the backend"
    );
    assert_eq!(fixture.snapshot(), before);
}

#[test]
#[cfg(not(feature = "coordination-branch"))]
fn legacy_read_uses_filesystem_without_creating_configured_sqlite_database() {
    let fixture = LegacyFixture::linked();
    let database = fixture.temp.path().join("sqlite/runtime.db");
    fixture.update_config(|config| {
        config["repository"] = serde_json::json!({
            "mode": "sqlite",
            "sqlite": { "dbPath": database }
        });
    });
    assert!(!database.exists());
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["list", "--json"],
        &fixture.project,
        &fixture.home,
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stderr.contains("read-only command allowed"));
    assert!(!database.exists(), "legacy read created SQLite state");
}

#[test]
#[cfg(not(feature = "coordination-branch"))]
fn invalid_command_is_blocked_before_invalid_command_logging() {
    let fixture = LegacyFixture::linked();
    fixture.update_config(|config| {
        config["logging"] = serde_json::json!({
            "invalid_commands": { "enabled": true }
        });
    });
    let before = fixture.snapshot();
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["definitely-not-an-ito-command"],
        &fixture.project,
        &fixture.home,
    );

    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("mutating command blocked"));
    assert_eq!(fixture.snapshot(), before);
}

fn snapshot_tree(root: &Path) -> BTreeMap<PathBuf, SnapshotEntry> {
    fn visit(root: &Path, path: &Path, snapshot: &mut BTreeMap<PathBuf, SnapshotEntry>) {
        let metadata = fs::symlink_metadata(path).expect("snapshot metadata");
        let relative = path.strip_prefix(root).expect("relative snapshot path");
        if relative
            .components()
            .any(|component| component.as_os_str() == ".git")
        {
            return;
        }

        if metadata.file_type().is_symlink() {
            snapshot.insert(
                relative.to_path_buf(),
                SnapshotEntry::Link(fs::read_link(path).expect("snapshot link")),
            );
        } else if metadata.is_dir() {
            if !relative.as_os_str().is_empty() {
                snapshot.insert(relative.to_path_buf(), SnapshotEntry::Directory);
            }
            for entry in fs::read_dir(path).expect("snapshot directory") {
                visit(root, &entry.expect("snapshot entry").path(), snapshot);
            }
        } else {
            snapshot.insert(
                relative.to_path_buf(),
                SnapshotEntry::File(fs::read(path).expect("snapshot file")),
            );
        }
    }

    let mut snapshot = BTreeMap::new();
    visit(root, root, &mut snapshot);
    snapshot
}

fn git_output(repo: &Path, args: &[&str]) -> String {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(repo)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git command");
    assert!(output.status.success(), "git {} failed", args.join(" "));
    String::from_utf8(output.stdout).expect("git output")
}

fn remove_link(path: &Path) {
    #[cfg(windows)]
    fs::remove_dir(path).expect("remove junction");
    #[cfg(not(windows))]
    fs::remove_file(path).expect("remove symlink");
}
