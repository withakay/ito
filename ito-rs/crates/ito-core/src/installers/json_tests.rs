//! Tests for JSON-related installer helpers: `merge_json_objects`,
//! `classify_project_file_ownership`, and `write_claude_settings`.

use super::*;

#[test]
fn merge_json_objects_keeps_existing_and_adds_template_keys() {
    let mut existing = serde_json::json!({
        "permissions": {
            "allow": ["Bash(ls)"]
        },
        "hooks": {
            "SessionStart": [
                {
                    "matcher": "*"
                }
            ]
        }
    });
    let template = serde_json::json!({
        "hooks": {
            "PreToolUse": [
                {
                    "matcher": "Bash|Edit|Write",
                    "hooks": [
                        {
                            "type": "command",
                            "command": "bash .claude/hooks/ito-audit.sh"
                        }
                    ]
                }
            ]
        }
    });

    merge_json_objects(&mut existing, &template);

    assert_eq!(
        existing
            .pointer("/permissions/allow/0")
            .and_then(Value::as_str),
        Some("Bash(ls)")
    );
    assert!(existing.pointer("/hooks/SessionStart/0/matcher").is_some());
    assert!(
        existing
            .pointer("/hooks/PreToolUse/0/hooks/0/command")
            .is_some()
    );
}

#[test]
fn classify_project_file_ownership_handles_user_owned_paths() {
    let ito_dir = ".ito";

    assert_eq!(
        classify_project_file_ownership(".ito/project.md", ito_dir),
        FileOwnership::UserOwned
    );
    assert_eq!(
        classify_project_file_ownership(".ito/config.json", ito_dir),
        FileOwnership::UserOwned
    );
    assert_eq!(
        classify_project_file_ownership(".ito/user-guidance.md", ito_dir),
        FileOwnership::UserOwned
    );
    assert_eq!(
        classify_project_file_ownership(".ito/user-prompts/tasks.md", ito_dir),
        FileOwnership::UserOwned
    );
    assert_eq!(
        classify_project_file_ownership(".ito/commands/review-edge.md", ito_dir),
        FileOwnership::ItoManaged
    );
}

#[test]
fn write_claude_settings_merges_existing_file_on_update() {
    let td = tempfile::tempdir().unwrap();
    let target = td.path().join(".claude/settings.json");
    std::fs::create_dir_all(target.parent().unwrap()).unwrap();
    std::fs::write(
        &target,
        "{\n  \"permissions\": {\n    \"allow\": [\"Bash(ls)\"]\n  }\n}\n",
    )
    .unwrap();

    let template = br#"{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash|Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "bash .claude/hooks/ito-audit.sh"
          }
        ]
      }
    ]
  }
}
"#;

    let opts = InitOptions::new(BTreeSet::new(), false, true);
    write_claude_settings(&target, template, InstallMode::Update, &opts).unwrap();

    let updated = std::fs::read_to_string(&target).unwrap();
    let value: Value = serde_json::from_str(&updated).unwrap();
    assert!(value.pointer("/permissions/allow").is_some());
    assert!(
        value
            .pointer("/hooks/PreToolUse/0/hooks/0/command")
            .is_some()
    );
}

#[test]
fn merge_json_objects_appends_and_deduplicates_array_entries() {
    let mut existing = serde_json::json!({
        "permissions": {
            "allow": ["Bash(ls)"]
        },
        "hooks": {
            "PreToolUse": [
                {
                    "matcher": "Bash",
                    "hooks": [{"type": "command", "command": "echo existing"}]
                }
            ]
        }
    });
    let template = serde_json::json!({
        "permissions": {
            "allow": ["Bash(ls)", "Bash(git status)"]
        },
        "hooks": {
            "PreToolUse": [
                {
                    "matcher": "Bash",
                    "hooks": [{"type": "command", "command": "echo existing"}]
                },
                {
                    "matcher": "Edit|Write",
                    "hooks": [{"type": "command", "command": "echo template"}]
                }
            ]
        }
    });

    merge_json_objects(&mut existing, &template);

    let permissions = existing
        .pointer("/permissions/allow")
        .and_then(Value::as_array)
        .expect("permissions allow should remain an array");
    assert_eq!(permissions.len(), 2);
    assert_eq!(permissions[0].as_str(), Some("Bash(ls)"));
    assert_eq!(permissions[1].as_str(), Some("Bash(git status)"));

    let hooks = existing
        .pointer("/hooks/PreToolUse")
        .and_then(Value::as_array)
        .expect("PreToolUse should remain an array");
    assert_eq!(hooks.len(), 2);
    assert_eq!(
        hooks[0].pointer("/hooks/0/command").and_then(Value::as_str),
        Some("echo existing")
    );
    assert_eq!(
        hooks[1].pointer("/hooks/0/command").and_then(Value::as_str),
        Some("echo template")
    );
}

#[test]
fn write_claude_settings_preserves_invalid_json_on_update() {
    let td = tempfile::tempdir().unwrap();
    let target = td.path().join(".claude/settings.json");
    std::fs::create_dir_all(target.parent().unwrap()).unwrap();
    std::fs::write(&target, "not-json\n").unwrap();

    let template = br#"{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash|Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "bash .claude/hooks/ito-audit.sh"
          }
        ]
      }
    ]
  }
}
"#;

    let opts = InitOptions::new(BTreeSet::new(), false, true);
    write_claude_settings(&target, template, InstallMode::Update, &opts).unwrap();

    let updated = std::fs::read_to_string(&target).unwrap();
    assert_eq!(updated, "not-json\n");
}
