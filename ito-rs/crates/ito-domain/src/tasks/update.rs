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

/// Update a checkbox-format task's status.
///
/// If a checkbox item's text begins with an explicit id token (e.g. `1.1 First`),
/// `task_id` is matched against that token.
///
/// Otherwise, `task_id` is interpreted as a 1-based index of the checkbox items
/// in the file.
///
/// Returns the full updated file content, or an error if the task id was not found.
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

/// Update an enhanced-format task's status and `Updated At` metadata.
///
/// Uses regex replacement to preserve the existing structure and formatting
/// of the task block.
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
