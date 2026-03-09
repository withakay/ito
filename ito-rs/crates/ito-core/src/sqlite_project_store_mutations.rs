use super::*;

pub(super) struct SqliteTaskMutationService {
    pub(super) conn: Arc<Mutex<Connection>>,
    pub(super) org: String,
    pub(super) repo: String,
}

impl SqliteTaskMutationService {
    fn lock_conn(&self) -> TaskMutationServiceResult<std::sync::MutexGuard<'_, Connection>> {
        self.conn.lock().map_err(|err| {
            task_mutation_error_from_core(CoreError::sqlite(format!(
                "locking sqlite connection: {err}"
            )))
        })
    }

    fn with_transaction<T, F>(&self, op: F) -> TaskMutationServiceResult<T>
    where
        F: FnOnce(&rusqlite::Transaction<'_>) -> TaskMutationServiceResult<T>,
    {
        let mut conn = self.lock_conn()?;
        let tx = conn.transaction().map_err(|err| {
            task_mutation_error_from_core(CoreError::sqlite(format!(
                "starting sqlite task mutation transaction: {err}"
            )))
        })?;
        let result = op(&tx)?;
        tx.commit().map_err(|err| {
            task_mutation_error_from_core(CoreError::sqlite(format!(
                "committing sqlite task mutation transaction: {err}"
            )))
        })?;
        Ok(result)
    }

    fn load_current_markdown_from_conn(
        conn: &Connection,
        org: &str,
        repo: &str,
        change_id: &str,
    ) -> TaskMutationServiceResult<Option<String>> {
        let mut stmt = conn
            .prepare("SELECT tasks_md FROM changes WHERE org = ?1 AND repo = ?2 AND change_id = ?3")
            .map_err(|err| {
                task_mutation_error_from_core(CoreError::sqlite(format!(
                    "preparing task mutation query: {err}"
                )))
            })?;
        let result: rusqlite::Result<Option<String>> =
            stmt.query_row(rusqlite::params![org, repo, change_id], |row| row.get(0));
        match result {
            Ok(markdown) => Ok(markdown),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                Err(ito_domain::tasks::TaskMutationError::not_found(format!(
                    "Change '{change_id}' not found"
                )))
            }
            Err(err) => Err(task_mutation_error_from_core(CoreError::sqlite(format!(
                "loading sqlite tasks markdown: {err}"
            )))),
        }
    }

    fn store_markdown_in_conn(
        conn: &Connection,
        org: &str,
        repo: &str,
        change_id: &str,
        markdown: &str,
    ) -> TaskMutationServiceResult<()> {
        let updated = conn
            .execute(
                "UPDATE changes SET tasks_md = ?1, updated_at = ?2 WHERE org = ?3 AND repo = ?4 AND change_id = ?5",
                rusqlite::params![markdown, Utc::now().to_rfc3339(), org, repo, change_id,],
            )
            .map_err(|err| {
                task_mutation_error_from_core(CoreError::sqlite(format!(
                    "updating sqlite tasks markdown: {err}"
                )))
            })?;
        if updated == 0 {
            return Err(ito_domain::tasks::TaskMutationError::not_found(format!(
                "Change '{change_id}' not found"
            )));
        }
        Ok(())
    }

    fn load_current_markdown(&self, change_id: &str) -> TaskMutationServiceResult<Option<String>> {
        let conn = self.lock_conn()?;
        Self::load_current_markdown_from_conn(&conn, &self.org, &self.repo, change_id)
    }

    fn mutate<F>(&self, change_id: &str, op: F) -> TaskMutationServiceResult<TaskMutationResult>
    where
        F: FnOnce(&str) -> Result<crate::tasks::TaskMutationOutcome, CoreError>,
    {
        self.with_transaction(|tx| {
            let markdown = Self::load_current_markdown_from_conn(tx, &self.org, &self.repo, change_id)?;
            let tasks = markdown.ok_or_else(|| {
                ito_domain::tasks::TaskMutationError::not_found(format!(
                    "No backend tasks found for \"{change_id}\". Run \"ito tasks init {change_id}\" first."
                ))
            })?;
            let outcome = op(&tasks).map_err(task_mutation_error_from_core)?;
            Self::store_markdown_in_conn(
                tx,
                &self.org,
                &self.repo,
                change_id,
                &outcome.updated_content,
            )?;
            Ok(TaskMutationResult {
                change_id: change_id.to_string(),
                task: outcome.task,
                revision: None,
            })
        })
    }
}

impl TaskMutationService for SqliteTaskMutationService {
    fn load_tasks_markdown(&self, change_id: &str) -> TaskMutationServiceResult<Option<String>> {
        self.load_current_markdown(change_id)
    }

    fn init_tasks(&self, change_id: &str) -> TaskMutationServiceResult<TaskInitResult> {
        self.with_transaction(|tx| {
            let current =
                Self::load_current_markdown_from_conn(tx, &self.org, &self.repo, change_id)?;
            if current.is_some() {
                return Ok(TaskInitResult {
                    change_id: change_id.to_string(),
                    path: None,
                    existed: true,
                    revision: None,
                });
            }
            let contents = enhanced_tasks_template(change_id, chrono::Local::now());
            Self::store_markdown_in_conn(tx, &self.org, &self.repo, change_id, &contents)?;
            Ok(TaskInitResult {
                change_id: change_id.to_string(),
                path: None,
                existed: false,
                revision: None,
            })
        })
    }

    fn start_task(
        &self,
        change_id: &str,
        task_id: &str,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        self.mutate(change_id, |tasks| {
            apply_start_task(tasks, change_id, task_id, "backend tasks")
        })
    }

    fn complete_task(
        &self,
        change_id: &str,
        task_id: &str,
        _note: Option<String>,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        self.mutate(change_id, |tasks| {
            apply_complete_task(tasks, task_id, "backend tasks")
        })
    }

    fn shelve_task(
        &self,
        change_id: &str,
        task_id: &str,
        _reason: Option<String>,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        self.mutate(change_id, |tasks| {
            apply_shelve_task(tasks, task_id, "backend tasks")
        })
    }

    fn unshelve_task(
        &self,
        change_id: &str,
        task_id: &str,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        self.mutate(change_id, |tasks| {
            apply_unshelve_task(tasks, task_id, "backend tasks")
        })
    }

    fn add_task(
        &self,
        change_id: &str,
        title: &str,
        wave: Option<u32>,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        self.mutate(change_id, |tasks| {
            apply_add_task(tasks, title, wave, "backend tasks")
        })
    }
}
