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

/// Reads from `pipe` in byte-level chunks, forwarding output to stdout/stderr
/// and updating `last_activity` on every read. Byte-level reads (vs line-based)
/// ensure inactivity is tracked even when tools stream output without newlines.
///
/// Incomplete UTF-8 sequences at chunk boundaries are buffered and prepended to
/// the next read, so multi-byte characters are never split by replacement chars.
fn stream_pipe(
    pipe: Option<impl std::io::Read>,
    last_activity: &std::sync::Mutex<Instant>,
    is_stdout: bool,
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
            Err(_) => break,
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
            if is_stdout {
                print!("{valid}");
                let _ = std::io::stdout().flush();
            } else {
                eprint!("{valid}");
                let _ = std::io::stderr().flush();
            }
            collected.push_str(valid);
        }

        leftover = remaining.to_vec();
    }

    // Flush any final leftover bytes (incomplete sequence at EOF) as lossy UTF-8.
    if !leftover.is_empty() {
        let tail = String::from_utf8_lossy(&leftover);
        if is_stdout {
            print!("{tail}");
            let _ = std::io::stdout().flush();
        } else {
            eprint!("{tail}");
            let _ = std::io::stderr().flush();
        }
        collected.push_str(&tail);
    }

    collected
}

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
