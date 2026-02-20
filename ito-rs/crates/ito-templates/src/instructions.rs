//! Embedded instruction template loading and rendering.

use include_dir::{Dir, include_dir};
use minijinja::{Environment, UndefinedBehavior};
use serde::Serialize;

static INSTRUCTIONS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/instructions");

/// List all embedded instruction template paths.
pub fn list_instruction_templates() -> Vec<&'static str> {
    let mut out = Vec::new();
    collect_paths(&INSTRUCTIONS_DIR, &mut out);
    out.sort_unstable();
    out
}

/// Fetch an embedded instruction template as raw bytes.
pub fn get_instruction_template_bytes(path: &str) -> Option<&'static [u8]> {
    INSTRUCTIONS_DIR.get_file(path).map(|f| f.contents())
}

/// Fetch an embedded instruction template as UTF-8 text.
pub fn get_instruction_template(path: &str) -> Option<&'static str> {
    let bytes = get_instruction_template_bytes(path)?;
    std::str::from_utf8(bytes).ok()
}

/// Render an instruction template by path using a serializable context.
pub fn render_instruction_template<T: Serialize>(
    path: &str,
    ctx: &T,
) -> Result<String, minijinja::Error> {
    let template = get_instruction_template(path).ok_or_else(|| {
        minijinja::Error::new(minijinja::ErrorKind::TemplateNotFound, path.to_string())
    })?;
    render_template_str(template, ctx)
}

/// Render an arbitrary template string using a serializable context.
pub fn render_template_str<T: Serialize>(
    template: &str,
    ctx: &T,
) -> Result<String, minijinja::Error> {
    let mut env = Environment::new();
    env.set_undefined_behavior(UndefinedBehavior::Strict);

    // Templates are markdown; we don't want any escaping.
    env.set_auto_escape_callback(|_name| minijinja::AutoEscape::None);

    env.add_template("_inline", template)?;
    env.get_template("_inline")?.render(ctx)
}

fn collect_paths(dir: &'static Dir<'static>, out: &mut Vec<&'static str>) {
    for f in dir.files() {
        if let Some(p) = f.path().to_str() {
            out.push(p);
        }
    }
    for d in dir.dirs() {
        collect_paths(d, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_template_str_renders_from_serialize_ctx() {
        #[derive(Serialize)]
        struct Ctx {
            name: &'static str,
        }

        let out = render_template_str("hello {{ name }}", &Ctx { name: "world" }).unwrap();
        assert_eq!(out, "hello world");
    }

    #[test]
    fn render_template_str_is_strict_on_undefined() {
        #[derive(Serialize)]
        struct Ctx {}

        let err = render_template_str("hello {{ missing }}", &Ctx {}).unwrap_err();
        assert_eq!(err.kind(), minijinja::ErrorKind::UndefinedError);
    }

    #[test]
    fn list_instruction_templates_is_sorted_and_non_empty() {
        let templates = list_instruction_templates();
        assert!(!templates.is_empty());

        let mut sorted = templates.clone();
        sorted.sort_unstable();
        assert_eq!(templates, sorted);
    }

    #[test]
    fn template_fetchers_work_for_known_and_unknown_paths() {
        let templates = list_instruction_templates();
        let known = *templates
            .first()
            .expect("expected at least one embedded instruction template");

        let bytes = get_instruction_template_bytes(known);
        assert!(bytes.is_some());

        let text = get_instruction_template(known);
        assert!(text.is_some());

        assert_eq!(get_instruction_template_bytes("missing/template.md"), None);
        assert_eq!(get_instruction_template("missing/template.md"), None);
    }

    #[test]
    fn render_instruction_template_returns_not_found_for_missing_template() {
        #[derive(Serialize)]
        struct Ctx {}

        let err = render_instruction_template("missing/template.md", &Ctx {}).unwrap_err();
        assert_eq!(err.kind(), minijinja::ErrorKind::TemplateNotFound);
    }

    #[test]
    fn review_template_renders_conditional_sections() {
        #[derive(Serialize)]
        struct Artifact {
            id: &'static str,
            path: &'static str,
            present: bool,
        }

        #[derive(Serialize)]
        struct ValidationIssue {
            level: &'static str,
            path: &'static str,
            message: &'static str,
            line: Option<u32>,
            column: Option<u32>,
        }

        #[derive(Serialize)]
        struct TaskSummary {
            total: usize,
            complete: usize,
            in_progress: usize,
            pending: usize,
            shelved: usize,
            wave_count: usize,
        }

        #[derive(Serialize)]
        struct AffectedSpec {
            spec_id: &'static str,
            operation: &'static str,
            description: &'static str,
        }

        #[derive(Serialize)]
        struct TestingPolicy {
            tdd_workflow: &'static str,
            coverage_target_percent: u64,
        }

        #[derive(Serialize)]
        struct Ctx {
            change_name: &'static str,
            change_dir: &'static str,
            schema_name: &'static str,
            module_id: &'static str,
            module_name: &'static str,
            artifacts: Vec<Artifact>,
            validation_issues: Vec<ValidationIssue>,
            validation_passed: bool,
            task_summary: TaskSummary,
            affected_specs: Vec<AffectedSpec>,
            user_guidance: &'static str,
            testing_policy: TestingPolicy,
            generated_at: &'static str,
        }

        let ctx = Ctx {
            change_name: "000-01_test-change",
            change_dir: "/tmp/.ito/changes/000-01_test-change",
            schema_name: "spec-driven",
            module_id: "000_ungrouped",
            module_name: "Ungrouped",
            artifacts: vec![
                Artifact {
                    id: "proposal",
                    path: "/tmp/.ito/changes/000-01_test-change/proposal.md",
                    present: true,
                },
                Artifact {
                    id: "design",
                    path: "/tmp/.ito/changes/000-01_test-change/design.md",
                    present: false,
                },
                Artifact {
                    id: "tasks",
                    path: "/tmp/.ito/changes/000-01_test-change/tasks.md",
                    present: true,
                },
                Artifact {
                    id: "specs",
                    path: "/tmp/.ito/changes/000-01_test-change/specs",
                    present: true,
                },
            ],
            validation_issues: vec![ValidationIssue {
                level: "warning",
                path: ".ito/changes/000-01_test-change/tasks.md",
                message: "sample warning",
                line: Some(3),
                column: Some(1),
            }],
            validation_passed: false,
            task_summary: TaskSummary {
                total: 4,
                complete: 1,
                in_progress: 1,
                pending: 2,
                shelved: 0,
                wave_count: 2,
            },
            affected_specs: vec![AffectedSpec {
                spec_id: "agent-instructions",
                operation: "MODIFIED",
                description: "Review routing",
            }],
            user_guidance: "Follow strict review format.",
            testing_policy: TestingPolicy {
                tdd_workflow: "red-green-refactor",
                coverage_target_percent: 80,
            },
            generated_at: "2026-02-19T00:00:00Z",
        };

        let out = render_instruction_template("agent/review.md.j2", &ctx).unwrap();
        assert!(out.contains("Peer Review"));
        assert!(out.contains("## Proposal Review"));
        assert!(out.contains("## Spec Review"));
        assert!(out.contains("## Task Review"));
        assert!(!out.contains("## Design Review"));
        assert!(out.contains("## Testing Policy"));
        assert!(out.contains("<user_guidance>"));
        assert!(out.contains("## Output Format"));
        assert!(out.contains("Verdict: needs-discussion"));
    }
}
