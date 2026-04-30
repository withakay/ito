# ito-logging — L1 (Domain)

Append-only JSONL telemetry. **Resilience invariant: logging failures MUST NEVER crash or slow down a command.**
See [`ito-rs/AGENTS.md`](../../AGENTS.md). See [`.ito/architecture.md`](../../../.ito/architecture.md).

## Key Exports
|Logger: append-only logger with write_start()/write_end()
|Outcome: Success or Error enum for command outcomes

## Design
|all I/O failures silently swallowed (debug-logged only) |anonymized via SHA-256+per-user salt — no file paths/user-identifiable data
|append-only JSONL; no rotation, no DB, no background threads |opt-out: ITO_DISABLE_LOGGING env var

## Dependencies
|none (standalone crate, external deps only)

## Constraints
**MUST NOT:** depend on any workspace crate | allow telemetry failures to propagate | log user-identifiable data/file paths/content
**MUST:** remain resilient (best-effort, never blocking) | keep #![warn(missing_docs)]

## Quality
```bash
make check && make test && make arch-guardrails
```
|rust-quality-checker: style |rust-code-reviewer: resilience invariants — pay attention to new I/O paths that could panic or propagate errors
