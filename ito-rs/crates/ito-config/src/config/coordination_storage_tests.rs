use super::*;

#[test]
fn coordination_storage_default_is_worktree() {
    assert_eq!(
        CoordinationStorage::default(),
        CoordinationStorage::Worktree
    );
}

#[test]
fn coordination_storage_serializes_worktree_as_lowercase() {
    let json = serde_json::to_string(&CoordinationStorage::Worktree).unwrap();
    assert_eq!(json, "\"worktree\"");
}

#[test]
fn coordination_storage_serializes_embedded_as_lowercase() {
    let json = serde_json::to_string(&CoordinationStorage::Embedded).unwrap();
    assert_eq!(json, "\"embedded\"");
}

#[test]
fn coordination_storage_round_trips_worktree() {
    let original = CoordinationStorage::Worktree;
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: CoordinationStorage = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, original);
}

#[test]
fn coordination_storage_round_trips_embedded() {
    let original = CoordinationStorage::Embedded;
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: CoordinationStorage = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, original);
}

#[test]
fn coordination_branch_config_missing_storage_defaults_to_worktree() {
    let json = r#"{"enabled": true, "name": "ito/internal/changes"}"#;
    let config: CoordinationBranchConfig = serde_json::from_str(json).unwrap();
    assert_eq!(config.storage, CoordinationStorage::Worktree);
}

#[test]
fn coordination_branch_config_missing_worktree_path_is_none() {
    let json = r#"{"enabled": true, "name": "ito/internal/changes"}"#;
    let config: CoordinationBranchConfig = serde_json::from_str(json).unwrap();
    assert!(config.worktree_path.is_none());
}

#[test]
fn coordination_branch_config_worktree_path_round_trips() {
    let config = CoordinationBranchConfig {
        worktree_path: Some("/tmp/my-worktree".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: CoordinationBranchConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(
        deserialized.worktree_path,
        Some("/tmp/my-worktree".to_string())
    );
}

#[test]
fn coordination_branch_config_worktree_path_absent_not_serialized() {
    let config = CoordinationBranchConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    assert!(!json.contains("worktree_path"));
}
