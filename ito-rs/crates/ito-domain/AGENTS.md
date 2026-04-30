# ito-domain — L1 (Domain)

Domain models and repository ports. **Domain purity is the highest-severity architectural constraint.**
See [`ito-rs/AGENTS.md`](../../AGENTS.md). See [`.ito/architecture.md`](../../../.ito/architecture.md).

## Modules
|changes: Change model, computed status (Draft/Ready/InProgress/Paused/Complete), ChangeRepository trait
|modules: Module model, ModuleRepository trait, dependency graph
|tasks: Task model, parsing, computation, TaskRepository trait
|audit: event types + pure functions |discovery: project discovery + fs traversal
|errors: DomainError (Io, NotFound, AmbiguousTarget)
|planning: planning primitives + execution plan construction |schemas: workflow/plan/execution state serde types
|workflow: workflow models + execution helpers

## Dependencies
|ito-common only

## Constraints
**Enforced by arch_guardrails.py with baseline counts — violations fail CI**
**MUST NOT:** depend on ito-core/ito-cli/ito-web | use miette:: (zero tolerance) | use std::fs (baseline-constrained) | use std::process::Command (baseline-constrained) | contain presentation logic | perform I/O directly (use FileSystem trait)
**MUST:** remain deterministic+side-effect-free | keep #![warn(missing_docs)] | own DomainError | define repo traits as ports (impls in ito-core)

## Common Mistakes
|adding std::fs → use FileSystem trait (guardrails enforce baseline count)
|using miette → use thiserror for DomainError; only adapters use miette
|adding ito-core dependency → domain defines interfaces; core implements them
|repo implementations here → only traits (ports) in domain; fs-backed impls go in ito-core

## Quality
```bash
make check && make test && make arch-guardrails   # CRITICAL
```
|rust-quality-checker: style |rust-code-reviewer: mandatory after any change (domain purity violations = highest severity)
