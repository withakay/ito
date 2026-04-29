//! Integration tests for worktree-aware template rendering.
//!
//! Renders both AGENTS.md and the worktree skill template with each of the
//! four worktree configuration states and asserts the output contains
//! expected content and does not contain discovery heuristics.

use ito_templates::project_templates::{WorktreeTemplateContext, render_project_template};

const READ_ONLY_MAIN_RULE: &str = "Treat the main/control checkout";
const MAIN_BRANCH_EXCLUSIVE_RULE: &str =
    "The main worktree is the only worktree that may check out";
const BEFORE_WRITE_WORKTREE_RULE: &str = "Before any write operation, create a dedicated change worktree or move into the existing worktree for that change";
const NO_MAIN_WRITE_RULE: &str = "Do not write there: no proposal artifacts, code edits, documentation edits, generated asset updates, commits, or implementation work";

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

/// Returns the raw bytes of the `ito-using-git-worktrees/SKILL.md` template.
///
/// Panics if the template file is not present in the registered skill templates.
///
/// # Examples
///
/// ```
/// let bytes = skill_md_bytes();
/// let s = std::str::from_utf8(bytes).unwrap();
/// assert!(!s.is_empty());
/// ```
fn skill_md_bytes() -> &'static [u8] {
    ito_templates::skills_files()
        .into_iter()
        .find(|f| f.relative_path == "ito-using-git-worktrees/SKILL.md")
        .expect("ito-using-git-worktrees/SKILL.md should exist in skill templates")
        .contents
}

fn render_text(template: &[u8], ctx: &WorktreeTemplateContext) -> String {
    let bytes = render_project_template(template, ctx).expect("template should render");
    String::from_utf8(bytes).expect("rendered output should be UTF-8")
}

fn assert_main_worktree_guardrails(text: &str) {
    assert!(text.contains(READ_ONLY_MAIN_RULE));
    assert!(text.contains(MAIN_BRANCH_EXCLUSIVE_RULE));
    assert!(text.contains(BEFORE_WRITE_WORKTREE_RULE));
    assert!(text.contains(NO_MAIN_WRITE_RULE));
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

/// Asserts that the rendered text does not contain a machine-specific absolute project root.
///
/// Panics if `text` contains `project_root`; the panic message includes `label` and the
/// offending `project_root`.
///
/// # Examples
///
/// ```
/// let rendered = "Use .ito-worktrees/feature-x/ for worktree layout.";
/// assert_no_absolute_project_root(rendered, "/home/user/project", "agents_md_checkout_subdir");
/// ```
fn assert_no_absolute_project_root(text: &str, project_root: &str, label: &str) {
    assert!(
        !text.contains(project_root),
        "{label}: should not embed machine-specific absolute project_root '{project_root}'"
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

/// Verifies that AGENTS.md renders appropriate worktree instructions and paths for the `checkout_subdir` strategy.
///
/// Ensures the rendered text includes the "Worktree Workflow" section, the configured strategy label,
/// a repository-relative `.ito-worktrees/<full-change-id>/` worktree path and the corresponding `git worktree add` example,
/// and that it does not contain raw template syntax, discovery heuristics, or an embedded absolute project root.
///
/// # Examples
///
/// ```
/// let ctx = checkout_subdir_ctx();
/// let text = render_text(agents_md_bytes(), &ctx);
/// assert!(text.contains("**Strategy:** `checkout_subdir`"));
/// assert!(text.contains(".ito-worktrees/<full-change-id>/"));
/// ```
#[test]
fn agents_md_checkout_subdir() {
    let ctx = checkout_subdir_ctx();
    let text = render_text(agents_md_bytes(), &ctx);

    assert!(text.contains("## Worktree Workflow"));
    assert!(text.contains("**Strategy:** `checkout_subdir`"));
    assert!(
        text.contains(
            "git worktree add \".ito-worktrees/<full-change-id>\" -b <full-change-id> main"
        )
    );
    assert_main_worktree_guardrails(&text);
    assert!(
        text.contains("Use the full change ID as the branch and primary worktree directory name")
    );
    assert!(text.contains("Do not reuse one worktree for two changes"));
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "agents_md_checkout_subdir");
    assert!(
        text.contains(".ito-worktrees/<full-change-id>/"),
        "should show repo-relative worktree path"
    );
    assert_no_absolute_project_root(&text, &ctx.project_root, "agents_md_checkout_subdir");
}

#[test]
fn agents_md_checkout_siblings() {
    let ctx = checkout_siblings_ctx();
    let text = render_text(agents_md_bytes(), &ctx);

    assert!(text.contains("**Strategy:** `checkout_siblings`"));
    assert_main_worktree_guardrails(&text);
    assert!(
        text.contains("Use the full change ID as the branch and primary worktree directory name")
    );
    assert!(text.contains("Do not reuse one worktree for two changes"));
    assert!(text.contains(
        "git worktree add \"../<project-name>-wt/<full-change-id>\" -b <full-change-id> develop"
    ));
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "agents_md_checkout_siblings");
    assert!(
        text.contains("../<project-name>-wt/<full-change-id>/"),
        "should show repo-relative sibling worktree path"
    );
    assert_no_absolute_project_root(&text, &ctx.project_root, "agents_md_checkout_siblings");
}

/// Integration test that verifies AGENTS.md rendering for the `bare_control_siblings` worktree configuration.
///
/// This test renders the AGENTS.md template with a `bare_control_siblings` context and asserts the output
/// includes the configured strategy, the expected repository-relative layout snippets (including a `.bare/`
/// and `ito-worktrees/` appearance), contains no raw template syntax, and does not embed the machine-specific
/// absolute project root or vague discovery heuristics.
///
/// # Examples
///
/// ```
/// let ctx = bare_control_siblings_ctx();
/// let text = render_text(agents_md_bytes(), &ctx);
/// assert!(text.contains("**Strategy:** `bare_control_siblings`"));
/// ```
#[test]
fn agents_md_bare_control_siblings() {
    let ctx = bare_control_siblings_ctx();
    let text = render_text(agents_md_bytes(), &ctx);

    assert!(text.contains("**Strategy:** `bare_control_siblings`"));
    assert_main_worktree_guardrails(&text);
    assert!(
        text.contains("Use the full change ID as the branch and primary worktree directory name")
    );
    assert!(text.contains("Do not reuse one worktree for two changes"));
    assert!(text.contains(".bare/"));
    assert!(text.contains("ito-worktrees/"));
    assert!(text.contains(
        "git worktree add \"../ito-worktrees/<full-change-id>\" -b <full-change-id> main"
    ));
    assert!(text.contains("Do not create them from the bare/control repo placeholder `HEAD`"));
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "agents_md_bare_control_siblings");
    let layout_line = text
        .lines()
        .find(|l| l.contains("# bare/control repo"))
        .expect("should contain bare/control repo layout line");
    assert!(
        layout_line.contains("../"),
        "should show repo-relative bare/control layout"
    );
    assert_no_absolute_project_root(&text, &ctx.project_root, "agents_md_bare_control_siblings");
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
    assert!(
        text.contains(
            "git worktree add \".ito-worktrees/<full-change-id>\" -b <full-change-id> main"
        )
    );
    assert_main_worktree_guardrails(&text);
    assert!(
        text.contains("Use the full change ID as the branch and primary worktree directory name")
    );
    assert!(text.contains("Do not reuse one worktree for two changes"));
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "skill_checkout_subdir");
    assert_no_absolute_project_root(&text, &ctx.project_root, "skill_checkout_subdir");
}

#[test]
fn skill_checkout_siblings() {
    let ctx = checkout_siblings_ctx();
    let text = render_text(skill_md_bytes(), &ctx);

    assert!(text.contains("**Configured strategy:** `checkout_siblings`"));
    assert_main_worktree_guardrails(&text);
    assert!(
        text.contains("Use the full change ID as the branch and primary worktree directory name")
    );
    assert!(text.contains("Do not reuse one worktree for two changes"));
    assert!(text.contains(
        "git worktree add \"../<project-name>-wt/<full-change-id>\" -b <full-change-id> develop"
    ));
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "skill_checkout_siblings");
    assert_no_absolute_project_root(&text, &ctx.project_root, "skill_checkout_siblings");
}

/// Integration test that renders the worktree skill template using the
/// `bare_control_siblings` configuration and asserts the rendered output
/// contains the expected fragments and omits machine-specific or discovery
/// heuristics.
///
/// # Examples
///
/// ```
/// let ctx = bare_control_siblings_ctx();
/// let text = render_text(skill_md_bytes(), &ctx);
/// assert!(text.contains("**Configured strategy:** `bare_control_siblings`"));
/// assert!(text.contains("ito-worktrees/"));
/// assert!(!text.contains("{{"));
/// assert_no_discovery_heuristics(&text, "skill_bare_control_siblings");
/// assert_no_absolute_project_root(&text, &ctx.project_root, "skill_bare_control_siblings");
/// ```
#[test]
fn skill_bare_control_siblings() {
    let ctx = bare_control_siblings_ctx();
    let text = render_text(skill_md_bytes(), &ctx);

    assert!(text.contains("**Configured strategy:** `bare_control_siblings`"));
    assert_main_worktree_guardrails(&text);
    assert!(
        text.contains("Use the full change ID as the branch and primary worktree directory name")
    );
    assert!(text.contains("Do not reuse one worktree for two changes"));
    assert!(text.contains("ito-worktrees/"));
    assert!(text.contains(
        "git worktree add \"../ito-worktrees/<full-change-id>\" -b <full-change-id> main"
    ));
    assert!(
        text.contains("Never use the bare/control repo placeholder `HEAD` as the checkout source")
    );
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "skill_bare_control_siblings");
    assert_no_absolute_project_root(&text, &ctx.project_root, "skill_bare_control_siblings");
}

#[test]
fn skill_disabled() {
    let text = render_text(skill_md_bytes(), &disabled_ctx());

    assert!(text.contains("Worktrees are not configured for this project."));
    assert!(text.contains("Work in the current checkout."));
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "skill_disabled");
}
