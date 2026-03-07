#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

fn backend_config_path(home: &std::path::Path) -> std::path::PathBuf {
    home.join(".config/ito/config.json")
}

#[test]
fn service_mode_bootstraps_missing_auth_silently() {
    let repo = fixtures::make_empty_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["serve-api", "--service", "--bind", "not-an-address"],
        repo.path(),
        home.path(),
    );

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

    let config_path = backend_config_path(home.path());
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
    let repo = fixtures::make_empty_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let config_path = backend_config_path(home.path());
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

    let out = run_rust_candidate(
        rust_path,
        &["serve-api", "--service", "--bind", "not-an-address"],
        repo.path(),
        home.path(),
    );

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

    let after = std::fs::read_to_string(&config_path).unwrap();
    assert_eq!(after, before);
}

#[test]
fn service_mode_reports_malformed_backend_config() {
    let repo = fixtures::make_empty_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let config_path = backend_config_path(home.path());
    fixtures::write(&config_path, r#"{"backendServer":"bad"}"#);

    let out = run_rust_candidate(
        rust_path,
        &["serve-api", "--service", "--bind", "not-an-address"],
        repo.path(),
        home.path(),
    );

    assert_ne!(out.code, 0);
    assert!(
        out.stderr.contains("'backendServer' must be a JSON object"),
        "stderr={}",
        out.stderr
    );
}
