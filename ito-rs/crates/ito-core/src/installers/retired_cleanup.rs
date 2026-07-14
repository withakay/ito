use std::collections::BTreeSet;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::errors::{CoreError, CoreResult};

use super::{TOOL_CLAUDE, TOOL_CODEX, TOOL_GITHUB_COPILOT, TOOL_OPENCODE, TOOL_PI};

#[path = "retired_fingerprints.rs"]
mod retired_fingerprints;

use retired_fingerprints::{
    EMPTY_SHA256, retired_codex_role_prefix_sha256, retired_command_prefix_sha256,
    retired_skill_prefix_sha256,
};

#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct RetiredCleanupReport {
    pub(super) removed: Vec<RetiredSurfaceReportEntry>,
    pub(super) preserved: Vec<RetiredSurfaceReportEntry>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct RetiredSurfaceReportEntry {
    pub(super) path: PathBuf,
    pub(super) replacement: Option<&'static str>,
}

pub(super) fn cleanup_retired_surfaces(
    project_root: &Path,
    tools: &BTreeSet<String>,
) -> CoreResult<RetiredCleanupReport> {
    let mut report = RetiredCleanupReport::default();

    for harness in HARNESS_SURFACES {
        if !tools.contains(harness.tool) {
            continue;
        }

        let skill_root = project_root.join(harness.skill_root);
        if has_symlink_component(project_root, &skill_root)? {
            report.preserved.push(RetiredSurfaceReportEntry {
                path: skill_root.clone(),
                replacement: Some("manual lifecycle-skill migration required"),
            });
        } else {
            for retired in ito_templates::legacy::RETIRED_SKILLS
                .iter()
                .chain(ito_templates::legacy::HISTORICAL_RETIRED_SKILLS)
            {
                cleanup_known_file(
                    &skill_root.join(retired.name).join("SKILL.md"),
                    &skill_root,
                    retired.replacement,
                    KnownFileKind::ManagedMarkdown {
                        generated_prefix_sha256: retired_skill_prefix_sha256(retired.name),
                    },
                    &mut report,
                )?;
            }
            for resource in RETIRED_SKILL_RESOURCES {
                cleanup_known_file(
                    &skill_root.join(resource.relative_path),
                    &skill_root,
                    resource.replacement,
                    resource.kind,
                    &mut report,
                )?;
            }
        }

        let command_root = project_root.join(harness.command_root);
        if has_symlink_component(project_root, &command_root)? {
            report.preserved.push(RetiredSurfaceReportEntry {
                path: command_root.clone(),
                replacement: Some("manual lifecycle-command migration required"),
            });
        } else {
            for retired in ito_templates::legacy::RETIRED_COMMANDS {
                let file_name = format!("{}{}", retired.name, harness.command_suffix);
                cleanup_known_file(
                    &command_root.join(file_name),
                    &command_root,
                    retired.replacement,
                    KnownFileKind::ManagedMarkdown {
                        generated_prefix_sha256: retired_command_prefix_sha256(retired.name),
                    },
                    &mut report,
                )?;
            }
        }
    }

    if tools.contains(TOOL_CODEX) {
        let legacy_command_root = project_root.join(".codex/commands");
        if has_symlink_component(project_root, &legacy_command_root)? {
            report.preserved.push(RetiredSurfaceReportEntry {
                path: legacy_command_root.clone(),
                replacement: Some("manual lifecycle-command migration required"),
            });
        } else {
            cleanup_known_file(
                &legacy_command_root.join("ito-project-setup.md"),
                &legacy_command_root,
                Some("ito"),
                KnownFileKind::ManagedMarkdown {
                    generated_prefix_sha256: Some(EMPTY_SHA256),
                },
                &mut report,
            )?;
        }

        let agent_root = project_root.join(".agents/skills");
        if has_symlink_component(project_root, &agent_root)? {
            report.preserved.push(RetiredSurfaceReportEntry {
                path: agent_root.clone(),
                replacement: Some("manual Codex role-skill migration required"),
            });
        } else {
            for role in RETIRED_CODEX_ROLE_SKILLS {
                cleanup_known_file(
                    &agent_root.join(role).join("SKILL.md"),
                    &agent_root,
                    Some("harness-native agent or ordinary delegation"),
                    KnownFileKind::ManagedMarkdown {
                        generated_prefix_sha256: retired_codex_role_prefix_sha256(role),
                    },
                    &mut report,
                )?;
            }
        }
    }

    report.removed.sort();
    report.preserved.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(report)
}

#[derive(Debug, Clone, Copy)]
struct HarnessSurface {
    tool: &'static str,
    skill_root: &'static str,
    command_root: &'static str,
    command_suffix: &'static str,
}

const HARNESS_SURFACES: &[HarnessSurface] = &[
    HarnessSurface {
        tool: TOOL_CLAUDE,
        skill_root: ".claude/skills",
        command_root: ".claude/commands",
        command_suffix: ".md",
    },
    HarnessSurface {
        tool: TOOL_CODEX,
        skill_root: ".codex/skills",
        command_root: ".codex/prompts",
        command_suffix: ".md",
    },
    HarnessSurface {
        tool: TOOL_GITHUB_COPILOT,
        skill_root: ".github/skills",
        command_root: ".github/prompts",
        command_suffix: ".prompt.md",
    },
    HarnessSurface {
        tool: TOOL_OPENCODE,
        skill_root: ".opencode/skills",
        command_root: ".opencode/commands",
        command_suffix: ".md",
    },
    HarnessSurface {
        tool: TOOL_PI,
        skill_root: ".pi/skills",
        command_root: ".pi/commands",
        command_suffix: ".md",
    },
];

#[derive(Debug, Clone, Copy)]
enum KnownFileKind {
    ManagedMarkdown {
        generated_prefix_sha256: Option<&'static str>,
    },
    ExactGenerated {
        file_sha256: &'static str,
    },
}

#[derive(Debug, Clone, Copy)]
struct RetiredResource {
    relative_path: &'static str,
    replacement: Option<&'static str>,
    kind: KnownFileKind,
}

const RETIRED_SKILL_RESOURCES: &[RetiredResource] = &[
    RetiredResource {
        relative_path: "ito-subagent-driven-development/implementer-prompt.md",
        replacement: Some("ito-apply"),
        kind: KnownFileKind::ManagedMarkdown {
            generated_prefix_sha256: Some(EMPTY_SHA256),
        },
    },
    RetiredResource {
        relative_path: "ito-subagent-driven-development/spec-reviewer-prompt.md",
        replacement: Some("ito-review"),
        kind: KnownFileKind::ManagedMarkdown {
            generated_prefix_sha256: Some(EMPTY_SHA256),
        },
    },
    RetiredResource {
        relative_path: "ito-subagent-driven-development/code-quality-reviewer-prompt.md",
        replacement: Some("ito-review"),
        kind: KnownFileKind::ManagedMarkdown {
            generated_prefix_sha256: Some(EMPTY_SHA256),
        },
    },
    RetiredResource {
        relative_path: "ito-tmux/scripts/find-sessions.sh",
        replacement: None,
        kind: KnownFileKind::ExactGenerated {
            file_sha256: "3628e421046f71781a8413365ee3782f78caf9b348fb25c2a9707e2c8029e5be",
        },
    },
    RetiredResource {
        relative_path: "ito-tmux/scripts/wait-for-text.sh",
        replacement: None,
        kind: KnownFileKind::ExactGenerated {
            file_sha256: "54ee13cb842a043eaf12129f40959e5a720a08ab96ff0f2f12df44e029aa4725",
        },
    },
    RetiredResource {
        relative_path: "tmux/scripts/find-sessions.sh",
        replacement: None,
        kind: KnownFileKind::ExactGenerated {
            file_sha256: "3628e421046f71781a8413365ee3782f78caf9b348fb25c2a9707e2c8029e5be",
        },
    },
    RetiredResource {
        relative_path: "tmux/scripts/wait-for-text.sh",
        replacement: None,
        kind: KnownFileKind::ExactGenerated {
            file_sha256: "54ee13cb842a043eaf12129f40959e5a720a08ab96ff0f2f12df44e029aa4725",
        },
    },
];

const RETIRED_CODEX_ROLE_SKILLS: &[&str] = &[
    "ito-general",
    "ito-thinking",
    "ito-orchestrator",
    "ito-quick",
    "ito-planner",
    "ito-researcher",
    "ito-reviewer",
    "ito-worker",
    "ito-test-runner",
    "ito-orchestrator-planner",
    "ito-orchestrator-researcher",
    "ito-orchestrator-reviewer",
    "ito-orchestrator-worker",
];

fn cleanup_known_file(
    path: &Path,
    surface_root: &Path,
    replacement: Option<&'static str>,
    kind: KnownFileKind,
    report: &mut RetiredCleanupReport,
) -> CoreResult<()> {
    let target_parent = path.parent().unwrap_or(surface_root);
    if has_symlink_component(surface_root, target_parent)? {
        report.preserved.push(RetiredSurfaceReportEntry {
            path: path.to_path_buf(),
            replacement,
        });
        return Ok(());
    }

    let metadata = match std::fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(CoreError::io(format!("reading {}", path.display()), error)),
    };

    if metadata.file_type().is_symlink() {
        match std::fs::metadata(path) {
            Err(error) if error.kind() == ErrorKind::NotFound => {
                remove_file(path, surface_root, replacement, report)?;
            }
            Ok(_) => report.preserved.push(RetiredSurfaceReportEntry {
                path: path.to_path_buf(),
                replacement,
            }),
            Err(error) => {
                return Err(CoreError::io(
                    format!("resolving symlink {}", path.display()),
                    error,
                ));
            }
        }
        return Ok(());
    }
    if !metadata.is_file() {
        report.preserved.push(RetiredSurfaceReportEntry {
            path: path.to_path_buf(),
            replacement,
        });
        return Ok(());
    }

    let generated = match kind {
        KnownFileKind::ManagedMarkdown {
            generated_prefix_sha256,
        } => {
            let contents = std::fs::read_to_string(path)
                .map_err(|error| CoreError::io(format!("reading {}", path.display()), error))?;
            generated_prefix_sha256
                .is_some_and(|expected| has_generated_markdown_shell(&contents, expected))
        }
        KnownFileKind::ExactGenerated { file_sha256 } => {
            let contents = std::fs::read(path)
                .map_err(|error| CoreError::io(format!("reading {}", path.display()), error))?;
            sha256_hex(&contents) == file_sha256
        }
    };
    if !generated {
        report.preserved.push(RetiredSurfaceReportEntry {
            path: path.to_path_buf(),
            replacement,
        });
        return Ok(());
    }

    remove_file(path, surface_root, replacement, report)
}

fn has_symlink_component(project_root: &Path, surface_root: &Path) -> CoreResult<bool> {
    let Ok(relative) = surface_root.strip_prefix(project_root) else {
        return Ok(true);
    };
    let mut current = project_root.to_path_buf();
    for component in relative.components() {
        current.push(component);
        match std::fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() => return Ok(true),
            Ok(_) => {}
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(false),
            Err(error) => {
                return Err(CoreError::io(
                    format!("reading {}", current.display()),
                    error,
                ));
            }
        }
    }
    Ok(false)
}

fn remove_file(
    path: &Path,
    surface_root: &Path,
    replacement: Option<&'static str>,
    report: &mut RetiredCleanupReport,
) -> CoreResult<()> {
    std::fs::remove_file(path)
        .map_err(|error| CoreError::io(format!("removing {}", path.display()), error))?;
    report.removed.push(RetiredSurfaceReportEntry {
        path: path.to_path_buf(),
        replacement,
    });
    prune_empty_parents(path.parent(), surface_root)
}

fn prune_empty_parents(start: Option<&Path>, surface_root: &Path) -> CoreResult<()> {
    let mut current = start.map(Path::to_path_buf);
    while let Some(directory) = current {
        if directory == surface_root || !directory.starts_with(surface_root) {
            break;
        }
        let mut entries = std::fs::read_dir(&directory)
            .map_err(|error| CoreError::io(format!("reading {}", directory.display()), error))?;
        if entries
            .next()
            .transpose()
            .map_err(|error| CoreError::io(format!("reading {}", directory.display()), error))?
            .is_some()
        {
            break;
        }
        std::fs::remove_dir(&directory)
            .map_err(|error| CoreError::io(format!("removing {}", directory.display()), error))?;
        current = directory.parent().map(Path::to_path_buf);
    }
    Ok(())
}

fn has_generated_markdown_shell(contents: &str, expected_prefix_sha256: &str) -> bool {
    let normalized = contents.replace("\r\n", "\n");
    let Some((prefix, after_start)) = normalized.split_once(ito_templates::ITO_START_MARKER) else {
        return false;
    };
    let Some((_, suffix)) = after_start.split_once(ito_templates::ITO_END_MARKER) else {
        return false;
    };
    if after_start.matches(ito_templates::ITO_END_MARKER).count() != 1
        || normalized.matches(ito_templates::ITO_START_MARKER).count() != 1
        || !suffix.trim().is_empty()
    {
        return false;
    }

    sha256_hex(prefix.trim().as_bytes()) == expected_prefix_sha256
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[cfg(test)]
#[path = "retired_cleanup_tests.rs"]
mod retired_cleanup_tests;
