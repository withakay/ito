<!-- ITO:START -->
## Why

`ito agent instruction apply` is substantially slower than other instruction artifacts because it synchronously fetches the coordination branch from `origin` on every invocation. Instruction generation is read-only; blocking on network I/O is surprising and makes agent workflows feel laggy (especially offline).

Instruction generation also reloads cascading project configuration multiple times per invocation, causing redundant filesystem work and making it harder to reason about config consistency.

## What Changes

- Make coordination-branch synchronization non-blocking for instruction generation:
  - Default: do not run `git fetch` as part of `ito agent instruction apply`.
  - Provide an explicit opt-in to sync when generating apply instructions (CLI flag and/or config).
- Cache resolved cascading project config once per CLI invocation and reuse it across instruction handlers (apply/review/etc.).
- Add regression tests that ensure apply instruction generation does not perform network I/O by default and that cascading config is resolved at most once per invocation.

## Capabilities

### Modified Capabilities

- `agent-instructions`: Apply instruction generation becomes fast/offline-friendly; coordination sync becomes opt-in.
- `change-coordination-branch`: Define how coordination-branch sync behaves for instruction generation.
- `cascading-config`: Avoid redundant config loads by reusing one resolved config view per invocation.

## Impact

- Improves perceived latency of `ito agent instruction apply` from ~1.3s to ~<100ms in typical local/offline scenarios.
- Reduces unnecessary load on git remotes (no repeated fetches for read-only commands).
- Internal refactor to thread cached config through CLI instruction code paths (no user-visible config semantics changes).
<!-- ITO:END -->
