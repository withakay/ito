#[path = "support/mod.rs"]
mod fixtures;

use std::collections::BTreeMap;
use std::path::Path;
use std::time::Duration;

use ito_config::types::{BackendAllowlistConfig, BackendAuthConfig, BackendRepoPolicy};
use ito_test_support::{CmdOutput, rust_candidate_command};

const ORG: &str = "acme";
const REPO: &str = "widgets";
const ADMIN_TOKEN: &str = "cli-remote-archive-token";

fn seed_remote_change(data_dir: &Path, change_id: &str) {
    let change_dir = data_dir
        .join("projects")
        .join(ORG)
        .join(REPO)
        .join(".ito")
        .join("changes")
        .join(change_id);
    std::fs::create_dir_all(change_dir.join("specs/spec-one")).expect("spec dir");
    fixtures::write(change_dir.join("proposal.md"), "# Proposal\n");
    fixtures::write(change_dir.join("tasks.md"), "- [x] done\n");
    fixtures::write(
        change_dir.join("specs/spec-one/spec.md"),
        "## ADDED Requirements\n",
    );
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
        token_seed: Some("cli-remote-archive-seed".to_string()),
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
    let out = cmd.output().expect("command should run");
    CmdOutput {
        code: out.status.code().unwrap_or(1),
        stdout: String::from_utf8_lossy(&out.stdout).to_string(),
        stderr: String::from_utf8_lossy(&out.stderr).to_string(),
    }
}

#[test]
fn remote_archive_succeeds_without_local_active_change_markdown() {
    let repo = fixtures::make_empty_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    fixtures::git_init_with_initial_commit(repo.path());

    let (base_url, data_dir) = spawn_backend_server();
    let change_id = "025-05_archive-me";
    seed_remote_change(data_dir.path(), change_id);
    write_backend_config(repo.path(), &base_url);

    let out = run_cli(
        rust_path,
        &["archive", change_id, "--yes"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stdout={} stderr={}", out.stdout, out.stderr);
    assert!(repo.path().join(".ito/specs/spec-one/spec.md").exists());
    assert!(repo.path().join(".ito/changes/archive").exists());
    assert!(
        !data_dir
            .path()
            .join("projects")
            .join(ORG)
            .join(REPO)
            .join(".ito/changes")
            .join(change_id)
            .exists()
    );
}
