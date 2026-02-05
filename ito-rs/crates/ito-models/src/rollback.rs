//! Agent file rollback utilities

use crate::agent::Harness;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Result of a rollback operation
#[derive(Debug, Clone)]
pub struct RollbackResult {
    /// Path to the restored file
    pub path: PathBuf,
    /// Path to the backup that was used
    pub backup_path: PathBuf,
}

/// Error during rollback
#[derive(Debug, thiserror::Error)]
pub enum RollbackError {
    #[error("Failed to restore file: {0}")]
    IoError(#[from] io::Error),

    #[error("Backup file not found: {0}")]
    BackupNotFound(PathBuf),
}

/// Find all backup files in agent directories
pub fn find_backup_files(project_root: Option<&Path>) -> Vec<PathBuf> {
    let mut backups = Vec::new();

    // Check global agent directories
    for harness in Harness::all() {
        for path in harness.global_agent_paths() {
            if path.exists() {
                backups.extend(find_backups_in_directory(&path));
            }
        }
    }

    // Check project agent directories
    if let Some(root) = project_root {
        for harness in Harness::all() {
            for rel_path in harness.project_agent_paths() {
                let path = root.join(rel_path);
                if path.exists() {
                    backups.extend(find_backups_in_directory(&path));
                }
            }
        }
    }

    backups
}

/// Find backup files in a specific directory
fn find_backups_in_directory(dir: &Path) -> Vec<PathBuf> {
    let mut backups = Vec::new();

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return backups,
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "bak" {
                    backups.push(path);
                }
            }
        } else if path.is_dir() {
            // Check for SKILL.md.bak in subdirectories (for Codex)
            let skill_backup = path.join("SKILL.md.bak");
            if skill_backup.exists() {
                backups.push(skill_backup);
            }
        }
    }

    backups
}

/// Restore a single file from its backup
pub fn restore_from_backup(backup_path: &Path) -> Result<RollbackResult, RollbackError> {
    if !backup_path.exists() {
        return Err(RollbackError::BackupNotFound(backup_path.to_path_buf()));
    }

    // Determine the original file path by removing .bak extension
    let original_path = if backup_path.extension() == Some(std::ffi::OsStr::new("bak")) {
        let stem = backup_path.file_stem().unwrap_or_default();
        backup_path.with_file_name(stem)
    } else {
        return Err(RollbackError::BackupNotFound(backup_path.to_path_buf()));
    };

    // Copy backup to original location
    fs::copy(backup_path, &original_path)?;

    // Remove the backup file
    fs::remove_file(backup_path)?;

    Ok(RollbackResult {
        path: original_path,
        backup_path: backup_path.to_path_buf(),
    })
}

/// Rollback all agent files from their backups
pub fn rollback_all(project_root: Option<&Path>) -> Vec<Result<RollbackResult, RollbackError>> {
    let backups = find_backup_files(project_root);

    backups
        .iter()
        .map(|backup| restore_from_backup(backup))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn find_backups_in_directory_finds_bak_files() {
        let dir = tempdir().unwrap();

        // Create some backup files
        fs::write(dir.path().join("agent1.md.bak"), "backup1").unwrap();
        fs::write(dir.path().join("agent2.md.bak"), "backup2").unwrap();
        fs::write(dir.path().join("agent3.md"), "not a backup").unwrap();

        let backups = find_backups_in_directory(dir.path());

        assert_eq!(backups.len(), 2);
        assert!(backups
            .iter()
            .any(|p| p.file_name().unwrap() == "agent1.md.bak"));
        assert!(backups
            .iter()
            .any(|p| p.file_name().unwrap() == "agent2.md.bak"));
    }

    #[test]
    fn restore_from_backup_works() {
        let dir = tempdir().unwrap();

        // Create original and backup files
        let original = dir.path().join("agent.md");
        let backup = dir.path().join("agent.md.bak");

        fs::write(&original, "modified content").unwrap();
        fs::write(&backup, "original content").unwrap();

        let result = restore_from_backup(&backup).unwrap();

        assert_eq!(result.path, original);
        assert!(!backup.exists()); // Backup should be removed
        assert_eq!(fs::read_to_string(&original).unwrap(), "original content");
    }

    #[test]
    fn restore_from_backup_fails_if_no_backup() {
        let dir = tempdir().unwrap();
        let backup = dir.path().join("nonexistent.md.bak");

        let result = restore_from_backup(&backup);

        assert!(matches!(result, Err(RollbackError::BackupNotFound(_))));
    }
}
