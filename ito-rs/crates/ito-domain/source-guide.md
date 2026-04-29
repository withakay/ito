# Source Guide: ito-domain

## Responsibility
`ito-domain` defines Ito's storage-independent data model: changes, modules, tasks, specs, backend ports, audit events, schemas, planning, and traceability. It should stay pure and avoid filesystem or UI concerns.

## Entry Points
- `src/lib.rs`: domain module exports.
- `src/changes`, `src/modules`, `src/tasks`, `src/specs`: core entities and repository traits.
- `src/audit`: audit event types and pure reconciliation concepts.
- `src/schemas`: workflow schema and orchestration state types.
- `src/traceability.rs`: requirement/task coverage computation.

## Design
- Repository traits define behavior expected by `ito-core` adapters.
- Pure computations such as status derivation and traceability belong here.
- Types should be serializable and stable enough for filesystem/backend representations.

## Flow
1. Artifacts are parsed by repositories into domain structs.
2. Core use-cases compute status, validation, and mutations against trait interfaces.
3. Concrete repositories serialize domain results back to storage.

## Integration
- Consumed heavily by `ito-core` and adapter crates.
- ID and path helpers come from `ito-common` where possible.

## Gotchas
- Adding fields can affect JSON/YAML schemas and backend contracts.
- Avoid leaking concrete filesystem paths into domain types unless the concept is truly storage-level.

## Tests
- Targeted: `cargo test -p ito-domain`.
- Trait behavior is often exercised through `ito-core` repository tests.
