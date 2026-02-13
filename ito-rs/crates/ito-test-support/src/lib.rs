//! Test helpers for the Ito workspace.
//!
//! This crate provides small utilities used in integration tests and snapshot
//! tests across the workspace. It is not intended for production code paths.

#![warn(missing_docs)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// In-memory mock implementations of domain repository traits for unit testing.
pub mod mock_repos;

/// PTY helpers for driving interactive commands in tests.
pub mod pty;

/// Captured output from running a command in tests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmdOutput {
    /// Process exit code (defaults to 1 when unavailable).
    pub code: i32,
    /// Captured stdout as UTF-8 (lossy).
    pub stdout: String,
    /// Captured stderr as UTF-8 (lossy).
    pub stderr: String,
}

impl CmdOutput {
    /// Return a version with normalized stdout/stderr.
    ///
    /// Normalization strips ANSI escapes, converts CRLF to LF, and replaces the
    /// provided `home` path with `<HOME>` for deterministic snapshots.
    pub fn normalized(&self, home: &Path) -> CmdOutput {
        CmdOutput {
            code: self.code,
            stdout: normalize_text(&self.stdout, home),
            stderr: normalize_text(&self.stderr, home),
        }
    }
}

/// Build a [`Command`] used to invoke the Rust candidate binary.
///
/// Tests use this to ensure a consistent base configuration before adding
/// arguments and environment.
pub fn rust_candidate_command(program: &Path) -> Command {
    Command::new(program)
}

/// Executes the Ito candidate binary with the supplied arguments in a deterministic test environment and returns the captured output.
///
/// The command is run with a stable set of environment variables (e.g. color and interactivity disabled, HOME and XDG_DATA_HOME set) and with repository-scoped Git environment variables removed to make outputs suitable for snapshot testing.
///
/// Returns a `CmdOutput` containing the process exit code, captured `stdout`, and captured `stderr`.
///
/// # Examples
///
/// ```ignore
/// use std::path::Path;
///
/// // Run the candidate binary and capture output for assertions or snapshots.
/// let out = run_rust_candidate(
///     Path::new("target/debug/ito"),
///     &["--version"],
///     Path::new("."),
///     Path::new("/home/example"),
/// );
/// assert!(out.stdout.contains("ito"));
/// ```
pub fn run_rust_candidate(program: &Path, args: &[&str], cwd: &Path, home: &Path) -> CmdOutput {
    let program = resolve_candidate_program(program);
    let mut cmd = rust_candidate_command(&program);
    cmd.args(args);
    run_with_env(&mut cmd, cwd, home)
}

/// Resolve a usable path to an `ito` candidate executable.
///
/// Attempts, in order: return `program` if it exists; use `CARGO_BIN_EXE_ito` if it points to an existing path; scan the `deps` directory adjacent to `program` for a file whose name starts with `ito-`, is not a common build artifact (`.d`, `.rlib`, `.rmeta`, `.o`), and appears executable for the current platform; otherwise returns the original `program` path.
///
/// # Returns
///
/// A `PathBuf` pointing to the resolved candidate executable path; this may be the original `program` if no alternative is found.
///
/// # Examples
///
/// ```ignore
/// use std::path::Path;
/// // If ./target/debug/ito exists this returns that path unchanged.
/// let p = resolve_candidate_program(Path::new("./target/debug/ito"));
/// assert!(p.ends_with("ito") || p.ends_with("ito.exe"));
/// ```
fn resolve_candidate_program(program: &Path) -> PathBuf {
    if program.exists() {
        return program.to_path_buf();
    }

    if let Some(path) = std::env::var_os("CARGO_BIN_EXE_ito") {
        let path = PathBuf::from(path);
        if path.exists() {
            return path;
        }
    }

    let Some(parent) = program.parent() else {
        return program.to_path_buf();
    };
    let deps = parent.join("deps");
    let Ok(entries) = std::fs::read_dir(&deps) else {
        return program.to_path_buf();
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|v| v.to_str()) else {
            continue;
        };
        if !name.starts_with("ito-") {
            continue;
        }
        if name.ends_with(".d")
            || name.ends_with(".rlib")
            || name.ends_with(".rmeta")
            || name.ends_with(".o")
        {
            continue;
        }
        if !is_executable_candidate(&path) {
            continue;
        }
        return path;
    }

    program.to_path_buf()
}

/// Returns whether the given path appears to be an executable on the current platform.
///
/// On Unix this requires the file to exist and have any executable permission bit set. On non-Unix platforms this accepts files with a case-insensitive `.exe` extension.
///
/// # Returns
///
/// `true` if the path points to a file that appears executable for the current platform, `false` otherwise.
///
/// # Examples
///
/// ```ignore
/// # use std::path::Path;
/// // Platform-dependent: on Unix this checks executable bits, on non-Unix this accepts `.exe`.
/// let _ = is_executable_candidate(Path::new("ito"));
/// ```
fn is_executable_candidate(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let Ok(metadata) = std::fs::metadata(path) else {
            return false;
        };
        metadata.permissions().mode() & 0o111 != 0
    }

    #[cfg(not(unix))]
    {
        path.extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("exe"))
    }
}

/// Run a Command with a deterministic, test-friendly environment and return its captured output.
///
/// This configures the command's working directory and environment for deterministic test runs
/// and clears repository-scoped Git environment variables so subprocesses do not inherit host
/// repository state. The function executes the command and returns a `CmdOutput` containing
/// the exit code, stdout, and stderr.
///
/// # Examples
///
/// ```ignore
/// use std::process::Command;
/// use std::path::Path;
///
/// let mut cmd = Command::new("echo");
/// cmd.arg("hello");
/// let out = crate::run_with_env(&mut cmd, Path::new("."), Path::new("/tmp"));
/// assert!(out.stdout.contains("hello"));
/// ```
fn run_with_env(cmd: &mut Command, cwd: &Path, home: &Path) -> CmdOutput {
    cmd.current_dir(cwd);

    // Determinism knobs.
    cmd.env("CI", "1");
    cmd.env("NO_COLOR", "1");
    cmd.env("ITO_INTERACTIVE", "0");
    cmd.env("TERM", "dumb");
    cmd.env("HOME", home);
    cmd.env("XDG_DATA_HOME", home);

    // Hooks (for example, git pre-push) can export repository-scoped Git
    // variables that break tests which create their own temporary repos.
    // Clear them so each test process resolves Git context from `cwd`.
    for key in [
        "GIT_DIR",
        "GIT_WORK_TREE",
        "GIT_COMMON_DIR",
        "GIT_INDEX_FILE",
        "GIT_OBJECT_DIRECTORY",
        "GIT_ALTERNATE_OBJECT_DIRECTORIES",
        "GIT_QUARANTINE_PATH",
        "GIT_PREFIX",
    ] {
        cmd.env_remove(key);
    }

    let out = cmd
        .output()
        .unwrap_or_else(|e| panic!("failed to execute {:?}: {e}", cmd));
    from_output(out)
}

fn from_output(out: Output) -> CmdOutput {
    CmdOutput {
        code: out.status.code().unwrap_or(1),
        stdout: bytes_to_string(&out.stdout),
        stderr: bytes_to_string(&out.stderr),
    }
}

fn bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).to_string()
}

/// Normalize text for deterministic snapshots.
///
/// This strips ANSI escape codes, converts CRLF to LF, and replaces occurrences
/// of the provided `home` path with `<HOME>`.
pub fn normalize_text(input: &str, home: &Path) -> String {
    let stripped = strip_ansi(input);
    let newlines = stripped.replace("\r\n", "\n");
    // Normalize temp HOME paths so snapshots are stable.
    let home_norm = home.to_string_lossy();
    newlines.replace(home_norm.as_ref(), "<HOME>")
}

/// Collect all file bytes under `root`, keyed by normalized relative paths.
///
/// Paths are normalized to use `/` separators so snapshots are stable across
/// platforms.
pub fn collect_file_bytes(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(base: &Path, dir: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for e in entries.flatten() {
            let Ok(ft) = e.file_type() else {
                continue;
            };
            let p = e.path();
            if ft.is_dir() {
                walk(base, &p, out);
                continue;
            }
            if !ft.is_file() {
                continue;
            }
            let rel = p
                .strip_prefix(base)
                .unwrap_or(&p)
                .to_string_lossy()
                .replace('\\', "/");
            let bytes = std::fs::read(&p).unwrap_or_default();
            out.insert(rel, bytes);
        }
    }

    let mut out: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    walk(root, root, &mut out);
    out
}

/// Replace the contents of `dst` with a recursive copy of `src`.
///
/// This is used in tests to reset a working directory to a known state without
/// needing platform-specific `rm -rf` behavior.
pub fn reset_dir(dst: &Path, src: &Path) -> std::io::Result<()> {
    let Ok(entries) = std::fs::read_dir(dst) else {
        return copy_dir_all(src, dst);
    };
    for e in entries.flatten() {
        let path = e.path();
        let Ok(ft) = e.file_type() else {
            continue;
        };
        if ft.is_dir() {
            let _ = std::fs::remove_dir_all(&path);
        } else {
            let _ = std::fs::remove_file(&path);
        }
    }
    copy_dir_all(src, dst)
}

/// Recursively copy `from` to `to`.
pub fn copy_dir_all(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;

    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src = entry.path();
        let dst = to.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&src, &dst)?;
        } else if ty.is_file() {
            std::fs::copy(&src, &dst)?;
        }
    }

    Ok(())
}

fn strip_ansi(input: &str) -> String {
    let bytes = strip_ansi_escapes::strip(input.as_bytes());
    bytes_to_string(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn normalize_strips_ansi_and_crlf() {
        let home = PathBuf::from("/tmp/home");
        let input = "\u{1b}[31mred\u{1b}[0m\r\nnext\r\n";
        let out = normalize_text(input, &home);
        assert_eq!(out, "red\nnext\n");
    }

    #[test]
    fn normalize_replaces_home_path() {
        let home = PathBuf::from("/tmp/some/home");
        let input = "path=/tmp/some/home/.ito";
        let out = normalize_text(input, &home);
        assert_eq!(out, "path=<HOME>/.ito");
    }

    #[test]
    fn copy_dir_all_copies_nested_files() {
        let src = tempfile::tempdir().expect("src");
        let dst = tempfile::tempdir().expect("dst");

        std::fs::create_dir_all(src.path().join("a/b")).unwrap();
        std::fs::write(src.path().join("a/b/file.txt"), "hello").unwrap();

        copy_dir_all(src.path(), dst.path()).unwrap();

        let copied = std::fs::read_to_string(dst.path().join("a/b/file.txt")).unwrap();
        assert_eq!(copied, "hello");
    }
}
