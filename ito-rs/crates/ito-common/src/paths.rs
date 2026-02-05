use std::path::{Path, PathBuf};

/// Canonical `.ito/` path builders.
///
/// These helpers intentionally take a `ito_path` (the configured ito root directory)
/// so callers do not duplicate `.join("changes")`, `.join("modules")`, or ad-hoc
/// string-based path formatting.
pub fn default_ito_root(workspace_root: &Path) -> PathBuf {
    workspace_root.join(".ito")
}

pub fn changes_dir(ito_path: &Path) -> PathBuf {
    ito_path.join("changes")
}

pub fn change_dir(ito_path: &Path, change_id: &str) -> PathBuf {
    changes_dir(ito_path).join(change_id)
}

pub fn change_meta_path(ito_path: &Path, change_id: &str) -> PathBuf {
    change_dir(ito_path, change_id).join(".ito.yaml")
}

pub fn change_specs_dir(ito_path: &Path, change_id: &str) -> PathBuf {
    change_dir(ito_path, change_id).join("specs")
}

pub fn changes_archive_dir(ito_path: &Path) -> PathBuf {
    changes_dir(ito_path).join("archive")
}

pub fn archive_changes_dir(ito_path: &Path) -> PathBuf {
    ito_path.join("archive").join("changes")
}

pub fn modules_dir(ito_path: &Path) -> PathBuf {
    ito_path.join("modules")
}

pub fn specs_dir(ito_path: &Path) -> PathBuf {
    ito_path.join("specs")
}

pub fn spec_markdown_path(ito_path: &Path, spec_id: &str) -> PathBuf {
    specs_dir(ito_path).join(spec_id).join("spec.md")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_ito_root_is_dot_ito() {
        let root = PathBuf::from("/repo");
        assert_eq!(default_ito_root(&root), PathBuf::from("/repo/.ito"));
    }

    #[test]
    fn builders_join_expected_paths() {
        let ito = PathBuf::from("/repo/.ito");

        assert_eq!(changes_dir(&ito), PathBuf::from("/repo/.ito/changes"));
        assert_eq!(
            change_dir(&ito, "001-01_test"),
            PathBuf::from("/repo/.ito/changes/001-01_test")
        );
        assert_eq!(
            change_meta_path(&ito, "001-01_test"),
            PathBuf::from("/repo/.ito/changes/001-01_test/.ito.yaml")
        );
        assert_eq!(
            change_specs_dir(&ito, "001-01_test"),
            PathBuf::from("/repo/.ito/changes/001-01_test/specs")
        );
        assert_eq!(
            changes_archive_dir(&ito),
            PathBuf::from("/repo/.ito/changes/archive")
        );
        assert_eq!(
            archive_changes_dir(&ito),
            PathBuf::from("/repo/.ito/archive/changes")
        );
        assert_eq!(modules_dir(&ito), PathBuf::from("/repo/.ito/modules"));
        assert_eq!(specs_dir(&ito), PathBuf::from("/repo/.ito/specs"));
        assert_eq!(
            spec_markdown_path(&ito, "cli-tasks"),
            PathBuf::from("/repo/.ito/specs/cli-tasks/spec.md")
        );
    }
}
