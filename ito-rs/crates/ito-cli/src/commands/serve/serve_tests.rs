use super::*;

/// Build a fake [`std::process::Output`] without spawning any process.
///
/// On Unix the raw wait-status word encodes the exit code in bits 8-15, so
/// `code << 8` is the minimal value that produces the desired exit code.
#[cfg(unix)]
fn make_output(code: i32, stdout: &[u8], stderr: &[u8]) -> std::io::Result<std::process::Output> {
    use std::os::unix::process::ExitStatusExt;
    Ok(std::process::Output {
        status: std::process::ExitStatus::from_raw(code << 8),
        stdout: stdout.to_vec(),
        stderr: stderr.to_vec(),
    })
}

#[test]
#[cfg(unix)]
fn detect_tailscale_ip_with_cmd_success() {
    let ip = detect_tailscale_ip_cmd(|| make_output(0, b"100.64.0.1\n", b"")).expect("detect");
    assert_eq!(ip, "100.64.0.1");
}

#[test]
#[cfg(unix)]
fn detect_tailscale_ip_with_cmd_errors_on_non_zero_exit() {
    let err = detect_tailscale_ip_cmd(|| make_output(1, b"", b"boom")).expect_err("should error");
    let msg = err.to_string();
    assert!(msg.contains("Tailscale command failed"));
    assert!(msg.contains("boom"));
}

#[test]
#[cfg(unix)]
fn detect_tailscale_ip_with_cmd_errors_on_empty_ip() {
    let err = detect_tailscale_ip_cmd(|| make_output(0, b"", b"")).expect_err("should error");
    assert!(err.to_string().contains("empty IP"));
}

#[test]
fn detect_tailscale_ip_with_cmd_errors_when_command_missing() {
    let err = detect_tailscale_ip_cmd(|| {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No such file or directory",
        ))
    })
    .expect_err("should error");
    assert!(err.to_string().contains("Failed to run 'tailscale ip -4'"));
}

#[test]
fn ensure_ito_dir_exists_errors_when_missing() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let err = ensure_ito_dir_exists(&ito).expect_err("should error");
    let msg = err.to_string();
    assert!(msg.contains("No .ito directory"));
    assert!(msg.contains("ito init"));
}

#[test]
fn ensure_ito_dir_exists_errors_when_path_is_file() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    std::fs::write(&ito, "not a dir").expect("write .ito file");

    let err = ensure_ito_dir_exists(&ito).expect_err("should error");
    assert!(err.to_string().contains("No .ito directory"));
}

#[test]
fn ensure_ito_dir_exists_ok_when_present() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).expect("create .ito");
    ensure_ito_dir_exists(&ito).expect("ok");
}
