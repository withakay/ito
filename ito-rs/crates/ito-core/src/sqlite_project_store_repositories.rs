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
#[path = "sqlite_project_store_repositories_tests.rs"]
mod sqlite_project_store_repositories_tests;
