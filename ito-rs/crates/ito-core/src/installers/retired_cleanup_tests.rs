use std::collections::BTreeSet;

use super::*;

fn codex_tools() -> BTreeSet<String> {
    ["codex".to_string()].into_iter().collect()
}

fn generated_skill(name: &str) -> String {
    let prefix = match name {
        "ito-plan" => {
            "---\nname: ito-plan\ndescription: Explore rough ideas before proposal scaffolding, including DDD discovery for ambiguous, architectural, or cross-context work.\n---"
        }
        "ito-memory" => {
            "---\nname: ito-memory\ndescription: Use Ito's configured memory provider to capture, search, and query project knowledge. Activate when users ask to remember, recall, search memory, query memory, save learnings, or use Ito memory. Provider-agnostic: routes through `ito agent instruction memory-capture`, `memory-search`, and `memory-query` rather than calling ByteRover or another backend directly.\n---"
        }
        _ => panic!("missing generated fixture for {name}"),
    };
    format!("{prefix}\n\n<!-- ITO:START -->\nmanaged\n<!-- ITO:END -->\n")
}

fn generated_plan_command() -> String {
    "---\nname: ito-plan\ndescription: Explore rough ideas before proposal scaffolding, including DDD discovery when useful.\ncategory: Ito\ntags: [ito, plan, discovery, ddd]\n---\n\n<UserRequest>\n$ARGUMENTS\n</UserRequest>\n\n<!-- ITO:START -->\nmanaged\n<!-- ITO:END -->\n".to_string()
}

fn generated_from_prefix(prefix: &str) -> String {
    format!("{prefix}\n\n<!-- ITO:START -->\nmanaged\n<!-- ITO:END -->\n")
}

#[test]
fn retired_cleanup_removes_managed_skill_and_preserves_unrelated_skill() {
    let root = tempfile::tempdir().expect("root");
    let retired = root.path().join(".codex/skills/ito-plan/SKILL.md");
    let unrelated = root.path().join(".codex/skills/my-skill/SKILL.md");
    std::fs::create_dir_all(retired.parent().expect("parent")).expect("mkdir");
    std::fs::create_dir_all(unrelated.parent().expect("parent")).expect("mkdir");
    std::fs::write(&retired, generated_skill("ito-plan")).expect("retired");
    std::fs::write(&unrelated, "user skill\n").expect("user");

    let report = cleanup_retired_surfaces(root.path(), &codex_tools()).expect("cleanup");
    assert!(!retired.exists());
    assert!(unrelated.exists());
    assert_eq!(
        report.removed,
        [RetiredSurfaceReportEntry {
            path: retired,
            replacement: Some("ito-proposal"),
        }]
    );
    assert!(report.preserved.is_empty());

    let second = cleanup_retired_surfaces(root.path(), &codex_tools()).expect("second cleanup");
    assert_eq!(second, RetiredCleanupReport::default());
}

#[test]
fn retired_cleanup_preserves_markdown_with_user_content() {
    let root = tempfile::tempdir().expect("root");
    let retired = root.path().join(".codex/skills/ito-plan/SKILL.md");
    std::fs::create_dir_all(retired.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &retired,
        format!("user preface\n{}", generated_skill("ito-plan")),
    )
    .expect("retired");

    let report = cleanup_retired_surfaces(root.path(), &codex_tools()).expect("cleanup");
    assert!(retired.exists());
    assert!(report.removed.is_empty());
    assert_eq!(
        report.preserved,
        [RetiredSurfaceReportEntry {
            path: retired,
            replacement: Some("ito-proposal"),
        }]
    );
}

#[test]
fn retired_cleanup_preserves_partial_marker_markdown() {
    let root = tempfile::tempdir().expect("root");
    let retired = root.path().join(".codex/skills/ito-plan/SKILL.md");
    std::fs::create_dir_all(retired.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &retired,
        "---\nname: ito-plan\n---\n\n<!-- ITO:START -->\npartial\n",
    )
    .expect("retired");

    let report = cleanup_retired_surfaces(root.path(), &codex_tools()).expect("cleanup");
    assert!(retired.exists());
    assert_eq!(report.preserved.len(), 1);
}

#[test]
fn retired_cleanup_removes_generated_command_shell() {
    let root = tempfile::tempdir().expect("root");
    let retired = root.path().join(".codex/prompts/ito-plan.md");
    std::fs::create_dir_all(retired.parent().expect("parent")).expect("mkdir");
    std::fs::write(&retired, generated_plan_command()).expect("command");

    let report = cleanup_retired_surfaces(root.path(), &codex_tools()).expect("cleanup");
    assert!(!retired.exists());
    assert_eq!(
        report.removed,
        [RetiredSurfaceReportEntry {
            path: retired,
            replacement: Some("ito-proposal"),
        }]
    );
}

#[test]
fn retired_cleanup_removes_assets_from_skipped_release_generations() {
    let root = tempfile::tempdir().expect("root");
    let old_apply = root
        .path()
        .join(".codex/skills/ito-apply-change-proposal/SKILL.md");
    let old_tmux = root.path().join(".codex/skills/tmux/SKILL.md");
    let old_loop = root.path().join(".codex/prompts/loop.md");
    for path in [&old_apply, &old_tmux, &old_loop] {
        std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    }
    std::fs::write(
        &old_apply,
        generated_from_prefix(
            "---\nname: ito-apply-change-proposal\ndescription: Use when implementing, executing, applying, building, coding, or developing a feature, change, requirement, enhancement, fix, or modification. Use when running tasks from a spec, proposal, or plan.\n---\n\nRun the CLI-generated apply instructions for a specific change.\n\n**Steps**\n\n1. Determine the target change ID.\n\n   - If the user provides one, use it.\n   - Otherwise run `ito list --ready` to see changes ready for implementation.\n   - Ask the user which change to apply if multiple are ready.\n\n2. Generate instructions (source of truth):\n   ```bash\n   ito agent instruction apply --change \"<change-id>\"\n   ```\n\n3. Follow the printed instructions exactly.\n\n4. Use `ito tasks ready <change-id>` to see actionable tasks at any point.",
        ),
    )
    .expect("old apply");
    std::fs::write(
        &old_tmux,
        generated_from_prefix(
            "---\nname: tmux\ndescription: \"Remote control tmux sessions for interactive CLIs (python, gdb, etc.) by sending keystrokes and scraping pane output.\"\nmetadata:\n  upstream: https://github.com/mitsuhiko/agent-stuff/tree/main/skills/tmux\n  license: Vibecoded\n---\n\n# tmux Skill",
        ),
    )
    .expect("old tmux");
    std::fs::write(
        &old_loop,
        generated_from_prefix(
            "---\nname: loop\ndescription: Run an Ito Ralph loop for a change.\ncategory: Ito\ntags: [ito, ralph, loop]\n---\n\n<UserRequest>\n$ARGUMENTS\n</UserRequest>",
        ),
    )
    .expect("old loop");

    let report = cleanup_retired_surfaces(root.path(), &codex_tools()).expect("cleanup");
    assert!(!old_apply.exists());
    assert!(!old_tmux.exists());
    assert!(!old_loop.exists());
    assert_eq!(report.removed.len(), 3);
    assert!(report.preserved.is_empty());
}

#[cfg(unix)]
#[test]
fn retired_cleanup_unlinks_broken_symlink_and_preserves_live_symlink() {
    use std::os::unix::fs::symlink;

    let root = tempfile::tempdir().expect("root");
    let external = tempfile::tempdir().expect("external");
    let external_file = external.path().join("keep.md");
    std::fs::write(&external_file, "keep\n").expect("external file");

    let broken = root.path().join(".codex/skills/ito-plan/SKILL.md");
    let linked = root.path().join(".codex/skills/ito-memory/SKILL.md");
    std::fs::create_dir_all(broken.parent().expect("parent")).expect("mkdir");
    std::fs::create_dir_all(linked.parent().expect("parent")).expect("mkdir");
    symlink(root.path().join("missing"), &broken).expect("broken link");
    symlink(&external_file, &linked).expect("external link");

    let report = cleanup_retired_surfaces(root.path(), &codex_tools()).expect("cleanup");
    assert!(std::fs::symlink_metadata(&broken).is_err());
    assert!(std::fs::symlink_metadata(&linked).is_ok());
    assert_eq!(std::fs::read_to_string(&external_file).unwrap(), "keep\n");
    assert_eq!(report.removed.len(), 1);
    assert_eq!(report.preserved.len(), 1);
}

#[cfg(unix)]
#[test]
fn retired_cleanup_does_not_traverse_symlinked_surface_root() {
    use std::os::unix::fs::symlink;

    let root = tempfile::tempdir().expect("root");
    let external = tempfile::tempdir().expect("external");
    let retired = external.path().join("ito-plan/SKILL.md");
    std::fs::create_dir_all(retired.parent().expect("parent")).expect("mkdir");
    std::fs::write(&retired, generated_skill("ito-plan")).expect("retired");

    std::fs::create_dir_all(root.path().join(".codex")).expect("codex");
    symlink(external.path(), root.path().join(".codex/skills")).expect("surface link");

    let report = cleanup_retired_surfaces(root.path(), &codex_tools()).expect("cleanup");
    assert!(retired.exists());
    assert!(report.removed.is_empty());
    assert!(
        report
            .preserved
            .iter()
            .any(|entry| entry.path == root.path().join(".codex/skills"))
    );
}

#[cfg(unix)]
#[test]
fn retired_cleanup_does_not_traverse_symlinked_retired_directory() {
    use std::os::unix::fs::symlink;

    let root = tempfile::tempdir().expect("root");
    let external = tempfile::tempdir().expect("external");
    let external_skill = external.path().join("SKILL.md");
    std::fs::write(&external_skill, generated_skill("ito-plan")).expect("external skill");

    let skill_root = root.path().join(".codex/skills");
    std::fs::create_dir_all(&skill_root).expect("skill root");
    symlink(external.path(), skill_root.join("ito-plan")).expect("retired directory link");

    let report = cleanup_retired_surfaces(root.path(), &codex_tools()).expect("cleanup");
    assert!(external_skill.exists());
    assert!(report.removed.is_empty());
    assert!(
        report
            .preserved
            .iter()
            .any(|entry| entry.path.ends_with("ito-plan/SKILL.md"))
    );
}

#[test]
fn retired_cleanup_preserves_customized_frontmatter_and_command_shell() {
    let root = tempfile::tempdir().expect("root");
    let skill = root.path().join(".codex/skills/ito-plan/SKILL.md");
    let command = root.path().join(".codex/prompts/ito-plan.md");
    std::fs::create_dir_all(skill.parent().expect("skill parent")).expect("skill dir");
    std::fs::create_dir_all(command.parent().expect("command parent")).expect("command dir");
    std::fs::write(
        &skill,
        generated_skill("ito-plan").replace(
            "description: Explore rough ideas",
            "description: User-customized planning guidance",
        ),
    )
    .expect("skill");
    std::fs::write(
        &command,
        generated_plan_command().replace("<UserRequest>", "<ProjectRequest>"),
    )
    .expect("command");

    let report = cleanup_retired_surfaces(root.path(), &codex_tools()).expect("cleanup");
    assert!(skill.exists());
    assert!(command.exists());
    assert_eq!(report.preserved.len(), 2);
}

#[test]
fn exact_generated_cleanup_preserves_modified_script() {
    let root = tempfile::tempdir().expect("root");
    let surface_root = root.path().join("skills");
    let script = surface_root.join("ito-tmux/scripts/find-sessions.sh");
    std::fs::create_dir_all(script.parent().expect("parent")).expect("mkdir");
    std::fs::write(&script, "user script\n").expect("script");
    let mut report = RetiredCleanupReport::default();

    cleanup_known_file(
        &script,
        &surface_root,
        None,
        KnownFileKind::ExactGenerated {
            file_sha256: "030e201a84290d24eb2bad0120b27898a89191907956fe2b603c592f60e2f4da",
        },
        &mut report,
    )
    .expect("cleanup");

    assert!(script.exists());
    assert_eq!(report.preserved.len(), 1);
}

#[test]
fn generated_prefix_fingerprints_cover_the_retired_manifests() {
    assert!(
        ito_templates::legacy::RETIRED_SKILLS
            .iter()
            .all(|retired| retired_skill_prefix_sha256(retired.name).is_some())
    );
    assert!(
        ito_templates::legacy::HISTORICAL_RETIRED_SKILLS
            .iter()
            .all(|retired| retired_skill_prefix_sha256(retired.name).is_some())
    );
    assert!(
        ito_templates::legacy::RETIRED_COMMANDS
            .iter()
            .all(|retired| retired_command_prefix_sha256(retired.name).is_some())
    );
}
