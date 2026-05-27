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
