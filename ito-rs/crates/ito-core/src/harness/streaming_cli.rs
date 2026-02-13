use super::types::{HarnessRunConfig, HarnessRunResult};
use miette::{Result, miette};
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

/// Default inactivity timeout for CLI harnesses.
pub const DEFAULT_INACTIVITY_TIMEOUT: Duration = Duration::from_secs(15 * 60);

/// Run a binary with the given arguments, streaming its stdout and stderr and enforcing an inactivity timeout.
///
/// Streams the child process's stdout and stderr to the current process while collecting their output. The child is started in `config.cwd` with `config.env`. An inactivity timer (from `config.inactivity_timeout` or `DEFAULT_INACTIVITY_TIMEOUT`) is reset on any output; if the timeout elapses with no activity, the child is terminated and the result is marked as timed out.
///
/// The returned `HarnessRunResult` contains the accumulated `stdout` and `stderr`, the total `duration`, an `exit_code` (set to `-1` if the child was terminated due to inactivity, otherwise the child's exit code or `1` if unavailable), and a `timed_out` flag.
///
/// # Examples
///
/// ```
/// // Construct a HarnessRunConfig appropriate for your environment.
/// let cfg = HarnessRunConfig {
///     cwd: std::env::current_dir().unwrap(),
///     env: std::env::vars().collect(),
///     inactivity_timeout: None,
/// };
/// let res = run_streaming_cli("echo", &vec!["hello".to_string()], &cfg).unwrap();
/// assert!(res.stdout.contains("hello"));
/// ```
pub(super) fn run_streaming_cli(
    binary: &str,
    args: &[String],
    config: &HarnessRunConfig,
) -> Result<HarnessRunResult> {
    let mut cmd = Command::new(binary);
    cmd.args(args);
    cmd.current_dir(&config.cwd);
    cmd.envs(&config.env);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let start = Instant::now();
    let mut child = cmd
        .spawn()
        .map_err(|e| miette!("Failed to spawn {binary}: {e}"))?;

    let child_id = child.id();
    let stdout_pipe = child.stdout.take();
    let stderr_pipe = child.stderr.take();

    let last_activity = Arc::new(std::sync::Mutex::new(Instant::now()));
    let timed_out = Arc::new(AtomicBool::new(false));
    let done = Arc::new(AtomicBool::new(false));

    let last_activity_stdout = Arc::clone(&last_activity);
    let stdout_handle =
        thread::spawn(move || stream_pipe(stdout_pipe, &last_activity_stdout, true));

    let last_activity_stderr = Arc::clone(&last_activity);
    let stderr_handle =
        thread::spawn(move || stream_pipe(stderr_pipe, &last_activity_stderr, false));

    let timeout = config
        .inactivity_timeout
        .unwrap_or(DEFAULT_INACTIVITY_TIMEOUT);
    let last_activity_monitor = Arc::clone(&last_activity);
    let timed_out_monitor = Arc::clone(&timed_out);
    let done_monitor = Arc::clone(&done);

    let monitor_handle = thread::spawn(move || {
        monitor_timeout(
            child_id,
            timeout,
            &last_activity_monitor,
            &timed_out_monitor,
            &done_monitor,
        )
    });

    let status = child
        .wait()
        .map_err(|e| miette!("Failed to wait for {binary}: {e}"))?;
    done.store(true, Ordering::SeqCst);

    let stdout = stdout_handle.join().unwrap_or_default();
    let stderr = stderr_handle.join().unwrap_or_default();
    let _ = monitor_handle.join();

    let duration = start.elapsed();
    let was_timed_out = timed_out.load(Ordering::SeqCst);

    Ok(HarnessRunResult {
        stdout,
        stderr,
        exit_code: if was_timed_out {
            -1
        } else {
            status.code().unwrap_or(1)
        },
        duration,
        timed_out: was_timed_out,
    })
}

/// Read from an optional reader, write each chunk to either stdout or stderr, update the provided
/// `last_activity` timestamp on each read, and return all bytes read as a UTF-8 lossily-converted `String`.
///
/// The function does nothing if `pipe` is `None`. On each successful read it sets `*last_activity` to
/// the current instant and forwards the read bytes to stdout when `is_stdout` is `true`, otherwise to stderr.
///
/// # Parameters
///
/// - `last_activity`: a mutex protecting the Instant to update when new data is observed.
/// - `is_stdout`: when `true`, write chunks to stdout; otherwise write to stderr.
///
/// # Returns
///
/// The concatenated output read from `pipe`, converted using UTF-8 lossy conversion.
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// use std::sync::Mutex;
/// use std::time::Instant;
///
/// let reader = Cursor::new(b"hello\nworld");
/// let last_activity = Mutex::new(Instant::now());
/// let collected = stream_pipe(Some(reader), &last_activity, true);
/// assert_eq!(collected, "hello\nworld");
/// ```
fn stream_pipe(
    pipe: Option<impl std::io::Read>,
    last_activity: &std::sync::Mutex<Instant>,
    is_stdout: bool,
) -> String {
    let mut collected = String::new();
    if let Some(mut pipe) = pipe {
        let mut buf = [0u8; 4096];
        loop {
            let n = match pipe.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };

            if let Ok(mut last) = last_activity.lock() {
                *last = Instant::now();
            }

            let chunk = String::from_utf8_lossy(&buf[..n]);

            if is_stdout {
                print!("{}", chunk);
                let _ = std::io::stdout().flush();
            } else {
                eprint!("{}", chunk);
                let _ = std::io::stderr().flush();
            }

            collected.push_str(&chunk);
        }
    }
    collected
}

/// Monitors a child process for inactivity and forcefully terminates it if no activity occurs within `timeout`.
///
/// Periodically checks the elapsed time since `last_activity`; if the elapsed time meets or exceeds
/// `timeout`, prints an inactivity message to stderr, sets `timed_out` to `true`, and attempts to
/// kill the process with `child_id` (platform-specific: `kill -9` on Unix, `taskkill /F /PID` on Windows).
/// The monitor exits early if `done` becomes `true` or if `last_activity` cannot be locked.
///
/// # Parameters
///
/// - `child_id`: process identifier of the child to terminate on timeout.
/// - `timeout`: duration of allowed inactivity before termination.
/// - `last_activity`: mutex-protected `Instant` updated by output-streaming threads on each read.
/// - `timed_out`: atomic flag set to `true` when a timeout-triggered termination occurs.
/// - `done`: atomic flag that, when set to `true`, stops the monitor loop.
///
/// # Examples
///
/// ```
/// use std::sync::{Arc, Mutex, AtomicBool, atomic::Ordering};
/// use std::time::{Duration, Instant};
/// use std::thread;
///
/// // Prepare shared state
/// let last_activity = Arc::new(Mutex::new(Instant::now()));
/// let timed_out = Arc::new(AtomicBool::new(false));
/// let done = Arc::new(AtomicBool::new(false));
///
/// // Clone for the monitor thread
/// let la = Arc::clone(&last_activity);
/// let to = Arc::clone(&timed_out);
/// let dn = Arc::clone(&done);
///
/// // Spawn the monitor in a thread (uses a dummy child id 0 for example)
/// let handle = thread::spawn(move || {
///     super::monitor_timeout(0, Duration::from_millis(10), &la.lock().unwrap(), &to, &dn);
/// });
///
/// // Signal completion to stop the monitor and join
/// done.store(true, Ordering::SeqCst);
/// let _ = handle.join();
/// ```
fn monitor_timeout(
    child_id: u32,
    timeout: Duration,
    last_activity: &std::sync::Mutex<Instant>,
    timed_out: &AtomicBool,
    done: &AtomicBool,
) {
    let check_interval = Duration::from_secs(1);

    loop {
        thread::sleep(check_interval);

        if done.load(Ordering::SeqCst) {
            break;
        }

        let elapsed = match last_activity.lock() {
            Ok(last) => last.elapsed(),
            Err(_) => break,
        };

        if elapsed >= timeout {
            eprintln!(
                "\n=== Inactivity timeout ({:?}) reached, killing process... ===\n",
                timeout
            );
            timed_out.store(true, Ordering::SeqCst);

            #[cfg(unix)]
            {
                let _ = std::process::Command::new("kill")
                    .args(["-9", &child_id.to_string()])
                    .status();
            }
            #[cfg(windows)]
            {
                let _ = std::process::Command::new("taskkill")
                    .args(["/F", "/PID", &child_id.to_string()])
                    .status();
            }

            break;
        }
    }
}