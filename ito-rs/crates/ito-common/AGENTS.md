# ito-common — L0 (Foundation)

Shared utilities. **Leaf crate — zero workspace dependencies, zero domain knowledge.**
See [`ito-rs/AGENTS.md`](../../AGENTS.md). See [`.ito/architecture.md`](../../../.ito/architecture.md).

## Modules
|fs: FileSystem trait (testable I/O) |id: ID parsing+validation (change, module, spec)
|io: file I/O wrappers |match_: fuzzy matching |paths: .ito/ path builders (changes_dir, specs_dir, etc.)

## Constraints
**MOST RESTRICTED CRATE — violations cascade everywhere**
**MUST NOT:** depend on ANY workspace crate | contain domain types (no Change/Module/Task) | contain business logic/policy
**MUST:** remain small+boring — pure utility | keep #![warn(missing_docs)] | provide FileSystem trait for ito-domain testability

## When to Add Here
"Is this utility reusable across multiple crates with no domain knowledge?" If yes → here. If needs Change/Module/Task/Spec knowledge → ito-domain or ito-core.

## Quality
```bash
make check && make test && make arch-guardrails
```
ID parser changes affect CLI UX, validation, and backend routes — test carefully.
|rust-quality-checker: style |rust-code-reviewer: catch architectural drift
