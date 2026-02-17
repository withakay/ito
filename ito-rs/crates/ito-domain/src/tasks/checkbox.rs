/// Determines whether `id` looks like a checkbox task id token (e.g., `1`, `1.1`, `2.3.4`).
///
/// The token must be non-empty, start and end with an ASCII digit, contain only ASCII digits and
/// single dot separators, and must not contain consecutive dots.
///
/// # Examples
///
/// ```
/// assert!(is_checkbox_task_id_token("1"));
/// assert!(is_checkbox_task_id_token("1.2.3"));
/// assert!(!is_checkbox_task_id_token(""));
/// assert!(!is_checkbox_task_id_token(".1"));
/// assert!(!is_checkbox_task_id_token("1."));
/// assert!(!is_checkbox_task_id_token("1..2"));
/// ```
pub(super) fn is_checkbox_task_id_token(id: &str) -> bool {
    let bytes = id.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    if !bytes[0].is_ascii_digit() || !bytes[bytes.len() - 1].is_ascii_digit() {
        return false;
    }

    let mut prev_dot = false;
    for &b in bytes {
        if b.is_ascii_digit() {
            prev_dot = false;
            continue;
        }
        if b == b'.' {
            if prev_dot {
                return false;
            }
            prev_dot = true;
            continue;
        }
        return false;
    }
    true
}

/// Splits a string that begins with a checkbox-like ID token and returns the token and the remaining text.
///
/// Trims leading ASCII whitespace, accepts an ID token composed of digits separated by single dots (e.g., `1`, `1.1`, `2.3.4`),
/// and tolerates an optional trailing `:` or `.` on the token. Returns `None` if the input is empty, contains no ASCII whitespace
/// after a candidate token, or the token is not a valid checkbox task ID.
///
/// # Examples
///
/// ```
/// assert_eq!(split_checkbox_task_label("1.1: do this"), Some(("1.1", "do this")));
/// assert_eq!(split_checkbox_task_label("  2.3.4. do that"), Some(("2.3.4", "do that")));
/// assert_eq!(split_checkbox_task_label("no-id here"), None);
/// ```
pub(super) fn split_checkbox_task_label(s: &str) -> Option<(&str, &str)> {
    let s = s.trim_start();
    if s.is_empty() {
        return None;
    }

    // Slice at the first ASCII whitespace. This is safe because the prefix is ASCII.
    let bytes = s.as_bytes();
    let split_at = bytes.iter().position(|&b| b == b' ' || b == b'\t');

    let i = split_at?;
    let (token, rest) = s.split_at(i);
    let token = token.strip_suffix(':').unwrap_or(token);
    let token = token.strip_suffix('.').unwrap_or(token);
    if !is_checkbox_task_id_token(token) {
        return None;
    }

    Some((token, rest.trim()))
}