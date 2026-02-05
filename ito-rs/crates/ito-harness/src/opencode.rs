use crate::types::{Harness, HarnessName, HarnessRunConfig, HarnessRunResult};
use miette::{Result, miette};
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

/// Default inactivity timeout: 15 minutes
pub const DEFAULT_INACTIVITY_TIMEOUT: Duration = Duration::from_secs(15 * 60);

#[derive(Debug, Default)]
/// Harness implementation that executes the `opencode` CLI.
///
/// This harness is intended for local development and CI where OpenCode is
/// available on `PATH`.
pub struct OpencodeHarness;

impl Harness for OpencodeHarness {
    fn name(&self) -> HarnessName {
        HarnessName::OPENCODE
    }

    fn run(&mut self, config: &HarnessRunConfig) -> Result<HarnessRunResult> {
        let mut cmd = Command::new("opencode");
        cmd.arg("run");

        if let Some(model) = config.model.as_deref() {
            cmd.args(["-m", model]);
        }

        cmd.arg(&config.prompt);
        cmd.current_dir(&config.cwd);
        cmd.envs(&config.env);

        // Use spawn with piped stdout/stderr for streaming output
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let start = Instant::now();

        let mut child = cmd
            .spawn()
            .map_err(|e| miette!("Failed to spawn opencode: {e}"))?;

        let child_id = child.id();
        let stdout_pipe = child.stdout.take();
        let stderr_pipe = child.stderr.take();

        // Track last activity time for timeout detection
        let last_activity = Arc::new(std::sync::Mutex::new(Instant::now()));
        let timed_out = Arc::new(AtomicBool::new(false));
        let done = Arc::new(AtomicBool::new(false));

        // Spawn thread to stream stdout
        let last_activity_stdout = Arc::clone(&last_activity);
        let stdout_handle =
            thread::spawn(move || stream_pipe(stdout_pipe, &last_activity_stdout, true));

        // Spawn thread to stream stderr
        let last_activity_stderr = Arc::clone(&last_activity);
        let stderr_handle =
            thread::spawn(move || stream_pipe(stderr_pipe, &last_activity_stderr, false));

        // Spawn timeout monitor thread if timeout is configured
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

        // Wait for process to complete
        let status = child
            .wait()
            .map_err(|e| miette!("Failed to wait for opencode: {e}"))?;

        done.store(true, Ordering::SeqCst);

        // Wait for streaming threads to finish
        let stdout = stdout_handle.join().unwrap_or_default();
        let stderr = stderr_handle.join().unwrap_or_default();

        // Stop the monitor thread (it will exit on next check since process is done)
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

    fn stop(&mut self) {
        // No-op: `run` is synchronous.
    }

    fn streams_output(&self) -> bool {
        true
    }
}

/// Stream output from a pipe, updating last activity time on each line.
fn stream_pipe(
    pipe: Option<impl std::io::Read>,
    last_activity: &std::sync::Mutex<Instant>,
    is_stdout: bool,
) -> String {
    let mut collected = String::new();
    if let Some(pipe) = pipe {
        let reader = BufReader::new(pipe);
        for line in reader.lines().map_while(Result::ok) {
            // Update last activity time
            if let Ok(mut last) = last_activity.lock() {
                *last = Instant::now();
            }

            // Stream to console
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

/// Monitor for inactivity timeout and kill process if exceeded.
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

        // Check if process is still running by trying to get last activity
        let elapsed = match last_activity.lock() {
            Ok(last) => last.elapsed(),
            Err(_) => break, // Mutex poisoned, process likely done
        };

        if elapsed >= timeout {
            eprintln!(
                "\n=== Inactivity timeout ({:?}) reached, killing process... ===\n",
                timeout
            );
            timed_out.store(true, Ordering::SeqCst);

            // Kill the process
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

        // Check if process has exited (mutex would be poisoned or we'd be waiting forever)
        // The streaming threads will exit when the process exits, which will close the pipes
    }
}
