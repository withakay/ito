//! Project planning file templates and helpers.
//!
//! Ito's planning area lives under `{ito_path}/planning`.
//! This module provides helpers for initializing that structure and parsing
//! a small amount of structured data from the generated markdown.

use regex::Regex;
use std::path::{Path, PathBuf};

/// Path to the planning directory (`{ito_path}/planning`).
pub fn planning_dir(ito_path: &Path) -> PathBuf {
    ito_path.join("planning")
}

/// Path to the milestones directory (`{ito_path}/planning/milestones`).
pub fn milestones_dir(ito_path: &Path) -> PathBuf {
    planning_dir(ito_path).join("milestones")
}

/// Default contents for `planning/PROJECT.md`.
pub fn project_md_template(project_name: Option<&str>, description: Option<&str>) -> String {
    let project_name = project_name.unwrap_or("[Project Name]");
    let description =
        description.unwrap_or("[1-2 sentence description of what we're building and why]");
    format!(
        "# Project: {project_name}\n\n## Vision\n{description}\n\n## Core Value Proposition\n[What makes this valuable to users]\n\n## Constraints\n- Technical: [stack, compatibility requirements]\n- Resources: [team size, expertise gaps]\n\n## Stakeholders\n- [Role]: [Concerns and success criteria]\n\n## Out of Scope\n- [Explicitly excluded features/concerns]\n\n## AI Assistant Notes\n[Special instructions for AI tools working on this project]\n- Preferred patterns: [...]\n- Avoid: [...]\n- Always check: [...]\n"
    )
}

/// Default contents for `planning/ROADMAP.md`.
pub fn roadmap_md_template() -> String {
    "# Roadmap\n\n## Current Milestone: v1-core\n- Status: Not Started\n- Phase: 0 of 0\n\n## Milestones\n\n### v1-core\nTarget: [Define the goal for this milestone]\n\n| Phase | Name | Status | Changes |\n|-------|------|--------|---------|\n| 1 | [Phase Name] | Pending | - |\n\n## Completed Milestones\n[None yet]\n"
        .to_string()
}

/// Default contents for `planning/STATE.md`.
pub fn state_md_template(current_date: &str, ito_dir: &str) -> String {
    format!(
        "# Project State\n\nLast Updated: {current_date}\n\n## Current Focus\n[What we're working on right now]\n\n## Recent Decisions\n- {current_date}: Project initialized\n\n## Open Questions\n- [ ] [Question needing resolution]\n\n## Blockers\n[None currently]\n\n## Session Notes\n### {current_date} - Initial Setup\n- Completed: Project planning structure initialized\n- Next: Define project vision and first milestone\n\n---\n## For AI Assistants\nWhen resuming work on this project:\n1. Read this STATE.md first\n2. Check ROADMAP.md for current phase\n3. Review any in-progress changes in `{ito_dir}/changes/`\n4. Continue from \"Current Focus\" above\n"
    )
}

/// Initialize the planning directory structure under `ito_path`.
///
/// This is safe to call multiple times; existing files are left unchanged.
pub fn init_planning_structure(
    ito_path: &Path,
    current_date: &str,
    ito_dir: &str,
) -> std::io::Result<()> {
    let planning = planning_dir(ito_path);
    std::fs::create_dir_all(&planning)?;
    std::fs::create_dir_all(milestones_dir(ito_path))?;

    let project_path = planning.join("PROJECT.md");
    if !project_path.exists() {
        std::fs::write(project_path, project_md_template(None, None))?;
    }
    let roadmap_path = planning.join("ROADMAP.md");
    if !roadmap_path.exists() {
        std::fs::write(roadmap_path, roadmap_md_template())?;
    }
    let state_path = planning.join("STATE.md");
    if !state_path.exists() {
        std::fs::write(state_path, state_md_template(current_date, ito_dir))?;
    }
    Ok(())
}

/// Parse the "Current Milestone" header block from roadmap markdown.
pub fn read_current_progress(roadmap_contents: &str) -> Option<(String, String, String)> {
    let re = Regex::new(r"## Current Milestone: (.+)\n- Status: (.+)\n- Phase: (.+)").unwrap();
    let caps = re.captures(roadmap_contents)?;
    Some((
        caps[1].to_string(),
        caps[2].to_string(),
        caps[3].to_string(),
    ))
}

/// Read phase rows from the roadmap milestone table.
///
/// Returns `(phase, name, status, changes)`.
pub fn read_phase_rows(roadmap_contents: &str) -> Vec<(String, String, String, String)> {
    let mut rows: Vec<(String, String, String, String)> = Vec::new();

    let mut in_table = false;
    let mut saw_sep = false;
    for line in roadmap_contents.lines() {
        let t = line.trim();
        if t == "| Phase | Name | Status | Changes |" {
            in_table = true;
            saw_sep = false;
            continue;
        }
        if !in_table {
            continue;
        }
        if !saw_sep {
            if t.starts_with("|-------") {
                saw_sep = true;
            }
            continue;
        }
        if t.is_empty() || !t.starts_with('|') {
            break;
        }

        let cols: Vec<String> = t
            .split('|')
            .map(|c| c.trim().to_string())
            .filter(|c| !c.is_empty())
            .collect();
        if cols.len() >= 4 {
            rows.push((
                cols[0].clone(),
                cols[1].clone(),
                cols[2].clone(),
                cols[3].clone(),
            ));
        }
    }
    rows
}
