//! Change repository port definitions.

use super::{Change, ChangeSummary};
use crate::errors::DomainResult;

/// Lifecycle filter for change repository queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeLifecycleFilter {
    /// Only active (non-archived) changes.
    Active,
    /// Only archived changes.
    Archived,
    /// Both active and archived changes.
    All,
}

impl ChangeLifecycleFilter {
    /// Return `true` when active changes are included.
    pub fn includes_active(self) -> bool {
        matches!(
            self,
            ChangeLifecycleFilter::Active | ChangeLifecycleFilter::All
        )
    }

    /// Return `true` when archived changes are included.
    pub fn includes_archived(self) -> bool {
        matches!(
            self,
            ChangeLifecycleFilter::Archived | ChangeLifecycleFilter::All
        )
    }

    /// Render the filter as a lowercase string (for API usage).
    pub fn as_str(self) -> &'static str {
        match self {
            ChangeLifecycleFilter::Active => "active",
            ChangeLifecycleFilter::Archived => "archived",
            ChangeLifecycleFilter::All => "all",
        }
    }

    /// Parse a lowercase lifecycle filter string.
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_lowercase().as_str() {
            "active" => Some(ChangeLifecycleFilter::Active),
            "archived" => Some(ChangeLifecycleFilter::Archived),
            "all" => Some(ChangeLifecycleFilter::All),
            _ => None,
        }
    }
}

impl Default for ChangeLifecycleFilter {
    fn default() -> Self {
        ChangeLifecycleFilter::Active
    }
}

/// Deterministic resolution result for a change target input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeTargetResolution {
    /// Exactly one canonical change id matched.
    Unique(String),
    /// Multiple canonical change ids matched the target.
    Ambiguous(Vec<String>),
    /// No changes matched the target.
    NotFound,
}

/// Options for resolving a change target.
#[derive(Debug, Clone, Copy, Default)]
pub struct ResolveTargetOptions {
    /// Lifecycle filter to apply when resolving targets.
    pub lifecycle: ChangeLifecycleFilter,
}

/// Port for accessing change data.
///
/// Domain and adapters should depend on this interface rather than concrete
/// storage details.
pub trait ChangeRepository {
    /// Resolve an input change target into a canonical change id.
    fn resolve_target(&self, input: &str) -> ChangeTargetResolution {
        self.resolve_target_with_options(input, ResolveTargetOptions::default())
    }

    /// Resolve an input change target into a canonical change id using options.
    fn resolve_target_with_options(
        &self,
        input: &str,
        options: ResolveTargetOptions,
    ) -> ChangeTargetResolution;

    /// Return best-effort suggestions for a change target.
    fn suggest_targets(&self, input: &str, max: usize) -> Vec<String>;

    /// Check if a change exists (active lifecycle only).
    fn exists(&self, id: &str) -> bool;

    /// Check if a change exists with a lifecycle filter.
    fn exists_with_filter(&self, id: &str, filter: ChangeLifecycleFilter) -> bool;

    /// Get a full change with all artifacts loaded (active lifecycle only).
    fn get(&self, id: &str) -> DomainResult<Change> {
        self.get_with_filter(id, ChangeLifecycleFilter::Active)
    }

    /// Get a full change with all artifacts loaded, scoped by lifecycle.
    fn get_with_filter(&self, id: &str, filter: ChangeLifecycleFilter) -> DomainResult<Change>;

    /// List all active changes as summaries (lightweight).
    fn list(&self) -> DomainResult<Vec<ChangeSummary>> {
        self.list_with_filter(ChangeLifecycleFilter::Active)
    }

    /// List changes as summaries with a lifecycle filter.
    fn list_with_filter(&self, filter: ChangeLifecycleFilter) -> DomainResult<Vec<ChangeSummary>>;

    /// List active changes belonging to a specific module.
    fn list_by_module(&self, module_id: &str) -> DomainResult<Vec<ChangeSummary>> {
        self.list_by_module_with_filter(module_id, ChangeLifecycleFilter::Active)
    }

    /// List changes belonging to a specific module with a lifecycle filter.
    fn list_by_module_with_filter(
        &self,
        module_id: &str,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>>;

    /// List active changes with incomplete tasks.
    fn list_incomplete(&self) -> DomainResult<Vec<ChangeSummary>> {
        self.list_incomplete_with_filter(ChangeLifecycleFilter::Active)
    }

    /// List changes with incomplete tasks and a lifecycle filter.
    fn list_incomplete_with_filter(
        &self,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>>;

    /// List active changes with all tasks complete.
    fn list_complete(&self) -> DomainResult<Vec<ChangeSummary>> {
        self.list_complete_with_filter(ChangeLifecycleFilter::Active)
    }

    /// List changes with all tasks complete and a lifecycle filter.
    fn list_complete_with_filter(
        &self,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>>;

    /// Get a summary for a specific active change (lightweight).
    fn get_summary(&self, id: &str) -> DomainResult<ChangeSummary> {
        self.get_summary_with_filter(id, ChangeLifecycleFilter::Active)
    }

    /// Get a summary for a specific change (lightweight) with a lifecycle filter.
    fn get_summary_with_filter(
        &self,
        id: &str,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<ChangeSummary>;
}
