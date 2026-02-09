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

fn checkout_subdir_ctx() -> WorktreeTemplateContext {
    WorktreeTemplateContext {
        enabled: true,
        strategy: "checkout_subdir".to_string(),
        layout_dir_name: "ito-worktrees".to_string(),
        integration_mode: "commit_pr".to_string(),
        default_branch: "main".to_string(),
    }
}

fn checkout_siblings_ctx() -> WorktreeTemplateContext {
    WorktreeTemplateContext {
        enabled: true,
        strategy: "checkout_siblings".to_string(),
        layout_dir_name: "wt".to_string(),
        integration_mode: "merge_parent".to_string(),
        default_branch: "develop".to_string(),
    }
}

fn bare_control_siblings_ctx() -> WorktreeTemplateContext {
    WorktreeTemplateContext {
        enabled: true,
        strategy: "bare_control_siblings".to_string(),
        layout_dir_name: "ito-worktrees".to_string(),
        integration_mode: "commit_pr".to_string(),
        default_branch: "main".to_string(),
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
    let text = render_text(agents_md_bytes(), &checkout_subdir_ctx());

    assert!(
        text.contains("ito agent instruction worktrees"),
        "should delegate worktree guidance to CLI"
    );
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "agents_md_checkout_subdir");
}

#[test]
fn agents_md_checkout_siblings() {
    let text = render_text(agents_md_bytes(), &checkout_siblings_ctx());

    assert!(
        text.contains("ito agent instruction worktrees"),
        "should delegate worktree guidance to CLI"
    );
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "agents_md_checkout_siblings");
}

#[test]
fn agents_md_bare_control_siblings() {
    let text = render_text(agents_md_bytes(), &bare_control_siblings_ctx());

    assert!(
        text.contains("ito agent instruction worktrees"),
        "should delegate worktree guidance to CLI"
    );
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "agents_md_bare_control_siblings");
}

#[test]
fn agents_md_disabled() {
    let text = render_text(agents_md_bytes(), &disabled_ctx());

    assert!(
        text.contains("ito agent instruction worktrees"),
        "should delegate worktree guidance to CLI"
    );
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "agents_md_disabled");
}

// ===========================================================================
// Worktree skill tests
// ===========================================================================

#[test]
fn skill_checkout_subdir() {
    let text = render_text(skill_md_bytes(), &checkout_subdir_ctx());

    assert!(
        text.contains("ito agent instruction worktrees"),
        "should delegate worktree guidance to CLI"
    );
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "skill_checkout_subdir");
}

#[test]
fn skill_checkout_siblings() {
    let text = render_text(skill_md_bytes(), &checkout_siblings_ctx());

    assert!(
        text.contains("ito agent instruction worktrees"),
        "should delegate worktree guidance to CLI"
    );
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "skill_checkout_siblings");
}

#[test]
fn skill_bare_control_siblings() {
    let text = render_text(skill_md_bytes(), &bare_control_siblings_ctx());

    assert!(
        text.contains("ito agent instruction worktrees"),
        "should delegate worktree guidance to CLI"
    );
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "skill_bare_control_siblings");
}

#[test]
fn skill_disabled() {
    let text = render_text(skill_md_bytes(), &disabled_ctx());

    assert!(
        text.contains("ito agent instruction worktrees"),
        "should delegate worktree guidance to CLI"
    );
    assert!(!text.contains("{{"), "should not contain raw jinja");
    assert_no_discovery_heuristics(&text, "skill_disabled");
}
