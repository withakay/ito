//! Checkbox-format helpers.

/// Return true if `id` looks like a checkbox task id token (e.g. `1`, `1.1`, `2.3.4`).
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

/// If `s` begins with an id token (like `1.1`) followed by whitespace, split it into (id, rest).
///
/// Also tolerates `:` or `.` suffix on the token (`1.1:` / `1.1.`).
pub(super) fn split_checkbox_task_label(s: &str) -> Option<(&str, &str)> {
    let s = s.trim_start();
    if s.is_empty() {
        return None;
    }

    // Slice at the first ASCII whitespace. This is safe because the prefix is ASCII.
    let bytes = s.as_bytes();
    let mut split_at: Option<usize> = None;
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' || b == b'\t' {
            split_at = Some(i);
            break;
        }
    }

    let i = split_at?;
    let (token, rest) = s.split_at(i);
    let token = token.strip_suffix(':').unwrap_or(token);
    let token = token.strip_suffix('.').unwrap_or(token);
    if !is_checkbox_task_id_token(token) {
        return None;
    }

    Some((token, rest.trim()))
}
