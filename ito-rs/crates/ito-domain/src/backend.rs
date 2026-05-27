//! Backend coordination port definitions.
//!
//! Traits and DTOs for backend API operations: change leases (claim/release),
//! allocation, and artifact synchronization. Implementations live in `ito-core`.

use serde::{Deserialize, Serialize};

use crate::changes::ChangeLifecycleFilter;
use crate::errors::DomainResult;

// ── Lease DTOs ──────────────────────────────────────────────────────

/// Result of a successful change lease claim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimResult {
    /// The change that was claimed.
    pub change_id: String,
    /// Identity of the lease holder.
    pub holder: String,
    /// Lease expiry as ISO-8601 timestamp, if available.
    pub expires_at: Option<String>,
}

/// Result of a lease release operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseResult {
    /// The change that was released.
    pub change_id: String,
}

/// Result of an allocation operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocateResult {
    /// The allocated change, if any work was available.
    pub claim: Option<ClaimResult>,
}

/// Conflict detail when a lease claim fails because another holder owns it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseConflict {
    /// The change that is already claimed.
    pub change_id: String,
    /// Current holder identity.
    pub holder: String,
    /// Lease expiry as ISO-8601 timestamp, if available.
    pub expires_at: Option<String>,
}

// ── Sync DTOs ───────────────────────────────────────────────────────

/// An artifact bundle pulled from the backend for a single change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactBundle {
    /// The change this bundle belongs to.
    pub change_id: String,
    /// Proposal markdown content, if present.
    pub proposal: Option<String>,
    /// Design markdown content, if present.
    pub design: Option<String>,
    /// Tasks markdown content, if present.
    pub tasks: Option<String>,
    /// Spec delta files: `(capability_name, content)` pairs.
    pub specs: Vec<(String, String)>,
    /// Backend revision identifier for optimistic concurrency.
    pub revision: String,
}

/// Result of a push operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushResult {
    /// The change whose artifacts were pushed.
    pub change_id: String,
    /// New revision after the push.
    pub new_revision: String,
}

/// Conflict detail when a push fails due to a stale revision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionConflict {
    /// The change with the conflict.
    pub change_id: String,
    /// The local revision that was sent.
    pub local_revision: String,
    /// The current server revision.
    pub server_revision: String,
}

// ── Backend error ───────────────────────────────────────────────────

/// Backend operation error category.
///
/// Adapters convert this into the appropriate layer error type.
#[derive(Debug, Clone)]
pub enum BackendError {
    /// The requested lease is held by another client.
    LeaseConflict(LeaseConflict),
    /// The push revision is stale.
    RevisionConflict(RevisionConflict),
    /// The backend is not reachable or returned a server error.
    Unavailable(String),
    /// Authentication failed (invalid or missing token).
    Unauthorized(String),
    /// The requested resource was not found.
    NotFound(String),
    /// A catch-all for unexpected errors.
    Other(String),
}

impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendError::LeaseConflict(c) => {
                write!(
                    f,
                    "change '{}' is already claimed by '{}'",
                    c.change_id, c.holder
                )
            }
            BackendError::RevisionConflict(c) => {
                write!(
                    f,
                    "revision conflict for '{}': local={}, server={}",
                    c.change_id, c.local_revision, c.server_revision
                )
            }
            BackendError::Unavailable(msg) => write!(f, "backend unavailable: {msg}"),
            BackendError::Unauthorized(msg) => write!(f, "backend auth failed: {msg}"),
            BackendError::NotFound(msg) => write!(f, "not found: {msg}"),
            BackendError::Other(msg) => write!(f, "backend error: {msg}"),
        }
    }
}

impl std::error::Error for BackendError {}

// ── Project store port ──────────────────────────────────────────────

/// Port for resolving `{org}/{repo}` to project-level repositories.
///
/// The backend server uses this trait to obtain domain repository instances
/// for a given project namespace. Implementations live in `ito-core` and
/// may be backed by the filesystem or a database.
///
/// This trait is `Send + Sync` so it can be shared across async request
/// handlers via `Arc`.
pub trait BackendProjectStore: Send + Sync {
    /// Obtain a change repository for the given project.
    fn change_repository(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn crate::changes::ChangeRepository + Send>>;

    /// Obtain a module repository for the given project.
    fn module_repository(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn crate::modules::ModuleRepository + Send>>;

    /// Obtain a task repository for the given project.
    fn task_repository(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn crate::tasks::TaskRepository + Send>>;

    /// Obtain a task mutation service for the given project.
    fn task_mutation_service(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn crate::tasks::TaskMutationService + Send>>;

    /// Obtain a promoted spec repository for the given project.
    fn spec_repository(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn crate::specs::SpecRepository + Send>>;

    /// Pull the latest artifact bundle for a change from backend-managed storage.
    fn pull_artifact_bundle(
        &self,
        org: &str,
        repo: &str,
        change_id: &str,
    ) -> Result<ArtifactBundle, BackendError>;

    /// Push an updated artifact bundle into backend-managed storage.
    fn push_artifact_bundle(
        &self,
        org: &str,
        repo: &str,
        change_id: &str,
        bundle: &ArtifactBundle,
    ) -> Result<PushResult, BackendError>;

    /// Archive a change in backend-managed storage and mirror promoted specs.
    fn archive_change(
        &self,
        org: &str,
        repo: &str,
        change_id: &str,
    ) -> Result<ArchiveResult, BackendError>;

    /// Ensure the project directory/storage structure exists.
    ///
    /// Called before first write to a project. Implementations should
    /// create whatever backing store structure is needed.
    fn ensure_project(&self, org: &str, repo: &str) -> DomainResult<()>;

    /// Check whether the project exists in the store.
    fn project_exists(&self, org: &str, repo: &str) -> bool;
}

// ── Port traits ─────────────────────────────────────────────────────

/// Port for backend lease operations (claim, release, allocate).
///
/// Implementations handle HTTP communication and token management.
/// The domain layer uses this trait to remain decoupled from transport.
pub trait BackendLeaseClient {
    /// Claim a lease on a change.
    fn claim(&self, change_id: &str) -> Result<ClaimResult, BackendError>;

    /// Release a held lease.
    fn release(&self, change_id: &str) -> Result<ReleaseResult, BackendError>;

    /// Request the backend to allocate the next available change.
    fn allocate(&self) -> Result<AllocateResult, BackendError>;
}

/// Port for backend artifact synchronization operations.
///
/// Pull retrieves the latest artifact bundle for a change. Push sends
/// local updates using optimistic concurrency (revision checks).
pub trait BackendSyncClient {
    /// Pull the latest artifact bundle for a change from the backend.
    fn pull(&self, change_id: &str) -> Result<ArtifactBundle, BackendError>;

    /// Push local artifact updates to the backend with a revision check.
    fn push(&self, change_id: &str, bundle: &ArtifactBundle) -> Result<PushResult, BackendError>;
}

/// Port for backend-backed change listing (read path).
///
/// Used by repository adapters to resolve change data from the backend
/// instead of the filesystem when backend mode is enabled.
pub trait BackendChangeReader {
    /// List all change summaries from the backend.
    fn list_changes(
        &self,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<crate::changes::ChangeSummary>>;

    /// Get a full change from the backend.
    fn get_change(
        &self,
        change_id: &str,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<crate::changes::Change>;
}

/// Port for backend-backed module listing.
///
/// Used by repository adapters to resolve module data from the backend when
/// backend mode is enabled.
pub trait BackendModuleReader {
    /// List all module summaries from the backend.
    fn list_modules(&self) -> DomainResult<Vec<crate::modules::ModuleSummary>>;

    /// Get a full module from the backend.
    fn get_module(&self, module_id: &str) -> DomainResult<crate::modules::Module>;
}

/// Port for backend-backed task reading.
///
/// Used by repository adapters to resolve task data from the backend
/// when backend mode is enabled.
pub trait BackendTaskReader {
    /// Load tasks content (raw markdown) from the backend for a change.
    fn load_tasks_content(&self, change_id: &str) -> DomainResult<Option<String>>;
}

/// Port for backend-backed promoted spec reading.
pub trait BackendSpecReader {
    /// List all promoted specs from the backend.
    fn list_specs(&self) -> DomainResult<Vec<crate::specs::SpecSummary>>;

    /// Get a promoted spec from the backend.
    fn get_spec(&self, spec_id: &str) -> DomainResult<crate::specs::SpecDocument>;
}

// ── Event ingest DTOs ──────────────────────────────────────────────

/// A batch of audit events to send to the backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBatch {
    /// Events in this batch, serialized as JSON objects.
    pub events: Vec<crate::audit::event::AuditEvent>,
    /// Client-generated idempotency key for safe retries.
    pub idempotency_key: String,
}

/// Result of a successful event ingest operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventIngestResult {
    /// Number of events accepted by the backend.
    pub accepted: usize,
    /// Number of events that were duplicates (already ingested).
    pub duplicates: usize,
}

/// Port for backend event ingestion.
///
/// Implementations handle HTTP communication to submit local audit events
/// to the backend for centralized observability.
pub trait BackendEventIngestClient {
    /// Submit a batch of audit events to the backend.
    ///
    /// The batch includes an idempotency key so retries do not produce
    /// duplicate events on the server.
    fn ingest(&self, batch: &EventBatch) -> Result<EventIngestResult, BackendError>;
}

// ── Archive DTOs ───────────────────────────────────────────────────

/// Result of marking a change as archived on the backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveResult {
    /// The change that was archived.
    pub change_id: String,
    /// Timestamp when the backend recorded the archive (ISO-8601).
    pub archived_at: String,
}

/// Port for backend archive lifecycle operations.
///
/// Marks a change as archived on the backend, making it immutable
/// for subsequent backend operations (no further writes or leases).
pub trait BackendArchiveClient {
    /// Mark a change as archived on the backend.
    ///
    /// After this call succeeds, the backend SHALL reject further
    /// write or lease operations for the change.
    fn mark_archived(&self, change_id: &str) -> Result<ArchiveResult, BackendError>;
}

#[cfg(test)]
#[path = "backend_tests.rs"]
mod backend_tests;
