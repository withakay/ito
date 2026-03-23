//! SQLite-backed [`BackendProjectStore`] proof-of-concept implementation.
//!
//! Stores project data (changes, modules, tasks) in a single SQLite database
//! keyed by `{org}/{repo}`. This is a proof-of-concept demonstrating the
//! storage abstraction — it stores serialized markdown content as blobs
//! rather than fully normalized relational data.
//!
//! Database location: configurable via `BackendSqliteConfig::db_path`, with a
//! default of `<data_dir>/sqlite/ito-backend.db`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use chrono::Utc;
use rusqlite::Connection;

use ito_common::match_::nearest_matches;
use ito_domain::backend::BackendProjectStore;
use ito_domain::changes::{
    Change, ChangeLifecycleFilter, ChangeRepository, ChangeSummary, ChangeTargetResolution,
    ResolveTargetOptions, Spec, parse_change_id, parse_module_id,
};
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::modules::{Module, ModuleRepository, ModuleSummary};
use ito_domain::specs::{SpecDocument, SpecRepository, SpecSummary};
use ito_domain::tasks::{
    TaskInitResult, TaskMutationResult, TaskMutationService, TaskMutationServiceResult,
    TaskRepository, TasksParseResult, parse_tasks_tracking_file,
};
use regex::Regex;

use crate::errors::{CoreError, CoreResult};
use crate::repository_runtime::RepositorySet;
use crate::task_mutations::task_mutation_error_from_core;
use crate::tasks::{
    apply_add_task, apply_complete_task, apply_shelve_task, apply_start_task, apply_unshelve_task,
    enhanced_tasks_template,
};

#[path = "sqlite_project_store_backend.rs"]
mod backend_store;
#[path = "sqlite_project_store_mutations.rs"]
mod task_mutations_impl;

#[path = "sqlite_project_store_repositories.rs"]
mod repositories;

use repositories::{
    SqliteChangeRepository, SqliteModuleRepository, SqliteSpecRepository, SqliteTaskRepository,
};
use task_mutations_impl::SqliteTaskMutationService;

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
    /// Optional sub-module this change belongs to (e.g., `"005.01"`).
    pub sub_module_id: Option<&'a str>,
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
    conn: Arc<Mutex<Connection>>,
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
            conn: Arc::new(Mutex::new(conn)),
        };
        store.initialize_schema()?;
        Ok(store)
    }

    /// Open an in-memory SQLite project store (for testing and integration tests).
    pub fn open_in_memory() -> Result<Self, CoreError> {
        let conn = Connection::open_in_memory()
            .map_err(|e| CoreError::sqlite(format!("opening in-memory database: {e}")))?;
        let store = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        store.initialize_schema()?;
        Ok(store)
    }

    fn initialize_schema(&self) -> Result<(), CoreError> {
        let conn = self.lock_conn()?;
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
                sub_module_id TEXT,
                proposal TEXT,
                design TEXT,
                tasks_md TEXT,
                archived_at TEXT,
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
            );

            CREATE TABLE IF NOT EXISTS promoted_specs (
                org TEXT NOT NULL,
                repo TEXT NOT NULL,
                spec_id TEXT NOT NULL,
                markdown TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                PRIMARY KEY (org, repo, spec_id),
                FOREIGN KEY (org, repo) REFERENCES projects(org, repo)
            );",
        )
        .map_err(|e| CoreError::sqlite(format!("initializing schema: {e}")))?;

        // Migrate pre-existing databases that were created before the sub_module_id column
        // was added.  SQLite does not support IF NOT EXISTS in ALTER TABLE, so we probe
        // pragma_table_info first and only run the migration when the column is absent.
        let has_col = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('changes') WHERE name = 'sub_module_id'",
                [],
                |row| row.get::<_, i64>(0),
            )
            .map_err(|e| CoreError::sqlite(format!("checking schema migration: {e}")))?
            > 0;
        if !has_col {
            conn.execute_batch("ALTER TABLE changes ADD COLUMN sub_module_id TEXT")
                .map_err(|e| CoreError::sqlite(format!("migrating schema (sub_module_id): {e}")))?;
        }

        Ok(())
    }

    /// Insert or update a change in the store (for seeding test data).
    pub fn upsert_change(&self, params: &UpsertChangeParams<'_>) -> Result<(), CoreError> {
        let UpsertChangeParams {
            org,
            repo,
            change_id,
            module_id,
            sub_module_id,
            proposal,
            design,
            tasks_md,
            specs,
        } = params;
        let conn = self.lock_conn()?;
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT OR REPLACE INTO changes
             (org, repo, change_id, module_id, sub_module_id, proposal, design, tasks_md, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![
                org, repo, change_id, module_id, sub_module_id, proposal, design, tasks_md, now, now
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
        let conn = self.lock_conn()?;
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

    pub(crate) fn repository_set(&self, org: &str, repo: &str) -> CoreResult<RepositorySet> {
        let conn = self.lock_conn()?;
        let changes = load_changes_from_db(&conn, org, repo)?;
        let modules = load_modules_from_db(&conn, org, repo)?;
        let tasks_data = load_tasks_data_from_db(&conn, org, repo)?;
        let specs = load_promoted_specs_from_db(&conn, org, repo)?;

        Ok(RepositorySet {
            changes: Arc::new(SqliteChangeRepository { changes }),
            modules: Arc::new(SqliteModuleRepository { modules }),
            tasks: Arc::new(SqliteTaskRepository { tasks_data }),
            task_mutations: Arc::new(SqliteTaskMutationService {
                conn: Arc::clone(&self.conn),
                org: org.to_string(),
                repo: repo.to_string(),
            }),
            specs: Arc::new(SqliteSpecRepository { specs }),
        })
    }

    fn lock_conn(&self) -> DomainResult<std::sync::MutexGuard<'_, Connection>> {
        self.conn.lock().map_err(|e| {
            DomainError::io(
                "locking sqlite connection",
                std::io::Error::other(e.to_string()),
            )
        })
    }
}

// ── Data loading helpers ───────────────────────────────────────────

fn load_changes_from_db(conn: &Connection, org: &str, repo: &str) -> DomainResult<Vec<ChangeRow>> {
    let mut stmt = conn
        .prepare(
            "SELECT change_id, module_id, sub_module_id, proposal, design, tasks_md, created_at, updated_at, archived_at
             FROM changes WHERE org = ?1 AND repo = ?2",
        )
        .map_err(|e| map_sqlite_err("preparing change query", e))?;

    let rows = stmt
        .query_map(rusqlite::params![org, repo], |row| {
            Ok(ChangeRow {
                change_id: row.get(0)?,
                module_id: row.get(1)?,
                sub_module_id: row.get(2)?,
                proposal: row.get(3)?,
                design: row.get(4)?,
                tasks_md: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
                archived_at: row.get(8)?,
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

fn load_promoted_specs_from_db(
    conn: &Connection,
    org: &str,
    repo: &str,
) -> DomainResult<Vec<SpecDocument>> {
    let mut stmt = conn
        .prepare(
            "SELECT spec_id, markdown, updated_at FROM promoted_specs WHERE org = ?1 AND repo = ?2",
        )
        .map_err(|e| map_sqlite_err("preparing promoted specs query", e))?;

    let rows = stmt
        .query_map(rusqlite::params![org, repo], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .map_err(|e| map_sqlite_err("querying promoted specs", e))?;

    let mut specs = Vec::new();
    for row in rows {
        let (id, markdown, updated_at) =
            row.map_err(|e| map_sqlite_err("reading promoted spec row", e))?;
        let last_modified = chrono::DateTime::parse_from_rfc3339(&updated_at)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        specs.push(SpecDocument {
            id: id.clone(),
            path: PathBuf::from(format!(".ito/specs/{id}/spec.md")),
            markdown,
            last_modified,
        });
    }
    specs.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(specs)
}

fn map_sqlite_err(context: &'static str, err: rusqlite::Error) -> DomainError {
    DomainError::io(context, std::io::Error::other(err.to_string()))
}

// ── In-memory row types ────────────────────────────────────────────

#[derive(Debug)]
struct ChangeRow {
    change_id: String,
    module_id: Option<String>,
    sub_module_id: Option<String>,
    proposal: Option<String>,
    design: Option<String>,
    tasks_md: Option<String>,
    #[allow(dead_code)]
    created_at: String,
    updated_at: String,
    archived_at: Option<String>,
    specs: Vec<(String, String)>,
}

#[derive(Debug)]
struct ModuleRow {
    module_id: String,
    name: String,
    description: Option<String>,
}
