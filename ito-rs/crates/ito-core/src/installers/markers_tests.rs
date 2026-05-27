use super::*;
use std::path::PathBuf;

const START: &str = "<!-- ITO:START -->";
const END: &str = "<!-- ITO:END -->";

fn p(name: &str) -> PathBuf {
    PathBuf::from(name)
}

#[test]
fn marker_must_be_on_own_line() {
    let content = format!("prefix {START}\nX\n{END}\n");
    let err = update_content_with_markers(&p("f"), Some(&content), "NEW", START, END).unwrap_err();
    assert_eq!(
        err,
        MarkerError::MissingMarker {
            file_path: "f".to_string(),
            found_start: false,
            found_end: true
        }
    );
}

#[test]
fn replaces_existing_block_preserving_unmanaged_content() {
    let existing = format!("line1\n{START}\nold\n{END}\nline2\n");
    let out = update_content_with_markers(&p("f"), Some(&existing), "new", START, END).unwrap();
    assert_eq!(out, format!("line1\n{START}\nnew\n{END}\nline2\n"));
}

#[test]
fn inserts_block_when_missing() {
    let existing = "hello\nworld\n";
    let out = update_content_with_markers(&p("f"), Some(existing), "x", START, END).unwrap();
    assert_eq!(out, format!("{START}\nx\n{END}\n\nhello\nworld\n"));
}

#[test]
fn errors_when_only_one_marker_found() {
    let existing = format!("{START}\nno end\n");
    let err = update_content_with_markers(&p("f"), Some(&existing), "x", START, END).unwrap_err();
    assert_eq!(
        err,
        MarkerError::MissingMarker {
            file_path: "f".to_string(),
            found_start: true,
            found_end: false
        }
    );
}

#[test]
fn updates_file_on_disk() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("a.txt");
    let out = update_file_with_markers(&file, "hello", START, END).unwrap();
    assert_eq!(std::fs::read_to_string(&file).unwrap(), out);
}

#[test]
fn idempotent_when_applying_same_content_twice() {
    let existing = format!("{START}\nhello\n{END}\n");
    let once = update_content_with_markers(&p("f"), Some(&existing), "hello", START, END).unwrap();
    let twice = update_content_with_markers(&p("f"), Some(&once), "hello", START, END).unwrap();
    assert_eq!(once, twice);
}
