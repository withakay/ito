use super::{
    BackendHttpClient, backend_error_from_domain, is_not_found_error, map_status_to_backend_error,
    map_status_to_domain_error, map_status_to_task_error, optional_task_text_body, parse_timestamp,
    request_retries_enabled, retries_enabled_by_default, task_error_from_domain,
};
use crate::backend_client::BackendRuntime;
use ito_domain::backend::{
    BackendArchiveClient, BackendChangeReader, BackendEventIngestClient, BackendModuleReader,
    BackendSpecReader, BackendSyncClient, EventBatch,
};
use ito_domain::changes::ChangeLifecycleFilter;
use ito_domain::errors::DomainError;
use ito_domain::tasks::{TaskMutationService, TaskStatus, TasksFormat};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn get_requests_are_retried_by_default() {
    assert!(retries_enabled_by_default("GET"));
}

#[test]
fn post_requests_are_not_retried_by_default() {
    assert!(!retries_enabled_by_default("POST"));
    assert!(!retries_enabled_by_default("PUT"));
    assert!(!retries_enabled_by_default("PATCH"));
    assert!(!retries_enabled_by_default("DELETE"));
    assert!(!retries_enabled_by_default("HEAD"));
    assert!(!retries_enabled_by_default("OPTIONS"));
    assert!(!retries_enabled_by_default("TRACE"));
}

#[test]
fn audit_ingest_posts_can_opt_into_retries() {
    assert!(request_retries_enabled("POST", true));
    assert!(!request_retries_enabled("POST", false));
}

#[test]
fn optional_task_text_body_serializes_payload_when_present() {
    assert_eq!(
        optional_task_text_body("note", Some("done".to_string())),
        r#"{"note":"done"}"#
    );
    assert_eq!(
        optional_task_text_body("reason", Some("blocked".to_string())),
        r#"{"reason":"blocked"}"#
    );
}

#[test]
fn optional_task_text_body_uses_empty_object_when_absent() {
    assert_eq!(optional_task_text_body("note", None), "{}");
}

#[test]
fn parse_timestamp_returns_error_for_invalid_rfc3339() {
    assert!(parse_timestamp("not-a-timestamp").is_err());
}

#[test]
fn archived_task_fallback_only_treats_not_found_as_missing() {
    assert!(is_not_found_error(&DomainError::not_found("task", "1.1")));
    assert!(!is_not_found_error(&DomainError::io(
        "reading tasks",
        std::io::Error::other("boom"),
    )));
}

#[test]
fn status_mapping_functions_cover_domain_task_and_backend_errors() {
    let not_found = map_status_to_domain_error(404, "change", Some("025-11_demo"), "");
    assert!(matches!(not_found, DomainError::NotFound { .. }));

    let forbidden = map_status_to_domain_error(403, "change", None, "forbidden");
    assert!(forbidden.to_string().contains("forbidden"));

    let server = map_status_to_domain_error(503, "change", None, "");
    assert!(server.to_string().contains("HTTP 503"));

    assert!(
        map_status_to_task_error(404, r#"{"error":"missing","code":"not_found"}"#)
            .to_string()
            .contains("missing")
    );
    assert!(
        map_status_to_task_error(400, r#"{"error":"bad","code":"bad_request"}"#)
            .to_string()
            .contains("bad")
    );
    assert!(
        map_status_to_task_error(418, r#"{"error":"teapot","code":"other"}"#)
            .to_string()
            .contains("teapot")
    );
    assert!(
        map_status_to_task_error(409, "conflict")
            .to_string()
            .contains("HTTP 409")
    );
    assert!(
        map_status_to_task_error(503, "")
            .to_string()
            .contains("HTTP 503")
    );

    assert!(
        map_status_to_backend_error(401, "nope")
            .to_string()
            .contains("auth failed")
    );
    assert!(
        map_status_to_backend_error(403, "")
            .to_string()
            .contains("auth failed")
    );
    assert!(
        map_status_to_backend_error(404, "gone")
            .to_string()
            .contains("gone")
    );
    assert!(
        map_status_to_backend_error(409, "stale")
            .to_string()
            .contains("stale")
    );
    assert!(
        map_status_to_backend_error(503, "down")
            .to_string()
            .contains("down")
    );

    assert!(
        task_error_from_domain(DomainError::not_found("task", "1.1"))
            .to_string()
            .contains("task not found")
    );
    assert!(
        backend_error_from_domain(DomainError::not_found("spec", "alpha"))
            .to_string()
            .contains("spec not found")
    );
}

#[test]
fn backend_http_client_maps_project_read_task_sync_archive_and_event_paths() {
    let (base_url, requests) = serve_responses(vec![
        json_response(
            200,
            r#"[{"id":"025-11_demo","module_id":"007_core","completed_tasks":1,"shelved_tasks":0,"in_progress_tasks":1,"pending_tasks":2,"total_tasks":4,"has_proposal":true,"has_design":false,"has_specs":true,"has_tasks":true,"work_status":"in_progress","last_modified":"2026-05-27T10:00:00Z"}]"#,
        ),
        json_response(
            200,
            r###"{"id":"025-11_demo","module_id":"007_core","proposal":"# Proposal","design":null,"specs":[{"name":"alpha","content":"## ADDED"}],"progress":{"total":4,"complete":1,"shelved":0,"in_progress":1,"pending":2,"remaining":3},"last_modified":"2026-05-27T10:00:00Z"}"###,
        ),
        json_response(404, r#"{"error":"missing tasks","code":"not_found"}"#),
        json_response(
            200,
            r#"[{"id":"007_core","name":"Core","change_count":2,"sub_modules":[{"id":"007.01_api","name":"Api","change_count":1}]}]"#,
        ),
        json_response(
            200,
            r#"{"id":"007_core","name":"Core","description":"Core module","sub_modules":[{"id":"007.01_api","name":"Api","description":"Api submodule","change_count":1}]}"#,
        ),
        json_response(
            200,
            r#"[{"id":"alpha","path":".ito/specs/alpha/spec.md","last_modified":"2026-05-27T10:00:00Z"}]"#,
        ),
        json_response(
            200,
            r##"{"id":"alpha","path":".ito/specs/alpha/spec.md","markdown":"# Alpha","last_modified":"2026-05-27T10:00:00Z"}"##,
        ),
        json_response(
            200,
            r#"{"change_id":"025-11_demo","format":"enhanced","tasks":[{"id":"1.1","name":"Implement","wave":2,"status":"in-progress","requirements":["REQ-1"]},{"id":"1.2","name":"Verify","wave":2,"status":"unknown","dependencies":["1.1"]}],"progress":{"total":2,"complete":0,"shelved":0,"in_progress":1,"pending":1,"remaining":2}}"#,
        ),
        json_response(200, r##"{"change_id":"025-11_demo","content":"# Tasks"}"##),
        json_response(
            200,
            r#"{"change_id":"025-11_demo","path":".ito/changes/025-11_demo/tasks.md","existed":false,"revision":"rev-init"}"#,
        ),
        task_mutation_response("in-progress", "normal"),
        task_mutation_response("complete", "checkpoint"),
        task_mutation_response("shelved", "normal"),
        task_mutation_response("pending", "normal"),
        task_mutation_response("pending", "normal"),
        json_response(
            200,
            r###"{"change_id":"025-11_demo","proposal":"# Proposal","design":null,"tasks":"# Tasks","specs":[["alpha","## ADDED"]],"revision":"rev-1"}"###,
        ),
        json_response(200, r#"{"change_id":"025-11_demo","new_revision":"rev-2"}"#),
        json_response(
            200,
            r#"{"change_id":"025-11_demo","archived_at":"2026-05-27T10:00:00Z"}"#,
        ),
        json_response(200, r#"[]"#),
        json_response(200, r#"{"accepted":1,"duplicates":0}"#),
    ]);
    let client = BackendHttpClient::new(runtime_for(base_url));

    let changes = client
        .list_changes(ChangeLifecycleFilter::Active)
        .expect("list changes");
    assert_eq!(changes[0].sub_module_id, None);
    assert_eq!(changes[0].total_tasks, 4);

    let change = client
        .get_change("025-11_demo", ChangeLifecycleFilter::Archived)
        .expect("get archived change with task fallback");
    assert_eq!(change.specs[0].name, "alpha");
    assert_eq!(change.tasks.progress.total, 4);

    let modules = client.list_modules().expect("list modules");
    assert_eq!(modules[0].sub_modules[0].id, "007.01_api");
    let module = client.get_module("007_core").expect("get module");
    assert_eq!(module.sub_modules[0].sub_id, "01_api");

    let specs = client.list_specs().expect("list specs");
    assert_eq!(specs[0].id, "alpha");
    let spec = client.get_spec("alpha").expect("get spec");
    assert_eq!(spec.markdown, "# Alpha");

    let parsed = client
        .load_tasks_parse_result("025-11_demo")
        .expect("parse backend tasks");
    assert_eq!(parsed.format, TasksFormat::Enhanced);
    assert_eq!(parsed.tasks[0].status, TaskStatus::InProgress);
    assert_eq!(parsed.tasks[1].status, TaskStatus::Pending);
    assert_eq!(parsed.waves[0].wave, 2);
    assert_eq!(parsed.diagnostics.len(), 1);

    assert_eq!(
        client
            .load_tasks_markdown("025-11_demo")
            .expect("raw tasks")
            .as_deref(),
        Some("# Tasks")
    );
    assert_eq!(
        client
            .init_tasks("025-11_demo")
            .expect("init tasks")
            .revision
            .as_deref(),
        Some("rev-init")
    );
    assert_eq!(
        client
            .start_task("025-11_demo", "1.1")
            .expect("start task")
            .task
            .status,
        TaskStatus::InProgress
    );
    assert_eq!(
        client
            .complete_task("025-11_demo", "1.1", Some("done".to_string()))
            .expect("complete task")
            .task
            .kind,
        ito_domain::tasks::TaskKind::Checkpoint
    );
    assert_eq!(
        client
            .shelve_task("025-11_demo", "1.2", Some("blocked".to_string()))
            .expect("shelve task")
            .task
            .status,
        TaskStatus::Shelved
    );
    client
        .unshelve_task("025-11_demo", "1.2")
        .expect("unshelve task");
    client
        .add_task("025-11_demo", "Extra", Some(3))
        .expect("add task");

    let bundle = client.pull("025-11_demo").expect("pull bundle");
    assert_eq!(bundle.revision, "rev-1");
    let push = client.push("025-11_demo", &bundle).expect("push bundle");
    assert_eq!(push.new_revision, "rev-2");
    let archive = client.mark_archived("025-11_demo").expect("archive");
    assert_eq!(archive.change_id, "025-11_demo");
    assert!(client.list_audit_events().expect("list events").is_empty());
    assert_eq!(
        client
            .ingest(&EventBatch {
                events: vec![],
                idempotency_key: "empty-batch".to_string(),
            })
            .expect("ingest events")
            .accepted,
        1
    );

    let joined = requests.lock().expect("requests lock").join("\n");
    assert!(joined.contains("GET /api/v1/projects/withakay/ito/changes?lifecycle=active"));
    assert!(
        joined
            .contains("POST /api/v1/projects/withakay/ito/changes/025-11_demo/tasks/1.1/complete")
    );
    assert!(joined.contains(r#"{"note":"done"}"#));
    assert!(joined.contains(r#"{"title":"Extra","wave":3}"#));
}

fn runtime_for(base_url: String) -> BackendRuntime {
    BackendRuntime {
        base_url,
        token: "token".to_string(),
        timeout: Duration::from_secs(5),
        max_retries: 0,
        backup_dir: PathBuf::from("/tmp/ito-test-backups"),
        org: "withakay".to_string(),
        repo: "ito".to_string(),
    }
}

fn serve_responses(responses: Vec<String>) -> (String, Arc<Mutex<Vec<String>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind test server");
    let addr = listener.local_addr().expect("local addr");
    let requests = Arc::new(Mutex::new(Vec::new()));
    let captured = Arc::clone(&requests);
    thread::spawn(move || {
        for response in responses {
            let (mut stream, _) = listener.accept().expect("accept connection");
            let mut request = Vec::new();
            let mut buffer = [0; 1024];
            loop {
                let read = stream.read(&mut buffer).expect("read request");
                if read == 0 {
                    break;
                }
                request.extend_from_slice(&buffer[..read]);
                if request.windows(4).any(|window| window == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&request);
                    let content_length = headers
                        .lines()
                        .find_map(|line| {
                            let (name, value) = line.split_once(':')?;
                            name.eq_ignore_ascii_case("content-length")
                                .then(|| value.trim().parse::<usize>().ok())
                                .flatten()
                        })
                        .unwrap_or(0);
                    let header_end = request
                        .windows(4)
                        .position(|window| window == b"\r\n\r\n")
                        .map(|pos| pos + 4)
                        .expect("headers end");
                    while request.len().saturating_sub(header_end) < content_length {
                        let read = stream.read(&mut buffer).expect("read request body");
                        if read == 0 {
                            break;
                        }
                        request.extend_from_slice(&buffer[..read]);
                    }
                    break;
                }
            }
            captured
                .lock()
                .expect("request capture lock")
                .push(String::from_utf8_lossy(&request).to_string());
            stream
                .write_all(response.as_bytes())
                .expect("write response");
        }
    });
    (format!("http://{addr}"), requests)
}

fn json_response(status: u16, body: &str) -> String {
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        409 => "Conflict",
        503 => "Service Unavailable",
        _ => "Status",
    };
    format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
}

fn task_mutation_response(status: &str, kind: &str) -> String {
    json_response(
        200,
        &format!(
            r#"{{"change_id":"025-11_demo","task":{{"id":"1.1","name":"Task","wave":1,"status":"{status}","updated_at":"2026-05-27","dependencies":["1.0"],"files":["src/lib.rs"],"action":"Do it","verify":"cargo test","done_when":"done","kind":"{kind}","header_line_index":7,"requirements":["REQ-1"]}},"revision":"rev-task"}}"#
        ),
    )
}
