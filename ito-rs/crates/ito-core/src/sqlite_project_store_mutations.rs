use super::*;

pub(super) struct SqliteTaskMutationService {
    pub(super) conn: Arc<Mutex<Connection>>,
    pub(super) org: String,
    pub(super) repo: String,
}

impl SqliteTaskMutationService {
    fn load_current_markdown(&self, change_id: &str) -> TaskMutationServiceResult<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT tasks_md FROM changes WHERE org = ?1 AND repo = ?2 AND change_id = ?3")
            .map_err(|err| {
                task_mutation_error_from_core(CoreError::sqlite(format!(
                    "preparing task mutation query: {err}"
                )))
            })?;
        let result: rusqlite::Result<Option<String>> = stmt
            .query_row(rusqlite::params![&self.org, &self.repo, change_id], |row| {
                row.get(0)
            });
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

    fn store_markdown(&self, change_id: &str, markdown: &str) -> TaskMutationServiceResult<()> {
        let conn = self.conn.lock().unwrap();
        let updated = conn
            .execute(
                "UPDATE changes SET tasks_md = ?1, updated_at = ?2 WHERE org = ?3 AND repo = ?4 AND change_id = ?5",
                rusqlite::params![
                    markdown,
                    Utc::now().to_rfc3339(),
                    &self.org,
                    &self.repo,
                    change_id,
                ],
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

    fn mutate<F>(&self, change_id: &str, op: F) -> TaskMutationServiceResult<TaskMutationResult>
    where
        F: FnOnce(&str) -> Result<crate::tasks::TaskMutationOutcome, CoreError>,
    {
        let markdown = self.load_current_markdown(change_id)?;
        let tasks = markdown.ok_or_else(|| {
            ito_domain::tasks::TaskMutationError::not_found(format!(
                "No backend tasks found for \"{change_id}\". Run \"ito tasks init {change_id}\" first."
            ))
        })?;
        let outcome = op(&tasks).map_err(task_mutation_error_from_core)?;
        self.store_markdown(change_id, &outcome.updated_content)?;
        Ok(TaskMutationResult {
            change_id: change_id.to_string(),
            task: outcome.task,
            revision: None,
        })
    }
}

impl TaskMutationService for SqliteTaskMutationService {
    fn load_tasks_markdown(&self, change_id: &str) -> TaskMutationServiceResult<Option<String>> {
        self.load_current_markdown(change_id)
    }

    fn init_tasks(&self, change_id: &str) -> TaskMutationServiceResult<TaskInitResult> {
        let current = self.load_current_markdown(change_id)?;
        if current.is_some() {
            return Ok(TaskInitResult {
                change_id: change_id.to_string(),
                path: None,
                existed: true,
                revision: None,
            });
        }
        let contents = enhanced_tasks_template(change_id, chrono::Local::now());
        self.store_markdown(change_id, &contents)?;
        Ok(TaskInitResult {
            change_id: change_id.to_string(),
            path: None,
            existed: false,
            revision: None,
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
