use super::*;

pub(super) struct SqliteChangeRepository {
    pub(super) changes: Vec<ChangeRow>,
}

impl SqliteChangeRepository {
    fn matches_lifecycle(&self, change: &ChangeRow, filter: ChangeLifecycleFilter) -> bool {
        let is_archived = change.archived_at.is_some();
        match filter {
            ChangeLifecycleFilter::Active => !is_archived,
            ChangeLifecycleFilter::Archived => is_archived,
            ChangeLifecycleFilter::All => true,
        }
    }

    fn change_names(&self, filter: ChangeLifecycleFilter) -> Vec<String> {
        let mut names = Vec::with_capacity(self.changes.len());
        for change in &self.changes {
            if !self.matches_lifecycle(change, filter) {
                continue;
            }
            names.push(change.change_id.clone());
        }
        names.sort();
        names.dedup();
        names
    }

    fn split_canonical_change_id<'b>(&self, name: &'b str) -> Option<(String, String, &'b str)> {
        let (module_id, change_num) = parse_change_id(name)?;
        let slug = name.split_once('_').map(|(_id, s)| s).unwrap_or("");
        Some((module_id, change_num, slug))
    }

    fn tokenize_query(&self, input: &str) -> Vec<String> {
        let mut out = Vec::new();
        for part in input.split_whitespace() {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                continue;
            }
            out.push(trimmed.to_lowercase());
        }
        out
    }

    fn normalized_slug_text(&self, slug: &str) -> String {
        let mut out = String::new();
        for ch in slug.chars() {
            if ch.is_ascii_alphanumeric() {
                out.push(ch.to_ascii_lowercase());
            } else {
                out.push(' ');
            }
        }
        out
    }

    fn slug_matches_tokens(&self, slug: &str, tokens: &[String]) -> bool {
        if tokens.is_empty() {
            return false;
        }
        let text = self.normalized_slug_text(slug);
        for token in tokens {
            if !text.contains(token) {
                return false;
            }
        }
        true
    }

    fn is_numeric_module_selector(&self, input: &str) -> bool {
        let trimmed = input.trim();
        !trimmed.is_empty() && trimmed.chars().all(|ch| ch.is_ascii_digit())
    }

    fn extract_two_numbers_as_change_id(&self, input: &str) -> Option<(String, String)> {
        let re = Regex::new(r"\d+").ok()?;
        let mut parts: Vec<&str> = Vec::new();
        for m in re.find_iter(input) {
            parts.push(m.as_str());
            if parts.len() > 2 {
                return None;
            }
        }
        if parts.len() != 2 {
            return None;
        }
        let parsed = format!("{}-{}", parts[0], parts[1]);
        parse_change_id(&parsed)
    }
}

impl ChangeRepository for SqliteChangeRepository {
    fn resolve_target_with_options(
        &self,
        input: &str,
        options: ResolveTargetOptions,
    ) -> ChangeTargetResolution {
        let names = self.change_names(options.lifecycle);
        if names.is_empty() {
            return ChangeTargetResolution::NotFound;
        }

        let input = input.trim();
        if input.is_empty() {
            return ChangeTargetResolution::NotFound;
        }

        if names.iter().any(|name| name == input) {
            return ChangeTargetResolution::Unique(input.to_string());
        }

        let mut numeric_matches: BTreeSet<String> = BTreeSet::new();
        let numeric_selector =
            parse_change_id(input).or_else(|| self.extract_two_numbers_as_change_id(input));
        if let Some((module_id, change_num)) = numeric_selector {
            let numeric_prefix = format!("{module_id}-{change_num}");
            let with_separator = format!("{numeric_prefix}_");
            for name in &names {
                if name == &numeric_prefix || name.starts_with(&with_separator) {
                    numeric_matches.insert(name.clone());
                }
            }
            if !numeric_matches.is_empty() {
                let numeric_matches: Vec<String> = numeric_matches.into_iter().collect();
                if numeric_matches.len() == 1 {
                    return ChangeTargetResolution::Unique(numeric_matches[0].clone());
                }
                return ChangeTargetResolution::Ambiguous(numeric_matches);
            }
        }

        if let Some((module, query)) = input.split_once(':') {
            let query = query.trim();
            if !query.is_empty() {
                let module_id = parse_module_id(module);
                let tokens = self.tokenize_query(query);
                let mut scoped_matches: BTreeSet<String> = BTreeSet::new();
                for name in &names {
                    let Some((name_module, _name_change, slug)) =
                        self.split_canonical_change_id(name)
                    else {
                        continue;
                    };
                    if name_module != module_id {
                        continue;
                    }
                    if self.slug_matches_tokens(slug, &tokens) {
                        scoped_matches.insert(name.clone());
                    }
                }

                if scoped_matches.is_empty() {
                    return ChangeTargetResolution::NotFound;
                }
                let scoped_matches: Vec<String> = scoped_matches.into_iter().collect();
                if scoped_matches.len() == 1 {
                    return ChangeTargetResolution::Unique(scoped_matches[0].clone());
                }
                return ChangeTargetResolution::Ambiguous(scoped_matches);
            }
        }

        if self.is_numeric_module_selector(input) {
            let module_id = parse_module_id(input);
            let mut module_matches: BTreeSet<String> = BTreeSet::new();
            for name in &names {
                let Some((name_module, _name_change, _slug)) = self.split_canonical_change_id(name)
                else {
                    continue;
                };
                if name_module == module_id {
                    module_matches.insert(name.clone());
                }
            }

            if !module_matches.is_empty() {
                let module_matches: Vec<String> = module_matches.into_iter().collect();
                if module_matches.len() == 1 {
                    return ChangeTargetResolution::Unique(module_matches[0].clone());
                }
                return ChangeTargetResolution::Ambiguous(module_matches);
            }
        }

        let mut matches: BTreeSet<String> = BTreeSet::new();
        for name in &names {
            if name.starts_with(input) {
                matches.insert(name.clone());
            }
        }

        if matches.is_empty() {
            let tokens = self.tokenize_query(input);
            for name in &names {
                let Some((_module, _change, slug)) = self.split_canonical_change_id(name) else {
                    continue;
                };
                if self.slug_matches_tokens(slug, &tokens) {
                    matches.insert(name.clone());
                }
            }
        }

        if matches.is_empty() {
            return ChangeTargetResolution::NotFound;
        }

        let matches: Vec<String> = matches.into_iter().collect();
        if matches.len() == 1 {
            return ChangeTargetResolution::Unique(matches[0].clone());
        }

        ChangeTargetResolution::Ambiguous(matches)
    }

    fn suggest_targets(&self, input: &str, max: usize) -> Vec<String> {
        let input = input.trim().to_lowercase();
        if input.is_empty() || max == 0 {
            return Vec::new();
        }

        let names = self.change_names(ChangeLifecycleFilter::Active);
        let mut canonical_names: Vec<String> = Vec::new();
        for name in &names {
            if self.split_canonical_change_id(name).is_some() {
                canonical_names.push(name.clone());
            }
        }

        let mut scored: Vec<(usize, String)> = Vec::new();
        let tokens = self.tokenize_query(&input);

        for name in &canonical_names {
            let lower = name.to_lowercase();
            let mut score = 0;

            if lower.starts_with(&input) {
                score = score.max(100);
            }
            if lower.contains(&input) {
                score = score.max(80);
            }

            let Some((_module, _change, slug)) = self.split_canonical_change_id(name) else {
                continue;
            };
            if !tokens.is_empty() && self.slug_matches_tokens(slug, &tokens) {
                score = score.max(70);
            }

            if let Some((module_id, change_num)) = parse_change_id(&input) {
                let numeric_prefix = format!("{module_id}-{change_num}");
                if name.starts_with(&numeric_prefix) {
                    score = score.max(95);
                }
            }

            if score > 0 {
                scored.push((score, name.clone()));
            }
        }

        scored.sort_by(|(a_score, a_name), (b_score, b_name)| {
            b_score.cmp(a_score).then_with(|| a_name.cmp(b_name))
        });

        let mut out: Vec<String> = Vec::new();
        for (_score, name) in scored.into_iter() {
            out.push(name);
            if out.len() == max {
                break;
            }
        }

        if out.len() < max {
            let nearest = nearest_matches(&input, &canonical_names, max * 2);
            for candidate in nearest {
                if out.iter().any(|existing| existing == &candidate) {
                    continue;
                }
                out.push(candidate);
                if out.len() == max {
                    break;
                }
            }
        }

        out
    }

    fn exists(&self, id: &str) -> bool {
        self.exists_with_filter(id, ChangeLifecycleFilter::Active)
    }

    fn exists_with_filter(&self, id: &str, filter: ChangeLifecycleFilter) -> bool {
        self.changes
            .iter()
            .any(|c| c.change_id == id && self.matches_lifecycle(c, filter))
    }

    fn get_with_filter(&self, id: &str, filter: ChangeLifecycleFilter) -> DomainResult<Change> {
        let Some(row) = self
            .changes
            .iter()
            .find(|c| c.change_id == id && self.matches_lifecycle(c, filter))
        else {
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
            sub_module_id: row.sub_module_id.clone(),
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
            orchestrate: ito_domain::changes::ChangeOrchestrateMetadata::default(),
            last_modified,
        })
    }

    fn list_with_filter(&self, filter: ChangeLifecycleFilter) -> DomainResult<Vec<ChangeSummary>> {
        let mut summaries = Vec::with_capacity(self.changes.len());
        for row in &self.changes {
            if !self.matches_lifecycle(row, filter) {
                continue;
            }
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
                sub_module_id: row.sub_module_id.clone(),
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
                orchestrate: ito_domain::changes::ChangeOrchestrateMetadata::default(),
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
pub(super) struct SqliteModuleRepository {
    pub(super) modules: Vec<ModuleRow>,
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
            sub_modules: Vec::new(),
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
                sub_modules: Vec::new(),
            })
            .collect())
    }
}

/// Task repository backed by pre-loaded SQLite data.
pub(super) struct SqliteTaskRepository {
    pub(super) tasks_data: Vec<(String, Option<String>)>,
}

pub(super) struct SqliteSpecRepository {
    pub(super) specs: Vec<SpecDocument>,
}

impl SpecRepository for SqliteSpecRepository {
    fn list(&self) -> DomainResult<Vec<SpecSummary>> {
        let mut specs: Vec<SpecSummary> = self
            .specs
            .iter()
            .map(|spec| SpecSummary {
                id: spec.id.clone(),
                path: spec.path.clone(),
                last_modified: spec.last_modified,
            })
            .collect();
        specs.sort_by(|left, right| left.id.cmp(&right.id));
        Ok(specs)
    }

    fn get(&self, id: &str) -> DomainResult<SpecDocument> {
        self.specs
            .iter()
            .find(|spec| spec.id == id)
            .cloned()
            .ok_or_else(|| DomainError::not_found("spec", id))
    }
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
                sub_module_id: None,
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
                sub_module_id: None,
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
                sub_module_id: None,
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
                sub_module_id: None,
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
                sub_module_id: None,
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
                    sub_module_id: None,
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

    #[test]
    fn push_artifact_bundle_rolls_back_partial_writes_on_failure() {
        let store = SqliteBackendProjectStore::open_in_memory().unwrap();
        store.ensure_project("acme", "widgets").unwrap();
        store
            .upsert_change(&UpsertChangeParams {
                org: "acme",
                repo: "widgets",
                change_id: "025-01_atomic-push",
                module_id: Some("025"),
                sub_module_id: None,
                proposal: Some("# Original Proposal"),
                design: Some("# Original Design"),
                tasks_md: Some("## 1. Tasks\n- [ ] 1.1 Keep original"),
                specs: &[("spec-one", "## ADDED Original")],
            })
            .unwrap();

        let mut bundle = store
            .pull_artifact_bundle("acme", "widgets", "025-01_atomic-push")
            .unwrap();
        bundle.proposal = Some("# Updated Proposal".to_string());
        bundle.specs = vec![
            ("duplicate".to_string(), "## ADDED First".to_string()),
            ("duplicate".to_string(), "## ADDED Second".to_string()),
        ];

        let err = store
            .push_artifact_bundle("acme", "widgets", "025-01_atomic-push", &bundle)
            .unwrap_err();
        assert!(matches!(
            err,
            ito_domain::backend::BackendError::Other(message)
                if message.contains("UNIQUE constraint failed")
        ));

        let current = store
            .pull_artifact_bundle("acme", "widgets", "025-01_atomic-push")
            .unwrap();
        assert_eq!(current.proposal.as_deref(), Some("# Original Proposal"));
        assert_eq!(current.design.as_deref(), Some("# Original Design"));
        assert_eq!(
            current.tasks.as_deref(),
            Some("## 1. Tasks\n- [ ] 1.1 Keep original")
        );
        assert_eq!(
            current.specs,
            vec![("spec-one".to_string(), "## ADDED Original".to_string())]
        );
    }

    #[test]
    fn archive_change_rolls_back_when_spec_promotion_fails() {
        let store = SqliteBackendProjectStore::open_in_memory().unwrap();
        store.ensure_project("acme", "widgets").unwrap();
        store
            .upsert_change(&UpsertChangeParams {
                org: "acme",
                repo: "widgets",
                change_id: "025-02_atomic-archive",
                module_id: Some("025"),
                sub_module_id: None,
                proposal: Some("# Proposal"),
                design: None,
                tasks_md: Some("## 1. Tasks\n- [x] 1.1 Done"),
                specs: &[("spec-one", "## ADDED Archive me")],
            })
            .unwrap();

        {
            let conn = store.conn.lock().unwrap();
            conn.execute_batch(
                "CREATE TRIGGER fail_promoted_spec_insert
                 BEFORE INSERT ON promoted_specs
                 BEGIN
                     SELECT RAISE(ABORT, 'promoted spec insert failed');
                 END;",
            )
            .unwrap();
        }

        let err = store
            .archive_change("acme", "widgets", "025-02_atomic-archive")
            .unwrap_err();
        assert!(matches!(
            err,
            ito_domain::backend::BackendError::Other(message)
                if message.contains("promoted spec insert failed")
        ));

        let change_repo = store.change_repository("acme", "widgets").unwrap();
        assert!(
            change_repo.exists_with_filter("025-02_atomic-archive", ChangeLifecycleFilter::Active,)
        );
        assert!(
            !change_repo
                .exists_with_filter("025-02_atomic-archive", ChangeLifecycleFilter::Archived,)
        );

        let spec_repo = store.spec_repository("acme", "widgets").unwrap();
        assert!(spec_repo.list().unwrap().is_empty());
    }

    #[test]
    fn task_mutation_service_reports_poisoned_connection_without_panicking() {
        let store = SqliteBackendProjectStore::open_in_memory().unwrap();
        store.ensure_project("acme", "widgets").unwrap();
        store
            .upsert_change(&UpsertChangeParams {
                org: "acme",
                repo: "widgets",
                change_id: "025-03_poisoned-lock",
                module_id: Some("025"),
                sub_module_id: None,
                proposal: None,
                design: None,
                tasks_md: Some("## 1. Tasks\n- [ ] 1.1 Pending"),
                specs: &[],
            })
            .unwrap();

        let service = SqliteTaskMutationService {
            conn: Arc::clone(&store.conn),
            org: "acme".to_string(),
            repo: "widgets".to_string(),
        };

        let poisoned_conn = Arc::clone(&store.conn);
        let result = std::thread::spawn(move || {
            let _guard = poisoned_conn.lock().unwrap();
            panic!("poison sqlite connection mutex");
        })
        .join();
        assert!(result.is_err());

        let init_err = service.init_tasks("025-03_poisoned-lock").unwrap_err();
        assert!(matches!(
            init_err,
            ito_domain::tasks::TaskMutationError::Other(message)
                if message.contains("locking sqlite connection")
        ));

        let mutate_err = service
            .start_task("025-03_poisoned-lock", "1.1")
            .unwrap_err();
        assert!(matches!(
            mutate_err,
            ito_domain::tasks::TaskMutationError::Other(message)
                if message.contains("locking sqlite connection")
        ));
    }
}
