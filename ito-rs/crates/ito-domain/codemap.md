[Codemap: ito-domain]|L1: storage-independent data model (changes, modules, tasks, specs, audit, schemas, planning, traceability); no fs or UI concerns

[Entry Points]|src/lib.rs: domain exports |src/{changes,modules,tasks,specs}: entities + repo traits
|src/audit: event types + pure reconciliation |src/schemas: workflow+orchestration state types |src/traceability.rs: req/task coverage computation

[Design]|repo traits = interfaces ito-core adapters implement |pure computations (status derivation, traceability) here
|banned: miette, std::fs, std::process::Command (enforced by arch_guardrails.py) |types must be serializable+stable for fs/backend

[Gotchas]|adding fields affects JSON/YAML schemas + backend contracts |no concrete fs paths in domain types unless truly storage-level

[Tests]|targeted: cargo test -p ito-domain |trait behavior exercised via ito-core repo tests
