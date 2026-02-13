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

/// Which standard stream a pipe should forward output to.
enum StreamTarget {
    /// Forward to stdout.
    Stdout,
    /// Forward to stderr.
    Stderr,
}

/// Spawns a CLI binary with streaming stdout/stderr and an inactivity monitor.
///
/// All harnesses delegate to this function so they share consistent streaming
/// behaviour: output is forwarded to the terminal in real time, an inactivity
/// timer kills the process when it stalls, and incomplete UTF-8 sequences at
/// chunk boundaries are handled correctly.
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
    let stdout_handle = thread::spawn(move || {
        stream_pipe(stdout_pipe, &last_activity_stdout, StreamTarget::Stdout)
    });

    let last_activity_stderr = Arc::clone(&last_activity);
    let stderr_handle = thread::spawn(move || {
        stream_pipe(stderr_pipe, &last_activity_stderr, StreamTarget::Stderr)
    });

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

/// Reads from `pipe` in byte-level chunks, forwarding output to stdout/stderr
/// and updating `last_activity` on every read. Byte-level reads (vs line-based)
/// ensure inactivity is tracked even when tools stream output without newlines.
///
/// Incomplete UTF-8 sequences at chunk boundaries are buffered and prepended to
/// the next read, so multi-byte characters are never split by replacement chars.
fn stream_pipe(
    pipe: Option<impl std::io::Read>,
    last_activity: &std::sync::Mutex<Instant>,
    target: StreamTarget,
) -> String {
    let mut collected = String::new();
    let Some(mut pipe) = pipe else {
        return collected;
    };

    let mut buf = [0u8; 4096];
    // Bytes from the tail of the previous read that form an incomplete UTF-8
    // sequence. At most 3 bytes (the longest incomplete prefix of a 4-byte char).
    let mut leftover = Vec::new();

    loop {
        let n = match pipe.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_err) => break,
        };

        if let Ok(mut last) = last_activity.lock() {
            *last = Instant::now();
        }

        // Prepend any leftover bytes from the previous chunk.
        let data = if leftover.is_empty() {
            &buf[..n]
        } else {
            leftover.extend_from_slice(&buf[..n]);
            leftover.as_slice()
        };

        // Find the longest valid UTF-8 prefix. Any trailing bytes that form an
        // incomplete character are saved for the next iteration.
        let (valid, remaining) = match std::str::from_utf8(data) {
            Ok(s) => (s, &[][..]),
            Err(e) => {
                let valid_up_to = e.valid_up_to();
                // SAFETY: from_utf8 guarantees bytes up to valid_up_to are valid UTF-8.
                let valid = unsafe { std::str::from_utf8_unchecked(&data[..valid_up_to]) };
                (valid, &data[valid_up_to..])
            }
        };

        if !valid.is_empty() {
            match target {
                StreamTarget::Stdout => {
                    print!("{valid}");
                    let _ = std::io::stdout().flush();
                }
                StreamTarget::Stderr => {
                    eprint!("{valid}");
                    let _ = std::io::stderr().flush();
                }
            }
            collected.push_str(valid);
        }

        leftover = remaining.to_vec();
    }

    // Flush any final leftover bytes (incomplete sequence at EOF) as lossy UTF-8.
    if !leftover.is_empty() {
        let tail = String::from_utf8_lossy(&leftover);
        match target {
            StreamTarget::Stdout => {
                print!("{tail}");
                let _ = std::io::stdout().flush();
            }
            StreamTarget::Stderr => {
                eprint!("{tail}");
                let _ = std::io::stderr().flush();
            }
        }
        collected.push_str(&tail);
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
/// ```ignore
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
            Err(_poisoned) => break,
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
