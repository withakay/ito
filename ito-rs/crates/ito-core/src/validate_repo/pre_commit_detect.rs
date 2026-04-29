//! Detect which pre-commit hook framework, if any, is wired into a project.
//!
//! The detector is read-only: it inspects filesystem markers under the
//! project root and returns a [`PreCommitSystem`] enum value. It never
//! installs anything, modifies files, or contacts the network.
//!
//! Detection order (deterministic; later checks only run if earlier checks
//! did not match):
//!
//! 1. **`Prek`** — `.pre-commit-config.yaml` is present AND a prek-specific
//!    marker is present (`prek` resolvable on `PATH`, a `mise.toml` mentioning
//!    `prek`, or a `prek:` toolchain hint inside `.pre-commit-config.yaml`).
//! 2. **`PreCommit`** — `.pre-commit-config.yaml` exists but no prek marker
//!    is present.
//! 3. **`Husky`** — `.husky/` directory exists at the project root, OR
//!    `package.json` contains a top-level `husky` key.
//! 4. **`Lefthook`** — any of `lefthook.{yml,yaml}` or `.lefthook.{yml,yaml}`
//!    exists at the project root.
//! 5. **`None`** — no markers found.
//!
//! Used by the Wave 4 `ito init` advisory and by the `ito-update-repo`
//! skill to decide which file to edit when wiring the `ito validate repo`
//! pre-commit hook.

use std::path::{Path, PathBuf};

/// The pre-commit hook framework detected in a project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreCommitSystem {
    /// `prek` is wired up (`.pre-commit-config.yaml` plus a prek toolchain marker).
    Prek,
    /// Plain `pre-commit` (`.pre-commit-config.yaml` only, no prek markers).
    PreCommit,
    /// `Husky` (npm/yarn/pnpm Git hooks) — `.husky/` or `package.json` `husky` key.
    Husky,
    /// `lefthook` — any `lefthook.{yml,yaml}` or `.lefthook.{yml,yaml}`.
    Lefthook,
    /// No pre-commit framework detected.
    None,
}

impl PreCommitSystem {
    /// Short, lowercase identifier suitable for log lines and CLI output.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Prek => "prek",
            Self::PreCommit => "pre-commit",
            Self::Husky => "husky",
            Self::Lefthook => "lefthook",
            Self::None => "none",
        }
    }
}

impl std::fmt::Display for PreCommitSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Detect the pre-commit framework wired into the project at `project_root`.
///
/// Pure function over filesystem reads plus a single `PATH` lookup for the
/// `prek` binary. See the module-level documentation for the detection
/// order.
#[must_use]
pub fn detect_pre_commit_system(project_root: &Path) -> PreCommitSystem {
    detect_pre_commit_system_with(project_root, prek_on_path())
}

/// Detect the pre-commit framework with an explicit answer for "is `prek`
/// on PATH?".
///
/// `pub(crate)` so unit tests across the `validate_repo` module tree can
/// avoid mutating the global `PATH` environment variable during parallel
/// test runs. External consumers should call [`detect_pre_commit_system`].
#[must_use]
pub(crate) fn detect_pre_commit_system_with(
    project_root: &Path,
    prek_on_path: bool,
) -> PreCommitSystem {
    let pre_commit_yaml = project_root.join(".pre-commit-config.yaml");
    let pre_commit_yml = project_root.join(".pre-commit-config.yml");

    let pre_commit_path = if pre_commit_yaml.is_file() {
        Some(pre_commit_yaml)
    } else if pre_commit_yml.is_file() {
        Some(pre_commit_yml)
    } else {
        None
    };

    if let Some(pre_commit_path) = pre_commit_path {
        if has_prek_marker(project_root, &pre_commit_path, prek_on_path) {
            return PreCommitSystem::Prek;
        }
        return PreCommitSystem::PreCommit;
    }

    if has_husky(project_root) {
        return PreCommitSystem::Husky;
    }

    if has_lefthook(project_root) {
        return PreCommitSystem::Lefthook;
    }

    PreCommitSystem::None
}

/// True when any of the prek-specific markers are present.
fn has_prek_marker(project_root: &Path, pre_commit_path: &Path, prek_on_path: bool) -> bool {
    if prek_on_path {
        return true;
    }
    if mise_mentions_prek(project_root) {
        return true;
    }
    pre_commit_yaml_mentions_prek(pre_commit_path)
}

/// True when the `prek` binary is found on `PATH`.
///
/// Implemented manually rather than via the `which` crate to avoid an extra
/// dependency for a single use-site.
fn prek_on_path() -> bool {
    let Some(path) = std::env::var_os("PATH") else {
        return false;
    };
    for dir in std::env::split_paths(&path) {
        if dir.join("prek").is_file() {
            return true;
        }
        // Windows: also check `.exe`.
        if dir.join("prek.exe").is_file() {
            return true;
        }
    }
    false
}

/// True when `mise.toml` or `.mise.toml` at `project_root` mentions `prek`.
fn mise_mentions_prek(project_root: &Path) -> bool {
    let candidates = [
        project_root.join("mise.toml"),
        project_root.join(".mise.toml"),
    ];
    candidates.iter().any(|p| file_contains(p, "prek"))
}

/// True when the pre-commit config file at `path` contains a `prek` mention.
///
/// We deliberately use a substring check rather than a full YAML parse: the
/// detector is called in tight loops (e.g. once per `ito init`) and a string
/// scan is cheap. False positives only land projects on the slightly more
/// strict "Prek" path, which is harmless when the user is running plain
/// pre-commit.
fn pre_commit_yaml_mentions_prek(path: &Path) -> bool {
    file_contains(path, "prek")
}

/// True when `path` is a regular file that contains `needle` as a substring.
///
/// Read errors and missing files return `false`.
fn file_contains(path: &Path, needle: &str) -> bool {
    let Ok(content) = std::fs::read_to_string(path) else {
        return false;
    };
    content.contains(needle)
}

/// True when Husky markers are present.
fn has_husky(project_root: &Path) -> bool {
    if project_root.join(".husky").is_dir() {
        return true;
    }
    package_json_has_husky_key(&project_root.join("package.json"))
}

/// True when `package.json` contains a top-level `husky` key.
///
/// Substring scan with a guard for the `"husky":` token. We avoid pulling
/// in a JSON parser for a one-off check; nested occurrences (e.g. inside
/// `devDependencies`) would also match, which still signals a Husky-using
/// project.
fn package_json_has_husky_key(path: &Path) -> bool {
    let Ok(content) = std::fs::read_to_string(path) else {
        return false;
    };
    content.contains("\"husky\"")
}

/// True when any lefthook configuration file is present at `project_root`.
fn has_lefthook(project_root: &Path) -> bool {
    const LEFTHOOK_FILES: &[&str] = &[
        "lefthook.yml",
        "lefthook.yaml",
        ".lefthook.yml",
        ".lefthook.yaml",
    ];
    LEFTHOOK_FILES
        .iter()
        .map(|name| project_root.join(name))
        .any(|p: PathBuf| p.is_file())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn new_root() -> TempDir {
        TempDir::new().expect("tempdir")
    }

    fn write_file(root: &Path, rel: &str, body: &str) {
        let path = root.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create parent");
        }
        fs::write(path, body).expect("write fixture");
    }

    fn snapshot_tree(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
        // Capture the tree contents so we can prove the detector did not
        // mutate anything (pre-commit-hook-detection:read-only).
        fn walk(root: &Path, base: &Path, out: &mut Vec<(PathBuf, Vec<u8>)>) {
            for entry in fs::read_dir(root).expect("read_dir") {
                let entry = entry.expect("entry");
                let path = entry.path();
                let rel = path.strip_prefix(base).expect("strip prefix").to_path_buf();
                if path.is_dir() {
                    walk(&path, base, out);
                } else if path.is_file() {
                    let bytes = fs::read(&path).expect("read");
                    out.push((rel, bytes));
                }
            }
        }
        let mut out = Vec::new();
        walk(root, root, &mut out);
        out.sort_by(|a, b| a.0.cmp(&b.0));
        out
    }

    #[test]
    fn empty_repo_returns_none() {
        let tmp = new_root();
        assert_eq!(
            detect_pre_commit_system_with(tmp.path(), false),
            PreCommitSystem::None,
        );
    }

    #[test]
    fn pre_commit_config_alone_returns_pre_commit() {
        let tmp = new_root();
        write_file(tmp.path(), ".pre-commit-config.yaml", "repos: []\n");
        assert_eq!(
            detect_pre_commit_system_with(tmp.path(), false),
            PreCommitSystem::PreCommit,
        );
    }

    #[test]
    fn pre_commit_config_with_prek_in_yaml_returns_prek() {
        let tmp = new_root();
        write_file(
            tmp.path(),
            ".pre-commit-config.yaml",
            "# prek: enabled\nrepos: []\n",
        );
        assert_eq!(
            detect_pre_commit_system_with(tmp.path(), false),
            PreCommitSystem::Prek,
        );
    }

    #[test]
    fn prek_on_path_promotes_pre_commit_to_prek() {
        let tmp = new_root();
        write_file(tmp.path(), ".pre-commit-config.yaml", "repos: []\n");
        assert_eq!(
            detect_pre_commit_system_with(tmp.path(), true),
            PreCommitSystem::Prek,
        );
    }

    #[test]
    fn mise_mentioning_prek_returns_prek() {
        let tmp = new_root();
        write_file(tmp.path(), ".pre-commit-config.yaml", "repos: []\n");
        write_file(tmp.path(), "mise.toml", "[tools]\nprek = \"latest\"\n");
        assert_eq!(
            detect_pre_commit_system_with(tmp.path(), false),
            PreCommitSystem::Prek,
        );
    }

    #[test]
    fn dot_mise_mentioning_prek_returns_prek() {
        let tmp = new_root();
        write_file(tmp.path(), ".pre-commit-config.yaml", "repos: []\n");
        write_file(tmp.path(), ".mise.toml", "[tools]\nprek = \"latest\"\n");
        assert_eq!(
            detect_pre_commit_system_with(tmp.path(), false),
            PreCommitSystem::Prek,
        );
    }

    #[test]
    fn husky_directory_returns_husky() {
        let tmp = new_root();
        fs::create_dir(tmp.path().join(".husky")).expect("create .husky");
        write_file(tmp.path(), ".husky/pre-commit", "echo hi\n");
        assert_eq!(
            detect_pre_commit_system_with(tmp.path(), false),
            PreCommitSystem::Husky,
        );
    }

    #[test]
    fn package_json_with_husky_key_returns_husky() {
        let tmp = new_root();
        write_file(
            tmp.path(),
            "package.json",
            "{\n  \"name\": \"foo\",\n  \"husky\": {}\n}\n",
        );
        assert_eq!(
            detect_pre_commit_system_with(tmp.path(), false),
            PreCommitSystem::Husky,
        );
    }

    #[test]
    fn lefthook_yml_returns_lefthook() {
        let tmp = new_root();
        write_file(tmp.path(), "lefthook.yml", "pre-commit:\n  commands: {}\n");
        assert_eq!(
            detect_pre_commit_system_with(tmp.path(), false),
            PreCommitSystem::Lefthook,
        );
    }

    #[test]
    fn dot_lefthook_yaml_returns_lefthook() {
        let tmp = new_root();
        write_file(tmp.path(), ".lefthook.yaml", "pre-commit: {}\n");
        assert_eq!(
            detect_pre_commit_system_with(tmp.path(), false),
            PreCommitSystem::Lefthook,
        );
    }

    #[test]
    fn detection_is_read_only_for_every_variant() {
        // Build one fixture per variant and confirm bytes are unchanged
        // across the detector call.
        let cases: &[(&str, &[(&str, &str)])] = &[
            ("none", &[]),
            ("pre_commit", &[(".pre-commit-config.yaml", "repos: []\n")]),
            (
                "prek",
                &[(".pre-commit-config.yaml", "# prek\nrepos: []\n")],
            ),
            ("husky", &[("package.json", "{\"husky\": {}}\n")]),
            ("lefthook", &[("lefthook.yml", "pre-commit: {}\n")]),
        ];
        for (label, files) in cases {
            let tmp = new_root();
            for (rel, body) in *files {
                write_file(tmp.path(), rel, body);
            }
            let before = snapshot_tree(tmp.path());
            let _ = detect_pre_commit_system_with(tmp.path(), false);
            let after = snapshot_tree(tmp.path());
            assert_eq!(before, after, "detector mutated tree for fixture: {label}");
        }
    }

    #[test]
    fn pre_commit_classification_overrides_husky() {
        // When both a husky directory AND a pre-commit config exist, the
        // pre-commit branch wins per the documented detection order.
        let tmp = new_root();
        write_file(tmp.path(), ".pre-commit-config.yaml", "repos: []\n");
        fs::create_dir(tmp.path().join(".husky")).expect("create .husky");
        assert_eq!(
            detect_pre_commit_system_with(tmp.path(), false),
            PreCommitSystem::PreCommit,
        );
    }

    #[test]
    fn pre_commit_system_as_str_round_trips() {
        for variant in [
            PreCommitSystem::Prek,
            PreCommitSystem::PreCommit,
            PreCommitSystem::Husky,
            PreCommitSystem::Lefthook,
            PreCommitSystem::None,
        ] {
            assert_eq!(format!("{variant}"), variant.as_str());
        }
    }
}
