//! PTY-based command runners for interactive integration tests.

use std::io::{Read, Write};
use std::path::Path;

use portable_pty::{CommandBuilder, PtySize, native_pty_system};

use crate::CmdOutput;

/// Runs a command in a PTY and captures output.
///
/// Notes:
/// - PTY output is a merged stream (stdout+stderr); we populate `stdout`.
/// - This helper is intentionally minimal; interactive parity tests can extend
///   it to incremental reads/writes.
pub fn run_pty(program: &Path, args: &[&str], cwd: &Path, home: &Path, input: &str) -> CmdOutput {
    run_pty_with_interactive_env(program, args, cwd, home, input, false, &[])
}

/// Runs a command in a PTY with `ITO_INTERACTIVE=1`.
///
/// This is useful for testing that interactive code paths behave as expected
/// under a pseudo-terminal.
pub fn run_pty_interactive(
    program: &Path,
    args: &[&str],
    cwd: &Path,
    home: &Path,
    input: &str,
) -> CmdOutput {
    run_pty_with_interactive_env(program, args, cwd, home, input, true, &[])
}

/// Runs a command in a PTY with `ITO_INTERACTIVE=1` and extra environment variables.
pub fn run_pty_interactive_with_env(
    program: &Path,
    args: &[&str],
    cwd: &Path,
    home: &Path,
    input: &str,
    env: &[(&str, &str)],
) -> CmdOutput {
    run_pty_with_interactive_env(program, args, cwd, home, input, true, env)
}

fn run_pty_with_interactive_env(
    program: &Path,
    args: &[&str],
    cwd: &Path,
    home: &Path,
    input: &str,
    interactive: bool,
    env: &[(&str, &str)],
) -> CmdOutput {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 120,
            pixel_width: 0,
            pixel_height: 0,
        })
        .expect("openpty");

    let mut cmd = CommandBuilder::new(program);
    cmd.args(args);
    cmd.cwd(cwd);
    cmd.env("CI", "1");
    cmd.env("NO_COLOR", "1");
    let interactive_value = match interactive {
        true => "1",
        false => "0",
    };
    cmd.env("ITO_INTERACTIVE", interactive_value);
    cmd.env("TERM", "dumb");
    cmd.env("HOME", home);
    cmd.env("XDG_DATA_HOME", home);

    for (k, v) in env {
        cmd.env(k, v);
    }

    let mut child = pair.slave.spawn_command(cmd).expect("spawn_command");
    drop(pair.slave);

    if !input.is_empty() {
        let mut writer = pair.master.take_writer().expect("take_writer");
        writer.write_all(input.as_bytes()).expect("write input");
        writer.flush().ok();
    }

    // Read until EOF.
    let mut reader = pair.master.try_clone_reader().expect("clone_reader");
    let mut out = String::new();
    reader.read_to_string(&mut out).ok();

    let status = child.wait().expect("wait");
    let code = status.exit_code() as i32;

    CmdOutput {
        code,
        stdout: out,
        stderr: String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Uses `cat` which is not available on Windows
    #[test]
    #[cfg(unix)]
    fn pty_can_echo_input_via_cat() {
        // Smoke test to prove PTY wiring works.
        let home = tempfile::tempdir().expect("home");
        let cwd = tempfile::tempdir().expect("cwd");

        let out = run_pty(Path::new("cat"), &[], cwd.path(), home.path(), "hello\n");
        assert_eq!(out.code, 0);
        assert!(out.stdout.contains("hello"));
    }
}
