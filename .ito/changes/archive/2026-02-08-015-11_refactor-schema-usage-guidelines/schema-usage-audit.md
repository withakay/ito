# Schema Usage Audit (`015-11`)

## Scope audited

- `ito-rs/crates/ito-domain`
- `ito-rs/crates/ito-core`
- Adapter crate(s): `ito-rs/crates/ito-cli`

## Findings

### `ito-domain`

- `ito-rs/crates/ito-domain/src/workflow.rs` uses `ito_schemas::WorkflowDefinition` for YAML parsing and task counting.
- Usage is pure data transport plus shape validation already owned by schema models.
- **Decision**: keep schema type directly (no domain wrapper needed).

### `ito-core`

- No `ito_schemas` usage found in `ito-rs/crates/ito-core`.
- **Decision**: no action required.

### `ito-cli` (adapter)

- `ito-rs/crates/ito-cli/src/commands/workflow.rs` matches on `ito_schemas::AgentType` only to render human-readable labels.
- This is adapter presentation logic over stable schema enum values, with no additional business rules.
- **Decision**: keep schema type directly.

## Leakage assessment

- No current hotspot requires introducing a separate domain type.
- Rule for follow-up refactors: when schema shape diverges from business rules (legacy fields, format-driven naming, behavior-heavy invariants), add a domain type and map at boundaries.
