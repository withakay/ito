# ito-config — Layer 0 (Foundation)

Configuration loading and normalization. Resolves the Ito directory, reads repo-local and global config, and exposes `ItoContext` for each CLI invocation.

For workspace-wide guidance see [`ito-rs/AGENTS.md`](../../AGENTS.md). For architectural context see [`.ito/architecture.md`](../../../.ito/architecture.md).

## Purpose

Own the logic for reading configuration files (repo-local and global), applying precedence rules, and exposing a single resolved view to the rest of the workspace. Intentionally small — does not perform domain operations.

## Modules

| Module | Responsibility |
|---|---|
| `config` (re-exported) | Configuration loading, defaults, schema, and types |
| `context` | `ItoContext` — resolved context for a single invocation (config dir, project root, ito path, resolved config) |
| `ito_dir` | Resolve the Ito working directory name and path |
| `output` | Console/UI behaviour (color, interactivity) derived from CLI flags and environment |

## Workspace Dependencies

- `ito-common` only

## Architectural Constraints

### MUST NOT

- Perform domain operations (no change/module/task logic)
- Depend on `ito-domain`, `ito-core`, `ito-cli`, or `ito-web`
- Parse or manipulate markdown content
- Contain business logic or workflow orchestration

### MUST

- Remain a thin configuration layer
- Keep `#![warn(missing_docs)]` enabled
- Only depend on `ito-common` from the workspace

## Quality Checks

```bash
make check              # fmt + clippy
make test               # all workspace tests
make arch-guardrails    # verify dependency rules
```

Use the `rust-quality-checker` subagent to verify style compliance and the `rust-code-reviewer` subagent to catch architectural drift.
