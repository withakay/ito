// These tests use Unix shell scripts and permission APIs, so they only run on Unix.
// The underlying harness code is cross-platform; only the test scaffolding is Unix-specific.
#![cfg(unix)]

use ito_core::harness::{
    ClaudeCodeHarness, CodexHarness, GitHubCopilotHarness, Harness, HarnessRunConfig,
    OpencodeHarness,
};
use std::collections::BTreeMap;
use std::os::unix::fs::PermissionsExt;
use std::sync::{Mutex, OnceLock};

fn write_executable(path: &std::path::Path, contents: &str) {
    std::fs::write(path, contents).unwrap();
    let mut perms = std::fs::metadata(path).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms).unwrap();
}

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

struct PathGuard {
    _lock: std::sync::MutexGuard<'static, ()>,
    old_path: String,
}

impl PathGuard {
    fn prepend(path: &std::path::Path) -> Self {
        let lock = ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let old_path = std::env::var("PATH").unwrap_or_default();

        unsafe {
            std::env::set_var("PATH", format!("{}:{}", path.to_string_lossy(), old_path));
        }

        Self {
            _lock: lock,
            old_path,
        }
    }

    fn set_exact(path: &std::path::Path) -> Self {
        let lock = ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let old_path = std::env::var("PATH").unwrap_or_default();

        unsafe {
            std::env::set_var("PATH", path.to_string_lossy().to_string());
        }

        Self {
            _lock: lock,
            old_path,
        }
    }
}

impl Drop for PathGuard {
    fn drop(&mut self) {
        unsafe {
            std::env::set_var("PATH", &self.old_path);
        }
    }
}

#[test]
fn opencode_harness_runs_opencode_binary_and_returns_outputs() {
    let dir = tempfile::tempdir().unwrap();
    let bin = dir.path().join("opencode");
    write_executable(
        &bin,
        "#!/bin/sh\n\n# Print args so test can validate wiring.\necho \"STDOUT:$@\"\necho \"oops\" 1>&2\nexit 42\n",
    );

    let _path_guard = PathGuard::prepend(dir.path());

    let mut h = OpencodeHarness;
    let r = h
        .run(&HarnessRunConfig {
            prompt: "hello".to_string(),
            model: Some("m1".to_string()),
            cwd: dir.path().to_path_buf(),
            env: BTreeMap::new(),
            interactive: false,
            allow_all: false,
            inactivity_timeout: None,
        })
        .unwrap();

    // opencode harness always passes: opencode run [-m model] <prompt>
    assert!(
        r.stdout.contains("STDOUT:run -m m1 hello"),
        "unexpected stdout/stderr/exit_code: stdout={:?} stderr={:?} exit_code={}",
        r.stdout,
        r.stderr,
        r.exit_code
    );
    assert_eq!(r.stderr.trim(), "oops");
    assert_eq!(r.exit_code, 42);
}

#[test]
fn opencode_harness_errors_when_opencode_missing() {
    let dir = tempfile::tempdir().unwrap();

    let _path_guard = PathGuard::set_exact(dir.path());

    let mut h = OpencodeHarness;
    let err = h
        .run(&HarnessRunConfig {
            prompt: "hello".to_string(),
            model: None,
            cwd: dir.path().to_path_buf(),
            env: BTreeMap::new(),
            interactive: false,
            allow_all: false,
            inactivity_timeout: None,
        })
        .expect_err("should error");

    assert!(err.to_string().contains("Failed to spawn opencode"));
}

#[test]
fn claude_harness_passes_model_and_allow_all_flags() {
    let dir = tempfile::tempdir().unwrap();
    let bin = dir.path().join("claude");
    write_executable(
        &bin,
        "#!/bin/sh\n\n# Print args so test can validate wiring.\necho \"STDOUT:$@\"\nexit 0\n",
    );

    let _path_guard = PathGuard::prepend(dir.path());

    let mut h = ClaudeCodeHarness;
    let r = h
        .run(&HarnessRunConfig {
            prompt: "hello".to_string(),
            model: Some("m1".to_string()),
            cwd: dir.path().to_path_buf(),
            env: BTreeMap::new(),
            interactive: false,
            allow_all: true,
            inactivity_timeout: None,
        })
        .unwrap();

    assert!(
        r.stdout
            .contains("STDOUT:--model m1 --dangerously-skip-permissions -p hello"),
        "unexpected stdout/stderr/exit_code: stdout={:?} stderr={:?} exit_code={}",
        r.stdout,
        r.stderr,
        r.exit_code
    );
}

#[test]
fn codex_harness_passes_model_and_allow_all_flags() {
    let dir = tempfile::tempdir().unwrap();
    let bin = dir.path().join("codex");
    write_executable(
        &bin,
        "#!/bin/sh\n\n# Print args so test can validate wiring.\necho \"STDOUT:$@\"\nexit 0\n",
    );

    let _path_guard = PathGuard::prepend(dir.path());

    let mut h = CodexHarness;
    let r = h
        .run(&HarnessRunConfig {
            prompt: "hello".to_string(),
            model: Some("m1".to_string()),
            cwd: dir.path().to_path_buf(),
            env: BTreeMap::new(),
            interactive: false,
            allow_all: true,
            inactivity_timeout: None,
        })
        .unwrap();

    assert!(
        r.stdout.contains("STDOUT:exec --model m1 --yolo hello"),
        "unexpected stdout/stderr/exit_code: stdout={:?} stderr={:?} exit_code={}",
        r.stdout,
        r.stderr,
        r.exit_code
    );
}

#[test]
fn github_copilot_harness_passes_model_and_allow_all_flags() {
    let dir = tempfile::tempdir().unwrap();
    let bin = dir.path().join("copilot");
    write_executable(
        &bin,
        "#!/bin/sh\n\n# Print args so test can validate wiring.\necho \"STDOUT:$@\"\nexit 0\n",
    );

    let _path_guard = PathGuard::prepend(dir.path());

    let mut h = GitHubCopilotHarness;
    let r = h
        .run(&HarnessRunConfig {
            prompt: "hello".to_string(),
            model: Some("m1".to_string()),
            cwd: dir.path().to_path_buf(),
            env: BTreeMap::new(),
            interactive: false,
            allow_all: true,
            inactivity_timeout: None,
        })
        .unwrap();

    assert!(
        r.stdout.contains("STDOUT:--model m1 --yolo -p hello"),
        "unexpected stdout/stderr/exit_code: stdout={:?} stderr={:?} exit_code={}",
        r.stdout,
        r.stderr,
        r.exit_code
    );
}
