#[path = "support/mod.rs"]
mod fixtures;

use std::collections::BTreeMap;
use std::path::Path;
use std::time::{Duration, Instant};

use ito_config::types::{BackendAllowlistConfig, BackendAuthConfig, BackendRepoPolicy};
use ito_test_support::{CmdOutput, rust_candidate_command};

const ORG: &str = "acme";
const REPO: &str = "widgets";
const ADMIN_TOKEN: &str = "cli-backend-import-token";

fn spawn_backend_server() -> (String, tempfile::TempDir) {
    let data_dir = tempfile::tempdir().expect("backend data dir");

    let mut repos = BTreeMap::new();
    repos.insert(
        ORG.to_string(),
        BackendRepoPolicy::List(vec![REPO.to_string()]),
    );
    let allowlist = BackendAllowlistConfig {
        orgs: vec![ORG.to_string()],
        repos,
    };

    let auth = BackendAuthConfig {
        admin_tokens: vec![ADMIN_TOKEN.to_string()],
        token_seed: Some("cli-backend-import-seed".to_string()),
    };

    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind backend port");
    let addr = listener.local_addr().expect("backend addr");
    let base_url = format!("http://{addr}");
    let config = ito_backend::BackendServerConfig {
        enabled: true,
        bind: "127.0.0.1".to_string(),
        port: addr.port(),
        data_dir: Some(data_dir.path().to_string_lossy().to_string()),
        allowed: allowlist,
        auth,
        ..Default::default()
    };
    drop(listener);

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("runtime");
        rt.block_on(async move {
            let _ = ito_backend::serve(config).await;
        });
    });

    wait_for_backend_ready(&base_url);
    (base_url, data_dir)
}

fn wait_for_backend_ready(base_url: &str) {
    let deadline = Instant::now() + Duration::from_secs(2);
    let health_url = format!("{base_url}/api/v1/health");

    while Instant::now() < deadline {
        if let Ok(response) = ureq::get(&health_url).call()
            && response.status() == 200
        {
            return;
        }
        std::thread::sleep(Duration::from_millis(20));
    }

    panic!("backend did not become ready: {health_url}");
}

fn write_backend_config(repo: &Path, base_url: &str) {
    fixtures::write(
        repo.join(".ito/config.json"),
        &format!(
            r#"{{
  "backend": {{
    "enabled": true,
    "url": "{base_url}",
    "token": "{ADMIN_TOKEN}",
    "project": {{
      "org": "{ORG}",
      "repo": "{REPO}"
    }}
  }}
}}"#
        ),
    );
}

fn seed_local_changes(repo: &Path) {
    fixtures::write(
        repo.join(".ito/changes/024-18_active-import/proposal.md"),
        "# Active\n",
    );
    fixtures::write(
        repo.join(".ito/changes/024-18_active-import/tasks.md"),
        "- [ ] task\n",
    );
    fixtures::write(
        repo.join(".ito/changes/024-18_active-import/specs/backend-import/spec.md"),
        "## ADDED Requirements\n",
    );

    fixtures::write(
        repo.join(".ito/changes/archive/2026-03-10-024-17_archived-import/proposal.md"),
        "# Archived\n",
    );
    fixtures::write(
        repo.join(".ito/changes/archive/2026-03-10-024-17_archived-import/tasks.md"),
        "- [x] done\n",
    );
    fixtures::write(
        repo.join(
            ".ito/changes/archive/2026-03-10-024-17_archived-import/specs/backend-import/spec.md",
        ),
        "## ADDED Archived Requirements\n",
    );
}

fn run_cli(program: &Path, args: &[&str], cwd: &Path, home: &Path) -> CmdOutput {
    let mut cmd = rust_candidate_command(program);
    cmd.args(args);
    cmd.current_dir(cwd);
    cmd.env("CI", "1");
    cmd.env("NO_COLOR", "1");
    cmd.env("ITO_INTERACTIVE", "0");
    cmd.env("TERM", "dumb");
    cmd.env("HOME", home);
    cmd.env("XDG_CONFIG_HOME", home.join(".config"));
    cmd.env("XDG_DATA_HOME", home);
    let out = cmd.output().expect("command should run");
    CmdOutput {
        code: out.status.code().unwrap_or(1),
        stdout: String::from_utf8_lossy(&out.stdout).to_string(),
        stderr: String::from_utf8_lossy(&out.stderr).to_string(),
    }
}

#[test]
fn backend_import_rejects_local_mode() {
    let repo = fixtures::make_empty_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    fixtures::git_init_with_initial_commit(repo.path());
    seed_local_changes(repo.path());

    let out = run_cli(rust_path, &["backend", "import"], repo.path(), home.path());

    assert_ne!(out.code, 0);
    assert!(
        out.stderr.contains("backend mode is required")
            || out.stdout.contains("backend mode is required")
    );
}

#[test]
fn backend_import_dry_run_reports_scope_without_writing_backend() {
    let repo = fixtures::make_empty_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    fixtures::git_init_with_initial_commit(repo.path());
    seed_local_changes(repo.path());
    let (base_url, data_dir) = spawn_backend_server();
    write_backend_config(repo.path(), &base_url);

    let out = run_cli(
        rust_path,
        &["backend", "import", "--dry-run"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stdout={} stderr={}", out.stdout, out.stderr);
    assert!(out.stdout.contains("Would import") || out.stdout.contains("preview"));
    assert!(
        !data_dir
            .path()
            .join("projects")
            .join(ORG)
            .join(REPO)
            .join(".ito/changes")
            .join("024-18_active-import")
            .exists()
    );
}

#[test]
fn backend_import_writes_active_and_archived_changes_to_backend() {
    let repo = fixtures::make_empty_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    fixtures::git_init_with_initial_commit(repo.path());
    seed_local_changes(repo.path());
    let (base_url, data_dir) = spawn_backend_server();
    write_backend_config(repo.path(), &base_url);

    let out = run_cli(rust_path, &["backend", "import"], repo.path(), home.path());

    assert_eq!(out.code, 0, "stdout={} stderr={}", out.stdout, out.stderr);
    assert!(out.stdout.contains("Imported"));
    assert!(
        data_dir
            .path()
            .join("projects")
            .join(ORG)
            .join(REPO)
            .join(".ito/changes")
            .join("024-18_active-import")
            .join("proposal.md")
            .exists()
    );
    let archive_dir = data_dir
        .path()
        .join("projects")
        .join(ORG)
        .join(REPO)
        .join(".ito/changes/archive");
    let archived_entries: Vec<_> = std::fs::read_dir(&archive_dir)
        .unwrap()
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect();
    assert!(
        archived_entries
            .iter()
            .any(|name| name.contains("024-17_archived-import"))
    );
}

#[test]
fn backend_import_is_idempotent_and_remote_reads_match_imported_changes() {
    let repo = fixtures::make_empty_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    fixtures::git_init_with_initial_commit(repo.path());
    seed_local_changes(repo.path());
    let (base_url, data_dir) = spawn_backend_server();
    write_backend_config(repo.path(), &base_url);

    let first = run_cli(rust_path, &["backend", "import"], repo.path(), home.path());
    assert_eq!(
        first.code, 0,
        "stdout={} stderr={}",
        first.stdout, first.stderr
    );

    let second = run_cli(rust_path, &["backend", "import"], repo.path(), home.path());
    assert_eq!(
        second.code, 0,
        "stdout={} stderr={}",
        second.stdout, second.stderr
    );
    assert!(second.stdout.contains("skipped"));

    let show_active = run_cli(
        rust_path,
        &["show", "024-18_active-import", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(
        show_active.code, 0,
        "stdout={} stderr={}",
        show_active.stdout, show_active.stderr
    );
    assert!(show_active.stdout.contains("024-18_active-import"));

    let archive_dir = data_dir
        .path()
        .join("projects")
        .join(ORG)
        .join(REPO)
        .join(".ito/changes/archive");
    let archived_entries: Vec<_> = std::fs::read_dir(&archive_dir)
        .unwrap()
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect();
    assert!(
        archived_entries
            .iter()
            .any(|name| name.contains("024-17_archived-import"))
    );
}
