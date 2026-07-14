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
//! Used by the `ito init` advisory for explicit downstream hook setup.

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
#[path = "pre_commit_detect_tests.rs"]
mod pre_commit_detect_tests;
