//! Jinja2 rendering for project templates (AGENTS.md, skills).
//!
//! Project templates may contain `minijinja` syntax (`{% ... %}` / `{{ ... }}`)
//! that gets rendered with a [`WorktreeTemplateContext`] before being written
//! to disk. Templates without Jinja2 syntax are returned unchanged.

use serde::Serialize;

use crate::instructions::render_template_str;

/// Context for rendering worktree-aware project templates.
///
/// This carries the resolved worktree configuration values. Templates use
/// these fields in `{% if %}` / `{{ }}` blocks to emit strategy-specific
/// instructions.
#[derive(Debug, Clone, Serialize)]
pub struct WorktreeTemplateContext {
    /// Whether worktrees are enabled.
    pub enabled: bool,
    /// Strategy name (e.g., `"checkout_subdir"`, `"checkout_siblings"`,
    /// `"bare_control_siblings"`). Empty string when disabled.
    pub strategy: String,
    /// Directory name for worktree layouts (e.g., `"ito-worktrees"`).
    pub layout_dir_name: String,
    /// Integration mode (e.g., `"commit_pr"`, `"merge_parent"`).
    /// Empty string when disabled.
    pub integration_mode: String,
    /// Default branch name (e.g., `"main"`).
    pub default_branch: String,
    /// Absolute path to the project root. Empty string when not resolved.
    pub project_root: String,
}

impl Default for WorktreeTemplateContext {
    /// Creates a WorktreeTemplateContext initialized with safe defaults for a disabled worktree setup.
    ///
    /// Defaults:
    /// - `enabled`: false
    /// - `strategy`: empty string
    /// - `layout_dir_name`: "ito-worktrees"
    /// - `integration_mode`: empty string
    /// - `default_branch`: "main"
    /// - `project_root`: empty string
    ///
    /// # Examples
    ///
    /// ```
    /// let ctx = WorktreeTemplateContext::default();
    /// assert!(!ctx.enabled);
    /// assert_eq!(ctx.layout_dir_name, "ito-worktrees");
    /// assert_eq!(ctx.default_branch, "main");
    /// assert!(ctx.project_root.is_empty());
    /// ```
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: String::new(),
            layout_dir_name: "ito-worktrees".to_string(),
            integration_mode: String::new(),
            default_branch: "main".to_string(),
            project_root: String::new(),
        }
    }
}

/// Render a project template with the given worktree context.
///
/// If the template bytes are not valid UTF-8 or do not contain Jinja2 syntax
/// (`{%` or `{{`), the bytes are returned unchanged. Otherwise the template is
/// rendered through `minijinja` with `ctx` as the context.
///
/// # Errors
///
/// Returns a `minijinja::Error` if the template contains Jinja2 syntax but
/// fails to render (e.g., undefined variable in strict mode).
pub fn render_project_template(
    template_bytes: &[u8],
    ctx: &WorktreeTemplateContext,
) -> Result<Vec<u8>, minijinja::Error> {
    let Ok(text) = std::str::from_utf8(template_bytes) else {
        return Ok(template_bytes.to_vec());
    };

    if !contains_jinja2_syntax(text) {
        return Ok(template_bytes.to_vec());
    }

    let rendered = render_template_str(text, ctx)?;
    Ok(rendered.into_bytes())
}

/// Check whether a string contains Jinja2 template syntax.
fn contains_jinja2_syntax(text: &str) -> bool {
    text.contains("{%") || text.contains("{{")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_project_template_passes_plain_text_through() {
        let bytes = b"Hello, this is plain text.";
        let ctx = WorktreeTemplateContext::default();
        let result = render_project_template(bytes, &ctx).unwrap();
        assert_eq!(result, bytes);
    }

    #[test]
    fn render_project_template_passes_non_utf8_through() {
        let bytes = [0xff, 0x00, 0x41];
        let ctx = WorktreeTemplateContext::default();
        let result = render_project_template(&bytes, &ctx).unwrap();
        assert_eq!(result, bytes);
    }

    #[test]
    fn render_project_template_renders_simple_variable() {
        let template = b"Strategy: {{ strategy }}";
        let ctx = WorktreeTemplateContext {
            strategy: "checkout_subdir".to_string(),
            ..Default::default()
        };
        let result = render_project_template(template, &ctx).unwrap();
        assert_eq!(
            String::from_utf8(result).unwrap(),
            "Strategy: checkout_subdir"
        );
    }

    #[test]
    fn render_project_template_renders_conditional() {
        let template = b"{% if enabled %}Worktrees ON{% else %}Worktrees OFF{% endif %}";
        let ctx_enabled = WorktreeTemplateContext {
            enabled: true,
            strategy: "checkout_subdir".to_string(),
            ..Default::default()
        };
        let ctx_disabled = WorktreeTemplateContext::default();

        let on = render_project_template(template, &ctx_enabled).unwrap();
        assert_eq!(String::from_utf8(on).unwrap(), "Worktrees ON");

        let off = render_project_template(template, &ctx_disabled).unwrap();
        assert_eq!(String::from_utf8(off).unwrap(), "Worktrees OFF");
    }

    #[test]
    fn render_project_template_strict_on_undefined() {
        let template = b"{{ missing_var }}";
        let ctx = WorktreeTemplateContext::default();
        let err = render_project_template(template, &ctx).unwrap_err();
        assert_eq!(err.kind(), minijinja::ErrorKind::UndefinedError);
    }

    #[test]
    fn default_context_is_disabled() {
        let ctx = WorktreeTemplateContext::default();
        assert!(!ctx.enabled);
        assert!(ctx.strategy.is_empty());
        assert!(ctx.integration_mode.is_empty());
        assert_eq!(ctx.layout_dir_name, "ito-worktrees");
        assert_eq!(ctx.default_branch, "main");
        assert!(ctx.project_root.is_empty());
    }

    #[test]
    fn render_agents_md_with_checkout_subdir() {
        let agents_md = crate::default_project_files()
            .into_iter()
            .find(|f| f.relative_path == "AGENTS.md")
            .expect("AGENTS.md should exist in project templates");

        let ctx = WorktreeTemplateContext {
            enabled: true,
            strategy: "checkout_subdir".to_string(),
            layout_dir_name: "ito-worktrees".to_string(),
            integration_mode: "commit_pr".to_string(),
            default_branch: "main".to_string(),
            project_root: "/home/user/project".to_string(),
        };
        let rendered = render_project_template(agents_md.contents, &ctx).unwrap();
        let text = String::from_utf8(rendered).unwrap();

        assert!(text.contains("## Worktree Workflow"));
        assert!(text.contains("**Strategy:** `checkout_subdir`"));
        assert!(text.contains("git worktree add \"ito-worktrees/<change-name>\" -b <change-name>"));
        assert!(
            text.contains("/home/user/project/ito-worktrees/<change-name>/"),
            "should contain absolute worktree path"
        );
        assert!(
            !text.contains("<project-root>/"),
            "should not contain placeholder root"
        );
    }

    #[test]
    fn render_agents_md_with_checkout_siblings() {
        let agents_md = crate::default_project_files()
            .into_iter()
            .find(|f| f.relative_path == "AGENTS.md")
            .expect("AGENTS.md should exist in project templates");

        let ctx = WorktreeTemplateContext {
            enabled: true,
            strategy: "checkout_siblings".to_string(),
            layout_dir_name: "worktrees".to_string(),
            integration_mode: "merge_parent".to_string(),
            default_branch: "develop".to_string(),
            project_root: "/home/user/project".to_string(),
        };
        let rendered = render_project_template(agents_md.contents, &ctx).unwrap();
        let text = String::from_utf8(rendered).unwrap();

        assert!(text.contains("**Strategy:** `checkout_siblings`"));
        assert!(text.contains(
            "git worktree add \"../<project-name>-worktrees/<change-name>\" -b <change-name>"
        ));
        assert!(
            text.contains("/home/user/project/../<project-name>-worktrees/<change-name>/"),
            "should contain absolute sibling worktree path"
        );
        assert!(
            !text.contains("<project-root>/"),
            "should not contain placeholder root"
        );
    }

    #[test]
    fn render_agents_md_with_bare_control_siblings() {
        let agents_md = crate::default_project_files()
            .into_iter()
            .find(|f| f.relative_path == "AGENTS.md")
            .expect("AGENTS.md should exist in project templates");

        let ctx = WorktreeTemplateContext {
            enabled: true,
            strategy: "bare_control_siblings".to_string(),
            layout_dir_name: "ito-worktrees".to_string(),
            integration_mode: "commit_pr".to_string(),
            default_branch: "main".to_string(),
            project_root: "/home/user/project".to_string(),
        };
        let rendered = render_project_template(agents_md.contents, &ctx).unwrap();
        let text = String::from_utf8(rendered).unwrap();

        assert!(text.contains("**Strategy:** `bare_control_siblings`"));
        assert!(text.contains(".bare/"));
        assert!(text.contains("ito-worktrees/"));
        assert!(
            text.contains("/home/user/project/"),
            "should contain absolute bare repo path"
        );
        assert!(
            !text.contains("<project>/"),
            "should not contain placeholder project root"
        );
    }

    #[test]
    fn render_agents_md_with_worktrees_disabled() {
        let agents_md = crate::default_project_files()
            .into_iter()
            .find(|f| f.relative_path == "AGENTS.md")
            .expect("AGENTS.md should exist in project templates");

        let ctx = WorktreeTemplateContext::default();
        let rendered = render_project_template(agents_md.contents, &ctx).unwrap();
        let text = String::from_utf8(rendered).unwrap();

        assert!(text.contains("Worktrees are not configured for this project."));
        assert!(text.contains("Do NOT create git worktrees by default."));
    }
}
