// These tests use Unix shell scripts and permission APIs, so they only run on Unix.
// The underlying harness code is cross-platform; only the test scaffolding is Unix-specific.
#![cfg(unix)]

use ito_core::harness::{Harness, HarnessRunConfig, OpencodeHarness};
use std::collections::BTreeMap;
use std::os::unix::fs::PermissionsExt;
use std::time::Duration;

fn write_executable(path: &std::path::Path, contents: &str) {
    std::fs::write(path, contents).unwrap();
    let mut perms = std::fs::metadata(path).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms).unwrap();
}

fn child_path_with_prepend(path: &std::path::Path) -> String {
    let old = std::env::var("PATH").unwrap_or_default();
    format!("{}:{}", path.to_string_lossy(), old)
}

#[test]
fn inactivity_timeout_kills_stalled_process() {
    let dir = tempfile::tempdir().unwrap();
    let bin = dir.path().join("opencode");

    // Create a script that prints one line then stalls with a busy loop
    // Using a busy loop instead of sleep to ensure the shell process itself is active
    write_executable(
        &bin,
        "#!/bin/sh\necho 'Starting...'\nwhile true; do sleep 1; done\necho 'Should not reach here'\n",
    );

    let mut env = BTreeMap::new();
    env.insert("PATH".to_string(), child_path_with_prepend(dir.path()));

    let mut h = OpencodeHarness;
    let start = std::time::Instant::now();

    let r = h
        .run(&HarnessRunConfig {
            prompt: "test".to_string(),
            model: None,
            cwd: dir.path().to_path_buf(),
            env,
            interactive: false,
            allow_all: false,
            inactivity_timeout: Some(Duration::from_secs(2)),
        })
        .unwrap();

    let elapsed = start.elapsed();

    // Verify the process was killed by timeout
    assert!(r.timed_out, "Process should have timed out");
    assert_eq!(r.exit_code, -1, "Exit code should be -1 for timeout");

    // Verify we got the initial output before timeout
    assert!(
        r.stdout.contains("Starting..."),
        "Should have captured initial output before timeout"
    );

    // Verify we didn't get output from after the sleep
    assert!(
        !r.stdout.contains("Should not reach here"),
        "Should not have output from after the sleep"
    );

    // Verify the test completed in reasonable time (< 10 seconds)
    // The timeout is 2 seconds, plus check interval (1s), plus some buffer
    assert!(
        elapsed < Duration::from_secs(10),
        "Test should complete quickly, took {:?}",
        elapsed
    );

    // Verify it took at least the timeout duration
    assert!(
        elapsed >= Duration::from_secs(2),
        "Should have waited at least the timeout duration, took {:?}",
        elapsed
    );
}

#[test]
fn no_timeout_when_process_exits_normally() {
    let dir = tempfile::tempdir().unwrap();
    let bin = dir.path().join("opencode");

    // Create a script that prints output and exits quickly
    write_executable(
        &bin,
        "#!/bin/sh\necho 'Line 1'\necho 'Line 2'\necho 'Line 3'\nexit 0\n",
    );

    let mut env = BTreeMap::new();
    env.insert("PATH".to_string(), child_path_with_prepend(dir.path()));

    let mut h = OpencodeHarness;
    let start = std::time::Instant::now();

    let r = h
        .run(&HarnessRunConfig {
            prompt: "test".to_string(),
            model: None,
            cwd: dir.path().to_path_buf(),
            env,
            interactive: false,
            allow_all: false,
            inactivity_timeout: Some(Duration::from_secs(2)),
        })
        .unwrap();

    let elapsed = start.elapsed();

    // Verify no timeout occurred
    assert!(!r.timed_out, "Process should not have timed out");
    assert_eq!(r.exit_code, 0, "Exit code should be 0 for normal exit");

    // Verify we got all the output
    assert!(
        r.stdout.contains("Line 1"),
        "Should have captured all output"
    );
    assert!(
        r.stdout.contains("Line 2"),
        "Should have captured all output"
    );
    assert!(
        r.stdout.contains("Line 3"),
        "Should have captured all output"
    );

    // Verify the test completed in reasonable time (well before a 10-second bound).
    // We don't assert below the inactivity timeout because monitor thread teardown
    // can take up to one check interval (~1 second) even after normal exit.
    assert!(
        elapsed < Duration::from_secs(10),
        "Test should complete quickly, took {:?}",
        elapsed
    );
}
