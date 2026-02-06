//! Helpers for updating `planning/STATE.md`.
//!
//! These functions implement small, targeted **pure** edits (e.g. updating the
//! last updated date or inserting new bullets) while keeping the rest of the
//! file intact.
//!
//! Clock helpers (`now_time`, `now_date`) live in `ito-core::time`.

use regex::Regex;

/// Update the `Last Updated:` line in a STATE.md document.
pub fn update_last_updated(contents: &str, date: &str) -> String {
    let re = Regex::new(r"(?m)^Last Updated: .+$").unwrap();
    if re.is_match(contents) {
        return re
            .replace_all(contents, format!("Last Updated: {date}").as_str())
            .to_string();
    }
    contents.to_string()
}

fn insert_after_heading(
    contents: &str,
    heading: &str,
    line_to_insert: &str,
) -> Result<String, String> {
    let mut out: Vec<String> = Vec::new();
    let mut inserted = false;
    for line in contents.lines() {
        out.push(line.to_string());
        if !inserted && line.trim() == heading {
            out.push(line_to_insert.to_string());
            inserted = true;
        }
    }
    if !inserted {
        return Err(format!("Missing heading: {heading}"));
    }
    let mut s = out.join("\n");
    s.push('\n');
    Ok(s)
}

/// Add a decision entry under "Recent Decisions" and update the last-updated date.
pub fn add_decision(contents: &str, date: &str, text: &str) -> Result<String, String> {
    let line = format!("- {date}: {text}");
    let updated = insert_after_heading(contents, "## Recent Decisions", &line)?;
    Ok(update_last_updated(&updated, date))
}

/// Add a question entry under "Open Questions" and update the last-updated date.
pub fn add_question(contents: &str, date: &str, text: &str) -> Result<String, String> {
    let line = format!("- [ ] {text}");
    let updated = insert_after_heading(contents, "## Open Questions", &line)?;
    Ok(update_last_updated(&updated, date))
}

/// Add a blocker entry under "Blockers" and update the last-updated date.
pub fn add_blocker(contents: &str, date: &str, text: &str) -> Result<String, String> {
    let mut lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();
    let mut i = 0usize;
    let mut in_blockers = false;
    let mut inserted = false;
    while i < lines.len() {
        let line = lines[i].as_str();
        if line.trim() == "## Blockers" {
            in_blockers = true;
            i += 1;
            continue;
        }
        if in_blockers {
            if line.starts_with("## ") {
                lines.insert(i, format!("- {text}"));
                inserted = true;
                break;
            }
            if line.trim() == "[None currently]" {
                lines[i] = format!("- {text}");
                inserted = true;
                break;
            }
            if line.trim().is_empty() {
                lines.insert(i, format!("- {text}"));
                inserted = true;
                break;
            }
        }
        i += 1;
    }
    if !inserted {
        return Err("Could not find Blockers section".to_string());
    }
    let mut out = lines.join("\n");
    out.push('\n');
    Ok(update_last_updated(&out, date))
}

/// Set the "Current Focus" section to `text` and update the last-updated date.
pub fn set_focus(contents: &str, date: &str, text: &str) -> Result<String, String> {
    // Match TS: /(## Current Focus\n)([^\n#]*)/
    let re = Regex::new(r"(?m)(## Current Focus\n)([^\n#]*)").unwrap();
    if !re.is_match(contents) {
        return Err("Could not find Current Focus section".to_string());
    }
    let updated = re
        .replace(contents, |caps: &regex::Captures<'_>| {
            format!("{}{}\n", &caps[1], text)
        })
        .to_string();
    Ok(update_last_updated(&updated, date))
}

/// Add a timestamped note under "Session Notes".
///
/// If a matching session header exists, the note is inserted immediately after it.
pub fn add_note(contents: &str, date: &str, time: &str, text: &str) -> Result<String, String> {
    let session_header = format!("### {date} Session");
    let entry = format!("- {time}: {text}");

    let mut lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();

    // If a matching session header exists, insert immediately after it.
    for i in 0..lines.len() {
        if lines[i].trim() == session_header {
            lines.insert(i + 1, entry);
            let mut out = lines.join("\n");
            out.push('\n');
            return Ok(update_last_updated(&out, date));
        }
    }

    // Otherwise, insert after Session Notes heading.
    for i in 0..lines.len() {
        if lines[i].trim() == "## Session Notes" {
            lines.insert(i + 1, session_header);
            lines.insert(i + 2, entry);
            let mut out = lines.join("\n");
            out.push('\n');
            return Ok(update_last_updated(&out, date));
        }
    }

    Err("Could not find Session Notes section".to_string())
}

// NOTE: `now_time()` and `now_date()` were moved to `ito_core::time` to
// keep the domain layer free of non-deterministic I/O.
