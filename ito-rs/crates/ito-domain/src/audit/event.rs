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
    /// Session and git context for traceability.
    pub ctx: EventContext,
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
mod tests {
    use super::*;

    fn test_ctx() -> EventContext {
        EventContext {
            session_id: "test-session-id".to_string(),
            harness_session_id: None,
            branch: Some("main".to_string()),
            worktree: None,
            commit: Some("abc12345".to_string()),
        }
    }

    #[test]
    fn audit_event_round_trip_serialization() {
        let event = AuditEvent {
            v: 1,
            ts: "2026-02-08T14:30:00.000Z".to_string(),
            entity: "task".to_string(),
            entity_id: "2.1".to_string(),
            scope: Some("009-02_audit-log".to_string()),
            op: "status_change".to_string(),
            from: Some("pending".to_string()),
            to: Some("in-progress".to_string()),
            actor: "cli".to_string(),
            by: "@jack".to_string(),
            meta: None,
            ctx: test_ctx(),
        };

        let json = serde_json::to_string(&event).expect("serialize");
        let parsed: AuditEvent = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(event, parsed);
    }

    #[test]
    fn audit_event_serializes_to_single_line() {
        let event = AuditEvent {
            v: 1,
            ts: "2026-02-08T14:30:00.000Z".to_string(),
            entity: "task".to_string(),
            entity_id: "1.1".to_string(),
            scope: Some("test-change".to_string()),
            op: "create".to_string(),
            from: None,
            to: Some("pending".to_string()),
            actor: "cli".to_string(),
            by: "@test".to_string(),
            meta: None,
            ctx: test_ctx(),
        };

        let json = serde_json::to_string(&event).expect("serialize");
        assert!(!json.contains('\n'));
    }

    #[test]
    fn optional_fields_omitted_when_none() {
        let event = AuditEvent {
            v: 1,
            ts: "2026-02-08T14:30:00.000Z".to_string(),
            entity: "change".to_string(),
            entity_id: "test".to_string(),
            scope: None,
            op: "create".to_string(),
            from: None,
            to: None,
            actor: "cli".to_string(),
            by: "@test".to_string(),
            meta: None,
            ctx: EventContext {
                session_id: "sid".to_string(),
                harness_session_id: None,
                branch: None,
                worktree: None,
                commit: None,
            },
        };

        let json = serde_json::to_string(&event).expect("serialize");
        assert!(!json.contains("scope"));
        assert!(!json.contains("from"));
        assert!(!json.contains("\"to\""));
        assert!(!json.contains("meta"));
        assert!(!json.contains("harness_session_id"));
        assert!(!json.contains("branch"));
        assert!(!json.contains("worktree"));
        assert!(!json.contains("commit"));
    }

    #[test]
    fn entity_type_serializes_to_lowercase() {
        let json = serde_json::to_string(&EntityType::Task).expect("serialize");
        assert_eq!(json, "\"task\"");

        let json = serde_json::to_string(&EntityType::Config).expect("serialize");
        assert_eq!(json, "\"config\"");
    }

    #[test]
    fn entity_type_round_trip() {
        let variants = [
            EntityType::Task,
            EntityType::Change,
            EntityType::Module,
            EntityType::Wave,
            EntityType::Planning,
            EntityType::Config,
        ];
        for variant in variants {
            let json = serde_json::to_string(&variant).expect("serialize");
            let parsed: EntityType = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn actor_serializes_to_lowercase() {
        assert_eq!(Actor::Cli.as_str(), "cli");
        assert_eq!(Actor::Reconcile.as_str(), "reconcile");
        assert_eq!(Actor::Ralph.as_str(), "ralph");
    }

    #[test]
    fn actor_round_trip() {
        let variants = [Actor::Cli, Actor::Reconcile, Actor::Ralph];
        for variant in variants {
            let json = serde_json::to_string(&variant).expect("serialize");
            let parsed: Actor = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn builder_produces_valid_event() {
        let event = AuditEventBuilder::new()
            .entity(EntityType::Task)
            .entity_id("1.1")
            .scope("test-change")
            .op(ops::TASK_STATUS_CHANGE)
            .from("pending")
            .to("in-progress")
            .actor(Actor::Cli)
            .by("@jack")
            .ctx(test_ctx())
            .build()
            .expect("should build");

        assert_eq!(event.v, SCHEMA_VERSION);
        assert_eq!(event.entity, "task");
        assert_eq!(event.entity_id, "1.1");
        assert_eq!(event.scope, Some("test-change".to_string()));
        assert_eq!(event.op, "status_change");
        assert_eq!(event.from, Some("pending".to_string()));
        assert_eq!(event.to, Some("in-progress".to_string()));
        assert_eq!(event.actor, "cli");
        assert_eq!(event.by, "@jack");
        assert!(!event.ts.is_empty());
    }

    #[test]
    fn builder_returns_none_without_required_fields() {
        assert!(AuditEventBuilder::new().build().is_none());

        // Missing entity_id
        assert!(
            AuditEventBuilder::new()
                .entity(EntityType::Task)
                .op("create")
                .actor(Actor::Cli)
                .by("@test")
                .ctx(test_ctx())
                .build()
                .is_none()
        );
    }

    #[test]
    fn builder_with_meta() {
        let meta = serde_json::json!({"wave": 2, "name": "Test task"});
        let event = AuditEventBuilder::new()
            .entity(EntityType::Task)
            .entity_id("2.1")
            .scope("change-1")
            .op(ops::TASK_ADD)
            .to("pending")
            .actor(Actor::Cli)
            .by("@test")
            .meta(meta.clone())
            .ctx(test_ctx())
            .build()
            .expect("should build");

        assert_eq!(event.meta, Some(meta));
    }

    #[test]
    fn schema_version_is_one() {
        assert_eq!(SCHEMA_VERSION, 1);
    }

    #[test]
    fn event_context_round_trip() {
        let ctx = EventContext {
            session_id: "a1b2c3d4-e5f6-7890-abcd-ef1234567890".to_string(),
            harness_session_id: Some("ses_abc123".to_string()),
            branch: Some("feat/audit-log".to_string()),
            worktree: Some("audit-log".to_string()),
            commit: Some("3a7f2b1c".to_string()),
        };

        let json = serde_json::to_string(&ctx).expect("serialize");
        let parsed: EventContext = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(ctx, parsed);
    }

    #[test]
    fn entity_type_display() {
        assert_eq!(EntityType::Task.to_string(), "task");
        assert_eq!(EntityType::Planning.to_string(), "planning");
    }

    #[test]
    fn entity_type_as_str_matches_serde() {
        let variants = [
            EntityType::Task,
            EntityType::Change,
            EntityType::Module,
            EntityType::Wave,
            EntityType::Planning,
            EntityType::Config,
        ];
        for variant in variants {
            let serde_str = serde_json::to_string(&variant)
                .expect("serialize")
                .trim_matches('"')
                .to_string();
            assert_eq!(variant.as_str(), serde_str);
        }
    }
}
