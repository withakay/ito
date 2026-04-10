//! Creation helpers for modules and changes.
//!
//! This module contains the filesystem operations behind `ito create module`
//! and `ito create change`.
//!
//! Functions here are designed to be called by the CLI layer and return
//! structured results suitable for JSON output.

use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use ito_common::fs::StdFs;
use ito_common::id::{parse_change_id, parse_module_id, parse_sub_module_id};
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

    /// The requested sub-module id does not exist.
    #[error("Sub-module '{0}' not found")]
    SubModuleNotFound(String),

    /// Mutually exclusive flags were both provided.
    #[error("{0}")]
    MutuallyExclusive(String),

    /// A change with the same id already exists.
    #[error("Change '{0}' already exists")]
    ChangeAlreadyExists(String),

    /// A sub-module with the same name already exists under the parent module.
    #[error("Sub-module '{0}' already exists under module '{1}'")]
    DuplicateSubModuleName(String, String),

    /// All sub-module number slots (01–99) under the parent module are exhausted.
    #[error("Sub-module number exhausted under module '{0}'; maximum of 99 sub-modules allowed")]
    SubModuleNumberExhausted(String),

    /// The change-allocation lock file could not be acquired.
    #[error(
        "Cannot acquire lock: {path}\n\
         \n\
         A previous `ito create` may have been interrupted, leaving a stale lock file.\n\
         Fix: delete the lock file and retry:\n\
         \n\
         \x20 rm {path}\n"
    )]
    LockAcquireFailed {
        /// Absolute path to the lock file that blocked acquisition.
        path: String,
    },

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

#[derive(Debug, Clone)]
/// Result of creating a sub-module.
pub struct CreateSubModuleResult {
    /// Canonical sub-module id (e.g., `"024.01"`).
    pub sub_module_id: String,
    /// Sub-module name (slug).
    pub sub_module_name: String,
    /// Parent module id (e.g., `"024"`).
    pub parent_module_id: String,
    /// Path to the sub-module directory.
    pub sub_module_dir: PathBuf,
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
    description: Option<&str>,
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
        description.or(Some("<!-- Describe the purpose of this module/epic -->")),
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
///
/// When `module` is `Some`, the change is scoped to that module:
/// - The allocation namespace is the module's `NNN` identifier.
/// - The folder name uses the `NNN-NN_name` canonical form.
/// - The checklist entry is written to the module's `module.md`.
///
/// When `module` is `None`, the change is placed in the default `000` namespace.
pub fn create_change(
    ito_path: &Path,
    name: &str,
    schema: &str,
    module: Option<&str>,
    description: Option<&str>,
) -> Result<CreateChangeResult, CreateError> {
    create_change_inner(ito_path, name, schema, module, None, description)
}

/// Create a new change scoped to a sub-module.
///
/// `sub_module` must be a valid `NNN.SS` (or `NNN.SS_name`) identifier.
/// The parent module must already exist; the sub-module directory must exist
/// under `modules/NNN_<name>/sub/SS_<name>/`.
pub fn create_change_in_sub_module(
    ito_path: &Path,
    name: &str,
    schema: &str,
    sub_module: &str,
    description: Option<&str>,
) -> Result<CreateChangeResult, CreateError> {
    create_change_inner(ito_path, name, schema, None, Some(sub_module), description)
}

/// Create a new sub-module directory under an existing parent module.
///
/// Allocates the next available sub-module number, creates the `sub/SS_name/`
/// directory, and writes a `module.md` with the given title and optional
/// description.
///
/// Returns [`CreateError::ModuleNotFound`] when the parent module does not
/// exist, and [`CreateError::DuplicateSubModuleName`] when a sub-module with
/// the same name already exists under that parent.
pub fn create_sub_module(
    ito_path: &Path,
    name: &str,
    parent_module: &str,
    description: Option<&str>,
) -> Result<CreateSubModuleResult, CreateError> {
    let name = name.trim();
    // Reuse change-name validation rules: lowercase kebab-case, no underscores.
    validate_change_name(name)?;

    let modules_dir = paths::modules_dir(ito_path);

    // Resolve the parent module id.
    let parent_id = parse_module_id(parent_module)
        .ok()
        .map(|p| p.module_id.to_string())
        .unwrap_or_else(|| parent_module.to_string());

    // Parent module must exist.
    let parent_folder = find_module_by_id(&modules_dir, &parent_id)
        .ok_or_else(|| CreateError::ModuleNotFound(parent_id.clone()))?;

    let parent_dir = modules_dir.join(&parent_folder);
    let sub_dir = parent_dir.join("sub");
    ito_common::io::create_dir_all_std(&sub_dir)?;

    // Check for duplicate name.
    let fs = ito_common::fs::StdFs;
    if let Ok(entries) = ito_domain::discovery::list_dir_names(&fs, &sub_dir) {
        for entry in &entries {
            if let Some((_, entry_name)) = entry.split_once('_')
                && entry_name == name
            {
                return Err(CreateError::DuplicateSubModuleName(
                    name.to_string(),
                    parent_id.clone(),
                ));
            }
        }
    }

    // Allocate the next sub-module number.
    let next_sub_num = next_sub_module_num(&sub_dir)?;
    if next_sub_num >= 100 {
        return Err(CreateError::SubModuleNumberExhausted(parent_id));
    }
    let folder_name = format!("{next_sub_num:02}_{name}");
    let sub_module_dir = sub_dir.join(&folder_name);
    ito_common::io::create_dir_all_std(&sub_module_dir)?;

    // Canonical composite id: "NNN.SS"
    let sub_module_id = format!("{parent_id}.{next_sub_num:02}");

    // Write module.md.
    let title = to_title_case(name);
    let md = generate_module_content(
        &title,
        description.or(Some("<!-- Describe the purpose of this sub-module -->")),
        &["*"],
        &[] as &[&str],
        &[],
    );
    ito_common::io::write_std(&sub_module_dir.join("module.md"), md)?;

    Ok(CreateSubModuleResult {
        sub_module_id,
        sub_module_name: name.to_string(),
        parent_module_id: parent_id,
        sub_module_dir,
    })
}

/// Scan a `sub/` directory and return the next available sub-module number.
fn next_sub_module_num(sub_dir: &Path) -> Result<u32, CreateError> {
    let mut max_seen: u32 = 0;
    let fs = ito_common::fs::StdFs;
    if let Ok(entries) = ito_domain::discovery::list_dir_names(&fs, sub_dir) {
        for entry in entries {
            if let Some((num_str, _)) = entry.split_once('_')
                && let Ok(n) = num_str.parse::<u32>()
            {
                max_seen = max_seen.max(n);
            }
        }
    }
    Ok(max_seen + 1)
}

fn create_change_inner(
    ito_path: &Path,
    name: &str,
    schema: &str,
    module: Option<&str>,
    sub_module: Option<&str>,
    description: Option<&str>,
) -> Result<CreateChangeResult, CreateError> {
    let name = name.trim();
    validate_change_name(name)?;

    let modules_dir = paths::modules_dir(ito_path);

    // Ensure modules dir exists.
    if !modules_dir.exists() {
        ito_common::io::create_dir_all_std(&modules_dir)?;
    }

    // Resolve the allocation namespace key and folder prefix.
    // When sub_module is provided, the namespace is "NNN.SS" and the folder
    // prefix is "NNN.SS". Otherwise the namespace is the parent module id.
    let (namespace_key, folder_prefix, checklist_target) = if let Some(sm) = sub_module {
        let parsed = parse_sub_module_id(sm).map_err(|e| {
            CreateError::InvalidChangeName(format!("Invalid sub-module id '{sm}': {}", e.error))
        })?;
        let parent_id = parsed.parent_module_id.as_str().to_string();
        let sub_id = parsed.sub_module_id.as_str().to_string();

        // Parent module must exist.
        if !module_exists(&modules_dir, &parent_id) {
            return Err(CreateError::ModuleNotFound(parent_id));
        }

        // Sub-module directory must exist.
        if !sub_module_exists(&modules_dir, &parent_id, &parsed.sub_num) {
            return Err(CreateError::SubModuleNotFound(sub_id.clone()));
        }

        (
            sub_id.clone(),
            sub_id,
            ChecklistTarget::SubModule(sm.to_string()),
        )
    } else {
        let module_id = module
            .and_then(|m| parse_module_id(m).ok().map(|p| p.module_id.to_string()))
            .unwrap_or_else(|| "000".to_string());

        if !module_exists(&modules_dir, &module_id) {
            if module_id == "000" {
                create_ungrouped_module(ito_path)?;
            } else {
                return Err(CreateError::ModuleNotFound(module_id.clone()));
            }
        }

        (
            module_id.clone(),
            module_id.clone(),
            ChecklistTarget::Module(module_id),
        )
    };

    let next_num = allocate_next_change_number(ito_path, &namespace_key)?;
    let folder = format!("{folder_prefix}-{next_num:02}_{name}");

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

    match checklist_target {
        ChecklistTarget::Module(module_id) => {
            add_change_to_module(ito_path, &module_id, &folder)?;
        }
        ChecklistTarget::SubModule(sub_module_id) => {
            add_change_to_sub_module(ito_path, &sub_module_id, &folder)?;
        }
    }

    Ok(CreateChangeResult {
        change_id: folder,
        change_dir,
    })
}

/// Identifies where the checklist entry for a new change should be written.
enum ChecklistTarget {
    /// Write to the parent module's `module.md`.
    Module(String),
    /// Write to the sub-module's `module.md` (composite id like `"024.01"`).
    SubModule(String),
}

fn write_change_metadata(change_dir: &Path, schema: &str) -> Result<(), CreateError> {
    let created = Utc::now().format("%Y-%m-%d").to_string();
    let content = format!("schema: {schema}\ncreated: {created}\n");
    ito_common::io::write_std(&change_dir.join(".ito.yaml"), content)?;
    Ok(())
}

fn allocate_next_change_number(ito_path: &Path, namespace_key: &str) -> Result<u32, CreateError> {
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
    max_seen = max_seen.max(max_change_num_in_dir(&changes_dir, namespace_key));
    max_seen = max_seen.max(max_change_num_in_archived_change_dirs(
        &paths::changes_archive_dir(ito_path),
        namespace_key,
    ));
    max_seen = max_seen.max(max_change_num_in_archived_change_dirs(
        &paths::archive_changes_dir(ito_path),
        namespace_key,
    ));

    // For sub-module namespaces (NNN.SS), also scan the sub-module's module.md.
    // For plain module namespaces (NNN), scan the parent module's module.md.
    if namespace_key.contains('.') {
        max_seen = max_seen.max(max_change_num_in_sub_module_md(ito_path, namespace_key)?);
    } else {
        max_seen = max_seen.max(max_change_num_in_module_md(ito_path, namespace_key)?);
    }
    if let Some(ms) = state.modules.get(namespace_key) {
        max_seen = max_seen.max(ms.last_change_num);
    }

    let next = max_seen + 1;
    let updated_at = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
    state.modules.insert(
        namespace_key.to_string(),
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

/// Maximum age (in seconds) before a lock file is considered stale and removed.
const LOCK_STALE_SECS: u64 = 30;

fn acquire_lock(path: &Path) -> Result<fs::File, CreateError> {
    for attempt in 0..10 {
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
        {
            Ok(f) => return Ok(f),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                // Check if the lock file is stale (older than LOCK_STALE_SECS).
                if let Ok(meta) = fs::metadata(path)
                    && let Ok(modified) = meta.modified()
                    && let Ok(age) = modified.elapsed()
                    && age.as_secs() >= LOCK_STALE_SECS
                {
                    let _ = fs::remove_file(path);
                    // Retry immediately after removing stale lock.
                    continue;
                }
                // Lock is fresh — another process may hold it. Wait and retry.
                if attempt < 9 {
                    thread::sleep(Duration::from_millis(50));
                }
            }
            Err(e) => return Err(CreateError::Io(e)),
        }
    }
    // All retries exhausted — produce an actionable error.
    Err(CreateError::LockAcquireFailed {
        path: path.display().to_string(),
    })
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct AllocationState {
    #[serde(default)]
    modules: BTreeMap<String, ModuleAllocationState>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ModuleAllocationState {
    last_change_num: u32,
    updated_at: String,
}

/// Returns `true` when a parsed change id belongs to the given namespace key.
///
/// - For a plain module namespace (`"024"`): matches legacy `NNN-NN_name` changes
///   where `module_id == "024"` and there is no sub-module component.
/// - For a sub-module namespace (`"024.01"`): matches sub-module changes where
///   `sub_module_id == "024.01"`.
fn change_belongs_to_namespace(
    parsed: &ito_common::id::ParsedChangeId,
    namespace_key: &str,
) -> bool {
    if namespace_key.contains('.') {
        // Sub-module namespace: match on sub_module_id.
        parsed
            .sub_module_id
            .as_ref()
            .map(|s| s.as_str() == namespace_key)
            .unwrap_or(false)
    } else {
        // Plain module namespace: match on module_id with no sub-module component.
        parsed.module_id.as_str() == namespace_key && parsed.sub_module_id.is_none()
    }
}

fn max_change_num_in_dir(dir: &Path, namespace_key: &str) -> u32 {
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
            && change_belongs_to_namespace(&parsed, namespace_key)
            && let Ok(n) = parsed.change_num.parse::<u32>()
        {
            max_seen = max_seen.max(n);
        }
    }
    max_seen
}

fn max_change_num_in_archived_change_dirs(archive_dir: &Path, namespace_key: &str) -> u32 {
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
            && change_belongs_to_namespace(&parsed, namespace_key)
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

/// Check whether a sub-module directory exists under `modules/NNN_<name>/sub/SS_<name>/`.
fn sub_module_exists(modules_dir: &Path, parent_module_id: &str, sub_num: &str) -> bool {
    let Some(parent_folder) = find_module_by_id(modules_dir, parent_module_id) else {
        return false;
    };
    let sub_dir = modules_dir.join(&parent_folder).join("sub");
    if !sub_dir.exists() {
        return false;
    }
    let fs = StdFs;
    let Ok(entries) = ito_domain::discovery::list_dir_names(&fs, &sub_dir) else {
        return false;
    };
    let prefix = format!("{sub_num}_");
    entries.iter().any(|e| e.starts_with(&prefix))
}

/// Find the sub-module directory path for a given composite id (e.g., `"024.01"`).
///
/// Returns the path to the sub-module directory (e.g.,
/// `.ito/modules/024_backend/sub/01_auth`) if it exists.
fn find_sub_module_dir(modules_dir: &Path, sub_module_id: &str) -> Option<std::path::PathBuf> {
    let parsed = parse_sub_module_id(sub_module_id).ok()?;
    let parent_id = parsed.parent_module_id.as_str();
    let sub_num = &parsed.sub_num;

    let parent_folder = find_module_by_id(modules_dir, parent_id)?;
    let sub_dir = modules_dir.join(&parent_folder).join("sub");

    let fs = StdFs;
    let entries = ito_domain::discovery::list_dir_names(&fs, &sub_dir).ok()?;
    let prefix = format!("{sub_num}_");
    let sub_folder = entries.into_iter().find(|e| e.starts_with(&prefix))?;
    Some(sub_dir.join(sub_folder))
}

/// Add a change entry to a sub-module's `module.md` checklist.
///
/// Mirrors [`add_change_to_module`] but targets the sub-module's own
/// `module.md` at `.ito/modules/NNN_<parent>/sub/SS_<name>/module.md`.
fn add_change_to_sub_module(
    ito_path: &Path,
    sub_module_id: &str,
    change_id: &str,
) -> Result<(), CreateError> {
    let modules_dir = paths::modules_dir(ito_path);
    let sub_module_dir = find_sub_module_dir(&modules_dir, sub_module_id)
        .ok_or_else(|| CreateError::SubModuleNotFound(sub_module_id.to_string()))?;

    let module_md = sub_module_dir.join("module.md");

    // If the sub-module doesn't have a module.md yet, create a minimal one.
    let existing = if module_md.exists() {
        ito_common::io::read_to_string_std(&module_md)?
    } else {
        let parsed = parse_sub_module_id(sub_module_id).map_err(|e| {
            CreateError::InvalidChangeName(format!(
                "Invalid sub-module id '{sub_module_id}': {}",
                e.error
            ))
        })?;
        let title = to_title_case(parsed.sub_name.as_deref().unwrap_or(sub_module_id));
        generate_module_content(&title, None, &["*"], &[] as &[&str], &[])
    };

    let title = extract_title(&existing)
        .or_else(|| {
            sub_module_dir
                .file_name()
                .and_then(|n| n.to_str())
                .and_then(|n| n.split_once('_').map(|(_, name)| to_title_case(name)))
        })
        .unwrap_or_else(|| "Sub-module".to_string());
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
    changes.sort_by(|a, b| a.id.cmp(&b.id));

    let md = generate_module_content(&title, purpose.as_deref(), &scope, &depends_on, &changes);
    ito_common::io::write_std(&module_md, md)?;
    Ok(())
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
    changes.sort_by(|a, b| a.id.cmp(&b.id));

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
        if let Ok(parsed) =
            parse_change_id(token.trim_matches(|c: char| {
                !c.is_ascii_alphanumeric() && c != '-' && c != '_' && c != '.'
            }))
            && change_belongs_to_namespace(&parsed, module_id)
            && let Ok(n) = parsed.change_num.parse::<u32>()
        {
            max_seen = max_seen.max(n);
        }
    }
    Ok(max_seen)
}

/// Scan a sub-module's `module.md` for the maximum change number seen.
///
/// `sub_module_id` is the composite `NNN.SS` key (e.g., `"024.01"`).
fn max_change_num_in_sub_module_md(
    ito_path: &Path,
    sub_module_id: &str,
) -> Result<u32, CreateError> {
    let modules_dir = paths::modules_dir(ito_path);
    let Some(sub_module_dir) = find_sub_module_dir(&modules_dir, sub_module_id) else {
        return Ok(0);
    };
    let module_md = sub_module_dir.join("module.md");
    let content = ito_common::io::read_to_string_or_default(&module_md);
    let mut max_seen: u32 = 0;
    for token in content.split_whitespace() {
        if let Ok(parsed) =
            parse_change_id(token.trim_matches(|c: char| {
                !c.is_ascii_alphanumeric() && c != '-' && c != '_' && c != '.'
            }))
            && change_belongs_to_namespace(&parsed, sub_module_id)
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

#[cfg(test)]
mod create_sub_module_tests;
