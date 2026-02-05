use std::fs;
use std::path::{Path, PathBuf};

// Soft limit: files exceeding this will generate a warning but not fail the test.
// Hard limit: files exceeding this will fail the test.
const SOFT_LIMIT: usize = 1000;
const HARD_LIMIT: usize = 1200;

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir).expect("read_dir should succeed");
    for entry in entries {
        let entry = entry.expect("dir entry should be readable");
        let typ = entry.file_type().expect("file type should be readable");
        let path = entry.path();

        if typ.is_dir() {
            collect_rs_files(&path, out);
            continue;
        }

        if !typ.is_file() {
            continue;
        }

        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }

        out.push(path);
    }
}

#[test]
fn ito_cli_source_files_are_reasonably_sized() {
    // Guardrail: ensure no Rust source file under `src/` grows past the per-file limits.
    // This counts physical lines (including blank lines), not logical SLOC.
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = crate_root.join("src");

    let mut files: Vec<PathBuf> = Vec::new();
    collect_rs_files(&src_dir, &mut files);
    files.sort();

    let mut warnings: Vec<(PathBuf, usize)> = Vec::new();
    let mut errors: Vec<(PathBuf, usize)> = Vec::new();

    for path in files {
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
        let line_count = content.lines().count();
        if line_count > HARD_LIMIT {
            errors.push((path, line_count));
        } else if line_count > SOFT_LIMIT {
            warnings.push((path, line_count));
        }
    }

    // Print warnings but don't fail
    if !warnings.is_empty() {
        eprintln!("Warning: Found Rust source files exceeding soft limit of {SOFT_LIMIT} lines:");
        for (path, lines) in &warnings {
            let rel = path
                .strip_prefix(crate_root)
                .unwrap_or(path.as_path())
                .display();
            eprintln!("  - {rel}: {lines} lines (consider splitting)");
        }
    }

    // Fail on hard limit violations
    if errors.is_empty() {
        return;
    }

    let mut msg = format!(
        "Found Rust source files exceeding hard limit of {HARD_LIMIT} lines (physical lines):\n"
    );
    for (path, lines) in errors {
        let rel = path
            .strip_prefix(crate_root)
            .unwrap_or(path.as_path())
            .display();
        msg.push_str(&format!("- {rel}: {lines} lines\n"));
    }
    panic!("{msg}");
}
