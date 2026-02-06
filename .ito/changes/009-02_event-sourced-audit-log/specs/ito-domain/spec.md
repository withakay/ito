# Spec: ito-domain (MODIFIED)

> Additions to the domain layer for audit event types.

## ADDED

### Requirement: Audit event domain types

The `ito-domain` crate SHALL provide the audit event data model in a new `audit` module.

#### Scenario: AuditEvent struct

WHEN the `audit` module is defined
THEN it SHALL export an `AuditEvent` struct with fields matching the audit-log spec schema (v, ts, entity, entity_id, scope, op, from, to, actor, by, meta)
AND the struct SHALL derive `Serialize`, `Deserialize`, `Debug`, `Clone`

#### Scenario: EntityType enum

WHEN the `audit` module is defined
THEN it SHALL export an `EntityType` enum with variants: `Task`, `Change`, `Module`, `Wave`, `Planning`, `Config`
AND it SHALL serialize to lowercase strings (`task`, `change`, `module`, `wave`, `planning`, `config`)

#### Scenario: Actor enum

WHEN the `audit` module is defined
THEN it SHALL export an `Actor` enum with variants: `Cli`, `Reconcile`, `Ralph`
AND it SHALL serialize to lowercase strings (`cli`, `reconcile`, `ralph`)

#### Scenario: AuditEventBuilder

WHEN constructing audit events
THEN a builder pattern SHALL be provided to construct `AuditEvent` instances
AND the builder SHALL auto-populate `v` (current schema version), `ts` (UTC now), and `by` (from git/env)
AND required fields (`entity`, `entity_id`, `op`) SHALL be enforced at compile time or via builder validation

#### Scenario: Crate dependency constraint

WHEN the `audit` module is added to `ito-domain`
THEN it SHALL NOT introduce any new crate dependencies beyond what `ito-domain` already depends on (`serde`, `serde_json`, `chrono`)
AND it SHALL NOT depend on `ito-core` or `ito-cli`

### Requirement: Audit writer trait

The `ito-domain` crate SHALL define a trait for audit log writing to enable dependency inversion.

#### Scenario: AuditWriter trait

WHEN the audit writer trait is defined
THEN it SHALL be named `AuditWriter`
AND it SHALL expose: `fn append(&self, event: &AuditEvent) -> Result<()>`
AND it SHALL be object-safe for dynamic dispatch

#### Scenario: NoopAuditWriter

WHEN audit logging is not configured or not desired
THEN a `NoopAuditWriter` implementation SHALL be provided that discards all events
AND this SHALL be the default when no audit log path is available
