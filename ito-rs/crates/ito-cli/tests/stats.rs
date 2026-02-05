use predicates::str::contains;

fn make_repo() -> tempfile::TempDir {
    let td = tempfile::tempdir().expect("repo");
    std::fs::write(td.path().join("README.md"), "# temp\n").unwrap();
    td
}

#[test]
fn stats_counts_command_end_events() {
    let repo = make_repo();
    let home = tempfile::tempdir().expect("home");
    let xdg = home.path().join("xdg");
    std::fs::create_dir_all(&xdg).unwrap();

    let sessions_dir = xdg
        .join("ito")
        .join("logs")
        .join("execution")
        .join("v1")
        .join("projects")
        .join("p1")
        .join("sessions");
    std::fs::create_dir_all(&sessions_dir).unwrap();

    let log_path = sessions_dir.join("s1.jsonl");
    let line = "{\"event_version\":1,\"event_id\":\"00000000-0000-0000-0000-000000000000\",\"timestamp\":\"2026-01-31T00:00:00Z\",\"event_type\":\"command_end\",\"ito_version\":\"0.0.0\",\"command_id\":\"ito.tasks.status\",\"session_id\":\"s1\",\"project_id\":\"p1\",\"pid\":1,\"outcome\":\"success\",\"duration_ms\":1}\n";
    std::fs::write(&log_path, line).unwrap();

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.current_dir(repo.path())
        .arg("stats")
        .env("CI", "1")
        .env("NO_COLOR", "1")
        .env("ITO_INTERACTIVE", "0")
        .env("TERM", "dumb")
        .env("HOME", home.path())
        .env("XDG_CONFIG_HOME", &xdg)
        .env("ITO_DISABLE_LOGGING", "1")
        .assert()
        .success()
        .stdout(contains("command_id: count"))
        .stdout(contains("ito.tasks.status: 1"))
        .stdout(contains("ito.init: 0"));
}
