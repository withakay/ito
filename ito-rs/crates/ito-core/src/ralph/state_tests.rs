use super::*;

#[test]
fn ralph_state_dir_uses_safe_fallback_for_invalid_change_ids() {
    let ito = std::path::Path::new("/tmp/repo/.ito");
    let path = ralph_state_dir(ito, "../escape");
    assert!(path.ends_with(".state/ralph/invalid-change-id"));
}

#[test]
fn save_and_load_state_round_trip() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let state = RalphState {
        change_id: change_id.to_string(),
        iteration: 5,
        history: vec![RalphHistoryEntry {
            timestamp: 1234567890,
            duration: 5000,
            completion_promise_found: true,
            file_changes_count: 3,
            harness_exit_code: 0,
            completion_validated: true,
            effective_cwd: "/tmp/worktree".to_string(),
        }],
        context_file: ".ito/.state/ralph/001-01_test/context.md".to_string(),
        last_outcome: Some("validated-complete".to_string()),
        last_failure: None,
    };
    save_state(&ito, change_id, &state).unwrap();
    let loaded = load_state(&ito, change_id).unwrap();
    assert!(loaded.is_some());
    let loaded = loaded.unwrap();
    assert_eq!(loaded.change_id, state.change_id);
    assert_eq!(loaded.iteration, state.iteration);
    assert_eq!(loaded.history.len(), state.history.len());
    assert_eq!(loaded.history[0].timestamp, state.history[0].timestamp);
    assert_eq!(loaded.history[0].duration, state.history[0].duration);
    assert_eq!(
        loaded.history[0].completion_promise_found,
        state.history[0].completion_promise_found
    );
    assert_eq!(
        loaded.history[0].file_changes_count,
        state.history[0].file_changes_count
    );
    assert_eq!(loaded.history[0].harness_exit_code, 0);
    assert!(loaded.history[0].completion_validated);
    assert_eq!(loaded.history[0].effective_cwd, "/tmp/worktree");
    assert_eq!(loaded.context_file, state.context_file);
    assert_eq!(loaded.last_outcome.as_deref(), Some("validated-complete"));
    assert_eq!(loaded.last_failure, None);
}

#[test]
fn load_state_returns_none_when_missing() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let result = load_state(&ito, "nonexistent").unwrap();
    assert!(result.is_none());
}

#[test]
fn load_state_backfills_missing_new_fields() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let dir = ralph_state_dir(&ito, change_id);
    std::fs::create_dir_all(&dir).unwrap();
    let raw = r#"{
  "changeId": "001-01_test",
  "iteration": 2,
  "history": [
    {
      "timestamp": 1,
      "duration": 2,
      "completionPromiseFound": true,
      "fileChangesCount": 3
    }
  ],
  "contextFile": ".ito/.state/ralph/001-01_test/context.md"
}"#;
    std::fs::write(ralph_state_json_path(&ito, change_id), raw).unwrap();

    let loaded = load_state(&ito, change_id).unwrap().unwrap();
    assert_eq!(loaded.history[0].harness_exit_code, 0);
    assert!(!loaded.history[0].completion_validated);
    assert_eq!(loaded.history[0].effective_cwd, "");
    assert_eq!(loaded.last_outcome, None);
    assert_eq!(loaded.last_failure, None);
}

#[test]
fn is_safe_change_id_segment_rejects_empty() {
    let ito = tempfile::tempdir().unwrap();
    let ito_path = ito.path().join(".ito");
    let path = ralph_state_dir(&ito_path, "");
    assert!(path.ends_with(".state/ralph/invalid-change-id"));
}

#[test]
fn is_safe_change_id_segment_rejects_too_long() {
    let ito = tempfile::tempdir().unwrap();
    let ito_path = ito.path().join(".ito");
    let long_id = "a".repeat(257);
    let path = ralph_state_dir(&ito_path, &long_id);
    assert!(path.ends_with(".state/ralph/invalid-change-id"));
}

#[test]
fn is_safe_change_id_segment_rejects_backslash() {
    let ito = tempfile::tempdir().unwrap();
    let ito_path = ito.path().join(".ito");
    let path = ralph_state_dir(&ito_path, "foo\\bar");
    assert!(path.ends_with(".state/ralph/invalid-change-id"));
}

#[test]
fn is_safe_change_id_segment_accepts_valid() {
    let ito = tempfile::tempdir().unwrap();
    let ito_path = ito.path().join(".ito");
    let path = ralph_state_dir(&ito_path, "003-05_my-change");
    assert!(path.ends_with("003-05_my-change"));
}

#[test]
fn append_context_no_op_on_whitespace() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    append_context(&ito, change_id, "   \n  ").unwrap();
    let context_path = ralph_context_path(&ito, change_id);
    if context_path.exists() {
        let content = ito_common::io::read_to_string_std(&context_path).unwrap();
        assert!(content.is_empty());
    }
}

#[test]
fn load_context_returns_empty_when_missing() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let result = load_context(&ito, "nonexistent").unwrap();
    assert_eq!(result, "");
}

#[test]
fn ralph_state_json_path_correct() {
    let ito = std::path::Path::new("/tmp/repo/.ito");
    let path = ralph_state_json_path(ito, "001-01_test");
    assert!(path.ends_with("state.json"));
}

#[test]
fn ralph_context_path_correct() {
    let ito = std::path::Path::new("/tmp/repo/.ito");
    let path = ralph_context_path(ito, "001-01_test");
    assert!(path.ends_with("context.md"));
}
