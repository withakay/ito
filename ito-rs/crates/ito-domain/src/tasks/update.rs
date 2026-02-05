use chrono::{DateTime, Local};
use regex::Regex;

use super::TaskStatus;

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

    let Ok(idx) = task_id.parse::<usize>() else {
        return Err(format!("Task \"{task_id}\" not found"));
    };
    if idx == 0 {
        return Err(format!("Task \"{task_id}\" not found"));
    }

    let mut count = 0usize;
    let mut lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();

    for line in &mut lines {
        let indent_len = line.len().saturating_sub(line.trim_start().len());
        let indent = &line[..indent_len];
        let t = &line[indent_len..];
        let bytes = t.as_bytes();
        if bytes.len() < 5 {
            continue;
        }
        let bullet = bytes[0] as char;
        if bullet != '-' && bullet != '*' {
            continue;
        }
        if bytes[1] != b' ' || bytes[2] != b'[' || bytes[4] != b']' {
            continue;
        }

        count += 1;
        if count != idx {
            continue;
        }

        let after = &t[5..];
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

    let mut lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();
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
            (Some(_), Some(_)) => {}
        }
    }

    // Preserve trailing newline behavior similar to TS templates.
    let mut out = lines.join("\n");
    out.push('\n');
    out
}
