//! Side-effect-free inspection of legacy coordination-worktree state.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use ito_config::ito_dir::lexical_normalize;
use ito_config::types::{BackendApiConfig, CoordinationBranchConfig, CoordinationStorage};
use serde::Serialize;

use crate::errors::{CoreError, CoreResult};
use crate::git_remote::resolve_org_repo_from_config_or_remote;
use crate::repo_paths::coordination_worktree_path;

const GITIGNORE_MARKER: &str = "# Ito coordination worktree symlinks";

/// Ito state directories formerly projected through the coordination worktree.
///
/// This neutral definition stays available when coordination runtime code is
/// excluded from a build, so detection and recovery do not depend on it.
pub const MANAGED_STATE_DIRS: &[&str] = &["changes", "specs", "modules", "workflows", "audit"];

const MANAGED_GITIGNORE_ENTRIES: &[&str] = &[
    ".ito/changes",
    ".ito/specs",
    ".ito/modules",
    ".ito/workflows",
    ".ito/audit",
];

/// Canonical legacy `.gitignore` entries corresponding to [`MANAGED_STATE_DIRS`].
#[must_use]
pub const fn managed_gitignore_entries() -> &'static [&'static str] {
    MANAGED_GITIGNORE_ENTRIES
}

/// Resolve the legacy coordination `.ito` root from explicit or local Git evidence.
///
/// This performs no network access and remains separate from coordination
/// provisioning, synchronization, and symlink-runtime code.
#[must_use]
pub fn expected_coordination_ito_root(
    project_root: &Path,
    ito_root: &Path,
    coordination: &CoordinationBranchConfig,
    backend: &BackendApiConfig,
) -> Option<PathBuf> {
    if coordination
        .worktree_path
        .as_deref()
        .is_some_and(|path| !path.trim().is_empty())
    {
        return Some(coordination_worktree_path(coordination, ito_root, "", "").join(".ito"));
    }

    resolve_org_repo_from_config_or_remote(project_root, backend).map(|(org, repo)| {
        coordination_worktree_path(coordination, ito_root, &org, &repo).join(".ito")
    })
}

/// Overall classification of coordination storage evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LegacyCoordinationClass {
    /// No managed Ito state paths exist yet.
    Absent,
    /// Managed Ito state is stored in real repository directories.
    Embedded,
    /// Legacy coordination storage is configured or visibly wired.
    Legacy,
    /// Evidence conflicts and requires human reconciliation.
    Ambiguous,
}

/// Resolved coordination configuration recorded by the detector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CoordinationConfigEvidence {
    /// Whether coordination synchronization is enabled.
    pub enabled: bool,
    /// Configured storage identifier.
    pub storage: String,
    /// Configured coordination branch.
    pub branch: String,
    /// Configured worktree path override, if any.
    pub worktree_path: Option<String>,
}

/// Filesystem kind observed at one managed Ito path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ManagedPathKind {
    /// The managed path is absent.
    Missing,
    /// The managed path is a real directory.
    Directory {
        /// Whether the directory contains no entries.
        empty: bool,
    },
    /// The managed path is a symbolic link or directory junction.
    Link {
        /// Target stored in the link itself.
        target: PathBuf,
        /// Link target lexically resolved against the Ito root when relative.
        resolved_target: PathBuf,
        /// Whether the resolved target matches the expected coordination target.
        matches_expected: Option<bool>,
        /// Whether the target currently exists.
        target_exists: bool,
    },
    /// The path exists but is neither a directory nor a coordination link.
    Other,
}

/// Evidence for one managed Ito state path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ManagedPathEvidence {
    /// Managed directory name beneath the Ito root.
    pub name: String,
    /// Full path inspected by the detector.
    pub path: PathBuf,
    /// Filesystem kind observed without following links.
    pub kind: ManagedPathKind,
}

/// Evidence found in the repository `.gitignore`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CoordinationGitignoreEvidence {
    /// Whether the managed coordination marker or its complete entry set exists.
    pub marker_present: bool,
    /// Canonical coordination entries found after trimming surrounding whitespace.
    pub matching_entries: Vec<String>,
}

/// Complete side-effect-free legacy coordination inspection report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LegacyCoordinationReport {
    /// Overall storage classification.
    pub classification: LegacyCoordinationClass,
    /// Resolved configuration evidence.
    pub config: CoordinationConfigEvidence,
    /// Per-path filesystem evidence.
    pub managed_paths: Vec<ManagedPathEvidence>,
    /// Managed `.gitignore` evidence.
    pub gitignore: CoordinationGitignoreEvidence,
}

/// Inspect coordination configuration and repository evidence without mutation.
///
/// `project_root` identifies the repository containing `.gitignore`, while
/// `ito_root` identifies its resolved Ito directory. `config` must be the
/// already-resolved coordination configuration. When
/// `expected_coordination_ito_root` is present, link targets are compared with
/// the expected shared `.ito` root; when it is absent, target matching remains
/// unknown and wrong-target detection is intentionally unavailable.
///
/// # Errors
///
/// Returns an error when filesystem metadata or repository files cannot be read.
pub fn inspect_legacy_coordination(
    project_root: &Path,
    ito_root: &Path,
    config: &CoordinationBranchConfig,
    expected_coordination_ito_root: Option<&Path>,
) -> CoreResult<LegacyCoordinationReport> {
    let managed_paths = MANAGED_STATE_DIRS
        .iter()
        .map(|name| inspect_managed_path(ito_root, name, expected_coordination_ito_root))
        .collect::<CoreResult<Vec<_>>>()?;
    let gitignore = inspect_gitignore(project_root)?;
    let classification = classify(config, &managed_paths, &gitignore);

    Ok(LegacyCoordinationReport {
        classification,
        config: CoordinationConfigEvidence {
            enabled: config.enabled.0,
            storage: config.storage.as_str().to_string(),
            branch: config.name.clone(),
            worktree_path: config.worktree_path.clone(),
        },
        managed_paths,
        gitignore,
    })
}

fn inspect_managed_path(
    ito_root: &Path,
    name: &str,
    expected_coordination_ito_root: Option<&Path>,
) -> CoreResult<ManagedPathEvidence> {
    let path = ito_root.join(name);
    let metadata = match fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(ManagedPathEvidence {
                name: name.to_string(),
                path,
                kind: ManagedPathKind::Missing,
            });
        }
        Err(error) => {
            return Err(CoreError::io(
                format!(
                    "cannot inspect legacy coordination path '{}': check filesystem permissions",
                    path.display()
                ),
                error,
            ));
        }
    };

    let link_target = match read_dir_link(&path) {
        Ok(target) => Some(target),
        Err(error) if metadata.file_type().is_symlink() => {
            return Err(CoreError::io(
                format!(
                    "cannot read legacy coordination link '{}': check filesystem permissions",
                    path.display()
                ),
                error,
            ));
        }
        Err(_) => None,
    };

    let kind = if let Some(target) = link_target {
        let resolved_target = if target.is_absolute() {
            lexical_normalize(&target)
        } else {
            lexical_normalize(&ito_root.join(&target))
        };
        let matches_expected = expected_coordination_ito_root
            .map(|expected_root| resolved_target == lexical_normalize(&expected_root.join(name)));
        let target_exists = path_exists(&resolved_target)?;

        ManagedPathKind::Link {
            target,
            resolved_target,
            matches_expected,
            target_exists,
        }
    } else if metadata.is_dir() {
        let mut entries = fs::read_dir(&path).map_err(|error| {
            CoreError::io(
                format!(
                    "cannot read legacy coordination directory '{}': check filesystem permissions",
                    path.display()
                ),
                error,
            )
        })?;
        let first_entry = entries.next().transpose().map_err(|error| {
            CoreError::io(
                format!(
                    "cannot inspect entries in legacy coordination directory '{}': check filesystem permissions",
                    path.display()
                ),
                error,
            )
        })?;
        ManagedPathKind::Directory {
            empty: first_entry.is_none(),
        }
    } else {
        ManagedPathKind::Other
    };

    Ok(ManagedPathEvidence {
        name: name.to_string(),
        path,
        kind,
    })
}

fn path_exists(path: &Path) -> CoreResult<bool> {
    match fs::metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(CoreError::io(
            format!(
                "cannot inspect legacy coordination link target '{}': check filesystem permissions",
                path.display()
            ),
            error,
        )),
    }
}

fn inspect_gitignore(project_root: &Path) -> CoreResult<CoordinationGitignoreEvidence> {
    let path = project_root.join(".gitignore");
    let contents = match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == io::ErrorKind::NotFound => String::new(),
        Err(error) => {
            return Err(CoreError::io(
                format!(
                    "cannot inspect legacy coordination marker in '{}': check filesystem permissions",
                    path.display()
                ),
                error,
            ));
        }
    };

    let lines = contents.lines().map(str::trim).collect::<Vec<_>>();
    let matching_entries = managed_gitignore_entries()
        .iter()
        .filter(|entry| lines.iter().any(|line| line == *entry))
        .map(|entry| (*entry).to_string())
        .collect::<Vec<_>>();
    let marker_present = lines.contains(&GITIGNORE_MARKER)
        || matching_entries.len() == managed_gitignore_entries().len();

    Ok(CoordinationGitignoreEvidence {
        marker_present,
        matching_entries,
    })
}

fn classify(
    config: &CoordinationBranchConfig,
    managed_paths: &[ManagedPathEvidence],
    gitignore: &CoordinationGitignoreEvidence,
) -> LegacyCoordinationClass {
    let configured_legacy = config.enabled.0 && config.storage == CoordinationStorage::Worktree;
    let configured_main = !config.enabled.0 || config.storage == CoordinationStorage::Embedded;
    let has_link = managed_paths
        .iter()
        .any(|evidence| matches!(evidence.kind, ManagedPathKind::Link { .. }));
    let has_wrong_link = managed_paths.iter().any(|evidence| {
        matches!(
            evidence.kind,
            ManagedPathKind::Link {
                matches_expected: Some(false),
                ..
            }
        )
    });
    let link_roots = managed_paths
        .iter()
        .filter_map(|evidence| match &evidence.kind {
            ManagedPathKind::Link {
                resolved_target, ..
            } => resolved_target.parent().map(lexical_normalize),
            _ => None,
        })
        .collect::<std::collections::BTreeSet<_>>();
    let has_inconsistent_link_roots = link_roots.len() > 1;
    let has_authority_link = managed_paths.iter().any(|evidence| {
        matches!(evidence.name.as_str(), "changes" | "specs")
            && matches!(evidence.kind, ManagedPathKind::Link { .. })
    });
    let has_non_empty_authority_directory = managed_paths.iter().any(|evidence| {
        matches!(evidence.name.as_str(), "changes" | "specs")
            && matches!(evidence.kind, ManagedPathKind::Directory { empty: false })
    });
    let has_non_empty_runtime_directory = managed_paths.iter().any(|evidence| {
        matches!(evidence.name.as_str(), "modules" | "workflows" | "audit")
            && matches!(evidence.kind, ManagedPathKind::Directory { empty: false })
    });
    let has_real_directory = managed_paths
        .iter()
        .any(|evidence| matches!(evidence.kind, ManagedPathKind::Directory { .. }));
    let has_other = managed_paths
        .iter()
        .any(|evidence| matches!(evidence.kind, ManagedPathKind::Other));
    let all_missing = managed_paths
        .iter()
        .all(|evidence| matches!(evidence.kind, ManagedPathKind::Missing));
    let has_gitignore_entries = !gitignore.matching_entries.is_empty();
    let partial_gitignore_marker = has_gitignore_entries && !gitignore.marker_present;
    let standalone_gitignore_marker =
        gitignore.marker_present && gitignore.matching_entries.is_empty();
    let worktree_config_with_materialized_paths =
        configured_legacy && !has_link && has_real_directory;

    let conflicting_evidence = has_wrong_link
        || has_inconsistent_link_roots
        || has_other
        || partial_gitignore_marker
        || standalone_gitignore_marker
        || worktree_config_with_materialized_paths
        || (has_authority_link && has_non_empty_authority_directory)
        || (has_link && has_non_empty_runtime_directory)
        || (configured_main && (has_link || has_gitignore_entries))
        || (gitignore.marker_present && !has_link && has_real_directory);

    if conflicting_evidence {
        LegacyCoordinationClass::Ambiguous
    } else if configured_legacy || has_link || has_gitignore_entries {
        LegacyCoordinationClass::Legacy
    } else if all_missing {
        LegacyCoordinationClass::Absent
    } else if has_real_directory {
        LegacyCoordinationClass::Embedded
    } else {
        LegacyCoordinationClass::Ambiguous
    }
}

fn read_dir_link(path: &Path) -> io::Result<PathBuf> {
    #[cfg(windows)]
    {
        junction::get_target(path)
    }

    #[cfg(not(windows))]
    {
        fs::read_link(path)
    }
}

#[cfg(test)]
#[path = "legacy_coordination_tests.rs"]
mod legacy_coordination_tests;
