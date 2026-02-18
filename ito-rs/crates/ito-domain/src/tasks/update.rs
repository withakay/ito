//! Helpers for updating task status in `tasks.md`.

use chrono::{DateTime, Local};
use regex::Regex;

use super::TaskStatus;

use super::checkbox::split_checkbox_task_label;
fn split_checkbox_line(t: &str) -> Option<(char, &str)> {
    let bytes = t.as_bytes();
    if bytes.len() < 5 {
        return None;
    }
    let bullet = bytes[0] as char;
    if bullet != '-' && bullet != '*' {
        return None;
    }
    if bytes[1] != b' ' || bytes[2] != b'[' || bytes[4] != b']' {
        return None;
    }
    Some((bullet, &t[5..]))
}

/// Update the status marker of a checkbox-formatted task in the given file contents.
///
/// This prefers an explicit numeric task label at the start of a checkbox item's text (e.g. `1.1 First`)
/// and falls back to interpreting `task_id` as a 1-based index of checkbox items when no explicit label matches.
/// Maps `TaskStatus` to checkbox markers: `Pending` -> `[ ]`, `InProgress` -> `[~]`, `Complete` -> `[x]`.
///
/// Returns `Ok(String)` with the full updated file content (always ending with a trailing newline),
/// or `Err(String)` if the requested task cannot be found or if `Shelved` is requested (not supported for checkbox-only tasks).
///
/// # Examples
///
/// ```
/// use ito_domain::tasks::{TaskStatus, update_checkbox_task_status};
/// let contents = "- [ ] 1.1 First task\n- [ ] Second task\n";
/// let updated = update_checkbox_task_status(contents, "1.1", TaskStatus::Complete).unwrap();
/// assert!(updated.contains("- [x] 1.1 First task"));
/// ```
pub fn update_checkbox_task_status(
    contents: &str,
    task_id: &str,
    new_status: TaskStatus,
) -> Result<String, String> {
    let new_marker = match new_status {
        TaskStatus::Pending => ' ',
        TaskStatus::InProgress => '~',
        TaskStatus::Complete => 'x',
        TaskStatus::Shelved => {
            return Err("Checkbox-only tasks.md does not support shelving".into());
        }
    };

    let mut lines: Vec<String> = Vec::new();
    for line in contents.lines() {
        lines.push(line.to_string());
    }

    // Prefer explicit ids when the task text starts with a numeric token (e.g. `1.1 First`).
    for line in &mut lines {
        let indent_len = line.len().saturating_sub(line.trim_start().len());
        let indent = &line[..indent_len];
        let t = &line[indent_len..];
        let Some((bullet, after)) = split_checkbox_line(t) else {
            continue;
        };

        let rest = after.trim_start();
        let Some((id, _name)) = split_checkbox_task_label(rest) else {
            continue;
        };
        if id != task_id {
            continue;
        }

        *line = format!("{indent}{bullet} [{new_marker}]{after}");

        let mut out = lines.join("\n");
        out.push('\n');
        return Ok(out);
    }

    let Ok(idx) = task_id.parse::<usize>() else {
        return Err(format!("Task \"{task_id}\" not found"));
    };
    if idx == 0 {
        return Err(format!("Task \"{task_id}\" not found"));
    }

    let mut count = 0usize;

    for line in &mut lines {
        let indent_len = line.len().saturating_sub(line.trim_start().len());
        let indent = &line[..indent_len];
        let t = &line[indent_len..];
        let Some((bullet, after)) = split_checkbox_line(t) else {
            continue;
        };

        count += 1;
        if count != idx {
            continue;
        }

        *line = format!("{indent}{bullet} [{new_marker}]{after}");
        break;
    }

    if count < idx {
        return Err(format!("Task \"{task_id}\" not found"));
    }

    let mut out = lines.join("\n");
    out.push('\n');
    Ok(out)
}

/// Update the status and "Updated At" metadata of an enhanced-format task block.
///
/// Locates the task block whose heading starts with `###` and contains the given `task_id`
/// (e.g., `### Task 123:` or `### 123:`), replaces or inserts the `- **Status**: ...` and
/// `- **Updated At**: YYYY-MM-DD` lines as needed, and returns the modified file contents
/// (ensuring a trailing newline).
///
/// # Examples
///
/// ```
/// use chrono::{Local, TimeZone};
/// use ito_domain::tasks::{TaskStatus, update_enhanced_task_status};
/// let contents = "## Project\n\n### Task 42: Example task\n- **Status**: [ ] pending\n";
/// let now = Local.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap();
/// let out = update_enhanced_task_status(contents, "42", TaskStatus::Complete, now);
/// assert!(out.contains("- **Status**: [x] complete"));
/// assert!(out.contains("- **Updated At**: 2025-02-01"));
/// ```
pub fn update_enhanced_task_status(
    contents: &str,
    task_id: &str,
    new_status: TaskStatus,
    now: DateTime<Local>,
) -> String {
    // Match TS: `^###\s+(?:Task\s+)?${taskId}\s*:`
    let heading = Regex::new(&format!(
        r"(?m)^###\s+(?:Task\s+)?{}\s*:\s*.+$",
        regex::escape(task_id)
    ))
    .unwrap();

    let status_line = match new_status {
        TaskStatus::Complete => "- **Status**: [x] complete".to_string(),
        TaskStatus::InProgress => "- **Status**: [ ] in-progress".to_string(),
        TaskStatus::Pending => "- **Status**: [ ] pending".to_string(),
        TaskStatus::Shelved => "- **Status**: [-] shelved".to_string(),
    };

    let date = now.format("%Y-%m-%d").to_string();
    let updated_at_line = format!("- **Updated At**: {date}");

    let mut lines: Vec<String> = Vec::new();
    for line in contents.lines() {
        lines.push(line.to_string());
    }
    let mut start_idx: Option<usize> = None;
    for (i, line) in lines.iter().enumerate() {
        if heading.is_match(line) {
            start_idx = Some(i);
            break;
        }
    }

    if let Some(start) = start_idx {
        let mut end = lines.len();
        for (i, line) in lines.iter().enumerate().skip(start + 1) {
            if line.starts_with("### ") || line.starts_with("## ") {
                end = i;
                break;
            }
        }

        let mut status_idx: Option<usize> = None;
        let mut updated_idx: Option<usize> = None;
        for (i, line) in lines.iter().enumerate().take(end).skip(start + 1) {
            let l = line.trim_start();
            if status_idx.is_none() && l.starts_with("- **Status**:") {
                status_idx = Some(i);
            }
            if updated_idx.is_none() && l.starts_with("- **Updated At**:") {
                updated_idx = Some(i);
            }
        }

        if let Some(i) = status_idx {
            lines[i] = status_line.clone();
        }
        if let Some(i) = updated_idx {
            lines[i] = updated_at_line.clone();
        }

        match (status_idx, updated_idx) {
            (Some(s), None) => {
                // Insert Updated At immediately before Status.
                lines.insert(s, updated_at_line);
            }
            (None, Some(u)) => {
                // Insert Status immediately after Updated At.
                lines.insert(u + 1, status_line);
            }
            (None, None) => {
                // Insert both at the end of the block.
                lines.insert(end, updated_at_line);
                lines.insert(end + 1, status_line);
            }
            (Some(_status_idx), Some(_updated_idx)) => {}
        }
    }

    // Preserve trailing newline behavior similar to TS templates.
    let mut out = lines.join("\n");
    out.push('\n');
    out
}
