[Codemap: ito-logging]|L1: append-only JSONL telemetry; resilient — logging failures MUST NOT break primary execution; zero workspace deps

[Entry Points]|src/lib.rs: Logger, Outcome, salt/session/project ID helpers, JSONL event writing

[Design]|per-user salt → SHA-256 anonymized project IDs; no file paths or user-identifiable data
|session IDs persisted to .ito/session.json; all I/O failures silently swallowed (debug-logged)

[Gotchas]|no sensitive payloads in telemetry |preserve non-blocking failure posture on all new I/O paths

[Tests]|cargo test -p ito-logging
