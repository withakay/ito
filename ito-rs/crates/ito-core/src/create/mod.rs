//! Creation helpers for modules and changes.
//!
//! This module contains the filesystem operations behind `ito create module`
//! and `ito create change`.
//!
//! Functions here are designed to be called by the CLI layer and return
//! structured results suitable for JSON output.

use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use ito_common::fs::StdFs;
use ito_common::id::{parse_change_id, parse_module_id};
use ito_common::paths;

#[derive(Debug, thiserror::Error)]
/// Errors that can occur while creating modules or changes.
pub enum CreateError {
    /// The provided module name is invalid.
    #[error("Invalid module name '{0}'")]
    InvalidModuleName(String),

    // Match TS: the message is already user-facing (e.g. "Change name must be lowercase ...").
    /// The provided change name is invalid.
    #[error("{0}")]
    InvalidChangeName(String),

    /// The requested module id does not exist.
    #[error("Module '{0}' not found")]
    ModuleNotFound(String),

    /// A change with the same id already exists.
    #[error("Change '{0}' already exists")]
    ChangeAlreadyExists(String),

    /// Underlying I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone)]
/// Result of creating (or resolving) a module.
pub struct CreateModuleResult {
    /// 3-digit module id.
    pub module_id: String,
    /// Module name (slug).
    pub module_name: String,
    /// Folder name under `{ito_path}/modules`.
    pub folder_name: String,
    /// `true` if the module was newly created.
    pub created: bool,
    /// Path to the module directory.
    pub module_dir: PathBuf,
    /// Path to `module.md`.
    pub module_md: PathBuf,
}

#[derive(Debug, Clone)]
/// Result of creating a change.
pub struct CreateChangeResult {
    /// Change id (folder name under `{ito_path}/changes`).
    pub change_id: String,
    /// Path to the change directory.
    pub change_dir: PathBuf,
}

/// Create (or resolve) a module by name.
///
/// If a module with the same name already exists, this returns it with
/// `created=false`.
pub fn create_module(
    ito_path: &Path,
    name: &str,
    scope: Vec<String>,
    depends_on: Vec<String>,
) -> Result<CreateModuleResult, CreateError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(CreateError::InvalidModuleName(name.to_string()));
    }

    let modules_dir = paths::modules_dir(ito_path);
    ito_common::io::create_dir_all_std(&modules_dir)?;

    // If a module with the same name already exists, return it.
    if let Some(existing) = find_module_by_name(&modules_dir, name) {
        // `find_module_by_name` only returns parseable module folder names.
        let parsed = parse_module_id(&existing).expect("module folder should be parseable");
        let module_id = parsed.module_id.to_string();
        let module_name = parsed.module_name.unwrap_or_else(|| name.to_string());
        let module_dir = modules_dir.join(&existing);
        return Ok(CreateModuleResult {
            module_id,
            module_name,
            folder_name: existing,
            created: false,
            module_dir: module_dir.clone(),
            module_md: module_dir.join("module.md"),
        });
    }

    let next_id = next_module_id(&modules_dir)?;
    let folder = format!("{next_id}_{name}");
    let module_dir = modules_dir.join(&folder);
    ito_common::io::create_dir_all_std(&module_dir)?;

    let title = to_title_case(name);
    let md = generate_module_content(
        &title,
        Some("<!-- Describe the purpose of this module/epic -->"),
        &scope,
        &depends_on,
        &[],
    );
    let module_md = module_dir.join("module.md");
    ito_common::io::write_std(&module_md, md)?;

    Ok(CreateModuleResult {
        module_id: next_id,
        module_name: name.to_string(),
        folder_name: folder,
        created: true,
        module_dir,
        module_md,
    })
}

/// Create a new change directory and update the module's `module.md` checklist.
pub fn create_change(
    ito_path: &Path,
    name: &str,
    schema: &str,
    module: Option<&str>,
    description: Option<&str>,
) -> Result<CreateChangeResult, CreateError> {
    let name = name.trim();
    validate_change_name(name)?;

    let modules_dir = paths::modules_dir(ito_path);
    let module_id = module
        .and_then(|m| parse_module_id(m).ok().map(|p| p.module_id.to_string()))
        .unwrap_or_else(|| "000".to_string());

    // Ensure module exists (create ungrouped if missing).
    if !modules_dir.exists() {
        ito_common::io::create_dir_all_std(&modules_dir)?;
    }
    if !module_exists(&modules_dir, &module_id) {
        if module_id == "000" {
            create_ungrouped_module(ito_path)?;
        } else {
            return Err(CreateError::ModuleNotFound(module_id));
        }
    }

    let next_num = allocate_next_change_number(ito_path, &module_id)?;
    let folder = format!("{module_id}-{next_num:02}_{name}");

    let changes_dir = paths::changes_dir(ito_path);
    ito_common::io::create_dir_all_std(&changes_dir)?;
    let change_dir = changes_dir.join(&folder);
    if change_dir.exists() {
        return Err(CreateError::ChangeAlreadyExists(folder));
    }
    ito_common::io::create_dir_all_std(&change_dir)?;

    write_change_metadata(&change_dir, schema)?;

    if let Some(desc) = description {
        // Match TS: README header uses the change id, not the raw name.
        let readme = format!("# {folder}\n\n{desc}\n");
        ito_common::io::write_std(&change_dir.join("README.md"), readme)?;
    }

    add_change_to_module(ito_path, &module_id, &folder)?;

    Ok(CreateChangeResult {
        change_id: folder,
        change_dir,
    })
}

fn write_change_metadata(change_dir: &Path, schema: &str) -> Result<(), CreateError> {
    let created = Utc::now().format("%Y-%m-%d").to_string();
    let content = format!("schema: {schema}\ncreated: {created}\n");
    ito_common::io::write_std(&change_dir.join(".ito.yaml"), content)?;
    Ok(())
}

fn allocate_next_change_number(ito_path: &Path, module_id: &str) -> Result<u32, CreateError> {
    // Lock file + JSON state mirrors TS implementation.
    let state_dir = ito_path.join("workflows").join(".state");
    ito_common::io::create_dir_all_std(&state_dir)?;
    let lock_path = state_dir.join("change-allocations.lock");
    let state_path = state_dir.join("change-allocations.json");

    let lock = acquire_lock(&lock_path)?;
    let mut state: AllocationState = if state_path.exists() {
        serde_json::from_str(&ito_common::io::read_to_string_std(&state_path)?)?
    } else {
        AllocationState::default()
    };

    let mut max_seen: u32 = 0;
    let changes_dir = paths::changes_dir(ito_path);
    max_seen = max_seen.max(max_change_num_in_dir(&changes_dir, module_id));
    max_seen = max_seen.max(max_change_num_in_archived_change_dirs(
        &paths::changes_archive_dir(ito_path),
        module_id,
    ));
    max_seen = max_seen.max(max_change_num_in_archived_change_dirs(
        &paths::archive_changes_dir(ito_path),
        module_id,
    ));

    max_seen = max_seen.max(max_change_num_in_module_md(ito_path, module_id)?);
    if let Some(ms) = state.modules.get(module_id) {
        max_seen = max_seen.max(ms.last_change_num);
    }

    let next = max_seen + 1;
    let updated_at = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
    state.modules.insert(
        module_id.to_string(),
        ModuleAllocationState {
            last_change_num: next,
            updated_at,
        },
    );

    ito_common::io::write_std(&state_path, serde_json::to_string_pretty(&state)?)?;

    drop(lock);
    let _ = fs::remove_file(&lock_path);

    Ok(next)
}

fn acquire_lock(path: &Path) -> Result<fs::File, CreateError> {
    for _ in 0..10 {
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
        {
            Ok(f) => return Ok(f),
            Err(_) => thread::sleep(Duration::from_millis(50)),
        }
    }
    // final attempt with the original error
    Ok(fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?)
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct AllocationState {
    #[serde(default)]
    modules: HashMap<String, ModuleAllocationState>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ModuleAllocationState {
    last_change_num: u32,
    updated_at: String,
}

fn max_change_num_in_dir(dir: &Path, module_id: &str) -> u32 {
    let mut max_seen = 0;
    let fs = StdFs;
    let Ok(entries) = ito_domain::discovery::list_dir_names(&fs, dir) else {
        return 0;
    };
    for name in entries {
        if name == "archive" {
            continue;
        }
        if let Ok(parsed) = parse_change_id(&name)
            && parsed.module_id.as_str() == module_id
            && let Ok(n) = parsed.change_num.parse::<u32>()
        {
            max_seen = max_seen.max(n);
        }
    }
    max_seen
}

fn max_change_num_in_archived_change_dirs(archive_dir: &Path, module_id: &str) -> u32 {
    let mut max_seen = 0;
    let fs = StdFs;
    let Ok(entries) = ito_domain::discovery::list_dir_names(&fs, archive_dir) else {
        return 0;
    };
    for name in entries {
        // archived dirs are like 2026-01-26-006-05_port-list-show-validate
        if name.len() <= 11 {
            continue;
        }
        // Find substring after first 11 chars date + dash
        let change_part = &name[11..];
        if let Ok(parsed) = parse_change_id(change_part)
            && parsed.module_id.as_str() == module_id
            && let Ok(n) = parsed.change_num.parse::<u32>()
        {
            max_seen = max_seen.max(n);
        }
    }
    max_seen
}

fn find_module_by_name(modules_dir: &Path, name: &str) -> Option<String> {
    let fs = StdFs;
    let Ok(entries) = ito_domain::discovery::list_dir_names(&fs, modules_dir) else {
        return None;
    };
    for folder in entries {
        if let Ok(parsed) = parse_module_id(&folder)
            && parsed.module_name.as_deref() == Some(name)
        {
            return Some(folder);
        }
    }
    None
}

fn module_exists(modules_dir: &Path, module_id: &str) -> bool {
    let fs = StdFs;
    let Ok(entries) = ito_domain::discovery::list_dir_names(&fs, modules_dir) else {
        return false;
    };
    for folder in entries {
        if let Ok(parsed) = parse_module_id(&folder)
            && parsed.module_id.as_str() == module_id
        {
            return true;
        }
    }
    false
}

fn next_module_id(modules_dir: &Path) -> Result<String, CreateError> {
    let mut max_seen: u32 = 0;
    let fs = StdFs;
    if let Ok(entries) = ito_domain::discovery::list_dir_names(&fs, modules_dir) {
        for folder in entries {
            if let Ok(parsed) = parse_module_id(&folder)
                && let Ok(n) = parsed.module_id.as_str().parse::<u32>()
            {
                max_seen = max_seen.max(n);
            }
        }
    }
    Ok(format!("{n:03}", n = max_seen + 1))
}

fn validate_change_name(name: &str) -> Result<(), CreateError> {
    // Mirrors `src/utils/change-utils.ts` validateChangeName.
    if name.is_empty() {
        return Err(CreateError::InvalidChangeName(
            "Change name cannot be empty".to_string(),
        ));
    }
    if name.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(CreateError::InvalidChangeName(
            "Change name must be lowercase (use kebab-case)".to_string(),
        ));
    }
    if name.chars().any(|c| c.is_whitespace()) {
        return Err(CreateError::InvalidChangeName(
            "Change name cannot contain spaces (use hyphens instead)".to_string(),
        ));
    }
    if name.contains('_') {
        return Err(CreateError::InvalidChangeName(
            "Change name cannot contain underscores (use hyphens instead)".to_string(),
        ));
    }
    if name.starts_with('-') {
        return Err(CreateError::InvalidChangeName(
            "Change name cannot start with a hyphen".to_string(),
        ));
    }
    if name.ends_with('-') {
        return Err(CreateError::InvalidChangeName(
            "Change name cannot end with a hyphen".to_string(),
        ));
    }
    if name.contains("--") {
        return Err(CreateError::InvalidChangeName(
            "Change name cannot contain consecutive hyphens".to_string(),
        ));
    }
    if name
        .chars()
        .any(|c| !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'))
    {
        return Err(CreateError::InvalidChangeName(
            "Change name can only contain lowercase letters, numbers, and hyphens".to_string(),
        ));
    }
    if name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        return Err(CreateError::InvalidChangeName(
            "Change name must start with a letter".to_string(),
        ));
    }

    // Structural check: ^[a-z][a-z0-9]*(-[a-z0-9]+)*$
    let mut parts = name.split('-');
    let Some(first) = parts.next() else {
        return Err(CreateError::InvalidChangeName(
            "Change name must follow kebab-case convention (e.g., add-auth, refactor-db)"
                .to_string(),
        ));
    };
    if first.is_empty() {
        return Err(CreateError::InvalidChangeName(
            "Change name must follow kebab-case convention (e.g., add-auth, refactor-db)"
                .to_string(),
        ));
    }
    let mut chars = first.chars();
    if !chars.next().is_some_and(|c| c.is_ascii_lowercase()) {
        return Err(CreateError::InvalidChangeName(
            "Change name must follow kebab-case convention (e.g., add-auth, refactor-db)"
                .to_string(),
        ));
    }
    if chars.any(|c| !(c.is_ascii_lowercase() || c.is_ascii_digit())) {
        return Err(CreateError::InvalidChangeName(
            "Change name must follow kebab-case convention (e.g., add-auth, refactor-db)"
                .to_string(),
        ));
    }
    for part in parts {
        if part.is_empty() {
            return Err(CreateError::InvalidChangeName(
                "Change name must follow kebab-case convention (e.g., add-auth, refactor-db)"
                    .to_string(),
            ));
        }
        if part
            .chars()
            .any(|c| !(c.is_ascii_lowercase() || c.is_ascii_digit()))
        {
            return Err(CreateError::InvalidChangeName(
                "Change name must follow kebab-case convention (e.g., add-auth, refactor-db)"
                    .to_string(),
            ));
        }
    }

    Ok(())
}

fn to_title_case(kebab: &str) -> String {
    kebab
        .split(|c: char| c == '-' || c == '_' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .map(|w| {
            let mut cs = w.chars();
            match cs.next() {
                None => String::new(),
                Some(first) => {
                    let mut out = String::new();
                    out.push(first.to_ascii_uppercase());
                    out.push_str(&cs.as_str().to_ascii_lowercase());
                    out
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Debug, Clone)]
struct ModuleChange {
    id: String,
    completed: bool,
    planned: bool,
}

fn add_change_to_module(
    ito_path: &Path,
    module_id: &str,
    change_id: &str,
) -> Result<(), CreateError> {
    let modules_dir = paths::modules_dir(ito_path);
    let module_folder = find_module_by_id(&modules_dir, module_id)
        .ok_or_else(|| CreateError::ModuleNotFound(module_id.to_string()))?;
    let module_md = modules_dir.join(&module_folder).join("module.md");
    let existing = ito_common::io::read_to_string_std(&module_md)?;

    let title = extract_title(&existing)
        .or_else(|| module_folder.split('_').nth(1).map(to_title_case))
        .unwrap_or_else(|| "Module".to_string());
    let purpose = extract_section(&existing, "Purpose")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let scope = parse_bullets(&extract_section(&existing, "Scope").unwrap_or_default());
    let depends_on = parse_bullets(&extract_section(&existing, "Depends On").unwrap_or_default());
    let mut changes = parse_changes(&extract_section(&existing, "Changes").unwrap_or_default());

    if !changes.iter().any(|c| c.id == change_id) {
        changes.push(ModuleChange {
            id: change_id.to_string(),
            completed: false,
            planned: false,
        });
    }

    let md = generate_module_content(&title, purpose.as_deref(), &scope, &depends_on, &changes);
    ito_common::io::write_std(&module_md, md)?;
    Ok(())
}

fn find_module_by_id(modules_dir: &Path, module_id: &str) -> Option<String> {
    let fs = StdFs;
    let Ok(entries) = ito_domain::discovery::list_dir_names(&fs, modules_dir) else {
        return None;
    };
    for folder in entries {
        if let Ok(parsed) = parse_module_id(&folder)
            && parsed.module_id.as_str() == module_id
        {
            return Some(folder);
        }
    }
    None
}

fn max_change_num_in_module_md(ito_path: &Path, module_id: &str) -> Result<u32, CreateError> {
    let modules_dir = paths::modules_dir(ito_path);
    let Some(folder) = find_module_by_id(&modules_dir, module_id) else {
        return Ok(0);
    };
    let module_md = modules_dir.join(folder).join("module.md");
    let content = ito_common::io::read_to_string_or_default(&module_md);
    let mut max_seen: u32 = 0;
    for token in content.split_whitespace() {
        if let Ok(parsed) = parse_change_id(
            token.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '-' && c != '_'),
        ) && parsed.module_id.as_str() == module_id
            && let Ok(n) = parsed.change_num.parse::<u32>()
        {
            max_seen = max_seen.max(n);
        }
    }
    Ok(max_seen)
}

fn extract_title(markdown: &str) -> Option<String> {
    for line in markdown.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("# ") {
            return Some(rest.trim().to_string());
        }
    }
    None
}

fn extract_section(markdown: &str, header: &str) -> Option<String> {
    let needle = format!("## {header}");
    let mut in_section = false;
    let mut out: Vec<&str> = Vec::new();
    for line in markdown.lines() {
        if line.trim() == needle {
            in_section = true;
            continue;
        }
        if in_section {
            if line.trim_start().starts_with("## ") {
                break;
            }
            out.push(line);
        }
    }
    if !in_section {
        return None;
    }
    Some(out.join("\n"))
}

fn parse_bullets(section: &str) -> Vec<String> {
    let mut items = Vec::new();
    for line in section.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("- ").or_else(|| t.strip_prefix("* ")) {
            let s = rest.trim();
            if !s.is_empty() {
                items.push(s.to_string());
            }
        }
    }
    items
}

fn parse_changes(section: &str) -> Vec<ModuleChange> {
    let mut out = Vec::new();
    for line in section.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("- [") {
            // - [x] id (planned)
            if rest.len() < 3 {
                continue;
            }
            let checked = rest.chars().next().unwrap_or(' ');
            let completed = checked == 'x' || checked == 'X';
            let after = rest[3..].trim();
            let mut parts = after.split_whitespace();
            let Some(id) = parts.next() else {
                continue;
            };
            let planned = after.contains("(planned)");
            out.push(ModuleChange {
                id: id.to_string(),
                completed,
                planned,
            });
            continue;
        }
        if let Some(rest) = t.strip_prefix("- ").or_else(|| t.strip_prefix("* ")) {
            let rest = rest.trim();
            if rest.is_empty() {
                continue;
            }
            let id = rest.split_whitespace().next().unwrap_or("");
            if id.is_empty() {
                continue;
            }
            let planned = rest.contains("(planned)");
            out.push(ModuleChange {
                id: id.to_string(),
                completed: false,
                planned,
            });
        }
    }
    out
}

fn generate_module_content<T: AsRef<str>>(
    title: &str,
    purpose: Option<&str>,
    scope: &[T],
    depends_on: &[T],
    changes: &[ModuleChange],
) -> String {
    let purpose = purpose
        .map(|s| s.to_string())
        .unwrap_or_else(|| "<!-- Describe the purpose of this module/epic -->".to_string());
    let scope_section = if scope.is_empty() {
        "<!-- List the scope of this module -->".to_string()
    } else {
        scope
            .iter()
            .map(|s| format!("- {}", s.as_ref()))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let changes_section = if changes.is_empty() {
        "<!-- Changes will be listed here as they are created -->".to_string()
    } else {
        changes
            .iter()
            .map(|c| {
                let check = if c.completed { "x" } else { " " };
                let planned = if c.planned { " (planned)" } else { "" };
                format!("- [{check}] {}{planned}", c.id)
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    // Match TS formatting (generateModuleContent):
    // - No blank line between section header and content
    // - Omit "Depends On" section when empty
    let mut out = String::new();
    out.push_str(&format!("# {title}\n\n"));

    out.push_str("## Purpose\n");
    out.push_str(&purpose);
    out.push_str("\n\n");

    out.push_str("## Scope\n");
    out.push_str(&scope_section);
    out.push_str("\n\n");

    if !depends_on.is_empty() {
        let depends_section = depends_on
            .iter()
            .map(|s| format!("- {}", s.as_ref()))
            .collect::<Vec<_>>()
            .join("\n");
        out.push_str("## Depends On\n");
        out.push_str(&depends_section);
        out.push_str("\n\n");
    }

    out.push_str("## Changes\n");
    out.push_str(&changes_section);
    out.push('\n');
    out
}

fn create_ungrouped_module(ito_path: &Path) -> Result<(), CreateError> {
    let modules_dir = paths::modules_dir(ito_path);
    ito_common::io::create_dir_all_std(&modules_dir)?;
    let dir = modules_dir.join("000_ungrouped");
    ito_common::io::create_dir_all_std(&dir)?;
    let empty: [&str; 0] = [];
    let md = generate_module_content(
        "Ungrouped",
        Some("Changes that do not belong to a specific module."),
        &["*"],
        &empty,
        &[],
    );
    ito_common::io::write_std(&dir.join("module.md"), md)?;
    Ok(())
}
