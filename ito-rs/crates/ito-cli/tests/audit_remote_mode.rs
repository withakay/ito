#[path = "support/mod.rs"]
mod fixtures;

use std::collections::BTreeMap;
use std::path::Path;
use std::time::Duration;

use ito_config::types::{BackendAllowlistConfig, BackendAuthConfig, BackendRepoPolicy};
use ito_test_support::{CmdOutput, rust_candidate_command};

const ORG: &str = "acme";
const REPO: &str = "widgets";
const ADMIN_TOKEN: &str = "cli-remote-audit-token";

fn project_change_dir(data_dir: &Path, change_id: &str) -> std::path::PathBuf {
    data_dir
        .join("projects")
        .join(ORG)
        .join(REPO)
        .join(".ito")
        .join("changes")
        .join(change_id)
}

fn seed_remote_change(data_dir: &Path, change_id: &str, tasks: &str) {
    let change_dir = project_change_dir(data_dir, change_id);
    std::fs::create_dir_all(&change_dir).expect("create remote change dir");
    fixtures::write(change_dir.join("proposal.md"), "# Proposal\n");
    fixtures::write(change_dir.join("tasks.md"), tasks);
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
        token_seed: Some("cli-remote-audit-seed".to_string()),
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
fn audit_commands_in_backend_mode_use_server_only_storage() {
    let repo = fixtures::make_empty_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    fixtures::git_init_with_initial_commit(repo.path());

    let (base_url, data_dir) = spawn_backend_server();
    let change_id = "001-03_remote-audit";
    seed_remote_change(
        data_dir.path(),
        change_id,
        "# Tasks for: 001-03_remote-audit\n\n## Wave 1\n\n- **Depends On**: None\n\n### Task 1.1: First task\n- **Dependencies**: None\n- **Updated At**: 2026-03-01\n- **Status**: [ ] pending\n",
    );
    write_backend_config(repo.path(), &base_url);

    let start = run_cli(
        rust_path,
        &["tasks", "start", change_id, "1.1"],
        repo.path(),
        home.path(),
    );
    assert_eq!(
        start.code, 0,
        "stdout={} stderr={}",
        start.stdout, start.stderr
    );

    assert!(
        !repo.path().join(".ito/.state/audit/events.jsonl").exists(),
        "backend mode should not create a tracked local audit log"
    );

    let log = run_cli(
        rust_path,
        &["audit", "log", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(log.code, 0, "stdout={} stderr={}", log.stdout, log.stderr);
    let events: serde_json::Value = serde_json::from_str(&log.stdout).expect("audit log json");
    let events = events.as_array().expect("audit log array");
    assert_eq!(events.len(), 1, "{events:?}");
    assert_eq!(events[0]["entity_id"], "1.1");
    assert_eq!(events[0]["op"], "status_change");

    let validate = run_cli(
        rust_path,
        &["audit", "validate", "--json", "--change", change_id],
        repo.path(),
        home.path(),
    );
    assert_eq!(
        validate.code, 0,
        "stdout={} stderr={}",
        validate.stdout, validate.stderr
    );
    let report: serde_json::Value =
        serde_json::from_str(&validate.stdout).expect("audit validate json");
    assert_eq!(report["event_count"], 1);
}
