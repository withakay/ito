//! Change repository port definitions.

use super::{Change, ChangeSummary};
use crate::errors::DomainResult;

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
    /// Include archived changes under `.ito/changes/archive/` as resolver candidates.
    pub include_archived: bool,
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

    /// Check if a change exists.
    fn exists(&self, id: &str) -> bool;

    /// Get a full change with all artifacts loaded.
    fn get(&self, id: &str) -> DomainResult<Change>;

    /// List all changes as summaries (lightweight).
    fn list(&self) -> DomainResult<Vec<ChangeSummary>>;

    /// List changes belonging to a specific module.
    fn list_by_module(&self, module_id: &str) -> DomainResult<Vec<ChangeSummary>>;

    /// List changes with incomplete tasks.
    fn list_incomplete(&self) -> DomainResult<Vec<ChangeSummary>>;

    /// List changes with all tasks complete.
    fn list_complete(&self) -> DomainResult<Vec<ChangeSummary>>;

    /// Get a summary for a specific change (lightweight).
    fn get_summary(&self, id: &str) -> DomainResult<ChangeSummary>;
}
