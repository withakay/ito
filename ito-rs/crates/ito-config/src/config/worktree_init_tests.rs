use super::*;

#[test]
fn worktree_init_config_default_has_empty_include_and_no_setup() {
    let config = WorktreeInitConfig::default();
    assert!(config.include.is_empty());
    assert!(config.setup.is_none());
}

#[test]
fn worktree_init_config_deserializes_with_include_only() {
    let json = r#"{"include": [".env", ".envrc"]}"#;
    let config: WorktreeInitConfig = serde_json::from_str(json).unwrap();
    assert_eq!(config.include, vec![".env", ".envrc"]);
    assert!(config.setup.is_none());
}

#[test]
fn worktree_init_config_absent_deserializes_to_default() {
    let json = r#"{}"#;
    let config: WorktreeInitConfig = serde_json::from_str(json).unwrap();
    assert!(config.include.is_empty());
    assert!(config.setup.is_none());
}

#[test]
fn worktree_setup_config_single_string_deserializes() {
    let json = r#""make init""#;
    let config: WorktreeSetupConfig = serde_json::from_str(json).unwrap();
    assert_eq!(config.to_commands(), vec!["make init"]);
}

#[test]
fn worktree_setup_config_array_deserializes() {
    let json = r#"["npm ci", "npm run build:types"]"#;
    let config: WorktreeSetupConfig = serde_json::from_str(json).unwrap();
    assert_eq!(config.to_commands(), vec!["npm ci", "npm run build:types"]);
}

#[test]
fn worktree_setup_config_single_round_trips() {
    let original = WorktreeSetupConfig::Single("make init".to_string());
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: WorktreeSetupConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.to_commands(), vec!["make init"]);
}

#[test]
fn worktree_setup_config_multiple_round_trips() {
    let original =
        WorktreeSetupConfig::Multiple(vec!["npm ci".to_string(), "npm run build".to_string()]);
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: WorktreeSetupConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.to_commands(), vec!["npm ci", "npm run build"]);
}

#[test]
fn worktree_setup_config_is_empty_single_empty_string() {
    let config = WorktreeSetupConfig::Single(String::new());
    assert!(config.is_empty());
}

#[test]
fn worktree_setup_config_is_empty_multiple_empty_vec() {
    let config = WorktreeSetupConfig::Multiple(Vec::new());
    assert!(config.is_empty());
}

#[test]
fn worktree_setup_config_is_not_empty_with_command() {
    let config = WorktreeSetupConfig::Single("make init".to_string());
    assert!(!config.is_empty());
}

#[test]
fn worktree_init_config_with_single_setup_deserializes() {
    let json = r#"{"include": [".env"], "setup": "make init"}"#;
    let config: WorktreeInitConfig = serde_json::from_str(json).unwrap();
    assert_eq!(config.include, vec![".env"]);
    let setup = config.setup.unwrap();
    assert_eq!(setup.to_commands(), vec!["make init"]);
}

#[test]
fn worktree_init_config_with_multiple_setup_deserializes() {
    let json = r#"{"include": [".env"], "setup": ["npm ci", "npm run build"]}"#;
    let config: WorktreeInitConfig = serde_json::from_str(json).unwrap();
    assert_eq!(config.include, vec![".env"]);
    let setup = config.setup.unwrap();
    assert_eq!(setup.to_commands(), vec!["npm ci", "npm run build"]);
}

#[test]
fn worktrees_config_with_init_section_deserializes() {
    let json = r#"{
        "enabled": true,
        "init": {
            "include": [".env", ".envrc"],
            "setup": "make init"
        }
    }"#;
    let config: WorktreesConfig = serde_json::from_str(json).unwrap();
    assert!(config.enabled);
    assert_eq!(config.init.include, vec![".env", ".envrc"]);
    let setup = config.init.setup.unwrap();
    assert_eq!(setup.to_commands(), vec!["make init"]);
}

#[test]
fn worktrees_config_without_init_section_uses_defaults() {
    let json = r#"{"enabled": true}"#;
    let config: WorktreesConfig = serde_json::from_str(json).unwrap();
    assert!(config.enabled);
    assert!(config.init.include.is_empty());
    assert!(config.init.setup.is_none());
}

#[test]
fn worktrees_config_init_does_not_break_existing_fields() {
    let json = r#"{
        "enabled": true,
        "strategy": "bare_control_siblings",
        "default_branch": "develop",
        "init": {"include": [".env"]}
    }"#;
    let config: WorktreesConfig = serde_json::from_str(json).unwrap();
    assert!(config.enabled);
    assert_eq!(config.strategy, WorktreeStrategy::BareControlSiblings);
    assert_eq!(config.default_branch, "develop");
    assert_eq!(config.init.include, vec![".env"]);
}

#[test]
fn full_ito_config_with_worktree_init_round_trips() {
    let json = r#"{
        "worktrees": {
            "enabled": true,
            "init": {
                "include": [".env", "*.local.toml"],
                "setup": ["npm ci", "npm run build:types"]
            }
        }
    }"#;
    let config: ItoConfig = serde_json::from_str(json).unwrap();
    assert!(config.worktrees.enabled);
    assert_eq!(config.worktrees.init.include, vec![".env", "*.local.toml"]);
    let setup = config.worktrees.init.setup.unwrap();
    assert_eq!(setup.to_commands(), vec!["npm ci", "npm run build:types"]);
}
