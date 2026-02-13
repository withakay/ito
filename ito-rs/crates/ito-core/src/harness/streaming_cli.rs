use super::types::{HarnessRunConfig, HarnessRunResult};
use miette::{Result, miette};
use std::io::{BufRead, BufReader, Write};
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

fn stream_pipe(
    pipe: Option<impl std::io::Read>,
    last_activity: &std::sync::Mutex<Instant>,
    is_stdout: bool,
) -> String {
    let mut collected = String::new();
    if let Some(pipe) = pipe {
        let reader = BufReader::new(pipe);
        for line in reader.lines().map_while(Result::ok) {
            if let Ok(mut last) = last_activity.lock() {
                *last = Instant::now();
            }

            if is_stdout {
                println!("{}", line);
                let _ = std::io::stdout().flush();
            } else {
                eprintln!("{}", line);
                let _ = std::io::stderr().flush();
            }

            collected.push_str(&line);
            collected.push('\n');
        }
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
