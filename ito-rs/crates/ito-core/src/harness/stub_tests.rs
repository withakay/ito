use super::*;
use std::collections::BTreeMap;

fn dummy_config() -> HarnessRunConfig {
    HarnessRunConfig {
        prompt: "test".to_string(),
        model: None,
        cwd: std::env::temp_dir(),
        env: BTreeMap::new(),
        interactive: false,
        allow_all: false,
        inactivity_timeout: None,
    }
}

#[test]
fn name_returns_stub() {
    let stub = StubHarness::new(vec![StubStep {
        stdout: "test".to_string(),
        stderr: String::new(),
        exit_code: 0,
    }]);
    assert_eq!(stub.name(), HarnessName::Stub);
}

#[test]
fn streams_output_returns_false() {
    let stub = StubHarness::new(vec![StubStep {
        stdout: "test".to_string(),
        stderr: String::new(),
        exit_code: 0,
    }]);
    assert!(!stub.streams_output());
}

#[test]
fn run_sets_timed_out_false() {
    let mut stub = StubHarness::new(vec![StubStep {
        stdout: "test".to_string(),
        stderr: String::new(),
        exit_code: 0,
    }]);
    let config = dummy_config();
    let result = stub.run(&config).unwrap();
    assert!(!result.timed_out);
}

#[test]
fn run_sets_nonzero_duration() {
    let mut stub = StubHarness::new(vec![StubStep {
        stdout: "test".to_string(),
        stderr: String::new(),
        exit_code: 0,
    }]);
    let config = dummy_config();
    let result = stub.run(&config).unwrap();
    assert!(result.duration > Duration::ZERO);
}

#[test]
fn from_env_or_default_with_explicit_path() {
    use std::io::Write;
    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
    let json = r#"[{"stdout": "hello", "stderr": "", "exitCode": 0}]"#;
    tmpfile.write_all(json.as_bytes()).unwrap();
    tmpfile.flush().unwrap();

    let mut stub = StubHarness::from_env_or_default(Some(tmpfile.path().to_path_buf())).unwrap();
    let config = dummy_config();
    let result = stub.run(&config).unwrap();
    assert_eq!(result.stdout, "hello");
}
