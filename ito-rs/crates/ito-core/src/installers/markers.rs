use std::path::Path;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum MarkerError {
    #[error("Invalid marker state in {file_path}. End marker appears before start marker.")]
    EndBeforeStart { file_path: String },

    #[error(
        "Invalid marker state in {file_path}. Found start: {found_start}, Found end: {found_end}"
    )]
    MissingMarker {
        file_path: String,
        found_start: bool,
        found_end: bool,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum FsEditError {
    #[error(transparent)]
    Marker(#[from] MarkerError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

fn is_marker_on_own_line(content: &str, marker_index: usize, marker_len: usize) -> bool {
    let bytes = content.as_bytes();

    let mut i = marker_index;
    while i > 0 {
        let c = bytes[i - 1];
        if c == b'\n' {
            break;
        }
        if c != b' ' && c != b'\t' && c != b'\r' {
            return false;
        }
        i -= 1;
    }

    let mut j = marker_index + marker_len;
    while j < bytes.len() {
        let c = bytes[j];
        if c == b'\n' {
            break;
        }
        if c != b' ' && c != b'\t' && c != b'\r' {
            return false;
        }
        j += 1;
    }

    true
}

fn find_marker_index(content: &str, marker: &str, from_index: usize) -> Option<usize> {
    let mut search_from = from_index;
    while let Some(rel) = content[search_from..].find(marker) {
        let idx = search_from + rel;
        if is_marker_on_own_line(content, idx, marker.len()) {
            return Some(idx);
        }
        search_from = idx + marker.len();
    }
    None
}

pub fn update_content_with_markers(
    file_path: &Path,
    existing: Option<&str>,
    new_block_content: &str,
    start_marker: &str,
    end_marker: &str,
) -> Result<String, MarkerError> {
    let Some(existing) = existing else {
        return Ok(format!(
            "{start}\n{body}\n{end}",
            start = start_marker,
            body = new_block_content,
            end = end_marker
        ));
    };

    let start = find_marker_index(existing, start_marker, 0);
    let end = match start {
        Some(start_idx) => find_marker_index(existing, end_marker, start_idx + start_marker.len()),
        None => find_marker_index(existing, end_marker, 0),
    };

    match (start, end) {
        (Some(start_idx), Some(end_idx)) => {
            if end_idx < start_idx {
                return Err(MarkerError::EndBeforeStart {
                    file_path: file_path.display().to_string(),
                });
            }
            let before = &existing[..start_idx];
            let after = &existing[end_idx + end_marker.len()..];
            Ok(format!(
                "{before}{start}\n{body}\n{end}{after}",
                before = before,
                start = start_marker,
                body = new_block_content,
                end = end_marker,
                after = after
            ))
        }
        (None, None) => Ok(format!(
            "{start}\n{body}\n{end}\n\n{rest}",
            start = start_marker,
            body = new_block_content,
            end = end_marker,
            rest = existing
        )),
        (Some(_), None) => Err(MarkerError::MissingMarker {
            file_path: file_path.display().to_string(),
            found_start: true,
            found_end: false,
        }),
        (None, Some(_)) => Err(MarkerError::MissingMarker {
            file_path: file_path.display().to_string(),
            found_start: false,
            found_end: true,
        }),
    }
}

pub fn update_file_with_markers(
    file_path: &Path,
    new_block_content: &str,
    start_marker: &str,
    end_marker: &str,
) -> Result<String, FsEditError> {
    let existing = std::fs::read_to_string(file_path).ok();
    let updated = update_content_with_markers(
        file_path,
        existing.as_deref(),
        new_block_content,
        start_marker,
        end_marker,
    )?;

    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(file_path, &updated)?;
    Ok(updated)
}

#[cfg(test)]
mod tests {
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
        let err =
            update_content_with_markers(&p("f"), Some(&content), "NEW", START, END).unwrap_err();
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
        let err =
            update_content_with_markers(&p("f"), Some(&existing), "x", START, END).unwrap_err();
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
        let once =
            update_content_with_markers(&p("f"), Some(&existing), "hello", START, END).unwrap();
        let twice = update_content_with_markers(&p("f"), Some(&once), "hello", START, END).unwrap();
        assert_eq!(once, twice);
    }
}
