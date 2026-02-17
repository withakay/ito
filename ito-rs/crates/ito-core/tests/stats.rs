use ito_core::stats::{collect_jsonl_files, compute_command_stats, known_command_ids};
use std::path::Path;

fn write(path: &Path, contents: &str) {
    let Some(parent) = path.parent() else {
        panic!("path has no parent: {}", path.display());
    };
    std::fs::create_dir_all(parent).expect("create dir should succeed");
    std::fs::write(path, contents).expect("write should succeed");
}

#[test]
fn collect_jsonl_files_finds_nested_jsonl_files() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let root = td.path();

    write(&root.join("a/one.jsonl"), "{}\n");
    write(&root.join("a/b/two.jsonl"), "{}\n");
    write(&root.join("a/b/not-jsonl.txt"), "nope\n");

    let files = collect_jsonl_files(root).expect("collect_jsonl_files");
    assert_eq!(files.len(), 2);

    let mut names = Vec::new();
    for p in &files {
        if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
            names.push(name);
        }
    }
    assert!(names.contains(&"one.jsonl"));
    assert!(names.contains(&"two.jsonl"));
}

#[test]
fn compute_command_stats_counts_command_end_events() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let root = td.path();

    // Mix valid/invalid lines and unrelated events.
    write(
        &root.join("events.jsonl"),
        r#"{"event_type":"command_end","command_id":"ito.init"}
{"event_type":"command_start","command_id":"ito.init"}
{"event_type":"command_end","command_id":"ito.init"}
{"event_type":"command_end","command_id":"ito.unknown"}
not json
{"event_type":"command_end"}
"#,
    );

    let stats = compute_command_stats(root).expect("compute_command_stats");

    // Known commands are always present with default 0.
    for id in known_command_ids() {
        assert!(stats.counts.contains_key(*id), "missing id: {id}");
    }

    assert_eq!(stats.counts.get("ito.init").copied(), Some(2));
    // Unknown command ids should also be counted (entry created on demand).
    assert_eq!(stats.counts.get("ito.unknown").copied(), Some(1));
}
