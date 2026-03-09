#[path = "support/mod.rs"]
mod fixtures;

use std::collections::BTreeMap;
use std::path::Path;
use std::time::Duration;

use ito_config::types::{BackendAllowlistConfig, BackendAuthConfig, BackendRepoPolicy};
use ito_test_support::{CmdOutput, rust_candidate_command};

const ORG: &str = "acme";
const REPO: &str = "widgets";
const ADMIN_TOKEN: &str = "cli-remote-task-token";

fn project_change_dir(data_dir: &Path, change_id: &str) -> std::path::PathBuf {
    data_dir
        .join("projects")
        .join(ORG)
        .join(REPO)
        .join(".ito")
        .join("changes")
        .join(change_id)
}

fn seed_remote_change(data_dir: &Path, change_id: &str, tasks: Option<&str>) {
    let change_dir = project_change_dir(data_dir, change_id);
    std::fs::create_dir_all(&change_dir).expect("create remote change dir");
    fixtures::write(change_dir.join("proposal.md"), "# Proposal\n");
    if let Some(tasks) = tasks {
        fixtures::write(change_dir.join("tasks.md"), tasks);
    }
}

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
        token_seed: Some("cli-remote-task-seed".to_string()),
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

    std::thread::sleep(Duration::from_millis(100));
    (base_url, data_dir)
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
    cmd.env_remove("ITO_BACKEND_TOKEN");
    cmd.env_remove("ITO_BACKEND_TOKEN_SEED");
    cmd.env_remove("ITO_BACKEND_PROJECT_ORG");
    cmd.env_remove("ITO_BACKEND_PROJECT_REPO");

    let out = cmd.output().expect("command should run");
    CmdOutput {
        code: out.status.code().unwrap_or(1),
        stdout: String::from_utf8_lossy(&out.stdout).to_string(),
        stderr: String::from_utf8_lossy(&out.stderr).to_string(),
    }
}

#[test]
fn remote_task_start_updates_backend_without_local_tasks_file() {
    let repo = fixtures::make_empty_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    fixtures::git_init_with_initial_commit(repo.path());

    let (base_url, data_dir) = spawn_backend_server();
    let change_id = "001-01_remote-start";
    seed_remote_change(
        data_dir.path(),
        change_id,
        Some(
            "# Tasks for: 001-01_remote-start\n\n## Wave 1\n\n- **Depends On**: None\n\n### Task 1.1: First task\n- **Dependencies**: None\n- **Updated At**: 2026-03-01\n- **Status**: [ ] pending\n",
        ),
    );
    write_backend_config(repo.path(), &base_url);

    let out = run_cli(
        &rust_path,
        &["tasks", "start", change_id, "1.1"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stdout={} stderr={}", out.stdout, out.stderr);
    assert!(
        !repo
            .path()
            .join(".ito/changes")
            .join(change_id)
            .join("tasks.md")
            .exists(),
        "remote mode should not create a local tasks.md primary write path"
    );

    let raw =
        std::fs::read_to_string(project_change_dir(data_dir.path(), change_id).join("tasks.md"))
            .expect("read backend tasks");
    assert!(raw.contains("- **Status**: [>] in-progress"), "{raw}");
}

#[test]
fn remote_missing_tasks_commands_do_not_hard_fail() {
    let repo = fixtures::make_empty_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    fixtures::git_init_with_initial_commit(repo.path());

    let (base_url, data_dir) = spawn_backend_server();
    let change_id = "001-02_missing-tasks";
    seed_remote_change(data_dir.path(), change_id, None);
    write_backend_config(repo.path(), &base_url);

    let status = run_cli(
        &rust_path,
        &["tasks", "status", change_id, "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(status.code, 0, "stderr={}", status.stderr);
    assert!(
        status.stdout.contains("\"exists\": false"),
        "{}",
        status.stdout
    );
    // In JSON output the change_id is escaped; check for the unescaped substring
    assert!(
        status.stdout.contains("No backend tasks found for"),
        "{}",
        status.stdout
    );
    assert!(
        status.stdout.contains("ito tasks init"),
        "{}",
        status.stdout
    );

    let ready = run_cli(
        &rust_path,
        &["tasks", "ready", change_id],
        repo.path(),
        home.path(),
    );
    assert_eq!(ready.code, 0, "stderr={}", ready.stderr);
    // In plain text output the message is printed directly
    assert!(
        ready.stdout.contains("No backend tasks found for"),
        "{}",
        ready.stdout
    );

    let show = run_cli(
        &rust_path,
        &["tasks", "show", change_id, "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(show.code, 0, "stderr={}", show.stderr);
    assert!(show.stdout.contains("\"exists\": false"), "{}", show.stdout);
}
