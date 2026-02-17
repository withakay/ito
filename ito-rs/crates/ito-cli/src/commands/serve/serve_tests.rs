use super::*;
use std::path::Path;

#[cfg(unix)]
fn write_exe(path: &Path, contents: &str) {
    use std::os::unix::fs::PermissionsExt;

    let Some(parent) = path.parent() else {
        panic!("path has no parent: {}", path.display());
    };
    std::fs::create_dir_all(parent).expect("create dir");
    std::fs::write(path, contents).expect("write exe");
    let mut perms = std::fs::metadata(path).expect("metadata").permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms).expect("chmod");
}

#[test]
#[cfg(unix)]
fn detect_tailscale_ip_with_cmd_success() {
    let td = tempfile::tempdir().expect("tempdir");
    let cmd = td.path().join("tailscale");
    write_exe(&cmd, "#!/bin/sh\necho 100.64.0.1\n");

    let ip = detect_tailscale_ip_with(&cmd).expect("detect");
    assert_eq!(ip, "100.64.0.1");
}

#[test]
#[cfg(unix)]
fn detect_tailscale_ip_with_cmd_errors_on_non_zero_exit() {
    let td = tempfile::tempdir().expect("tempdir");
    let cmd = td.path().join("tailscale");
    write_exe(&cmd, "#!/bin/sh\necho boom 1>&2\nexit 1\n");

    let err = detect_tailscale_ip_with(&cmd).expect_err("should error");
    let msg = err.to_string();
    assert!(msg.contains("Tailscale command failed"));
    assert!(msg.contains("boom"));
}

#[test]
#[cfg(unix)]
fn detect_tailscale_ip_with_cmd_errors_on_empty_ip() {
    let td = tempfile::tempdir().expect("tempdir");
    let cmd = td.path().join("tailscale");
    write_exe(&cmd, "#!/bin/sh\nexit 0\n");

    let err = detect_tailscale_ip_with(&cmd).expect_err("should error");
    assert!(err.to_string().contains("empty IP"));
}

#[test]
fn detect_tailscale_ip_with_cmd_errors_when_command_missing() {
    let td = tempfile::tempdir().expect("tempdir");
    let cmd = td.path().join("missing-tailscale-bin");
    let err = detect_tailscale_ip_with(&cmd).expect_err("should error");
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
fn ensure_ito_dir_exists_ok_when_present() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).expect("create .ito");
    ensure_ito_dir_exists(&ito).expect("ok");
}
