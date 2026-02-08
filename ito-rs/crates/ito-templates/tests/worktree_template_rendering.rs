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

    assert!(text.contains("checkout_subdir"), "should name the strategy");
    assert!(
        text.contains(".ito-worktrees/"),
        "should contain the gitignored subdir path"
    );
    assert!(
        text.contains("git worktree add"),
        "should contain git worktree add command"
    );
    assert!(
        text.contains("Do NOT ask the user"),
        "should tell agents not to ask"
    );
    assert!(
        !text.contains("not configured"),
        "should not say worktrees are disabled"
    );
    assert_no_discovery_heuristics(&text, "agents_md_checkout_subdir");
}

#[test]
fn agents_md_checkout_siblings() {
    let text = render_text(agents_md_bytes(), &checkout_siblings_ctx());

    assert!(
        text.contains("checkout_siblings"),
        "should name the strategy"
    );
    assert!(
        text.contains("<project>-wt/"),
        "should use custom dir name 'wt' in sibling pattern"
    );
    assert!(
        text.contains("git worktree add"),
        "should contain git worktree add command"
    );
    assert!(
        text.contains("develop"),
        "should reference the configured default branch"
    );
    assert!(
        text.contains("merge the branch into `develop`"),
        "should describe merge_parent integration"
    );
    assert_no_discovery_heuristics(&text, "agents_md_checkout_siblings");
}

#[test]
fn agents_md_bare_control_siblings() {
    let text = render_text(agents_md_bytes(), &bare_control_siblings_ctx());

    assert!(
        text.contains("bare_control_siblings"),
        "should name the strategy"
    );
    assert!(text.contains(".bare/"), "should describe bare repo layout");
    assert!(
        text.contains("ito-worktrees/"),
        "should reference the worktree directory"
    );
    assert!(
        text.contains("git worktree add"),
        "should contain git worktree add command"
    );
    assert!(
        text.contains("pull request"),
        "should describe commit_pr integration"
    );
    assert_no_discovery_heuristics(&text, "agents_md_bare_control_siblings");
}

#[test]
fn agents_md_disabled() {
    let text = render_text(agents_md_bytes(), &disabled_ctx());

    assert!(
        text.contains("not configured"),
        "should say worktrees are not configured"
    );
    assert!(
        text.contains("Do NOT create git worktrees"),
        "should instruct agents not to create worktrees"
    );
    assert!(
        !text.contains("git worktree add"),
        "should not contain git worktree add command"
    );
    assert!(
        !text.contains("**Strategy:**"),
        "should not display a strategy header"
    );
    assert!(
        text.contains("ito config set"),
        "should show how to enable worktrees"
    );
    assert_no_discovery_heuristics(&text, "agents_md_disabled");
}

// ===========================================================================
// Worktree skill tests
// ===========================================================================

#[test]
fn skill_checkout_subdir() {
    let text = render_text(skill_md_bytes(), &checkout_subdir_ctx());

    assert!(text.contains("checkout_subdir"), "should name the strategy");
    assert!(
        text.contains(".ito-worktrees/"),
        "should contain the gitignored subdir path"
    );
    assert!(
        text.contains("git worktree add"),
        "should contain git worktree add command"
    );
    assert!(
        text.contains("Do NOT"),
        "should tell agents not to use other locations"
    );
    assert!(
        text.contains("git worktree remove"),
        "should include cleanup instructions"
    );
    assert!(
        !text.contains("not configured"),
        "should not say worktrees are disabled"
    );
    assert_no_discovery_heuristics(&text, "skill_checkout_subdir");
}

#[test]
fn skill_checkout_siblings() {
    let text = render_text(skill_md_bytes(), &checkout_siblings_ctx());

    assert!(
        text.contains("checkout_siblings"),
        "should name the strategy"
    );
    assert!(
        text.contains("<project>-wt/"),
        "should use custom dir name 'wt' in sibling pattern"
    );
    assert!(
        text.contains("git worktree add"),
        "should contain git worktree add command"
    );
    assert!(
        text.contains("develop"),
        "should reference the configured default branch"
    );
    assert_no_discovery_heuristics(&text, "skill_checkout_siblings");
}

#[test]
fn skill_bare_control_siblings() {
    let text = render_text(skill_md_bytes(), &bare_control_siblings_ctx());

    assert!(
        text.contains("bare_control_siblings"),
        "should name the strategy"
    );
    assert!(text.contains(".bare/"), "should describe bare repo layout");
    assert!(
        text.contains("ito-worktrees/"),
        "should reference the worktree directory"
    );
    assert!(
        text.contains("git worktree add"),
        "should contain git worktree add command"
    );
    assert!(
        text.contains("git worktree remove"),
        "should include cleanup instructions"
    );
    assert_no_discovery_heuristics(&text, "skill_bare_control_siblings");
}

#[test]
fn skill_disabled() {
    let text = render_text(skill_md_bytes(), &disabled_ctx());

    assert!(
        text.contains("not configured"),
        "should say worktrees are not configured"
    );
    assert!(
        text.contains("Do NOT create git worktrees"),
        "should instruct agents not to create worktrees"
    );
    assert!(
        !text.contains("git worktree add"),
        "should not contain git worktree add command"
    );
    assert!(
        !text.contains("**Strategy:**"),
        "should not display a strategy header"
    );
    assert!(
        text.contains("ito config set"),
        "should show how to enable worktrees"
    );
    assert_no_discovery_heuristics(&text, "skill_disabled");
}
