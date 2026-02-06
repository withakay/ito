//! Planning directory initialization.
//!
//! This module owns the filesystem I/O for bootstrapping the planning area.
//! Pure helpers (path builders, templates, parsers) remain in `ito_domain::planning`.

use ito_domain::planning::{
    milestones_dir, planning_dir, project_md_template, roadmap_md_template, state_md_template,
};
use std::path::Path;

/// Initialize the planning directory structure under `ito_path`.
///
/// This is safe to call multiple times; existing files are left unchanged.
pub fn init_planning_structure(
    ito_path: &Path,
    current_date: &str,
    ito_dir: &str,
) -> std::io::Result<()> {
    let planning = planning_dir(ito_path);
    std::fs::create_dir_all(&planning)?;
    std::fs::create_dir_all(milestones_dir(ito_path))?;

    let project_path = planning.join("PROJECT.md");
    if !project_path.exists() {
        std::fs::write(project_path, project_md_template(None, None))?;
    }
    let roadmap_path = planning.join("ROADMAP.md");
    if !roadmap_path.exists() {
        std::fs::write(roadmap_path, roadmap_md_template())?;
    }
    let state_path = planning.join("STATE.md");
    if !state_path.exists() {
        std::fs::write(state_path, state_md_template(current_date, ito_dir))?;
    }
    Ok(())
}
