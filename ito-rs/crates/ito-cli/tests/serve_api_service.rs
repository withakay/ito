#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

fn backend_config_path(home: &std::path::Path) -> std::path::PathBuf {
    home.join(".config/ito/config.json")
}

struct TestContext {
    repo: tempfile::TempDir,
    home: tempfile::TempDir,
    rust_path: &'static std::path::Path,
}

impl TestContext {
    fn new() -> Self {
        Self {
            repo: fixtures::make_empty_repo(),
            home: tempfile::tempdir().expect("home"),
            rust_path: assert_cmd::cargo::cargo_bin!("ito"),
        }
    }

    fn config_path(&self) -> std::path::PathBuf {
        backend_config_path(self.home.path())
    }

    fn run_service_mode(&self) -> ito_test_support::CmdOutput {
        self.run(&["serve-api", "--service", "--bind", "not-an-address"])
    }

    fn run(&self, args: &[&str]) -> ito_test_support::CmdOutput {
        run_rust_candidate(self.rust_path, args, self.repo.path(), self.home.path())
    }
}

fn assert_silent_invalid_address(out: &ito_test_support::CmdOutput) {
    assert_ne!(out.code, 0);
    assert!(
        out.stderr.contains("Invalid address"),
        "stderr={}",
        out.stderr
    );
    assert!(!out.stdout.contains("Generated backend server auth tokens."));
    assert!(
        !out.stdout
            .contains("Backend server auth is already configured.")
    );
}

#[test]
fn service_mode_bootstraps_missing_auth_silently() {
    let cx = TestContext::new();

    let out = cx.run_service_mode();

    assert_silent_invalid_address(&out);

    let config_path = cx.config_path();
    assert!(config_path.exists(), "stderr={}", out.stderr);

    let contents = std::fs::read_to_string(config_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&contents).unwrap();
    assert!(
        parsed["backendServer"]["auth"]["adminTokens"][0]
            .as_str()
            .is_some_and(|token| !token.is_empty())
    );
    assert!(
        parsed["backendServer"]["auth"]["tokenSeed"]
            .as_str()
            .is_some_and(|seed| !seed.is_empty())
    );
}

#[test]
fn service_mode_reuses_existing_auth_without_printing_init_output() {
    let cx = TestContext::new();

    let config_path = cx.config_path();
    fixtures::write(
        &config_path,
        concat!(
            "{\n",
            "  \"backendServer\": {\n",
            "    \"auth\": {\n",
            "      \"adminTokens\": [\"existing-token\"],\n",
            "      \"tokenSeed\": \"existing-seed\"\n",
            "    }\n",
            "  }\n",
            "}\n"
        ),
    );

    let before = std::fs::read_to_string(&config_path).unwrap();

    let out = cx.run_service_mode();

    assert_silent_invalid_address(&out);

    let after = std::fs::read_to_string(&config_path).unwrap();
    assert_eq!(after, before);
}

#[test]
fn service_mode_reports_malformed_backend_config() {
    let cx = TestContext::new();

    let config_path = cx.config_path();
    fixtures::write(&config_path, r#"{"backendServer":"bad"}"#);

    let out = cx.run_service_mode();

    assert_ne!(out.code, 0);
    assert!(
        out.stderr.contains("'backendServer' must be a JSON object"),
        "stderr={}",
        out.stderr
    );
}

#[test]
fn serve_api_reports_unknown_fields_in_explicit_config_file() {
    let cx = TestContext::new();
    let config_path = cx.repo.path().join("backend.json");
    fixtures::write(
        &config_path,
        r#"{"server":{"auth":{"adminTokens":["token"]}}}"#,
    );

    let out = cx.run(&[
        "serve-api",
        "--config",
        config_path.to_str().unwrap(),
        "--bind",
        "not-an-address",
    ]);

    assert_ne!(out.code, 0);
    assert!(
        out.stderr.contains("unknown field(s): server"),
        "stderr={}",
        out.stderr
    );
}
