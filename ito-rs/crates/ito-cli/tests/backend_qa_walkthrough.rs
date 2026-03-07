use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;

// These constants mirror the defaults in qa/backend/test-backend-walkthrough.sh.
// Keeping them here as named constants makes it obvious when a rename in the
// script requires a matching update in this test.
const QA_ORG_A: &str = "acme";
const QA_REPO_A: &str = "widgets";
// The first change seeded for org A — unused in assertions but listed here
// so it's easy to find if the script's default value ever changes.
#[allow(dead_code)]
const QA_CHANGE_A: &str = "001-01_alpha-feature";
const QA_INGEST_KEY: &str = "qa-key-001";
const QA_SESSION_ID: &str = "backend-qa-walkthrough";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root should resolve")
}

/// Bind to an ephemeral port and return both the port and the listener.
///
/// The caller must keep the `TcpListener` alive until the child server process
/// has started listening.  Because the bash script launches a separate process
/// we cannot hold the reservation across process boundaries, so we instead
/// retry the entire `verify` invocation on collision.
fn bind_free_port() -> (u16, TcpListener) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind test port");
    let port = listener.local_addr().expect("local addr").port();
    (port, listener)
}

#[test]
fn backend_qa_script_verify_runs_end_to_end() {
    let repo_root = repo_root();
    let script_path = repo_root.join("qa/backend/test-backend-walkthrough.sh");
    let ito_bin = assert_cmd::cargo::cargo_bin!("ito");

    // Retry up to 3 times in case another process grabs the port after we
    // release the reservation but before the child server binds to it.
    const MAX_RETRIES: u32 = 3;
    let mut last_output = None;

    for attempt in 0..MAX_RETRIES {
        let qa_root = tempfile::tempdir().expect("qa root");
        let (port, listener) = bind_free_port();

        // Release the reservation immediately before the child binds.
        // We cannot pass an open socket across process boundaries here, so we
        // accept the small TOCTOU window and retry on collision instead.
        drop(listener);

        let output = Command::new("bash")
            .arg(&script_path)
            .arg("verify")
            .current_dir(&repo_root)
            .env("BACKEND_QA_NO_PAUSE", "1")
            .env("ITO_BACKEND_QA_ROOT", qa_root.path())
            .env("ITO_BACKEND_PORT", port.to_string())
            .env("ITO_BIN", ito_bin)
            .output()
            .expect("verify command should run");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("Verification passed"));

            let audit_log = qa_root.path().join(format!(
                "data/projects/{QA_ORG_A}/{QA_REPO_A}/.ito/.state/audit/events.jsonl"
            ));
            let audit_content =
                std::fs::read_to_string(&audit_log).expect("audit log should exist");
            assert_eq!(
                audit_content.lines().count(),
                1,
                "audit log should dedupe retry"
            );
            assert!(audit_content.contains(QA_SESSION_ID));

            let ingest_key = qa_root.path().join(format!(
                "data/projects/{QA_ORG_A}/{QA_REPO_A}/.ito/.state/ingest-keys/{QA_INGEST_KEY}"
            ));
            let ingest_count =
                std::fs::read_to_string(&ingest_key).expect("idempotency key should exist");
            assert_eq!(ingest_count.trim(), "1");
            return;
        }

        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        // Only retry on address-in-use collisions; fail fast for other errors.
        if !stderr.contains("address already in use") && !stderr.contains("already in use") {
            last_output = Some(output);
            break;
        }

        eprintln!(
            "attempt {}/{}: port {} was grabbed by another process, retrying",
            attempt + 1,
            MAX_RETRIES,
            port
        );
        last_output = Some(output);
    }

    let output = last_output.expect("at least one attempt");
    panic!(
        "verify failed after {} attempts\nstdout:\n{}\nstderr:\n{}",
        MAX_RETRIES,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
