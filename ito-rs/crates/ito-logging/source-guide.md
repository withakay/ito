# Source Guide: ito-logging

## Responsibility
`ito-logging` records low-volume telemetry events to append-only JSONL files. It is designed to be resilient: logging failures should never break primary command execution.

## Entry Points
- `src/lib.rs`: `Logger`, `Outcome`, salt/session/project ID helpers, and JSONL event writing.

## Design
- Uses a per-user salt to derive stable anonymized project IDs.
- Persists session IDs under `.ito/session.json` when available.
- Stores coarse command metadata only.

## Flow
1. Callers construct `Logger::new` with config dir, project root, Ito path, command ID, and version.
2. If logging is disabled or setup fails, `None` is returned.
3. Commands write start/end events when a logger exists.

## Integration
- Consumed by `ito-cli` runtime/app setup.
- Uses filesystem paths but treats errors as non-fatal debug events.

## Gotchas
- Do not add sensitive payloads to telemetry events.
- Preserve the non-blocking failure posture.

## Tests
- Targeted: `cargo test -p ito-logging`.
