//! Audit event types and builder.
//!
//! Defines the core `AuditEvent` struct and associated enums for the
//! append-only audit log. Events are serialized as single-line JSON objects
//! (JSONL) and are never modified or deleted after creation.

use serde::{Deserialize, Serialize};

/// Current schema version. Bumped only on breaking changes.
pub const SCHEMA_VERSION: u32 = 1;

/// A single audit event recording a domain state transition.
///
/// Events are append-only: once written, they are never modified or deleted.
/// Corrections are recorded as new compensating events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditEvent {
    /// Schema version (currently 1).
    pub v: u32,
    /// UTC timestamp in RFC 3339 format with millisecond precision.
    pub ts: String,
    /// Entity type (task, change, module, wave, planning, config).
    pub entity: String,
    /// Entity identifier (task id, change id, module id, config key, etc.).
    pub entity_id: String,
    /// Scoping context — the change_id for task/wave events, None for global entities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// Operation type (e.g., status_change, create, archive).
    pub op: String,
    /// Previous state value (None for create operations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// New state value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Mutation source (cli, reconcile, ralph).
    pub actor: String,
    /// User/agent identity (e.g., @jack).
    pub by: String,
    /// Optional operation-specific metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    /// Number of equivalent adjacent events represented by this event.
    #[serde(default = "default_count", skip_serializing_if = "is_default_count")]
    pub count: u64,
    /// Session and git context for traceability.
    pub ctx: EventContext,
}

fn is_default_count(count: &u64) -> bool {
    *count <= 1
}

fn default_count() -> u64 {
    1
}

/// Known entity types for the audit log.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    /// A task within a change.
    Task,
    /// A change (proposal + specs + tasks).
    Change,
    /// A module grouping related changes.
    Module,
    /// A wave within a task plan.
    Wave,
    /// A planning entry (decision, blocker, note, etc.).
    Planning,
    /// A configuration key.
    Config,
}

impl EntityType {
    /// Returns the string representation used in event serialization.
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityType::Task => "task",
            EntityType::Change => "change",
            EntityType::Module => "module",
            EntityType::Wave => "wave",
            EntityType::Planning => "planning",
            EntityType::Config => "config",
        }
    }
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Known actor types for the audit log.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Actor {
    /// Event emitted from a normal CLI command.
    Cli,
    /// Compensating event emitted by reconciliation.
    Reconcile,
    /// Event emitted by the Ralph automation loop.
    Ralph,
}

impl Actor {
    /// Returns the string representation used in event serialization.
    pub fn as_str(&self) -> &'static str {
        match self {
            Actor::Cli => "cli",
            Actor::Reconcile => "reconcile",
            Actor::Ralph => "ralph",
        }
    }
}

impl std::fmt::Display for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Session and git context captured at event-write time.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventContext {
    /// Ito-generated UUID v4 per CLI process group.
    pub session_id: String,
    /// Optional harness session ID (from env vars).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harness_session_id: Option<String>,
    /// Current git branch name (None if detached HEAD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// Worktree name if not the main worktree.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worktree: Option<String>,
    /// Short HEAD commit hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
}

/// Information about a git worktree for multi-worktree streaming.
#[derive(Debug, Clone, PartialEq)]
pub struct WorktreeInfo {
    /// Worktree filesystem path.
    pub path: std::path::PathBuf,
    /// Branch checked out in this worktree (None if detached).
    pub branch: Option<String>,
    /// Whether this is the main worktree.
    pub is_main: bool,
}

/// An audit event tagged with its source worktree (used during streaming).
#[derive(Debug, Clone)]
pub struct TaggedAuditEvent {
    /// The audit event.
    pub event: AuditEvent,
    /// The worktree this event was read from.
    pub source: WorktreeInfo,
}

/// Operation type constants for each entity.
///
/// These are used as the `op` field in `AuditEvent`. Using constants instead
/// of free-form strings prevents typos and enables exhaustive matching.
pub mod ops {
    // Task operations
    /// Task created.
    pub const TASK_CREATE: &str = "create";
    /// Task status changed.
    pub const TASK_STATUS_CHANGE: &str = "status_change";
    /// Task added to an existing plan.
    pub const TASK_ADD: &str = "add";

    // Change operations
    /// Change created.
    pub const CHANGE_CREATE: &str = "create";
    /// Change archived.
    pub const CHANGE_ARCHIVE: &str = "archive";

    // Module operations
    /// Module created.
    pub const MODULE_CREATE: &str = "create";
    /// Change added to a module.
    pub const MODULE_CHANGE_ADDED: &str = "change_added";
    /// Change completed within a module.
    pub const MODULE_CHANGE_COMPLETED: &str = "change_completed";

    // Wave operations
    /// Wave unlocked (all predecessors complete).
    pub const WAVE_UNLOCK: &str = "unlock";

    // Planning operations
    /// Planning decision recorded.
    pub const PLANNING_DECISION: &str = "decision";
    /// Planning blocker recorded.
    pub const PLANNING_BLOCKER: &str = "blocker";
    /// Planning question recorded.
    pub const PLANNING_QUESTION: &str = "question";
    /// Planning note recorded.
    pub const PLANNING_NOTE: &str = "note";
    /// Planning focus changed.
    pub const PLANNING_FOCUS_CHANGE: &str = "focus_change";

    // Config operations
    /// Config key set.
    pub const CONFIG_SET: &str = "set";
    /// Config key unset.
    pub const CONFIG_UNSET: &str = "unset";

    // Reconciliation
    /// Reconciliation compensating event.
    pub const RECONCILED: &str = "reconciled";
}

/// Builder for constructing `AuditEvent` instances.
///
/// Auto-populates `v` (schema version), `ts` (UTC now), and enforces
/// required fields at build time.
pub struct AuditEventBuilder {
    entity: Option<EntityType>,
    entity_id: Option<String>,
    scope: Option<String>,
    op: Option<String>,
    from: Option<String>,
    to: Option<String>,
    actor: Option<Actor>,
    by: Option<String>,
    meta: Option<serde_json::Value>,
    ctx: Option<EventContext>,
}

impl AuditEventBuilder {
    /// Create a new builder with defaults.
    pub fn new() -> Self {
        Self {
            entity: None,
            entity_id: None,
            scope: None,
            op: None,
            from: None,
            to: None,
            actor: None,
            by: None,
            meta: None,
            ctx: None,
        }
    }

    /// Set the entity type.
    pub fn entity(mut self, entity: EntityType) -> Self {
        self.entity = Some(entity);
        self
    }

    /// Set the entity identifier.
    pub fn entity_id(mut self, id: impl Into<String>) -> Self {
        self.entity_id = Some(id.into());
        self
    }

    /// Set the scope (change_id for task/wave events).
    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }

    /// Set the operation type.
    pub fn op(mut self, op: impl Into<String>) -> Self {
        self.op = Some(op.into());
        self
    }

    /// Set the previous state value.
    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Set the new state value.
    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    /// Set the actor.
    pub fn actor(mut self, actor: Actor) -> Self {
        self.actor = Some(actor);
        self
    }

    /// Set the user identity.
    pub fn by(mut self, by: impl Into<String>) -> Self {
        self.by = Some(by.into());
        self
    }

    /// Set optional metadata.
    pub fn meta(mut self, meta: serde_json::Value) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Set the event context.
    pub fn ctx(mut self, ctx: EventContext) -> Self {
        self.ctx = Some(ctx);
        self
    }

    /// Build the `AuditEvent`, using the current UTC time for `ts`.
    ///
    /// Returns `None` if required fields (entity, entity_id, op, actor, by, ctx)
    /// are missing.
    pub fn build(self) -> Option<AuditEvent> {
        let entity = self.entity?;
        let entity_id = self.entity_id?;
        let op = self.op?;
        let actor = self.actor?;
        let by = self.by?;
        let ctx = self.ctx?;

        let ts = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string();

        Some(AuditEvent {
            v: SCHEMA_VERSION,
            ts,
            entity: entity.as_str().to_string(),
            entity_id,
            scope: self.scope,
            op,
            from: self.from,
            to: self.to,
            actor: actor.as_str().to_string(),
            by,
            meta: self.meta,
            count: 1,
            ctx,
        })
    }
}

impl Default for AuditEventBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "event_tests.rs"]
mod event_tests;
