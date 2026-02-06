use std::fs;
use std::path::PathBuf;

fn schema_source_files() -> Vec<PathBuf> {
    let src_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files: Vec<PathBuf> = Vec::new();

    let entries = fs::read_dir(src_dir).expect("read_dir should succeed");
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    files.sort();
    files
}

fn assert_sources_do_not_reference(forbidden: &str) {
    for path in schema_source_files() {
        let content = fs::read_to_string(&path).expect("read_to_string should succeed");
        assert!(
            !content.contains(forbidden),
            "{} must not reference {}",
            path.display(),
            forbidden
        );
    }
}

#[test]
fn crate_has_crate_level_docs() {
    let lib_rs = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("lib.rs");
    let content = fs::read_to_string(lib_rs).expect("read_to_string should succeed");
    assert!(content.starts_with("//!"));
}

#[test]
fn schema_sources_do_not_reference_std_fs() {
    assert_sources_do_not_reference("std::fs");
}

#[test]
fn schema_sources_do_not_reference_process_command() {
    assert_sources_do_not_reference("std::process::Command");
}
