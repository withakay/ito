// These tests use Unix shell scripts and permission APIs, so they only run on Unix.
// The underlying harness code is cross-platform; only the test scaffolding is Unix-specific.
#![cfg(unix)]

use ito_core::harness::{Harness, HarnessRunConfig, OpencodeHarness};
use std::collections::BTreeMap;
use std::os::unix::fs::PermissionsExt;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

/// Creates or overwrites a file at `path` with `contents` and sets its mode to `0o755` (executable).
///
/// This function will panic if writing the file or changing its permissions fails.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use std::os::unix::fs::PermissionsExt;
/// let dir = tempfile::tempdir().unwrap();
/// let file = dir.path().join("script.sh");
/// write_executable(&file, "#!/bin/sh\necho hello\n");
/// let contents = fs::read_to_string(&file).unwrap();
/// assert!(contents.contains("echo hello"));
/// let mode = fs::metadata(&file).unwrap().permissions().mode();
/// assert_eq!(mode & 0o777, 0o755);
/// ```
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
    /// Temporarily prepends `path` to the process `PATH` environment variable and returns a guard that restores the original `PATH` when dropped.
    ///
    /// The returned `PathGuard` holds a lock that serializes modifications to `PATH` across threads and stores the previous `PATH` value so it can be restored on `Drop`.
    ///
    /// # Parameters
    ///
    /// - `path`: filesystem path to insert at the front of `PATH`.
    ///
    /// # Returns
    ///
    /// A `PathGuard` which, while alive, keeps the new `PATH` in effect and restores the previous `PATH` when it is dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// // Prepend "/tmp/bin" to PATH for the duration of the guard.
    /// let guard = PathGuard::prepend(Path::new("/tmp/bin"));
    /// // PATH now begins with "/tmp/bin".
    /// drop(guard);
    /// // PATH restored to its previous value.
    /// ```
    fn prepend(path: &std::path::Path) -> Self {
        let lock = ENV_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let old_path = std::env::var("PATH").unwrap_or_default();

        unsafe {
            std::env::set_var("PATH", format!("{}:{}", path.to_string_lossy(), old_path));
        }

        Self {
            _lock: lock,
            old_path,
        }
    }
}

impl Drop for PathGuard {
    /// Restores the previous `PATH` environment variable when the guard is dropped.
    
    ///
    
    /// The guard sets `PATH` to a modified value for the guard's lifetime; dropping it
    
    /// restores the original `PATH` value that was present when the guard was created.
    
    ///
    
    /// # Examples
    
    ///
    
    /// ```
    
    /// // Save and set a known PATH for the test.
    
    /// std::env::set_var("PATH", "/original");
    
    /// {
    
    ///     // Prepend a directory for the duration of the scope.
    
    ///     let _guard = crate::PathGuard::prepend("/tmp/testbin");
    
    ///     // While `_guard` is in scope, PATH starts with "/tmp/testbin:".
    
    ///     assert!(std::env::var("PATH").unwrap().starts_with("/tmp/testbin:"));
    
    /// }
    
    /// // After the guard is dropped, PATH is restored to the previous value.
    
    /// assert_eq!(std::env::var("PATH").unwrap(), "/original");
    
    /// ```
    fn drop(&mut self) {
        unsafe {
            std::env::set_var("PATH", &self.old_path);
        }
    }
}

/// Ensures an opencode process that becomes inactive is terminated after the configured timeout.
///
/// Verifies that a stalled child process is killed due to inactivity, that the harness reports
/// `timed_out` and an exit code of `-1`, that output produced before the timeout is captured,
/// and that no output produced after the timeout is present. Also asserts the run duration is
/// at least the timeout and completes within a reasonable upper bound.
///
/// # Examples
///
/// ```
/// // Configure a harness with an inactivity timeout and run a command that stalls.
/// // The harness should report a timeout and an exit code of -1.
/// let mut h = OpencodeHarness;
/// let r = h.run(&HarnessRunConfig { inactivity_timeout: Some(Duration::from_secs(2)), ..Default::default() }).unwrap();
/// assert!(r.timed_out);
/// assert_eq!(r.exit_code, -1);
/// ```
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

    let _path_guard = PathGuard::prepend(dir.path());

    let mut h = OpencodeHarness;
    let start = std::time::Instant::now();

    let r = h
        .run(&HarnessRunConfig {
            prompt: "test".to_string(),
            model: None,
            cwd: dir.path().to_path_buf(),
            env: BTreeMap::new(),
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

    let _path_guard = PathGuard::prepend(dir.path());

    let mut h = OpencodeHarness;
    let start = std::time::Instant::now();

    let r = h
        .run(&HarnessRunConfig {
            prompt: "test".to_string(),
            model: None,
            cwd: dir.path().to_path_buf(),
            env: BTreeMap::new(),
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