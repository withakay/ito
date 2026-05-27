use super::*;

#[test]
fn merge_jsonl_dedupes_and_appends_local_lines() {
    let a = task_event_id("2026-04-26T11:28:00.000Z", "1", "create", "pending");
    let b = task_event_id("2026-04-26T11:28:01.000Z", "2", "create", "pending");
    let c = task_event_id("2026-04-26T11:28:02.000Z", "3", "create", "pending");
    let remote = format!("{a}\n{b}\n");
    let local = format!("{b}\n{c}\n");
    let merged = merge_jsonl_lines(&remote, &local);
    assert_eq!(merged, format!("{a}\n{b}\n{c}\n"));
}

#[test]
fn merge_jsonl_ignores_blank_lines() {
    let event = task_event("2026-04-26T11:28:00.000Z", "create", "pending");
    let remote = format!("\n{event}\n\n");
    let local = "\n\n";
    let merged = merge_jsonl_lines(&remote, local);
    assert_eq!(merged, format!("{event}\n"));
}

#[test]
fn merge_jsonl_aggregates_adjacent_equivalent_reconciled_events() {
    let remote = format!(
        "{}\n",
        reconcile_event("2026-04-26T11:28:01.873Z", "pending")
    );
    let local = format!(
        "{}\n",
        reconcile_event("2026-04-26T11:28:21.643Z", "pending")
    );

    let merged = merge_jsonl_lines(&remote, &local);
    let mut lines = merged.lines();
    let event: serde_json::Value = serde_json::from_str(lines.next().unwrap()).unwrap();
    assert_eq!(event["count"], serde_json::json!(2));
    assert_eq!(lines.next(), None);
}

#[test]
fn merge_jsonl_keeps_reconciled_events_after_different_event() {
    let remote = format!(
        "{}\n{}\n",
        reconcile_event("2026-04-26T11:28:01.873Z", "pending"),
        task_event("2026-04-26T11:28:10.000Z", "status_change", "complete")
    );
    let local = format!(
        "{}\n",
        reconcile_event("2026-04-26T11:28:21.643Z", "pending")
    );

    let merged = merge_jsonl_lines(&remote, &local);
    assert_eq!(merged, format!("{}{}", remote, local));
}

#[test]
fn merge_jsonl_drops_events_older_than_one_month_from_newest_event() {
    let old = task_event_id("2026-03-01T00:00:00.000Z", "old", "create", "pending");
    let recent = task_event_id("2026-03-28T00:00:00.000Z", "recent", "create", "pending");
    let newest = task_event_id("2026-04-26T00:00:00.000Z", "newest", "create", "pending");
    let remote = format!("{old}\n{recent}\n");
    let local = format!("{newest}\n");

    let merged = merge_jsonl_lines(&remote, &local);
    assert_eq!(merged, format!("{recent}\n{newest}\n"));
}

#[test]
fn merge_jsonl_caps_git_log_to_newest_1000_events() {
    let mut local = String::new();
    for i in 0..1005 {
        local.push_str(&task_event_id(
            "2026-04-26T00:00:00.000Z",
            &i.to_string(),
            "create",
            "pending",
        ));
        local.push('\n');
    }

    let merged = merge_jsonl_lines("", &local);
    let lines: Vec<&str> = merged.lines().collect();
    assert_eq!(lines.len(), 1000);
    assert!(lines[0].contains("\"entity_id\":\"5\""));
    assert!(lines[999].contains("\"entity_id\":\"1004\""));
}

#[test]
fn merge_jsonl_count_cap_uses_timestamp_not_input_position() {
    let old = task_event_id("2026-04-01T00:00:00.000Z", "old", "create", "pending");
    let mut local = format!("{old}\n");
    for i in 0..1000 {
        local.push_str(&task_event_id(
            "2026-04-26T00:00:00.000Z",
            &i.to_string(),
            "create",
            "pending",
        ));
        local.push('\n');
    }

    let merged = merge_jsonl_lines("", &local);
    let lines: Vec<&str> = merged.lines().collect();
    assert_eq!(lines.len(), 1000);
    assert!(!merged.contains("\"entity_id\":\"old\""));
}

fn reconcile_event(ts: &str, from: &str) -> String {
    serde_json::json!({
        "v": 1,
        "ts": ts,
        "entity": "task",
        "entity_id": "3.2",
        "scope": "001-33_enhance-spec-driven-workflow-validation",
        "op": "reconciled",
        "from": from,
        "actor": "reconcile",
        "by": "@reconcile",
        "meta": {
            "reason": "task '3.2' has audit status 'pending' but no file entry"
        },
        "ctx": {
            "session_id": "test",
            "branch": "main",
            "worktree": "main",
            "commit": "abc123"
        }
    })
    .to_string()
}

fn task_event(ts: &str, op: &str, to: &str) -> String {
    task_event_id(ts, "3.2", op, to)
}

fn task_event_id(ts: &str, entity_id: &str, op: &str, to: &str) -> String {
    serde_json::json!({
        "v": 1,
        "ts": ts,
        "entity": "task",
        "entity_id": entity_id,
        "scope": "001-33_enhance-spec-driven-workflow-validation",
        "op": op,
        "to": to,
        "actor": "cli",
        "by": "@test",
        "ctx": {
            "session_id": "test"
        }
    })
    .to_string()
}
