use super::*;
use std::fs;

fn write(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

#[test]
fn task_completion_passes_when_no_tasks() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    fs::create_dir_all(&ito).unwrap();
    let task_repo = crate::task_repository::FsTaskRepository::new(&ito);
    let r = check_task_completion(&task_repo, "001-01_missing").unwrap();
    assert!(r.success);
}

#[test]
fn task_completion_fails_when_remaining() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    fs::create_dir_all(ito.join("changes/001-01_test")).unwrap();
    write(
        &ito.join("changes/001-01_test/tasks.md"),
        "# Tasks\n\n- [x] done\n- [ ] todo\n",
    );
    let task_repo = crate::task_repository::FsTaskRepository::new(&ito);
    let r = check_task_completion(&task_repo, "001-01_test").unwrap();
    assert!(!r.success);
}

#[test]
fn project_validation_discovers_commands_from_repo_json() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    fs::create_dir_all(&ito).unwrap();
    write(
        &project_root.join("ito.json"),
        r#"{ "ralph": { "validationCommands": ["true"] } }"#,
    );
    let cmds = discover_project_validation_commands(project_root, &ito).unwrap();
    assert_eq!(cmds, vec!["true".to_string()]);
}

#[test]
fn shell_timeout_is_failure() {
    let td = tempfile::tempdir().unwrap();
    let out = run_shell_with_timeout(td.path(), "sleep 0.1", Duration::from_millis(50)).unwrap();
    assert!(out.timed_out);
    assert!(!out.success);
}

#[test]
fn extract_commands_from_markdown_finds_make_check() {
    let markdown = "Some text\nmake check\nMore text";
    let commands = extract_commands_from_markdown(markdown);
    assert_eq!(commands, vec!["make check"]);
}

#[test]
fn extract_commands_from_markdown_finds_make_test() {
    let markdown = "Some text\nmake test\nMore text";
    let commands = extract_commands_from_markdown(markdown);
    assert_eq!(commands, vec!["make test"]);
}

#[test]
fn extract_commands_from_markdown_ignores_other_lines() {
    let markdown = "echo hello\nsome other text";
    let commands = extract_commands_from_markdown(markdown);
    assert!(commands.is_empty());
}

#[test]
fn normalize_commands_value_string() {
    let value = Value::String("make test".to_string());
    let commands = normalize_commands_value(&value);
    assert_eq!(commands, vec!["make test"]);
}

#[test]
fn normalize_commands_value_array() {
    let value = Value::Array(vec![
        Value::String("make test".to_string()),
        Value::String("make lint".to_string()),
    ]);
    let commands = normalize_commands_value(&value);
    assert_eq!(commands, vec!["make test", "make lint"]);
}

#[test]
fn normalize_commands_value_null() {
    let value = Value::Null;
    let commands = normalize_commands_value(&value);
    assert!(commands.is_empty());
}

#[test]
fn normalize_commands_value_non_string() {
    let value = Value::Number(serde_json::Number::from(42));
    let commands = normalize_commands_value(&value);
    assert!(commands.is_empty());
}

#[test]
fn truncate_for_context_short_unchanged() {
    let short_text = "a".repeat(1000);
    let result = truncate_for_context(&short_text, 12_000);
    assert_eq!(result, short_text);
}

#[test]
fn truncate_for_context_long_truncated() {
    let long_text = "a".repeat(15_000);
    let result = truncate_for_context(&long_text, 12_000);
    assert!(result.len() < long_text.len());
    assert!(result.contains("... (truncated) ..."));
}

#[test]
fn truncate_for_context_multibyte_utf8() {
    // Each CJK character is 3 bytes in UTF-8.
    let text = "\u{65E5}".repeat(5_000); // 15,000 bytes
    let result = truncate_for_context(&text, 12_000);
    assert!(result.contains("... (truncated) ..."));
    // The truncated portion must be valid UTF-8 (no panic, no replacement chars).
    assert!(!result.contains('\u{FFFD}'));
}

#[test]
fn extract_commands_from_json_multiple_paths() {
    let json_str = r#"{ "ralph": { "validationCommands": ["make check"] } }"#;
    let value: Value = serde_json::from_str(json_str).unwrap();
    let commands = extract_commands_from_json_value(&value);
    assert_eq!(commands, vec!["make check"]);

    let json_str2 = r#"{ "project": { "validation": { "commands": ["make test"] } } }"#;
    let value2: Value = serde_json::from_str(json_str2).unwrap();
    let commands2 = extract_commands_from_json_value(&value2);
    assert_eq!(commands2, vec!["make test"]);

    let json_str3 = r#"{ "validationCommands": ["make lint"] }"#;
    let value3: Value = serde_json::from_str(json_str3).unwrap();
    let commands3 = extract_commands_from_json_value(&value3);
    assert_eq!(commands3, vec!["make lint"]);
}

#[test]
fn run_extra_validation_success() {
    let td = tempfile::tempdir().unwrap();
    let result = run_extra_validation(td.path(), "true", Duration::from_secs(10)).unwrap();
    assert!(result.success);
    assert!(result.message.contains("passed"));
}

#[test]
fn run_extra_validation_failure() {
    let td = tempfile::tempdir().unwrap();
    let result = run_extra_validation(td.path(), "false", Duration::from_secs(10)).unwrap();
    assert!(!result.success);
    assert!(result.message.contains("failed"));
}

#[test]
fn discover_commands_priority_ito_json_first() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito_path = project_root.join(".ito");
    fs::create_dir_all(&ito_path).unwrap();

    write(
        &project_root.join("ito.json"),
        r#"{"ralph":{"validationCommands":["make ito-check"]}}"#,
    );
    write(&project_root.join("AGENTS.md"), "make check");

    let commands = discover_project_validation_commands(project_root, &ito_path).unwrap();
    assert_eq!(commands, vec!["make ito-check"]);
}

#[test]
fn discover_commands_falls_back_to_agents_md() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito_path = project_root.join(".ito");
    fs::create_dir_all(&ito_path).unwrap();

    write(&project_root.join("AGENTS.md"), "make test");

    let commands = discover_project_validation_commands(project_root, &ito_path).unwrap();
    assert_eq!(commands, vec!["make test"]);
}

#[test]
fn discover_commands_falls_back_to_claude_md() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito_path = project_root.join(".ito");
    fs::create_dir_all(&ito_path).unwrap();

    write(&project_root.join("CLAUDE.md"), "make check");

    let commands = discover_project_validation_commands(project_root, &ito_path).unwrap();
    assert_eq!(commands, vec!["make check"]);
}

#[test]
fn discover_commands_ito_config_json() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito_path = project_root.join(".ito");
    fs::create_dir_all(&ito_path).unwrap();

    write(
        &ito_path.join("config.json"),
        r#"{"validationCommand": "make lint"}"#,
    );

    let commands = discover_project_validation_commands(project_root, &ito_path).unwrap();
    assert_eq!(commands, vec!["make lint"]);
}

#[test]
fn discover_commands_returns_empty_when_nothing_configured() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito_path = project_root.join(".ito");
    fs::create_dir_all(&ito_path).unwrap();

    let commands = discover_project_validation_commands(project_root, &ito_path).unwrap();
    assert!(commands.is_empty());
}
