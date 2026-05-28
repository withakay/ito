//! Jinja2 rendering for project templates (AGENTS.md, skills).
//!
//! Project templates may contain `minijinja` syntax (`{% ... %}` / `{{ ... }}`)
//! that gets rendered with a [`WorktreeTemplateContext`](crate::project_templates::WorktreeTemplateContext) before being written
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
    /// use ito_templates::project_templates::WorktreeTemplateContext;
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
#[path = "project_templates_tests.rs"]
mod project_templates_tests;
