#[path = "support/mod.rs"]
mod fixtures;

use std::collections::BTreeMap;
use std::path::Path;
use std::time::Duration;

use ito_config::types::{BackendAllowlistConfig, BackendAuthConfig, BackendRepoPolicy};
use ito_test_support::{CmdOutput, rust_candidate_command};

const ORG: &str = "acme";
const REPO: &str = "widgets";
const ADMIN_TOKEN: &str = "cli-remote-spec-token";

fn project_specs_dir(data_dir: &Path) -> std::path::PathBuf {
    data_dir
        .join("projects")
        .join(ORG)
        .join(REPO)
        .join(".ito")
        .join("specs")
}

fn seed_remote_specs(data_dir: &Path) {
    let specs_dir = project_specs_dir(data_dir);
    std::fs::create_dir_all(specs_dir.join("beta")).expect("beta spec dir");
    std::fs::create_dir_all(specs_dir.join("alpha")).expect("alpha spec dir");
    fixtures::write(specs_dir.join("beta/spec.md"), "# Beta\n");
    fixtures::write(specs_dir.join("alpha/spec.md"), "# Alpha\n");
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
        token_seed: Some("cli-remote-spec-seed".to_string()),
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
fn show_specs_reads_backend_specs_without_local_markdown() {
    let repo = fixtures::make_empty_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    fixtures::git_init_with_initial_commit(repo.path());

    let (base_url, data_dir) = spawn_backend_server();
    seed_remote_specs(data_dir.path());
    write_backend_config(repo.path(), &base_url);

    let output = run_cli(
        &rust_path,
        &["show", "specs", "--json"],
        repo.path(),
        home.path(),
    );

    assert_eq!(
        output.code, 0,
        "stdout={} stderr={}",
        output.stdout, output.stderr
    );
    let json: serde_json::Value = serde_json::from_str(&output.stdout).expect("json output");
    let specs = json["specs"].as_array().expect("spec list");
    assert_eq!(specs.len(), 2);
    assert_eq!(specs[0]["id"].as_str(), Some("alpha"));
    assert_eq!(specs[1]["id"].as_str(), Some("beta"));
    assert!(
        specs[0]["markdown"]
            .as_str()
            .unwrap_or_default()
            .contains("# Alpha")
    );
    assert!(
        specs[1]["markdown"]
            .as_str()
            .unwrap_or_default()
            .contains("# Beta")
    );
    assert!(
        !repo.path().join(".ito/specs").exists(),
        "remote mode should not require local promoted spec markdown"
    );
}
