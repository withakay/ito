use ito_templates::agents::{Harness, get_agent_files};
/// CI guard: every embedded markdown file (not `.md.j2`) must contain both
/// `<!-- ITO:START -->` and `<!-- ITO:END -->` on their own lines.
///
/// This test walks every embedded asset via the public API and asserts that
/// each `.md` file contains the managed-block pair.  On failure the diagnostic
/// lists exactly which files are missing which marker so contributors know what
/// to fix.
use ito_templates::{
    ITO_END_MARKER, ITO_START_MARKER, commands_files, default_project_files, schema_files,
    skills_files,
};

/// Returns `true` when `marker` appears on its own line in `text`.
fn marker_on_own_line(text: &str, marker: &str) -> bool {
    text.lines().any(|line| line.trim() == marker)
}

struct Violation {
    path: String,
    missing_start: bool,
    missing_end: bool,
}

impl Violation {
    fn message(&self) -> String {
        match (self.missing_start, self.missing_end) {
            (true, true) => format!(
                "  {} — missing both ITO:START and ITO:END markers",
                self.path
            ),
            (true, false) => format!("  {} — missing ITO:START marker", self.path),
            (false, true) => format!("  {} — missing ITO:END marker", self.path),
            (false, false) => unreachable!("Violation constructed without missing markers"),
        }
    }
}

fn check_bytes(path: &str, contents: &[u8], violations: &mut Vec<Violation>) {
    // Only check plain markdown files, not Jinja templates.
    if !path.ends_with(".md") || path.ends_with(".md.j2") {
        return;
    }

    let Ok(text) = std::str::from_utf8(contents) else {
        // Non-UTF-8 markdown is itself a problem, but out of scope here.
        return;
    };

    let missing_start = !marker_on_own_line(text, ITO_START_MARKER);
    let missing_end = !marker_on_own_line(text, ITO_END_MARKER);

    if missing_start || missing_end {
        violations.push(Violation {
            path: path.to_string(),
            missing_start,
            missing_end,
        });
    }
}

#[test]
fn skills_have_managed_markers() {
    let mut violations = Vec::new();
    for f in skills_files() {
        check_bytes(f.relative_path, f.contents, &mut violations);
    }
    assert_violations("assets/skills/", violations);
}

#[test]
fn commands_have_managed_markers() {
    let mut violations = Vec::new();
    for f in commands_files() {
        check_bytes(f.relative_path, f.contents, &mut violations);
    }
    assert_violations("assets/commands/", violations);
}

#[test]
fn agents_have_managed_markers() {
    let mut violations = Vec::new();
    for harness in Harness::all() {
        for (rel_path, contents) in get_agent_files(*harness) {
            let full_path = format!("assets/agents/{}/{}", harness.dir_name(), rel_path);
            check_bytes(&full_path, contents, &mut violations);
        }
    }
    assert_violations("assets/agents/", violations);
}

#[test]
fn default_project_files_have_managed_markers() {
    let mut violations = Vec::new();
    for f in default_project_files() {
        check_bytes(f.relative_path, f.contents, &mut violations);
    }
    assert_violations("assets/default/project/", violations);
}

#[test]
fn schema_files_have_managed_markers() {
    let mut violations = Vec::new();
    for f in schema_files() {
        check_bytes(f.relative_path, f.contents, &mut violations);
    }
    assert_violations("assets/schemas/", violations);
}

fn assert_violations(dir: &str, violations: Vec<Violation>) {
    if violations.is_empty() {
        return;
    }
    let messages: Vec<String> = violations.iter().map(|v| v.message()).collect();
    panic!(
        "Managed-marker violations in {}:\n{}\n\n\
        Fix: wrap the Ito-owned body of each file in:\n\
        \n\
        <!-- ITO:START -->\n\
        <content>\n\
        <!-- ITO:END -->",
        dir,
        messages.join("\n")
    );
}
