use std::collections::{BTreeMap, BTreeSet};

use crate::agents::{AgentActivationMode, agent_surface_inventory};
use crate::{AGENTS_DIR, commands_files, default_project_files, skills_files};

#[test]
fn agent_templates_declare_activation_contract() {
    let inventory: BTreeMap<&str, AgentActivationMode> = agent_surface_inventory()
        .iter()
        .map(|surface| (surface.name, surface.activation))
        .collect();

    for harness_dir in AGENTS_DIR.dirs() {
        let Some(harness) = harness_dir.path().file_name().and_then(|s| s.to_str()) else {
            continue;
        };

        for entry_file in harness_dir.files() {
            let Some(name) = agent_name_from_asset_path(entry_file.path()) else {
                continue;
            };
            let activation = inventory
                .get(name)
                .copied()
                .expect("agent should be classified");
            let text = std::str::from_utf8(entry_file.contents()).expect("agent utf8");
            assert_agent_activation_contract(harness, name, activation, text);
        }

        for nested in harness_dir.dirs() {
            let Some(name) = nested.path().file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            let activation = inventory
                .get(name)
                .copied()
                .expect("agent should be classified");
            let Some(skill) = nested.get_file("SKILL.md") else {
                continue;
            };
            let text = std::str::from_utf8(skill.contents()).expect("agent skill utf8");
            assert_agent_activation_contract(harness, name, activation, text);
        }
    }
}

fn agent_name_from_asset_path(path: &std::path::Path) -> Option<&str> {
    let name = path.file_name()?.to_str()?;
    Some(
        name.strip_suffix(".md")
            .or_else(|| name.strip_suffix(".md.j2"))
            .unwrap_or(name),
    )
}

fn assert_agent_activation_contract(
    harness: &str,
    name: &str,
    activation: AgentActivationMode,
    text: &str,
) {
    match activation {
        AgentActivationMode::DirectEntryPoint => {
            assert!(
                text.contains("activation: direct"),
                "expected {harness}/{name} to declare direct activation"
            );
            assert!(
                !text.contains("mode: subagent"),
                "direct entrypoint {harness}/{name} must not be marked as subagent"
            );
        }
        AgentActivationMode::DelegatedRole => {
            assert!(
                text.contains("activation: delegated"),
                "expected {harness}/{name} to declare delegated activation"
            );
            if harness == "opencode" {
                assert!(
                    text.contains("mode: subagent"),
                    "expected OpenCode delegated agent {name} to remain subagent-only"
                );
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum WorkflowSurfaceClass {
    WorkflowAdapter,
    ProjectGuidance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct WorkflowSurface {
    path: &'static str,
    class: WorkflowSurfaceClass,
}

const WORKFLOW_SURFACE_INVENTORY: &[WorkflowSurface] = &[
    WorkflowSurface {
        path: "commands/ito-orchestrate.md",
        class: WorkflowSurfaceClass::WorkflowAdapter,
    },
    WorkflowSurface {
        path: "skills/ito-memory/SKILL.md",
        class: WorkflowSurfaceClass::WorkflowAdapter,
    },
    WorkflowSurface {
        path: "skills/ito-orchestrate/SKILL.md",
        class: WorkflowSurfaceClass::WorkflowAdapter,
    },
    WorkflowSurface {
        path: "skills/ito-orchestrate-setup/SKILL.md",
        class: WorkflowSurfaceClass::WorkflowAdapter,
    },
    WorkflowSurface {
        path: "skills/ito-orchestrator-workflow/SKILL.md",
        class: WorkflowSurfaceClass::ProjectGuidance,
    },
    WorkflowSurface {
        path: "skills/ito-subagent-driven-development/SKILL.md",
        class: WorkflowSurfaceClass::WorkflowAdapter,
    },
    WorkflowSurface {
        path: "skills/ito-test-with-subagent/SKILL.md",
        class: WorkflowSurfaceClass::WorkflowAdapter,
    },
    WorkflowSurface {
        path: "default-project/.ito/user-prompts/orchestrate.md",
        class: WorkflowSurfaceClass::ProjectGuidance,
    },
];

#[test]
fn orchestration_adjacent_surfaces_are_classified() {
    let inventory_paths: BTreeSet<&str> = WORKFLOW_SURFACE_INVENTORY
        .iter()
        .map(|surface| surface.path)
        .collect();
    assert_eq!(inventory_paths.len(), WORKFLOW_SURFACE_INVENTORY.len());

    let mut asset_paths: BTreeSet<String> = BTreeSet::new();

    for file in commands_files() {
        if file.relative_path == "ito-orchestrate.md" {
            asset_paths.insert(format!("commands/{}", file.relative_path));
        }
    }

    for file in skills_files() {
        let Some(skill) = file.relative_path.split('/').next() else {
            continue;
        };
        if is_orchestration_adjacent_skill(skill) && file.relative_path.ends_with("/SKILL.md") {
            asset_paths.insert(format!("skills/{}", file.relative_path));
        }
    }

    for file in default_project_files() {
        if file.relative_path == ".ito/user-prompts/orchestrate.md" {
            asset_paths.insert(format!("default-project/{}", file.relative_path));
        }
    }

    let missing_from_inventory: Vec<&str> = asset_paths
        .iter()
        .map(String::as_str)
        .filter(|path| !inventory_paths.contains(path))
        .collect();
    let missing_from_assets: Vec<&str> = inventory_paths
        .iter()
        .copied()
        .filter(|path| !asset_paths.contains(*path))
        .collect();

    assert!(
        missing_from_inventory.is_empty() && missing_from_assets.is_empty(),
        "workflow surface inventory mismatch — missing from inventory: {missing_from_inventory:?}; missing from assets: {missing_from_assets:?}"
    );
}

#[allow(clippy::match_like_matches_macro)]
fn is_orchestration_adjacent_skill(skill: &str) -> bool {
    match skill {
        "ito-memory" => true,
        "ito-orchestrate" => true,
        "ito-orchestrate-setup" => true,
        "ito-orchestrator-workflow" => true,
        "ito-subagent-driven-development" => true,
        "ito-test-with-subagent" => true,
        _ => false,
    }
}
