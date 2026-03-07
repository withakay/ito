use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root should resolve")
}

fn pick_free_port() -> u16 {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind test port");
    listener.local_addr().expect("local addr").port()
}

#[test]
fn backend_qa_script_verify_runs_end_to_end() {
    let repo_root = repo_root();
    let script_path = repo_root.join("scripts/backend-qa-walkthrough.sh");
    let qa_root = tempfile::tempdir().expect("qa root");
    let port = pick_free_port();
    let ito_bin = assert_cmd::cargo::cargo_bin!("ito");

    let output = Command::new("bash")
        .arg(script_path)
        .arg("verify")
        .current_dir(&repo_root)
        .env("BACKEND_QA_NO_PAUSE", "1")
        .env("ITO_BACKEND_QA_ROOT", qa_root.path())
        .env("ITO_BACKEND_PORT", port.to_string())
        .env("ITO_BIN", ito_bin)
        .output()
        .expect("verify command should run");

    assert!(
        output.status.success(),
        "verify failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Verification passed"));

    let audit_log = qa_root
        .path()
        .join("data/projects/acme/widgets/.ito/.state/audit/events.jsonl");
    let audit_content = std::fs::read_to_string(&audit_log).expect("audit log should exist");
    assert_eq!(
        audit_content.lines().count(),
        1,
        "audit log should dedupe retry"
    );
    assert!(audit_content.contains("backend-qa-walkthrough"));

    let ingest_key = qa_root
        .path()
        .join("data/projects/acme/widgets/.ito/.state/ingest-keys/qa-key-001");
    let ingest_count = std::fs::read_to_string(&ingest_key).expect("idempotency key should exist");
    assert_eq!(ingest_count.trim(), "1");
}
