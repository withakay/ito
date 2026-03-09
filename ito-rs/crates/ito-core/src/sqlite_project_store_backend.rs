use super::*;

impl BackendProjectStore for SqliteBackendProjectStore {
    fn change_repository(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn ChangeRepository + Send>> {
        let conn = self.lock_conn()?;
        let changes = load_changes_from_db(&conn, org, repo)?;
        Ok(Box::new(SqliteChangeRepository { changes }))
    }

    fn module_repository(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn ModuleRepository + Send>> {
        let conn = self.lock_conn()?;
        let modules = load_modules_from_db(&conn, org, repo)?;
        Ok(Box::new(SqliteModuleRepository { modules }))
    }

    fn task_repository(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn TaskRepository + Send>> {
        let conn = self.lock_conn()?;
        let tasks_data = load_tasks_data_from_db(&conn, org, repo)?;
        Ok(Box::new(SqliteTaskRepository { tasks_data }))
    }

    fn task_mutation_service(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn TaskMutationService + Send>> {
        Ok(Box::new(SqliteTaskMutationService {
            conn: Arc::clone(&self.conn),
            org: org.to_string(),
            repo: repo.to_string(),
        }))
    }

    fn spec_repository(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn SpecRepository + Send>> {
        let conn = self.lock_conn()?;
        let specs = load_promoted_specs_from_db(&conn, org, repo)?;
        Ok(Box::new(SqliteSpecRepository { specs }))
    }

    fn pull_artifact_bundle(
        &self,
        org: &str,
        repo: &str,
        change_id: &str,
    ) -> Result<ito_domain::backend::ArtifactBundle, ito_domain::backend::BackendError> {
        let conn = self
            .lock_conn()
            .map_err(|err| ito_domain::backend::BackendError::Other(err.to_string()))?;
        let changes = load_changes_from_db(&conn, org, repo)
            .map_err(|err| ito_domain::backend::BackendError::Other(err.to_string()))?;
        let Some(change) = changes
            .into_iter()
            .find(|change| change.change_id == change_id)
        else {
            return Err(ito_domain::backend::BackendError::NotFound(format!(
                "change '{change_id}'"
            )));
        };

        Ok(ito_domain::backend::ArtifactBundle {
            change_id: change.change_id.clone(),
            proposal: change.proposal,
            design: change.design,
            tasks: change.tasks_md,
            specs: change.specs,
            revision: change.updated_at,
        })
    }

    fn push_artifact_bundle(
        &self,
        org: &str,
        repo: &str,
        change_id: &str,
        bundle: &ito_domain::backend::ArtifactBundle,
    ) -> Result<ito_domain::backend::PushResult, ito_domain::backend::BackendError> {
        let conn = self
            .lock_conn()
            .map_err(|err| ito_domain::backend::BackendError::Other(err.to_string()))?;
        let current_updated_at: Result<String, rusqlite::Error> = conn.query_row(
            "SELECT updated_at FROM changes WHERE org = ?1 AND repo = ?2 AND change_id = ?3",
            rusqlite::params![org, repo, change_id],
            |row| row.get(0),
        );
        let current_updated_at = match current_updated_at {
            Ok(value) => value,
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                return Err(ito_domain::backend::BackendError::NotFound(format!(
                    "change '{change_id}'"
                )));
            }
            Err(err) => {
                return Err(ito_domain::backend::BackendError::Other(err.to_string()));
            }
        };

        if !bundle.revision.trim().is_empty() && bundle.revision != current_updated_at {
            return Err(ito_domain::backend::BackendError::RevisionConflict(
                ito_domain::backend::RevisionConflict {
                    change_id: change_id.to_string(),
                    local_revision: bundle.revision.clone(),
                    server_revision: current_updated_at,
                },
            ));
        }

        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE changes SET proposal = ?1, design = ?2, tasks_md = ?3, updated_at = ?4 WHERE org = ?5 AND repo = ?6 AND change_id = ?7",
            rusqlite::params![
                bundle.proposal,
                bundle.design,
                bundle.tasks,
                now,
                org,
                repo,
                change_id,
            ],
        )
        .map_err(|err| ito_domain::backend::BackendError::Other(err.to_string()))?;

        conn.execute(
            "DELETE FROM change_specs WHERE org = ?1 AND repo = ?2 AND change_id = ?3",
            rusqlite::params![org, repo, change_id],
        )
        .map_err(|err| ito_domain::backend::BackendError::Other(err.to_string()))?;
        for (capability, markdown) in &bundle.specs {
            conn.execute(
                "INSERT INTO change_specs (org, repo, change_id, capability, content) VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![org, repo, change_id, capability, markdown],
            )
            .map_err(|err| ito_domain::backend::BackendError::Other(err.to_string()))?;
        }

        Ok(ito_domain::backend::PushResult {
            change_id: change_id.to_string(),
            new_revision: now,
        })
    }

    fn archive_change(
        &self,
        org: &str,
        repo: &str,
        change_id: &str,
    ) -> Result<ito_domain::backend::ArchiveResult, ito_domain::backend::BackendError> {
        let conn = self
            .lock_conn()
            .map_err(|err| ito_domain::backend::BackendError::Other(err.to_string()))?;
        let changes = load_changes_from_db(&conn, org, repo)
            .map_err(|err| ito_domain::backend::BackendError::Other(err.to_string()))?;
        let Some(change) = changes
            .into_iter()
            .find(|change| change.change_id == change_id)
        else {
            return Err(ito_domain::backend::BackendError::NotFound(format!(
                "change '{change_id}'"
            )));
        };

        let archived_at = Utc::now().to_rfc3339();
        for (spec_id, markdown) in &change.specs {
            conn.execute(
                "INSERT OR REPLACE INTO promoted_specs (org, repo, spec_id, markdown, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![org, repo, spec_id, markdown, archived_at],
            )
            .map_err(|err| ito_domain::backend::BackendError::Other(err.to_string()))?;
        }
        conn.execute(
            "UPDATE changes SET archived_at = ?1, updated_at = ?1 WHERE org = ?2 AND repo = ?3 AND change_id = ?4",
            rusqlite::params![archived_at, org, repo, change_id],
        )
        .map_err(|err| ito_domain::backend::BackendError::Other(err.to_string()))?;

        Ok(ito_domain::backend::ArchiveResult {
            change_id: change_id.to_string(),
            archived_at,
        })
    }

    fn ensure_project(&self, org: &str, repo: &str) -> DomainResult<()> {
        let conn = self.lock_conn()?;
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
        let Ok(conn) = self.lock_conn() else {
            return false;
        };
        conn.query_row(
            "SELECT 1 FROM projects WHERE org = ?1 AND repo = ?2",
            rusqlite::params![org, repo],
            |_| Ok(()),
        )
        .is_ok()
    }
}
