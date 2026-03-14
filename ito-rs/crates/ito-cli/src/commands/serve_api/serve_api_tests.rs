use super::*;
use std::collections::BTreeMap;
use tempfile::tempdir;

#[test]
fn builds_config_with_defaults() {
    let args = BackendServeArgs {
        init: false,
        service: false,
        port: None,
        bind: None,
        data_dir: None,
        admin_token: vec![],
        token_seed: None,
        allow_org: vec![],
        config: None,
    };
    assert!(args.port.is_none());
    assert!(args.bind.is_none());
}

#[test]
fn builds_allowlist_from_allow_org_args() {
    let orgs = vec!["acme".to_string(), "globex".to_string()];
    let mut repos = BTreeMap::new();
    for org in &orgs {
        repos.insert(org.clone(), BackendRepoPolicy::All("*".to_string()));
    }
    let allowlist = BackendAllowlistConfig {
        orgs: orgs.clone(),
        repos,
    };
    assert!(allowlist.is_allowed("acme", "any-repo"));
    assert!(allowlist.is_allowed("globex", "another-repo"));
    assert!(!allowlist.is_allowed("unknown-org", "repo"));
}

#[test]
fn merge_allow_orgs_preserves_existing_repo_rules() {
    let mut repos = BTreeMap::new();
    repos.insert(
        "withakay".to_string(),
        BackendRepoPolicy::List(vec!["ito".to_string()]),
    );
    let allowlist = BackendAllowlistConfig {
        orgs: vec!["withakay".to_string()],
        repos,
    };

    let merged = merge_allow_orgs(allowlist, &["acme".to_string()]);

    assert!(merged.is_allowed("withakay", "ito"));
    assert!(!merged.is_allowed("withakay", "other"));
    assert!(merged.is_allowed("acme", "anything"));
}

#[test]
fn load_backend_server_config_file_rejects_unknown_json_fields() {
    let temp = tempdir().unwrap();
    let path = temp.path().join("backend.json");
    std::fs::write(&path, r#"{"server":{"auth":{}}}"#).unwrap();

    let err = load_backend_server_config_file(&path).unwrap_err();
    assert!(err.to_string().contains("unknown field(s): server"));
}

#[test]
fn load_backend_server_config_file_rejects_trailing_json_content() {
    let temp = tempdir().unwrap();
    let path = temp.path().join("backend.json");
    std::fs::write(&path, r#"{"bind":"127.0.0.1"} garbage"#).unwrap();

    let err = load_backend_server_config_file(&path).unwrap_err();
    assert!(err.to_string().contains("trailing characters"));
}

#[test]
fn load_backend_server_config_file_reads_toml() {
    let temp = tempdir().unwrap();
    let path = temp.path().join("backend.toml");
    std::fs::write(
        &path,
        "bind = \"0.0.0.0\"\nport = 9020\n[auth]\nadminTokens = [\"token\"]\n",
    )
    .unwrap();

    let config = load_backend_server_config_file(&path).unwrap();
    assert_eq!(config.bind, "0.0.0.0");
    assert_eq!(config.port, 9020);
    assert_eq!(config.auth.admin_tokens, vec!["token".to_string()]);
}

#[test]
fn load_backend_server_config_file_accepts_full_ito_json_config() {
    let temp = tempdir().unwrap();
    let path = temp.path().join("config.json");
    std::fs::write(
        &path,
        r#"{
            "backendServer": {
                "bind": "0.0.0.0",
                "port": 9030,
                "auth": {
                    "adminTokens": ["token"]
                }
            },
            "changes": {
                "coordination_branch": {
                    "enabled": false
                }
            }
        }"#,
    )
    .unwrap();

    let config = load_backend_server_config_file(&path).unwrap();
    assert_eq!(config.bind, "0.0.0.0");
    assert_eq!(config.port, 9030);
    assert_eq!(config.auth.admin_tokens, vec!["token".to_string()]);
}
