//! Cross-artifact repository integrity checks.
//!
//! These checks validate relationships between on-disk Ito artifacts that are
//! hard to express as a single file-local validation.

use crate::error_bridge::IntoCoreResult;
use crate::errors::{CoreError, CoreResult};
use crate::validate::{ValidationIssue, error};
use ito_common::fs::StdFs;
use ito_common::id;
use ito_domain::discovery;
use rusqlite::Connection;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// Convert a `rusqlite::Result` into a `CoreResult`.
fn sqlite<T>(r: rusqlite::Result<T>) -> CoreResult<T> {
    r.map_err(|e| CoreError::Sqlite(format!("sqlite error: {e}")))
}

/// Extract the 3-digit module id prefix from a module directory name.
///
/// Module directories are expected to be in the format `NNN_slug`.
fn parse_module_id_from_dir_name(dir_name: &str) -> Option<String> {
    let (id_part, _slug) = dir_name.split_once('_')?;
    if id_part.len() != 3 || !id_part.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    Some(id_part.to_string())
}

/// Validate change directory naming and cross-references.
///
/// The returned map is keyed by change directory name. A directory is present in
/// the map only if at least one integrity issue was found.
pub fn validate_change_dirs_repo_integrity(
    ito_path: &Path,
) -> CoreResult<BTreeMap<String, Vec<ValidationIssue>>> {
    let mut by_dir: BTreeMap<String, Vec<ValidationIssue>> = BTreeMap::new();

    let mut module_ids: BTreeSet<String> = BTreeSet::new();
    for m in discovery::list_module_dir_names(&StdFs, ito_path).into_core()? {
        if let Some(id) = parse_module_id_from_dir_name(&m) {
            module_ids.insert(id);
        }
    }

    let change_dirs = discovery::list_change_dir_names(&StdFs, ito_path).into_core()?;
    if change_dirs.is_empty() {
        return Ok(by_dir);
    }

    let conn = sqlite(Connection::open_in_memory())?;
    sqlite(conn.execute_batch(
        r#"
CREATE TABLE change_dir (
  dir_name TEXT NOT NULL,
  numeric_id TEXT NOT NULL,
  module_id TEXT NOT NULL,
  change_num TEXT NOT NULL,
  slug TEXT NOT NULL
);
"#,
    ))?;

    for dir_name in &change_dirs {
        match id::parse_change_id(dir_name) {
            Ok(p) => {
                let numeric_id = format!("{}-{}", p.module_id, p.change_num);
                let slug = p.name;
                sqlite(conn.execute(
                    "INSERT INTO change_dir (dir_name, numeric_id, module_id, change_num, slug) VALUES (?1, ?2, ?3, ?4, ?5)",
                    rusqlite::params![dir_name, numeric_id, p.module_id.as_str(), p.change_num, slug],
                ))?;
            }
            Err(e) => {
                let msg = if let Some(hint) = e.hint.as_deref() {
                    format!(
                        "Invalid change directory name '{dir_name}': {} (hint: {hint})",
                        e.error
                    )
                } else {
                    format!("Invalid change directory name '{dir_name}': {}", e.error)
                };
                by_dir
                    .entry(dir_name.clone())
                    .or_default()
                    .push(error("id", msg));
            }
        }
    }

    // Module existence for parsed change dirs.
    {
        let mut stmt = sqlite(conn.prepare("SELECT dir_name, module_id FROM change_dir"))?;
        let mut rows = sqlite(stmt.query([]))?;
        while let Some(row) = sqlite(rows.next())? {
            let dir_name: String = sqlite(row.get(0))?;
            let module_id: String = sqlite(row.get(1))?;
            if !module_ids.contains(&module_id) {
                by_dir.entry(dir_name.clone()).or_default().push(error(
                    "module",
                    format!("Change '{dir_name}' refers to missing module '{module_id}'"),
                ));
            }
        }
    }

    // Duplicate numeric change IDs.
    {
        let mut stmt = sqlite(conn.prepare(
            "SELECT numeric_id FROM change_dir GROUP BY numeric_id HAVING COUNT(*) > 1 ORDER BY numeric_id",
        ))?;
        let mut rows = sqlite(stmt.query([]))?;
        while let Some(row) = sqlite(rows.next())? {
            let numeric_id: String = sqlite(row.get(0))?;
            let mut stmt2 = sqlite(conn.prepare(
                "SELECT dir_name FROM change_dir WHERE numeric_id = ?1 ORDER BY dir_name",
            ))?;
            let dirs: Vec<String> = stmt2
                .query_map([numeric_id.as_str()], |r| r.get(0))
                .map_err(|e| CoreError::Sqlite(format!("sqlite error: {e}")))?
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| CoreError::Sqlite(format!("sqlite error: {e}")))?;
            for d in &dirs {
                let others: Vec<&str> = dirs
                    .iter()
                    .filter(|x| *x != d)
                    .map(|s| s.as_str())
                    .collect();
                by_dir.entry(d.clone()).or_default().push(error(
                    "id",
                    format!(
                        "Duplicate numeric change id {numeric_id}: also found at {}",
                        others.join(", ")
                    ),
                ));
            }
        }
    }

    Ok(by_dir)
}
