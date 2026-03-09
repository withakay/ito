//! SQLite-backed [`BackendProjectStore`] proof-of-concept implementation.
//!
//! Stores project data (changes, modules, tasks) in a single SQLite database
//! keyed by `{org}/{repo}`. This is a proof-of-concept demonstrating the
//! storage abstraction — it stores serialized markdown content as blobs
//! rather than fully normalized relational data.
//!
//! Database location: configurable via `BackendSqliteConfig::db_path`, with a
//! default of `<data_dir>/sqlite/ito-backend.db`.

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use chrono::Utc;
use rusqlite::Connection;

use ito_domain::backend::BackendProjectStore;
use ito_domain::changes::{
    Change, ChangeLifecycleFilter, ChangeRepository, ChangeSummary, ChangeTargetResolution,
    ResolveTargetOptions, Spec, parse_change_id, parse_module_id,
};
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::modules::{Module, ModuleRepository, ModuleSummary};
use ito_domain::tasks::{parse_tasks_tracking_file, TaskRepository, TasksParseResult};

use crate::errors::CoreError;

/// Parameters for inserting or updating a change in the SQLite store.
pub struct UpsertChangeParams<'a> {
    /// Organization namespace.
    pub org: &'a str,
    /// Repository namespace.
    pub repo: &'a str,
    /// Change identifier.
    pub change_id: &'a str,
    /// Optional module this change belongs to.
    pub module_id: Option<&'a str>,
    /// Optional proposal markdown content.
    pub proposal: Option<&'a str>,
    /// Optional design markdown content.
    pub design: Option<&'a str>,
    /// Optional tasks.md content.
    pub tasks_md: Option<&'a str>,
    /// Spec deltas as `(capability, content)` pairs.
    pub specs: &'a [(&'a str, &'a str)],
}

/// SQLite-backed project store using a single database file.
///
/// All projects share one database, namespaced by `{org}/{repo}`.
/// The connection is protected by a `Mutex` for thread safety.
pub struct SqliteBackendProjectStore {
    conn: Mutex<Connection>,
}

impl SqliteBackendProjectStore {
    /// Open (or create) a SQLite project store at the given path.
    pub fn open(db_path: &Path) -> Result<Self, CoreError> {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| CoreError::io("creating sqlite database directory", e))?;
        }

        let conn = Connection::open(db_path)
            .map_err(|e| CoreError::sqlite(format!("opening database: {e}")))?;

        let store = Self {
            conn: Mutex::new(conn),
        };
        store.initialize_schema()?;
        Ok(store)
    }

    /// Open an in-memory SQLite project store (for testing).
    #[cfg(test)]
    pub fn open_in_memory() -> Result<Self, CoreError> {
        let conn = Connection::open_in_memory()
            .map_err(|e| CoreError::sqlite(format!("opening in-memory database: {e}")))?;
        let store = Self {
            conn: Mutex::new(conn),
        };
        store.initialize_schema()?;
        Ok(store)
    }

    fn initialize_schema(&self) -> Result<(), CoreError> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS projects (
                org TEXT NOT NULL,
                repo TEXT NOT NULL,
                created_at TEXT NOT NULL,
                PRIMARY KEY (org, repo)
            );

            CREATE TABLE IF NOT EXISTS changes (
                org TEXT NOT NULL,
                repo TEXT NOT NULL,
                change_id TEXT NOT NULL,
                module_id TEXT,
                proposal TEXT,
                design TEXT,
                tasks_md TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                PRIMARY KEY (org, repo, change_id),
                FOREIGN KEY (org, repo) REFERENCES projects(org, repo)
            );

            CREATE TABLE IF NOT EXISTS change_specs (
                org TEXT NOT NULL,
                repo TEXT NOT NULL,
                change_id TEXT NOT NULL,
                capability TEXT NOT NULL,
                content TEXT NOT NULL,
                PRIMARY KEY (org, repo, change_id, capability),
                FOREIGN KEY (org, repo, change_id)
                    REFERENCES changes(org, repo, change_id)
            );

            CREATE TABLE IF NOT EXISTS modules (
                org TEXT NOT NULL,
                repo TEXT NOT NULL,
                module_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                PRIMARY KEY (org, repo, module_id),
                FOREIGN KEY (org, repo) REFERENCES projects(org, repo)
            );",
        )
        .map_err(|e| CoreError::sqlite(format!("initializing schema: {e}")))
    }

    /// Insert or update a change in the store (for seeding test data).
    pub fn upsert_change(&self, params: &UpsertChangeParams<'_>) -> Result<(), CoreError> {
        let UpsertChangeParams {
            org,
            repo,
            change_id,
            module_id,
            proposal,
            design,
            tasks_md,
            specs,
        } = params;
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT OR REPLACE INTO changes
             (org, repo, change_id, module_id, proposal, design, tasks_md, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                org, repo, change_id, module_id, proposal, design, tasks_md, now, now
            ],
        )
        .map_err(|e| CoreError::sqlite(format!("upserting change: {e}")))?;

        // Delete old specs and insert new
        conn.execute(
            "DELETE FROM change_specs WHERE org = ?1 AND repo = ?2 AND change_id = ?3",
            rusqlite::params![org, repo, change_id],
        )
        .map_err(|e| CoreError::sqlite(format!("deleting old specs: {e}")))?;

        for (capability, content) in *specs {
            conn.execute(
                "INSERT INTO change_specs (org, repo, change_id, capability, content)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![org, repo, change_id, capability, content],
            )
            .map_err(|e| CoreError::sqlite(format!("inserting spec: {e}")))?;
        }

        Ok(())
    }

    /// Insert or update a module in the store (for seeding test data).
    pub fn upsert_module(
        &self,
        org: &str,
        repo: &str,
        module_id: &str,
        name: &str,
        description: Option<&str>,
    ) -> Result<(), CoreError> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT OR REPLACE INTO modules
             (org, repo, module_id, name, description, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![org, repo, module_id, name, description, now, now],
        )
        .map_err(|e| CoreError::sqlite(format!("upserting module: {e}")))?;

        Ok(())
    }
}

impl BackendProjectStore for SqliteBackendProjectStore {
    fn change_repository(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn ChangeRepository + Send>> {
        let conn = self.conn.lock().unwrap();
        let changes = load_changes_from_db(&conn, org, repo)?;
        Ok(Box::new(SqliteChangeRepository { changes }))
    }

    fn module_repository(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn ModuleRepository + Send>> {
        let conn = self.conn.lock().unwrap();
        let modules = load_modules_from_db(&conn, org, repo)?;
        Ok(Box::new(SqliteModuleRepository { modules }))
    }

    fn task_repository(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn TaskRepository + Send>> {
        let conn = self.conn.lock().unwrap();
        let tasks_data = load_tasks_data_from_db(&conn, org, repo)?;
        Ok(Box::new(SqliteTaskRepository { tasks_data }))
    }

    fn ensure_project(&self, org: &str, repo: &str) -> DomainResult<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT OR IGNORE INTO projects (org, repo, created_at) VALUES (?1, ?2, ?3)",
            rusqlite::params![org, repo, now],
        )
        .map_err(|e| {
            DomainError::io(
                "creating project in sqlite",
                std::io::Error::other(e.to_string()),
            )
        })?;
        Ok(())
    }

    fn project_exists(&self, org: &str, repo: &str) -> bool {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT 1 FROM projects WHERE org = ?1 AND repo = ?2",
            rusqlite::params![org, repo],
            |_| Ok(()),
        )
        .is_ok()
    }
}

// ── Data loading helpers ───────────────────────────────────────────

fn load_changes_from_db(conn: &Connection, org: &str, repo: &str) -> DomainResult<Vec<ChangeRow>> {
    let mut stmt = conn
        .prepare(
            "SELECT change_id, module_id, proposal, design, tasks_md, created_at, updated_at
             FROM changes WHERE org = ?1 AND repo = ?2",
        )
        .map_err(|e| map_sqlite_err("preparing change query", e))?;

    let rows = stmt
        .query_map(rusqlite::params![org, repo], |row| {
            Ok(ChangeRow {
                change_id: row.get(0)?,
                module_id: row.get(1)?,
                proposal: row.get(2)?,
                design: row.get(3)?,
                tasks_md: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
                specs: Vec::new(), // filled below
            })
        })
        .map_err(|e| map_sqlite_err("querying changes", e))?;

    let mut changes = Vec::new();
    for row in rows {
        let mut change = row.map_err(|e| map_sqlite_err("reading change row", e))?;

        // Load specs for this change
        let mut spec_stmt = conn
            .prepare(
                "SELECT capability, content FROM change_specs
                 WHERE org = ?1 AND repo = ?2 AND change_id = ?3",
            )
            .map_err(|e| map_sqlite_err("preparing spec query", e))?;

        let spec_rows = spec_stmt
            .query_map(rusqlite::params![org, repo, &change.change_id], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(|e| map_sqlite_err("querying specs", e))?;

        for spec_row in spec_rows {
            let (capability, content) =
                spec_row.map_err(|e| map_sqlite_err("reading spec row", e))?;
            change.specs.push((capability, content));
        }

        changes.push(change);
    }

    Ok(changes)
}

fn load_modules_from_db(conn: &Connection, org: &str, repo: &str) -> DomainResult<Vec<ModuleRow>> {
    let mut stmt = conn
        .prepare(
            "SELECT module_id, name, description FROM modules
             WHERE org = ?1 AND repo = ?2",
        )
        .map_err(|e| map_sqlite_err("preparing module query", e))?;

    let rows = stmt
        .query_map(rusqlite::params![org, repo], |row| {
            Ok(ModuleRow {
                module_id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
            })
        })
        .map_err(|e| map_sqlite_err("querying modules", e))?;

    let mut modules = Vec::new();
    for row in rows {
        modules.push(row.map_err(|e| map_sqlite_err("reading module row", e))?);
    }

    Ok(modules)
}

/// Mapping of change_id -> tasks_md content for task lookups.
fn load_tasks_data_from_db(
    conn: &Connection,
    org: &str,
    repo: &str,
) -> DomainResult<Vec<(String, Option<String>)>> {
    let mut stmt = conn
        .prepare("SELECT change_id, tasks_md FROM changes WHERE org = ?1 AND repo = ?2")
        .map_err(|e| map_sqlite_err("preparing tasks query", e))?;

    let rows = stmt
        .query_map(rusqlite::params![org, repo], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?))
        })
        .map_err(|e| map_sqlite_err("querying tasks data", e))?;

    let mut data = Vec::new();
    for row in rows {
        data.push(row.map_err(|e| map_sqlite_err("reading tasks row", e))?);
    }

    Ok(data)
}

fn map_sqlite_err(context: &'static str, err: rusqlite::Error) -> DomainError {
    DomainError::io(context, std::io::Error::other(err.to_string()))
}

// ── In-memory row types ────────────────────────────────────────────

#[derive(Debug)]
struct ChangeRow {
    change_id: String,
    module_id: Option<String>,
    proposal: Option<String>,
    design: Option<String>,
    tasks_md: Option<String>,
    #[allow(dead_code)]
    created_at: String,
    updated_at: String,
    specs: Vec<(String, String)>,
}

#[derive(Debug)]
struct ModuleRow {
    module_id: String,
    name: String,
    description: Option<String>,
}

// ── SQLite-backed repository adapters ──────────────────────────────
//
// These hold pre-loaded data from the database snapshot and implement
// the domain repository traits. They are `Send` because they own all
// their data (no references to the connection).

/// Change repository backed by pre-loaded SQLite data.
struct SqliteChangeRepository {
    changes: Vec<ChangeRow>,
}

impl ChangeRepository for SqliteChangeRepository {
    fn resolve_target_with_options(
        &self,
        input: &str,
        options: ResolveTargetOptions,
    ) -> ChangeTargetResolution {
        // SQLite stores only active changes; Archived-only queries always return NotFound.
        if !options.lifecycle.includes_active() {
            return ChangeTargetResolution::NotFound;
        }

        let input = input.trim();
        if input.is_empty() {
            return ChangeTargetResolution::NotFound;
        }

        let ids: Vec<&str> = self.changes.iter().map(|c| c.change_id.as_str()).collect();

        // 1. Exact match.
        if ids.iter().any(|&id| id == input) {
            return ChangeTargetResolution::Unique(input.to_string());
        }

        // 2. Numeric change-id match: `1-12` → `001-12_*`.
        if let Some((module_id, change_num)) = parse_change_id(input) {
            let numeric_prefix = format!("{module_id}-{change_num}");
            let with_separator = format!("{numeric_prefix}_");
            let mut numeric_matches: Vec<String> = Vec::new();
            for &id in &ids {
                if id == numeric_prefix || id.starts_with(&with_separator) {
                    numeric_matches.push(id.to_string());
                }
            }
            if !numeric_matches.is_empty() {
                if numeric_matches.len() == 1 {
                    return ChangeTargetResolution::Unique(
                        numeric_matches.into_iter().next().unwrap(),
                    );
                }
                return ChangeTargetResolution::Ambiguous(numeric_matches);
            }
        }

        // 3. Prefix match on full id string.
        let mut prefix_matches: Vec<String> = Vec::new();
        for &id in &ids {
            if id.starts_with(input) {
                prefix_matches.push(id.to_string());
            }
        }

        if prefix_matches.is_empty() {
            return ChangeTargetResolution::NotFound;
        }
        if prefix_matches.len() == 1 {
            return ChangeTargetResolution::Unique(prefix_matches.into_iter().next().unwrap());
        }
        ChangeTargetResolution::Ambiguous(prefix_matches)
    }

    fn suggest_targets(&self, input: &str, max: usize) -> Vec<String> {
        self.changes
            .iter()
            .filter(|c| c.change_id.contains(input))
            .take(max)
            .map(|c| c.change_id.clone())
            .collect()
    }

    fn exists(&self, id: &str) -> bool {
        self.exists_with_filter(id, ChangeLifecycleFilter::Active)
    }

    fn exists_with_filter(&self, id: &str, filter: ChangeLifecycleFilter) -> bool {
        if !filter.includes_active() {
            return false;
        }
        self.changes.iter().any(|c| c.change_id == id)
    }

    fn get_with_filter(&self, id: &str, filter: ChangeLifecycleFilter) -> DomainResult<Change> {
        if !filter.includes_active() {
            return Err(DomainError::not_found("change", id));
        }
        let Some(row) = self.changes.iter().find(|c| c.change_id == id) else {
            return Err(DomainError::not_found("change", id));
        };

        let tasks = row
            .tasks_md
            .as_deref()
            .map(parse_tasks_tracking_file)
            .unwrap_or_else(TasksParseResult::empty);

        let last_modified = chrono::DateTime::parse_from_rfc3339(&row.updated_at)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Ok(Change {
            id: row.change_id.clone(),
            module_id: row.module_id.clone(),
            path: PathBuf::new(),
            proposal: row.proposal.clone(),
            design: row.design.clone(),
            specs: row
                .specs
                .iter()
                .map(|(name, content)| Spec {
                    name: name.clone(),
                    content: content.clone(),
                })
                .collect(),
            tasks,
            last_modified,
        })
    }

    fn list_with_filter(&self, filter: ChangeLifecycleFilter) -> DomainResult<Vec<ChangeSummary>> {
        if !filter.includes_active() {
            return Ok(Vec::new());
        }
        let mut summaries = Vec::with_capacity(self.changes.len());
        for row in &self.changes {
            let tasks = row
                .tasks_md
                .as_deref()
                .map(parse_tasks_tracking_file)
                .unwrap_or_else(TasksParseResult::empty);

            let last_modified = chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            summaries.push(ChangeSummary {
                id: row.change_id.clone(),
                module_id: row.module_id.clone(),
                completed_tasks: tasks.progress.complete as u32,
                shelved_tasks: tasks.progress.shelved as u32,
                in_progress_tasks: tasks.progress.in_progress as u32,
                pending_tasks: tasks.progress.pending as u32,
                total_tasks: tasks.progress.total as u32,
                last_modified,
                has_proposal: row.proposal.is_some(),
                has_design: row.design.is_some(),
                has_specs: !row.specs.is_empty(),
                has_tasks: row.tasks_md.is_some(),
            });
        }
        Ok(summaries)
    }

    fn list_by_module_with_filter(
        &self,
        module_id: &str,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        let normalized_id = parse_module_id(module_id);
        let all = self.list_with_filter(filter)?;
        let mut out = Vec::new();
        for c in all {
            if c.module_id.as_deref() == Some(&normalized_id) {
                out.push(c);
            }
        }
        Ok(out)
    }

    fn list_incomplete_with_filter(
        &self,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        let all = self.list_with_filter(filter)?;
        Ok(all
            .into_iter()
            .filter(|c| c.total_tasks > 0 && c.completed_tasks < c.total_tasks)
            .collect())
    }

    fn list_complete_with_filter(
        &self,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        let all = self.list_with_filter(filter)?;
        Ok(all
            .into_iter()
            .filter(|c| c.total_tasks > 0 && c.completed_tasks >= c.total_tasks)
            .collect())
    }

    fn get_summary_with_filter(
        &self,
        id: &str,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<ChangeSummary> {
        let all = self.list_with_filter(filter)?;
        all.into_iter()
            .find(|c| c.id == id)
            .ok_or_else(|| DomainError::not_found("change", id))
    }
}

/// Module repository backed by pre-loaded SQLite data.
struct SqliteModuleRepository {
    modules: Vec<ModuleRow>,
}

impl ModuleRepository for SqliteModuleRepository {
    fn exists(&self, id: &str) -> bool {
        self.modules.iter().any(|m| m.module_id == id)
    }

    fn get(&self, id_or_name: &str) -> DomainResult<Module> {
        let Some(row) = self
            .modules
            .iter()
            .find(|m| m.module_id == id_or_name || m.name == id_or_name)
        else {
            return Err(DomainError::not_found("module", id_or_name));
        };
        Ok(Module {
            id: row.module_id.clone(),
            name: row.name.clone(),
            description: row.description.clone(),
            path: PathBuf::new(),
        })
    }

    fn list(&self) -> DomainResult<Vec<ModuleSummary>> {
        Ok(self
            .modules
            .iter()
            .map(|m| ModuleSummary {
                id: m.module_id.clone(),
                name: m.name.clone(),
                change_count: 0, // No cross-reference in PoC
            })
            .collect())
    }
}

/// Task repository backed by pre-loaded SQLite data.
struct SqliteTaskRepository {
    tasks_data: Vec<(String, Option<String>)>,
}

impl TaskRepository for SqliteTaskRepository {
    fn load_tasks(&self, change_id: &str) -> DomainResult<TasksParseResult> {
        let Some((_id, tasks_md)) = self.tasks_data.iter().find(|(id, _)| id == change_id) else {
            return Ok(TasksParseResult::empty());
        };

        let Some(md) = tasks_md else {
            return Ok(TasksParseResult::empty());
        };

        Ok(parse_tasks_tracking_file(md))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_in_memory_creates_schema() {
        let store = SqliteBackendProjectStore::open_in_memory().unwrap();
        assert!(!store.project_exists("org", "repo"));
    }

    #[test]
    fn ensure_project_creates_row() {
        let store = SqliteBackendProjectStore::open_in_memory().unwrap();
        store.ensure_project("acme", "widgets").unwrap();
        assert!(store.project_exists("acme", "widgets"));
    }

    #[test]
    fn ensure_project_is_idempotent() {
        let store = SqliteBackendProjectStore::open_in_memory().unwrap();
        store.ensure_project("acme", "widgets").unwrap();
        store.ensure_project("acme", "widgets").unwrap();
        assert!(store.project_exists("acme", "widgets"));
    }

    #[test]
    fn upsert_and_list_changes() {
        let store = SqliteBackendProjectStore::open_in_memory().unwrap();
        store.ensure_project("org", "repo").unwrap();
        store
            .upsert_change(&UpsertChangeParams {
                org: "org",
                repo: "repo",
                change_id: "001-01_my-change",
                module_id: Some("001"),
                proposal: Some("# Proposal"),
                design: None,
                tasks_md: Some("## 1. Tasks\n- [x] 1.1 Done\n- [ ] 1.2 Pending"),
                specs: &[("auth", "## ADDED\n### Requirement: Auth")],
            })
            .unwrap();

        let change_repo = store.change_repository("org", "repo").unwrap();
        let changes = change_repo.list().unwrap();
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].id, "001-01_my-change");
        assert_eq!(changes[0].module_id, Some("001".to_string()));
        assert!(changes[0].has_proposal);
        assert!(!changes[0].has_design);
        assert!(changes[0].has_specs);
    }

    #[test]
    fn get_change_returns_full_data() {
        let store = SqliteBackendProjectStore::open_in_memory().unwrap();
        store.ensure_project("org", "repo").unwrap();
        store
            .upsert_change(&UpsertChangeParams {
                org: "org",
                repo: "repo",
                change_id: "002-01_another",
                module_id: None,
                proposal: Some("# My Proposal"),
                design: Some("# Design"),
                tasks_md: None,
                specs: &[("config", "## MODIFIED")],
            })
            .unwrap();

        let change_repo = store.change_repository("org", "repo").unwrap();
        let change = change_repo.get("002-01_another").unwrap();
        assert_eq!(change.id, "002-01_another");
        assert_eq!(change.proposal, Some("# My Proposal".to_string()));
        assert_eq!(change.design, Some("# Design".to_string()));
        assert_eq!(change.specs.len(), 1);
        assert_eq!(change.specs[0].name, "config");
    }

    #[test]
    fn get_missing_change_returns_not_found() {
        let store = SqliteBackendProjectStore::open_in_memory().unwrap();
        store.ensure_project("org", "repo").unwrap();
        let change_repo = store.change_repository("org", "repo").unwrap();
        let err = change_repo.get("nonexistent").unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn upsert_and_list_modules() {
        let store = SqliteBackendProjectStore::open_in_memory().unwrap();
        store.ensure_project("org", "repo").unwrap();
        store
            .upsert_module("org", "repo", "001", "Backend", Some("Backend module"))
            .unwrap();

        let module_repo = store.module_repository("org", "repo").unwrap();
        let modules = module_repo.list().unwrap();
        assert_eq!(modules.len(), 1);
        assert_eq!(modules[0].id, "001");
        assert_eq!(modules[0].name, "Backend");
    }

    #[test]
    fn get_module_by_id() {
        let store = SqliteBackendProjectStore::open_in_memory().unwrap();
        store.ensure_project("org", "repo").unwrap();
        store
            .upsert_module("org", "repo", "001", "Backend", Some("Desc"))
            .unwrap();

        let module_repo = store.module_repository("org", "repo").unwrap();
        let module = module_repo.get("001").unwrap();
        assert_eq!(module.name, "Backend");
        assert_eq!(module.description, Some("Desc".to_string()));
    }

    #[test]
    fn task_repository_loads_tasks() {
        let store = SqliteBackendProjectStore::open_in_memory().unwrap();
        store.ensure_project("org", "repo").unwrap();
        store
            .upsert_change(&UpsertChangeParams {
                org: "org",
                repo: "repo",
                change_id: "001-01_change",
                module_id: None,
                proposal: None,
                design: None,
                tasks_md: Some("## 1. Tasks\n- [x] 1.1 Done\n- [ ] 1.2 Pending"),
                specs: &[],
            })
            .unwrap();

        let task_repo = store.task_repository("org", "repo").unwrap();
        let result = task_repo.load_tasks("001-01_change").unwrap();
        assert!(result.progress.total > 0);
    }

    #[test]
    fn task_repository_missing_change_returns_empty() {
        let store = SqliteBackendProjectStore::open_in_memory().unwrap();
        store.ensure_project("org", "repo").unwrap();
        let task_repo = store.task_repository("org", "repo").unwrap();
        let result = task_repo.load_tasks("nonexistent").unwrap();
        assert_eq!(result.progress.total, 0);
    }

    #[test]
    fn two_projects_are_isolated() {
        let store = SqliteBackendProjectStore::open_in_memory().unwrap();
        store.ensure_project("org1", "repo1").unwrap();
        store.ensure_project("org2", "repo2").unwrap();

        store
            .upsert_change(&UpsertChangeParams {
                org: "org1",
                repo: "repo1",
                change_id: "change-a",
                module_id: None,
                proposal: None,
                design: None,
                tasks_md: None,
                specs: &[],
            })
            .unwrap();
        store
            .upsert_change(&UpsertChangeParams {
                org: "org2",
                repo: "repo2",
                change_id: "change-b",
                module_id: None,
                proposal: None,
                design: None,
                tasks_md: None,
                specs: &[],
            })
            .unwrap();

        let repo1 = store.change_repository("org1", "repo1").unwrap();
        let repo2 = store.change_repository("org2", "repo2").unwrap();

        let changes1 = repo1.list().unwrap();
        let changes2 = repo2.list().unwrap();

        assert_eq!(changes1.len(), 1);
        assert_eq!(changes1[0].id, "change-a");
        assert_eq!(changes2.len(), 1);
        assert_eq!(changes2[0].id, "change-b");
    }

    #[test]
    fn store_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<SqliteBackendProjectStore>();
    }

    #[test]
    fn on_disk_database_persists() {
        let tmp = tempfile::tempdir().unwrap();
        let db_path = tmp.path().join("test.db");

        // Create and populate
        {
            let store = SqliteBackendProjectStore::open(&db_path).unwrap();
            store.ensure_project("org", "repo").unwrap();
            store
                .upsert_change(&UpsertChangeParams {
                    org: "org",
                    repo: "repo",
                    change_id: "change-1",
                    module_id: None,
                    proposal: Some("# P"),
                    design: None,
                    tasks_md: None,
                    specs: &[],
                })
                .unwrap();
        }

        // Re-open and verify
        {
            let store = SqliteBackendProjectStore::open(&db_path).unwrap();
            assert!(store.project_exists("org", "repo"));
            let repo = store.change_repository("org", "repo").unwrap();
            let changes = repo.list().unwrap();
            assert_eq!(changes.len(), 1);
            assert_eq!(changes[0].id, "change-1");
        }
    }
}
