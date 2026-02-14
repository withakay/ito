use ito_core::harness::stub::StubHarness;
use ito_core::harness::{Harness, HarnessRunConfig};
use std::collections::BTreeMap;

#[test]
fn stub_harness_default_returns_complete_promise() {
    unsafe {
        std::env::remove_var("ITO_STUB_SCRIPT");
    }
    let mut h = StubHarness::from_env_or_default(None).expect("default harness");
    let r = h
        .run(&HarnessRunConfig {
            prompt: "ignored".to_string(),
            model: None,
            cwd: std::env::current_dir().unwrap(),
            env: BTreeMap::new(),
            interactive: false,
            allow_all: false,
            inactivity_timeout: None,
        })
        .expect("run");
    assert!(r.stdout.contains("<promise>COMPLETE</promise>"));
    assert_eq!(r.exit_code, 0);
}

/// Verifies that a StubHarness can be loaded from a JSON script, that it runs steps
/// in sequence, and that it repeats the final step once the script's steps are exhausted.
///
/// # Examples
///
/// ```
/// # use ito_core::harness::stub::StubHarness;
/// # use ito_core::harness::HarnessRunConfig;
/// # use std::collections::BTreeMap;
/// let dir = tempfile::tempdir().unwrap();
/// let script_path = dir.path().join("stub.json");
/// std::fs::write(
///     &script_path,
///     r#"[
///   { "stdout": "one\n", "stderr": "", "exitCode": 0 },
///   { "stdout": "two\n", "stderr": "e2\n", "exitCode": 2 }
/// ]"#,
/// ).unwrap();
///
/// let mut h = StubHarness::from_json_path(&script_path).expect("load json");
///
/// let cfg = HarnessRunConfig {
///     prompt: "ignored".to_string(),
///     model: None,
///     cwd: std::env::current_dir().unwrap(),
///     env: BTreeMap::new(),
///     interactive: false,
///     allow_all: false,
///     inactivity_timeout: None,
/// };
///
/// let r1 = h.run(&cfg).unwrap();
/// assert_eq!(r1.stdout, "one\n");
/// assert_eq!(r1.stderr, "");
/// assert_eq!(r1.exit_code, 0);
///
/// let r2 = h.run(&cfg).unwrap();
/// assert_eq!(r2.stdout, "two\n");
/// assert_eq!(r2.stderr, "e2\n");
/// assert_eq!(r2.exit_code, 2);
///
/// // After steps are exhausted, the harness repeats the last step.
/// let r3 = h.run(&cfg).unwrap();
/// assert_eq!(r3.stdout, "two\n");
/// assert_eq!(r3.exit_code, 2);
/// ```
#[test]
fn stub_harness_from_json_path_runs_steps_and_repeats_last() {
    let dir = tempfile::tempdir().unwrap();
    let script_path = dir.path().join("stub.json");
    std::fs::write(
        &script_path,
        r#"[
  { "stdout": "one\n", "stderr": "", "exitCode": 0 },
  { "stdout": "two\n", "stderr": "e2\n", "exitCode": 2 }
]"#,
    )
    .unwrap();

    let mut h = StubHarness::from_json_path(&script_path).expect("load json");

    let cfg = HarnessRunConfig {
        prompt: "ignored".to_string(),
        model: None,
        cwd: std::env::current_dir().unwrap(),
        env: BTreeMap::new(),
        interactive: false,
        allow_all: false,
        inactivity_timeout: None,
    };

    let r1 = h.run(&cfg).unwrap();
    assert_eq!(r1.stdout, "one\n");
    assert_eq!(r1.stderr, "");
    assert_eq!(r1.exit_code, 0);

    let r2 = h.run(&cfg).unwrap();
    assert_eq!(r2.stdout, "two\n");
    assert_eq!(r2.stderr, "e2\n");
    assert_eq!(r2.exit_code, 2);

    // Repeat last step after running out.
    let r3 = h.run(&cfg).unwrap();
    assert_eq!(r3.stdout, "two\n");
    assert_eq!(r3.exit_code, 2);
}

#[test]
fn stub_harness_from_env_prefers_env_over_default() {
    let dir = tempfile::tempdir().unwrap();
    let script_path = dir.path().join("stub.json");
    std::fs::write(&script_path, r#"[{"stdout":"x"}]"#).unwrap();
    unsafe {
        std::env::set_var("ITO_STUB_SCRIPT", script_path.to_string_lossy().to_string());
    }

    let h = StubHarness::from_env_or_default(None).expect("load from env");
    assert_eq!(h.name().as_str(), "stub");

    unsafe {
        std::env::remove_var("ITO_STUB_SCRIPT");
    }
}

/// Verifies that running a `StubHarness` with an empty step list produces an error.
///
/// Constructs a `StubHarness` with no steps and asserts that `.run(...)` returns an error
/// whose message contains the phrase "no steps".
///
/// # Examples
///
/// ```
/// let mut h = StubHarness::new(vec![]);
/// let err = h
///     .run(&HarnessRunConfig {
///         prompt: "ignored".to_string(),
///         model: None,
///         cwd: std::env::current_dir().unwrap(),
///         env: BTreeMap::new(),
///         interactive: false,
///         allow_all: false,
///         inactivity_timeout: None,
///     })
///     .expect_err("should error");
/// let msg = err.to_string();
/// assert!(msg.contains("no steps"));
/// ```
#[test]
fn stub_harness_errors_on_empty_steps() {
    let mut h = StubHarness::new(vec![]);
    let err = h
        .run(&HarnessRunConfig {
            prompt: "ignored".to_string(),
            model: None,
            cwd: std::env::current_dir().unwrap(),
            env: BTreeMap::new(),
            interactive: false,
            allow_all: false,
            inactivity_timeout: None,
        })
        .expect_err("should error");
    let msg = err.to_string();
    assert!(msg.contains("no steps"));
}

#[test]
fn stub_step_defaults_match_json_schema() {
    let dir = tempfile::tempdir().unwrap();
    let script_path = dir.path().join("stub.json");
    std::fs::write(&script_path, r#"[{"stdout":"ok"}]"#).unwrap();

    let mut h = StubHarness::from_json_path(&script_path).unwrap();
    let r = h
        .run(&HarnessRunConfig {
            prompt: "ignored".to_string(),
            model: None,
            cwd: std::env::current_dir().unwrap(),
            env: BTreeMap::new(),
            interactive: false,
            allow_all: false,
            inactivity_timeout: None,
        })
        .unwrap();

    assert_eq!(r.stdout, "ok");
    assert_eq!(r.stderr, "");
    assert_eq!(r.exit_code, 0);
}

#[test]
fn stub_harness_errors_on_missing_and_invalid_json() {
    let dir = tempfile::tempdir().unwrap();

    let missing = dir.path().join("missing.json");
    let err = StubHarness::from_json_path(&missing).expect_err("should error");
    assert!(err.to_string().contains("Failed to read"));

    let invalid = dir.path().join("invalid.json");
    std::fs::write(&invalid, "not json").unwrap();
    let err = StubHarness::from_json_path(&invalid).expect_err("should error");
    assert!(err.to_string().contains("Invalid stub script JSON"));
}
