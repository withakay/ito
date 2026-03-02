//! Grep-style search over Ito change artifacts.
//!
//! Provides a consistent search interface that works whether artifacts live on
//! the local filesystem (`.ito/`) or have been materialised from a remote
//! backend into a local cache. The search engine uses the ripgrep crate
//! ecosystem (`grep-regex`, `grep-searcher`) so callers get familiar regex
//! semantics without shelling out.

use std::path::{Path, PathBuf};

use grep_regex::RegexMatcher;
use grep_searcher::Searcher;
use grep_searcher::sinks::UTF8;

use crate::errors::{CoreError, CoreResult};

/// A single matching line returned by the grep engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrepMatch {
    /// Absolute path of the file that matched.
    pub path: PathBuf,
    /// 1-based line number within the file.
    pub line_number: u64,
    /// The full text of the matching line (without trailing newline).
    pub line: String,
}

/// Scope of the grep search.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrepScope {
    /// Search artifacts belonging to a single change.
    Change(String),
    /// Search artifacts belonging to all changes in a module.
    Module(String),
    /// Search artifacts across all changes in the project.
    All,
}

/// Input parameters for a grep operation.
#[derive(Debug, Clone)]
pub struct GrepInput {
    /// The regex pattern to search for.
    pub pattern: String,
    /// The scope of the search.
    pub scope: GrepScope,
    /// Maximum number of matching lines to return (0 = unlimited).
    pub limit: usize,
}

/// Result of a grep operation.
#[derive(Debug, Clone)]
pub struct GrepOutput {
    /// The matching lines found.
    pub matches: Vec<GrepMatch>,
    /// Whether the output was truncated due to the limit.
    pub truncated: bool,
}

/// Search the given files for lines matching `pattern`, returning at most
/// `limit` results (0 means unlimited).
///
/// This is the core search engine used by all grep scopes. It uses the
/// ripgrep crate ecosystem for fast, correct regex matching.
///
/// # Errors
///
/// Returns `CoreError::Validation` if the pattern is not a valid regex.
/// Files that cannot be read are skipped (logged at debug level) so one
/// unreadable file does not fail the entire search.
pub fn search_files(files: &[PathBuf], pattern: &str, limit: usize) -> CoreResult<GrepOutput> {
    let matcher = RegexMatcher::new(pattern)
        .map_err(|e| CoreError::validation(format!("invalid grep pattern: {e}")))?;

    let mut matches: Vec<GrepMatch> = Vec::new();
    let mut truncated = false;
    let mut searcher = Searcher::new();

    for file_path in files {
        if limit > 0 && matches.len() >= limit {
            truncated = true;
            break;
        }

        let search_result = searcher.search_path(
            &matcher,
            file_path,
            UTF8(|line_number, line_text| {
                if limit > 0 && matches.len() >= limit {
                    truncated = true;
                    return Ok(false);
                }

                matches.push(GrepMatch {
                    path: file_path.clone(),
                    line_number,
                    line: line_text
                        .trim_end_matches(&['\r', '\n'][..])
                        .to_string(),
                });

                if limit > 0 && matches.len() >= limit {
                    truncated = true;
                    Ok(false)
                } else {
                    Ok(true)
                }
            }),
        );

        // Skip files that cannot be read (e.g. binary, permission denied)
        // rather than failing the entire search.
        if let Err(e) = search_result {
            tracing::debug!("skipping file {}: {e}", file_path.display());
        }
    }

    Ok(GrepOutput { matches, truncated })
}

/// Collect all artifact markdown files for a single change directory.
///
/// Returns paths to `proposal.md`, `design.md`, `tasks.md`, and any
/// `specs/<name>/spec.md` files found under the change directory.
pub fn collect_change_artifact_files(change_dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let known_files = ["proposal.md", "design.md", "tasks.md"];
    for name in &known_files {
        let p = change_dir.join(name);
        if p.is_file() {
            files.push(p);
        }
    }

    let specs_dir = change_dir.join("specs");
    if specs_dir.is_dir()
        && let Ok(entries) = std::fs::read_dir(&specs_dir)
    {
        let mut spec_dirs: Vec<_> = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .collect();
        spec_dirs.sort_by_key(|e| e.file_name());

        for entry in spec_dirs {
            let spec_file = entry.path().join("spec.md");
            if spec_file.is_file() {
                files.push(spec_file);
            }
        }
    }

    files
}

/// Resolve the grep scope to a list of artifact files and execute the search.
///
/// # Arguments
///
/// * `ito_path` - Path to the `.ito/` directory.
/// * `input` - The grep parameters (pattern, scope, limit).
/// * `change_repo` - A change repository for resolving targets and listing changes.
/// * `module_repo` - A module repository for resolving module targets.
///
/// # Errors
///
/// Returns errors if the change/module cannot be found or the pattern is invalid.
pub fn grep<CR, MR>(
    ito_path: &Path,
    input: &GrepInput,
    change_repo: &CR,
    module_repo: &MR,
) -> CoreResult<GrepOutput>
where
    CR: ito_domain::changes::ChangeRepository,
    MR: ito_domain::modules::ModuleRepository,
{
    let files = resolve_scope_files(ito_path, &input.scope, change_repo, module_repo)?;
    search_files(&files, &input.pattern, input.limit)
}

/// Resolve a grep scope into the list of artifact files to search.
fn resolve_scope_files<CR, MR>(
    ito_path: &Path,
    scope: &GrepScope,
    change_repo: &CR,
    module_repo: &MR,
) -> CoreResult<Vec<PathBuf>>
where
    CR: ito_domain::changes::ChangeRepository,
    MR: ito_domain::modules::ModuleRepository,
{
    match scope {
        GrepScope::Change(change_id) => {
            let resolution = change_repo.resolve_target(change_id);
            let actual_id = match resolution {
                ito_domain::changes::ChangeTargetResolution::Unique(id) => id,
                ito_domain::changes::ChangeTargetResolution::Ambiguous(matches) => {
                    return Err(CoreError::validation(format!(
                        "ambiguous change target '{change_id}', matches: {}",
                        matches.join(", ")
                    )));
                }
                ito_domain::changes::ChangeTargetResolution::NotFound => {
                    return Err(CoreError::not_found(format!(
                        "change '{change_id}' not found"
                    )));
                }
            };
            let change_dir = ito_common::paths::change_dir(ito_path, &actual_id);
            Ok(collect_change_artifact_files(&change_dir))
        }

        GrepScope::Module(module_id) => {
            // Verify the module exists
            let module = module_repo
                .get(module_id)
                .map_err(|e| CoreError::not_found(format!("module '{module_id}': {e}")))?;

            // List all changes in the module
            let changes = change_repo.list_by_module(&module.id)?;
            let mut files = Vec::new();
            for change in &changes {
                let change_dir = ito_common::paths::change_dir(ito_path, &change.id);
                files.extend(collect_change_artifact_files(&change_dir));
            }
            Ok(files)
        }

        GrepScope::All => {
            let changes = change_repo.list()?;
            let mut files = Vec::new();
            for change in &changes {
                let change_dir = ito_common::paths::change_dir(ito_path, &change.id);
                files.extend(collect_change_artifact_files(&change_dir));
            }
            Ok(files)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_dir(tmp: &TempDir) -> PathBuf {
        let ito = tmp.path().join(".ito");
        fs::create_dir_all(ito.join("changes/001-01_test-change/specs/auth")).unwrap();
        fs::write(
            ito.join("changes/001-01_test-change/proposal.md"),
            "# Proposal\n\nThis adds auth support.\n",
        )
        .unwrap();
        fs::write(
            ito.join("changes/001-01_test-change/tasks.md"),
            "# Tasks\n- [ ] Add login endpoint\n- [ ] Add tests\n",
        )
        .unwrap();
        fs::write(
            ito.join("changes/001-01_test-change/specs/auth/spec.md"),
            "## ADDED Requirements\n\n### Requirement: Login\nThe system SHALL provide login.\n\n#### Scenario: Success\n- **WHEN** valid creds\n- **THEN** token returned\n",
        )
        .unwrap();
        ito
    }

    #[test]
    fn collect_change_artifact_files_finds_all_md_files() {
        let tmp = TempDir::new().unwrap();
        let ito = setup_test_dir(&tmp);
        let change_dir = ito.join("changes/001-01_test-change");

        let files = collect_change_artifact_files(&change_dir);
        assert_eq!(files.len(), 3); // proposal, tasks, specs/auth/spec.md (no design.md)

        let names: Vec<String> = files
            .iter()
            .map(|p| {
                p.strip_prefix(&change_dir)
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            })
            .collect();
        assert!(names.contains(&"proposal.md".to_string()));
        assert!(names.contains(&"tasks.md".to_string()));
        assert!(names.contains(&"specs/auth/spec.md".to_string()));
    }

    #[test]
    fn search_files_finds_matching_lines() {
        let tmp = TempDir::new().unwrap();
        let ito = setup_test_dir(&tmp);
        let change_dir = ito.join("changes/001-01_test-change");

        let files = collect_change_artifact_files(&change_dir);
        let output = search_files(&files, "Requirement:", 0).unwrap();

        assert_eq!(output.matches.len(), 1);
        assert!(output.matches[0].line.contains("Requirement: Login"));
        assert!(!output.truncated);
    }

    #[test]
    fn search_files_respects_limit() {
        let tmp = TempDir::new().unwrap();
        let ito = setup_test_dir(&tmp);
        let change_dir = ito.join("changes/001-01_test-change");

        let files = collect_change_artifact_files(&change_dir);
        // Search for something that matches many lines
        let output = search_files(&files, ".", 2).unwrap();

        assert_eq!(output.matches.len(), 2);
        assert!(output.truncated);
    }

    #[test]
    fn search_files_returns_empty_for_no_matches() {
        let tmp = TempDir::new().unwrap();
        let ito = setup_test_dir(&tmp);
        let change_dir = ito.join("changes/001-01_test-change");

        let files = collect_change_artifact_files(&change_dir);
        let output = search_files(&files, "ZZZZZZZ_NOMATCH", 0).unwrap();

        assert!(output.matches.is_empty());
        assert!(!output.truncated);
    }

    #[test]
    fn search_files_rejects_invalid_regex() {
        let files = vec![PathBuf::from("/nonexistent")];
        let result = search_files(&files, "[invalid", 0);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("invalid grep pattern"));
    }

    #[test]
    fn search_files_includes_correct_line_numbers() {
        let tmp = TempDir::new().unwrap();
        let ito = setup_test_dir(&tmp);

        let files = vec![ito.join("changes/001-01_test-change/tasks.md")];
        let output = search_files(&files, r"Add", 0).unwrap();

        assert_eq!(output.matches.len(), 2);
        // "Add login endpoint" should be line 2, "Add tests" should be line 3
        assert_eq!(output.matches[0].line_number, 2);
        assert_eq!(output.matches[1].line_number, 3);
    }
}
