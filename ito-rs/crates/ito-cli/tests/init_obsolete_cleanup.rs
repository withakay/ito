#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;
use std::collections::BTreeMap;
use std::path::Path;

use crate::fixtures::{installed_specialist_asset_paths, obsolete_specialist_asset_paths};

const COORDINATOR_PATHS: &[&str] = &[
    ".opencode/agents/ito-orchestrator.md",
    ".claude/agents/ito-orchestrator.md",
    ".github/agents/ito-orchestrator.md",
    ".pi/agents/ito-orchestrator.md",
];

#[test]
fn init_update_with_tools_all_removes_obsolete_specialist_orchestrator_assets() {
    assert_specialist_cleanup(&["--update"]);
}

#[test]
fn init_force_with_tools_all_removes_obsolete_specialist_orchestrator_assets() {
    assert_specialist_cleanup(&["--force"]);
}

#[test]
fn init_update_removes_obsolete_tmux_skills_from_every_harness() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    fixtures::reset_repo(repo.path(), base.path());

    let roots = [
        ".claude/skills",
        ".opencode/skills",
        ".codex/skills",
        ".github/skills",
        ".pi/skills",
    ];
    for root in roots {
        fixtures::write(
            repo.path().join(root).join("ito-tmux/SKILL.md"),
            "obsolete Ito-managed tmux skill\n",
        );
        fixtures::write(
            repo.path()
                .join(root)
                .join("ito-tmux/scripts/wait-for-text.sh"),
            "#!/bin/sh\n",
        );
    }
    fixtures::write(repo.path().join("user-tmux.conf"), "set -g mouse on\n");

    let out = run_rust_candidate(
        rust_path,
        &[
            "init",
            repo.path().to_string_lossy().as_ref(),
            "--tools",
            "all",
            "--update",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    for root in roots {
        assert!(
            !repo.path().join(root).join("ito-tmux").exists(),
            "obsolete tmux skill should be removed from {root}"
        );
    }
    assert!(
        repo.path().join("user-tmux.conf").exists(),
        "user-owned tmux files outside Ito-managed paths must remain"
    );
}

#[test]
fn update_command_removes_obsolete_tmux_skill() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    fixtures::reset_repo(repo.path(), base.path());

    let obsolete = repo.path().join(".codex/skills/ito-tmux/SKILL.md");
    fixtures::write(&obsolete, "obsolete Ito-managed tmux skill\n");

    let out = run_rust_candidate(rust_path, &["update", "."], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(
        !obsolete.parent().unwrap().exists(),
        "ito update should prune the obsolete managed skill directory"
    );
}

#[test]
fn init_update_prunes_retired_lifecycle_surfaces_safely() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    for rel in retired_surface_paths() {
        let contents = if rel.ends_with("ito-project-setup.md") {
            generated_markdown("ito-project-setup")
        } else if rel.contains("/commands/") || rel.contains("/prompts/") {
            generated_plan_command()
        } else {
            generated_markdown("ito-plan")
        };
        fixtures::write(repo.path().join(rel), &contents);
    }

    let preserved = ".codex/skills/ito-memory/SKILL.md";
    fixtures::write(
        repo.path().join(preserved),
        &format!("user preface\n{}", generated_markdown("ito-memory")),
    );
    let unrelated = ".codex/skills/my-project-skill/SKILL.md";
    fixtures::write(repo.path().join(unrelated), "user skill\n");

    let repo_path = repo.path().to_string_lossy();
    let argv = ["init", repo_path.as_ref(), "--tools", "all", "--update"];
    let first = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
    assert_eq!(first.code, 0, "stderr={}", first.stderr);

    for rel in retired_surface_paths() {
        assert!(
            !repo.path().join(&rel).exists(),
            "expected retired surface {rel} to be pruned"
        );
    }
    assert!(repo.path().join(preserved).exists());
    assert!(repo.path().join(unrelated).exists());
    assert!(first.stderr.contains("preserving retired Ito surface"));
    assert!(
        first
            .stderr
            .contains("replacement: ito-research + ito-archive")
    );

    let before = harness_tree(repo.path());
    let second = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
    assert_eq!(second.code, 0, "stderr={}", second.stderr);
    let after = harness_tree(repo.path());
    let changed = before
        .keys()
        .chain(after.keys())
        .filter(|path| before.get(*path) != after.get(*path))
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    assert!(changed.is_empty(), "second update changed: {changed:?}");
}

fn assert_specialist_cleanup(extra_args: &[&str]) {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let obsolete = obsolete_specialist_asset_paths();
    for rel in &obsolete {
        let name = Path::new(rel)
            .parent()
            .and_then(Path::file_name)
            .and_then(|name| name.to_str())
            .unwrap_or("ito-obsolete-agent");
        fixtures::write(repo.path().join(rel), &generated_markdown(name));
    }

    let repo_path = repo.path().to_string_lossy();
    let mut argv = vec!["init", repo_path.as_ref(), "--tools", "all"];
    argv.extend_from_slice(extra_args);
    let out = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    for rel in &obsolete {
        assert!(
            !repo.path().join(rel).exists(),
            "expected obsolete specialist asset {rel} to be removed"
        );
    }

    for rel in installed_specialist_asset_paths() {
        assert!(repo.path().join(&rel).exists(), "expected {rel} to install");
    }

    for rel in COORDINATOR_PATHS {
        assert!(
            repo.path().join(rel).exists(),
            "expected coordinator asset {rel} to remain installed"
        );
    }
}

fn generated_markdown(name: &str) -> String {
    if name == "ito-project-setup" {
        return "<!-- ITO:START -->\nmanaged\n<!-- ITO:END -->\n".to_string();
    }
    let prefix = match name {
        "ito-plan" => {
            "---\nname: ito-plan\ndescription: Explore rough ideas before proposal scaffolding, including DDD discovery for ambiguous, architectural, or cross-context work.\n---"
        }
        "ito-memory" => {
            "---\nname: ito-memory\ndescription: Use Ito's configured memory provider to capture, search, and query project knowledge. Activate when users ask to remember, recall, search memory, query memory, save learnings, or use Ito memory. Provider-agnostic: routes through `ito agent instruction memory-capture`, `memory-search`, and `memory-query` rather than calling ByteRover or another backend directly.\n---"
        }
        "ito-orchestrator-planner" => {
            "---\nname: ito-orchestrator-planner\ndescription: Plans Ito orchestration runs from change metadata and gates\ntools: read, grep, find, ls, bash\n---"
        }
        "ito-orchestrator-researcher" => {
            "---\nname: ito-orchestrator-researcher\ndescription: Read-only researcher for Ito orchestration context gathering\ntools: read, grep, find, ls\n---"
        }
        "ito-orchestrator-reviewer" => {
            "---\nname: ito-orchestrator-reviewer\ndescription: Reviews Ito orchestration gate results and worker changes\ntools: read, grep, find, ls, bash\n---"
        }
        "ito-orchestrator-worker" => {
            "---\nname: ito-orchestrator-worker\ndescription: Implements Ito orchestration work packets and remediation tasks\ntools: read, grep, find, ls, bash, edit, write\n---"
        }
        _ => {
            return format!(
                "---\nname: {name}\ndescription: generated\n---\n\n<!-- ITO:START -->\nmanaged\n<!-- ITO:END -->\n"
            );
        }
    };
    format!("{prefix}\n\n<!-- ITO:START -->\nmanaged\n<!-- ITO:END -->\n")
}

fn generated_plan_command() -> String {
    "---\nname: ito-plan\ndescription: Explore rough ideas before proposal scaffolding, including DDD discovery when useful.\ncategory: Ito\ntags: [ito, plan, discovery, ddd]\n---\n\n<UserRequest>\n$ARGUMENTS\n</UserRequest>\n\n<!-- ITO:START -->\nmanaged\n<!-- ITO:END -->\n"
        .to_string()
}

fn retired_surface_paths() -> Vec<String> {
    let mut paths = Vec::new();
    for root in [
        ".claude/skills",
        ".codex/skills",
        ".github/skills",
        ".opencode/skills",
        ".pi/skills",
    ] {
        paths.push(format!("{root}/ito-plan/SKILL.md"));
    }
    paths.extend([
        ".claude/commands/ito-plan.md".to_string(),
        ".codex/prompts/ito-plan.md".to_string(),
        ".github/prompts/ito-plan.prompt.md".to_string(),
        ".opencode/commands/ito-plan.md".to_string(),
        ".pi/commands/ito-plan.md".to_string(),
        ".claude/commands/ito-project-setup.md".to_string(),
        ".codex/commands/ito-project-setup.md".to_string(),
        ".opencode/commands/ito-project-setup.md".to_string(),
        ".pi/commands/ito-project-setup.md".to_string(),
    ]);
    paths
}

fn harness_tree(root: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut snapshot = BTreeMap::new();
    for harness_root in [
        ".claude",
        ".codex",
        ".github",
        ".opencode",
        ".pi",
        ".agents",
    ] {
        collect_tree(root, &root.join(harness_root), &mut snapshot);
    }
    snapshot
}

fn collect_tree(root: &Path, directory: &Path, snapshot: &mut BTreeMap<String, Vec<u8>>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        let Ok(metadata) = std::fs::symlink_metadata(&path) else {
            continue;
        };
        if metadata.is_dir() {
            collect_tree(root, &path, snapshot);
        } else if metadata.is_file()
            && let Ok(contents) = std::fs::read(&path)
            && let Ok(relative) = path.strip_prefix(root)
        {
            snapshot.insert(relative.to_string_lossy().into_owned(), contents);
        }
    }
}
