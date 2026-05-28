use super::*;

#[test]
fn invalid_command_logger_writes_jsonl_entry() {
    let dir = tempfile::tempdir().unwrap();
    let config_dir = dir.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();

    let project_root = dir.path().join("project");
    std::fs::create_dir_all(&project_root).unwrap();

    let logger =
        InvalidCommandLogger::new(Some(config_dir.clone()), &project_root, None, "0.0.0-test")
            .expect("logger should be created");

    logger.log_invalid_command(
        &[
            "agent".to_string(),
            "instruction".to_string(),
            "nonexistent".to_string(),
        ],
        "Unknown artifact 'nonexistent'",
    );

    // Find the written log file.
    let logs_dir = config_dir
        .join("logs")
        .join("invalid_commands")
        .join("v1")
        .join("projects");
    assert!(logs_dir.exists(), "logs directory should exist");

    let mut found = false;
    for entry in std::fs::read_dir(logs_dir).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            for file in std::fs::read_dir(entry.path()).unwrap() {
                let file = file.unwrap();
                let contents = std::fs::read_to_string(file.path()).unwrap();
                assert!(contents.contains("\"event_type\":\"invalid_command\""));
                assert!(contents.contains("ito agent instruction nonexistent"));
                assert!(contents.contains("Unknown artifact"));
                found = true;
            }
        }
    }
    assert!(found, "should have written at least one log entry");
}

#[test]
fn unsafe_session_ids_are_rejected() {
    assert!(!is_safe_session_id(""));
    assert!(!is_safe_session_id("../escape"));
    assert!(!is_safe_session_id("a/b"));
    assert!(!is_safe_session_id("abc def"));
    assert!(is_safe_session_id(
        "1739330000-550e8400e29b41d4a716446655440000"
    ));
}
