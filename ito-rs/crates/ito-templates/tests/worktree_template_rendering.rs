//! Integration tests for worktree-aware template rendering.
//!
//! Renders both AGENTS.md and the worktree skill template with each of the
//! four worktree configuration states and asserts the output contains
//! expected content and does not contain discovery heuristics.

use ito_templates::project_templates::{WorktreeTemplateContext, render_project_template};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn agents_md_bytes() -> &'static [u8] {
    ito_templates::default_project_files()
        .into_iter()
        .find(|f| f.relative_path == "AGENTS.md")
        .expect("AGENTS.md should exist in project templates")
        .contents
}

fn skill_md_bytes() -> &'static [u8] {
    ito_templates::skills_files()
        .into_iter()
        .find(|f| f.relative_path == "using-git-worktrees/SKILL.md")
        .expect("using-git-worktrees/SKILL.md should exist in skill templates")
        .contents
}

fn render_text(template: &[u8], ctx: &WorktreeTemplateContext) -> String {
    let bytes = render_project_template(template, ctx).expect("template should render");
    String::from_utf8(bytes).expect("rendered output should be UTF-8")
}

/// Strings that indicate vague directory-discovery heuristics. None of these
/// should appear in rendered output for any configured state. Note: phrases
/// like "Do NOT ask the user" are the *opposite* of a heuristic and are
/// acceptable.
const DISCOVERY_HEURISTICS: &[&str] = &[
    "grep AGENTS.md",
    "grep -i \"worktree",
    "grep CLAUDE.md",
    "No worktree directory found",
    "Check Existing Directories",
    "Check AGENTS.md or CLAUDE.md",
    "Check CLAUDE.md",
    "ls -d ../ito-worktrees",
    "ls -d .worktrees",
];

/// Placeholders that indicate unresolved project root. These should NOT appear
/// in rendered output when a `project_root` is set (i.e., `enabled == true`).
const PLACEHOLDER_ROOTS: &[&str] = &["<project-root>/", "<project>/"];

/// Asserts that the rendered text does not contain any project-root placeholder strings.
///
/// This helper checks the global `PLACEHOLDER_ROOTS` array and panics if any placeholder
/// appears in `text`, using `label` in the assertion message for context.
///
/// # Parameters
///
/// - `text`: The rendered output to inspect for unresolved placeholder roots.
/// - `label`: A short identifier used in the assertion message to indicate which test or case failed.
///
/// # Panics
///
/// Panics if `text` contains any string from `PLACEHOLDER_ROOTS`.
///
/// # Examples
///
/// ```
/// let rendered = "Project files are located at /home/user/project/src/";
/// assert_no_placeholder_roots(rendered, "example");
/// ```
fn assert_no_placeholder_roots(text: &str, label: &str) {
    for placeholder in PLACEHOLDER_ROOTS {
        assert!(
            !text.contains(placeholder),
            "{label}: should not contain placeholder '{placeholder}' when project_root is set"
        );
    }
}

/// Asserts the rendered text contains the provided absolute project root.
///
/// Panics if `project_root` is not found in `text`.
///
/// # Examples
///
/// ```
/// let text = "/home/user/project/ito-worktrees/<change-name>/";
/// assert_has_absolute_paths(text, "/home/user/project", "AGENTS.md");
/// ```
fn assert_has_absolute_paths(text: &str, project_root: &str, label: &str) {
    assert!(
        text.contains(project_root),
        "{label}: should contain absolute project_root '{project_root}' in rendered output"
    );
}

/// Asserts that the given text does not contain any known discovery-heuristic substrings.
///
/// This helper checks the global `DISCOVERY_HEURISTICS` list and panics if any heuristic
/// string is found in `text`; the panic message includes `label` to identify the checked output.
///
/// # Parameters
///
/// - `text`: The rendered output or text to scan for discovery heuristics.
/// - `label`: A short identifier included in the panic message to indicate which output was tested.
///
/// # Examples
///
/// ```
/// let clean = "This output contains no discovery heuristics.";
/// assert_no_discovery_heuristics(clean, "clean-case");
/// ```
fn assert_no_discovery_heuristics(text: &str, label: &str) {
    for heuristic in DISCOVERY_HEURISTICS {
        assert!(
            !text.contains(heuristic),
            "{label}: should not contain discovery heuristic '{heuristic}'"
        );
    }
}

// ---------------------------------------------------------------------------
// Context factories
// ---------------------------------------------------------------------------

/// Create a WorktreeTemplateContext configured for the "checkout_subdir" strategy with typical test defaults.
///
/// The context is prefilled for rendering templates or tests that expect a checkout_subdir worktree layout.
///
/// # Returns
///
/// A WorktreeTemplateContext with `enabled = true`, `strategy = "checkout_subdir"`, `layout_dir_name = "ito-worktrees"`, `integration_mode = "commit_pr"`, `default_branch = "main"`, and `project_root = "/home/user/project"`.
///
/// # Examples
///
/// ```
/// let ctx = checkout_subdir_ctx();
/// assert_eq!(ctx.strategy, "checkout_subdir");
/// assert!(ctx.enabled);
/// assert_eq!(ctx.project_root, "/home/user/project");
/// ```
fn checkout_subdir_ctx() -> WorktreeTemplateContext {
    WorktreeTemplateContext {
        enabled: true,
        strategy: "checkout_subdir".to_string(),
        layout_dir_name: "ito-worktrees".to_string(),
        integration_mode: "commit_pr".to_string(),
        default_branch: "main".to_string(),
        project_root: "/home/user/project".to_string(),
    }
}

/// Creates a WorktreeTemplateContext configured for the "checkout_siblings" strategy.
///
/// The returned context has:
/// - enabled = true
/// - strategy = "checkout_siblings"
/// - layout_dir_name = "wt"
/// - integration_mode = "merge_parent"
/// - default_branch = "develop"
/// - project_root = "/home/user/project"
///
/// # Examples
///
/// ```
/// let ctx = checkout_siblings_ctx();
/// assert!(ctx.enabled);
/// assert_eq!(ctx.strategy, "checkout_siblings");
/// assert_eq!(ctx.layout_dir_name, "wt");
/// assert_eq!(ctx.integration_mode, "merge_parent");
/// assert_eq!(ctx.default_branch, "develop");
/// assert_eq!(ctx.project_root, "/home/user/project");
/// ```
fn checkout_siblings_ctx() -> WorktreeTemplateContext {
    WorktreeTemplateContext {
        enabled: true,
        strategy: "checkout_siblings".to_string(),
        layout_dir_name: "wt".to_string(),
        integration_mode: "merge_parent".to_string(),
        default_branch: "develop".to_string(),
        project_root: "/home/user/project".to_string(),
    }
}

/// Creates a WorktreeTemplateContext configured for the `bare_control_siblings` strategy.
///
/// Returns a WorktreeTemplateContext with:
/// - enabled = true
/// - strategy = "bare_control_siblings"
/// - layout_dir_name = "ito-worktrees"
/// - integration_mode = "commit_pr"
/// - default_branch = "main"
/// - project_root = "/home/user/project"
///
/// # Examples
///
/// ```
/// let ctx = bare_control_siblings_ctx();
/// assert!(ctx.enabled);
/// assert_eq!(ctx.strategy, "bare_control_siblings");
/// assert_eq!(ctx.project_root, "/home/user/project");
/// ```
fn bare_control_siblings_ctx() -> WorktreeTemplateContext {
    WorktreeTemplateContext {
        enabled: true,
        strategy: "bare_control_siblings".to_string(),
        layout_dir_name: "ito-worktrees".to_string(),
        integration_mode: "commit_pr".to_string(),
        default_branch: "main".to_string(),
        project_root: "/home/user/project".to_string(),
    }
}

fn disabled_ctx() -> WorktreeTemplateContext {
    WorktreeTemplateContext::default()
}

// ===========================================================================
// AGENTS.md tests
// ===========================================================================

#[test]
fn agents_md_checkout_subdir() {
    let ctx = checkout_subdir_ctx();
    let text = render_text(agents_md_bytes(), &ctx);

    assert!(text.contains("## Worktree Workflow"));
    assert!(text.contains("**Strategy:** `checkout_subdir`"));
    assert!(text.contains("git worktree add \"ito-worktrees/<change-name>\" -b <change-name>"));
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "agents_md_checkout_subdir");
    assert_no_placeholder_roots(&text, "agents_md_checkout_subdir");
    assert_has_absolute_paths(&text, &ctx.project_root, "agents_md_checkout_subdir");
    assert!(
        text.contains("/home/user/project/ito-worktrees/<change-name>/"),
        "should show absolute worktree path"
    );
}

#[test]
fn agents_md_checkout_siblings() {
    let ctx = checkout_siblings_ctx();
    let text = render_text(agents_md_bytes(), &ctx);

    assert!(text.contains("**Strategy:** `checkout_siblings`"));
    assert!(
        text.contains("git worktree add \"../<project-name>-wt/<change-name>\" -b <change-name>")
    );
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "agents_md_checkout_siblings");
    assert_no_placeholder_roots(&text, "agents_md_checkout_siblings");
    assert_has_absolute_paths(&text, &ctx.project_root, "agents_md_checkout_siblings");
    assert!(
        text.contains("/home/user/project/../<project-name>-wt/<change-name>/"),
        "should show absolute sibling worktree path"
    );
}

#[test]
fn agents_md_bare_control_siblings() {
    let ctx = bare_control_siblings_ctx();
    let text = render_text(agents_md_bytes(), &ctx);

    assert!(text.contains("**Strategy:** `bare_control_siblings`"));
    assert!(text.contains(".bare/"));
    assert!(text.contains("ito-worktrees/"));
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "agents_md_bare_control_siblings");
    assert_no_placeholder_roots(&text, "agents_md_bare_control_siblings");
    assert_has_absolute_paths(&text, &ctx.project_root, "agents_md_bare_control_siblings");
    assert!(
        text.contains("/home/user/project/"),
        "should show absolute bare repo path"
    );
}

#[test]
fn agents_md_disabled() {
    let text = render_text(agents_md_bytes(), &disabled_ctx());

    assert!(text.contains("Worktrees are not configured for this project."));
    assert!(text.contains("Do NOT create git worktrees by default."));
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "agents_md_disabled");
}

// ===========================================================================
// Worktree skill tests
// ===========================================================================

#[test]
fn skill_checkout_subdir() {
    let ctx = checkout_subdir_ctx();
    let text = render_text(skill_md_bytes(), &ctx);

    assert!(text.contains("**Configured strategy:** `checkout_subdir`"));
    assert!(text.contains("git worktree add \"ito-worktrees/<change-name>\" -b <change-name>"));
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "skill_checkout_subdir");
    assert_no_placeholder_roots(&text, "skill_checkout_subdir");
    assert_has_absolute_paths(&text, &ctx.project_root, "skill_checkout_subdir");
}

#[test]
fn skill_checkout_siblings() {
    let ctx = checkout_siblings_ctx();
    let text = render_text(skill_md_bytes(), &ctx);

    assert!(text.contains("**Configured strategy:** `checkout_siblings`"));
    assert!(
        text.contains("git worktree add \"../<project-name>-wt/<change-name>\" -b <change-name>")
    );
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "skill_checkout_siblings");
    assert_no_placeholder_roots(&text, "skill_checkout_siblings");
    assert_has_absolute_paths(&text, &ctx.project_root, "skill_checkout_siblings");
}

#[test]
fn skill_bare_control_siblings() {
    let ctx = bare_control_siblings_ctx();
    let text = render_text(skill_md_bytes(), &ctx);

    assert!(text.contains("**Configured strategy:** `bare_control_siblings`"));
    assert!(text.contains("ito-worktrees/"));
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "skill_bare_control_siblings");
    assert_no_placeholder_roots(&text, "skill_bare_control_siblings");
    assert_has_absolute_paths(&text, &ctx.project_root, "skill_bare_control_siblings");
}

#[test]
fn skill_disabled() {
    let text = render_text(skill_md_bytes(), &disabled_ctx());

    assert!(text.contains("Worktrees are not configured for this project."));
    assert!(text.contains("Work in the current checkout."));
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "skill_disabled");
}
