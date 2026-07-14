use std::{io::ErrorKind, path::Path};

use crate::errors::{CoreError, CoreResult};

/// Legacy specialist asset paths renamed from `ito-orchestrator-*` to `ito-*`.
///
/// This migration intentionally excludes the top-level native
/// `ito-orchestrator` agent, which keeps its existing name.
const OBSOLETE_SPECIALIST_AGENT_REL_PATHS: &[&str] = &[
    "ito-orchestrator-planner.md",
    "ito-orchestrator-researcher.md",
    "ito-orchestrator-reviewer.md",
    "ito-orchestrator-worker.md",
    "ito-orchestrator-planner/SKILL.md",
    "ito-orchestrator-researcher/SKILL.md",
    "ito-orchestrator-reviewer/SKILL.md",
    "ito-orchestrator-worker/SKILL.md",
];

pub(super) fn remove_obsolete_specialist_agents(agent_dir: &Path) -> CoreResult<()> {
    for obsolete_rel_path in OBSOLETE_SPECIALIST_AGENT_REL_PATHS {
        remove_obsolete_specialist_agent(agent_dir, obsolete_rel_path)?;
    }

    Ok(())
}

pub(super) fn remove_obsolete_specialist_agent(
    agent_dir: &Path,
    obsolete_rel_path: &str,
) -> CoreResult<()> {
    let obsolete = agent_dir.join(obsolete_rel_path);
    let metadata = match std::fs::symlink_metadata(&obsolete) {
        Ok(metadata) => metadata,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(()),
        Err(err) => {
            return Err(CoreError::io(
                format!("reading {}", obsolete.display()),
                err,
            ));
        }
    };

    let file_type = metadata.file_type();
    if file_type.is_file() || file_type.is_symlink() {
        std::fs::remove_file(&obsolete)
            .map_err(|e| CoreError::io(format!("removing {}", obsolete.display()), e))?;
    } else {
        return Err(CoreError::Validation(format!(
            "expected obsolete specialist agent path to be a file or symlink: {}. Remove the directory manually and rerun the install.",
            obsolete.display()
        )));
    }
    prune_empty_agent_dirs(agent_dir, obsolete.parent())
}

fn prune_empty_agent_dirs(agent_dir: &Path, start: Option<&Path>) -> CoreResult<()> {
    let mut current = start.map(Path::to_path_buf);

    while let Some(dir) = current {
        if dir == agent_dir || !dir.starts_with(agent_dir) {
            break;
        }
        let is_empty = {
            let mut entries = std::fs::read_dir(&dir)
                .map_err(|e| CoreError::io(format!("reading {}", dir.display()), e))?;
            entries
                .next()
                .transpose()
                .map_err(|e| CoreError::io(format!("reading {}", dir.display()), e))?
                .is_none()
        };
        if !is_empty {
            break;
        }
        std::fs::remove_dir(&dir)
            .map_err(|e| CoreError::io(format!("removing {}", dir.display()), e))?;
        current = dir.parent().map(Path::to_path_buf);
    }
    Ok(())
}

#[cfg(test)]
#[path = "agents_cleanup_tests.rs"]
mod agents_cleanup_tests;
