# ito-core — L2 (Core)

Business logic and orchestration. Repository adapters, archive, audit, create, list, show, validate, workflow, harness integrations, installers. **"Policy heavy, UI light."**
See [`ito-rs/AGENTS.md`](../../AGENTS.md) for workspace guidance. See [`.ito/architecture.md`](../../../.ito/architecture.md) for arch context.

## Key Modules
|change_repository: fs-backed ChangeRepository |module_repository: fs-backed ModuleRepository |task_repository: fs-backed TaskRepository
|archive: archive completed changes + update specs |audit: log writer/reader/reconcile/validate
|config: JSON config CRUD |create: scaffold modules+changes |errors: CoreError wrapping DomainError
|harness: OpencodeHarness, StubHarness |installers: ito init/update templates
|list: query entities |planning_init: planning dir init |process: process execution boundary
|ralph: AI agent loop support |show: display+inspection |stats: statistics |tasks: task orchestration
|validate: on-disk state + repo integrity |validate_repo: config-aware repo validation engine (coordination/*, worktrees/*, pre-commit detection)
|workflow: execution + planning

## Dependencies
|ito-common (L0), ito-config (L0), ito-domain (L1, required edge), ito-templates (L1)

## Constraints
**MUST NOT:** depend on ito-cli/ito-web | own CLI arg parsing or output formatting | contain presentation logic | carry presentation in CoreError
**MUST:** depend on ito-domain (required edge) | implement repo traits from ito-domain | keep #![warn(missing_docs)]

## Common Mistakes
|CLI-specific formatting → belongs in ito-cli (core returns structured data; adapters format it)
|depending on ito-cli → design is inverted
|bypassing repo abstractions → use ChangeRepository/ModuleRepository/TaskRepository
|adding dialoguer/crossterm → adapter-layer concerns

## Quality
```bash
make check && make test && make arch-guardrails
```
|rust-quality-checker: proactively while working (largest crate)
|rust-code-reviewer: after features (verify no logic leaked to/from presentation)
|rust-test-engineer: complex orchestration logic tests
