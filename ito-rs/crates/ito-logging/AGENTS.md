# ito-logging — Layer 1 (Domain)

Append-only telemetry logging to JSONL files. Records low-volume execution events with anonymized project identification.

For workspace-wide guidance see [`ito-rs/AGENTS.md`](../../AGENTS.md). For architectural context see [`.ito/architecture.md`](../../../.ito/architecture.md).

## Purpose

Record coarse command execution metadata (start/end events, durations, outcomes) to a JSONL file under the user's config directory. Designed to be resilient — telemetry failures must never break the main command flow.

## Key Exports

| Export | Responsibility |
|---|---|
| `Logger` | Append-only telemetry logger with `write_start()` / `write_end()` |
| `Outcome` | `Success` or `Error` enum for command outcomes |

## Design Principles

- **Resilience**: All I/O failures are silently swallowed (debug-logged only). Telemetry must never crash or slow down a command.
- **Privacy**: Project IDs are anonymized via SHA-256 with a per-user salt. No file paths, content, or user-identifiable data is logged.
- **Simplicity**: Append-only JSONL — no rotation, no database, no background threads.
- **Opt-out**: Disabled via `ITO_DISABLE_LOGGING` environment variable.

## Workspace Dependencies

None — this is a standalone crate with only external dependencies.

## Architectural Constraints

### MUST NOT

- Depend on any other workspace crate
- Depend on `ito-core`, `ito-cli`, or `ito-web`
- Allow telemetry failures to propagate — all errors must be handled gracefully
- Log user-identifiable data, file paths, or file content

### MUST

- Remain resilient — logging is best-effort, never blocking
- Keep `#![warn(missing_docs)]` enabled

## Quality Checks

```bash
make check              # fmt + clippy
make test               # all workspace tests
make arch-guardrails    # verify dependency rules
```

Use the `rust-quality-checker` subagent for style verification. Use the `rust-code-reviewer` subagent to ensure resilience invariants are maintained — pay special attention to any new I/O paths that could panic or propagate errors.
