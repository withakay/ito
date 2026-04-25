use ito_templates::agents::{Harness, get_agent_files};
/// CI guard: every embedded skill, command, and agent file must have a basename
/// that is either exactly `ito` or starts with `ito-`.
///
/// The sole exception is the bare `ito` entrypoint (e.g. `skills/ito/SKILL.md`,
/// `commands/ito.md`). All other top-level directory/file names MUST begin with
/// `ito-`.
///
/// This test walks the embedded assets via the public `skills_files()`,
/// `commands_files()`, and `agents::get_agent_files()` APIs so it exercises the
/// same code paths used at install time.
use ito_templates::{commands_files, skills_files};

/// Extract the top-level component of a relative path (the skill/command name).
///
/// For `"ito-feature/SKILL.md"` this returns `"ito-feature"`.
/// For `"ito-apply.md"` this returns `"ito-apply.md"` (flat file, no subdirectory).
fn top_level_name(rel_path: &str) -> &str {
    rel_path.split('/').next().unwrap_or(rel_path)
}

/// Returns `true` when `name` satisfies the prefix rule.
///
/// Valid names: exactly `"ito"`, exactly `"ito.md"`, or anything starting with
/// `"ito-"`.
fn satisfies_prefix_rule(name: &str) -> bool {
    // Strip a trailing `.md` extension for the bare-name check.
    let stem = name.strip_suffix(".md").unwrap_or(name);
    stem == "ito" || name.starts_with("ito-")
}

#[test]
fn skills_satisfy_ito_prefix_rule() {
    let violations: Vec<String> = skills_files()
        .into_iter()
        .filter_map(|f| {
            let name = top_level_name(f.relative_path);
            if satisfies_prefix_rule(name) {
                None
            } else {
                Some(format!(
                    "  skills/{} — top-level name '{}' must be 'ito' or start with 'ito-'",
                    f.relative_path, name
                ))
            }
        })
        .collect();

    if !violations.is_empty() {
        panic!(
            "Prefix-rule violations in assets/skills/:\n{}\n\n\
            Fix: rename the offending skill directory to start with 'ito-'.",
            violations.join("\n")
        );
    }
}

#[test]
fn commands_satisfy_ito_prefix_rule() {
    let violations: Vec<String> = commands_files()
        .into_iter()
        .filter_map(|f| {
            let name = top_level_name(f.relative_path);
            if satisfies_prefix_rule(name) {
                None
            } else {
                Some(format!(
                    "  commands/{} — top-level name '{}' must be 'ito' or start with 'ito-'",
                    f.relative_path, name
                ))
            }
        })
        .collect();

    if !violations.is_empty() {
        panic!(
            "Prefix-rule violations in assets/commands/:\n{}\n\n\
            Fix: rename the offending command file to start with 'ito-'.",
            violations.join("\n")
        );
    }
}

#[test]
fn agents_satisfy_ito_prefix_rule() {
    let mut violations: Vec<String> = Vec::new();

    for harness in Harness::all() {
        for (rel_path, _contents) in get_agent_files(*harness) {
            let name = top_level_name(rel_path);
            if !satisfies_prefix_rule(name) {
                violations.push(format!(
                    "  agents/{}/{} — top-level name '{}' must be 'ito' or start with 'ito-'",
                    harness.dir_name(),
                    rel_path,
                    name
                ));
            }
        }
    }

    if !violations.is_empty() {
        panic!(
            "Prefix-rule violations in assets/agents/:\n{}\n\n\
            Fix: rename the offending agent file to start with 'ito-'.",
            violations.join("\n")
        );
    }
}
