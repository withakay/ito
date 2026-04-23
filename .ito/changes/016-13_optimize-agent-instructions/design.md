<!-- ITO:START -->
## Context

Empirically, `ito agent instruction apply --change <id>` takes ~1.3s while `proposal|specs|tasks|bootstrap` are ~15ms. The difference is a synchronous `git fetch` of the coordination branch during apply-instruction generation.

Separately, the instruction handler loads cascading project config multiple times per invocation (testing policy, coordination branch settings, worktree config), each time probing up to the full precedence chain.

## Goals / Non-Goals

**Goals:**

- Make apply instruction generation offline-friendly and fast by default.
- Preserve coordination-branch sync capability as an explicit opt-in.
- Ensure config resolution happens once per invocation and is reused consistently.

**Non-Goals:**

- Changing the coordination branch workflow semantics for stateful operations (reserve/push/provision).
- Introducing persistent caching across invocations (disk cache).

## Decisions

- Coordination sync for apply instructions:
  - Default behavior: do not run `git fetch` during `ito agent instruction apply`.
  - Add an explicit opt-in (CLI flag and/or config) to fetch the coordination branch before printing apply instructions.
  - If sync is enabled and the remote branch is missing, continue printing instructions and emit a warning.

- Cascading config caching:
  - Resolve cascading project config once per CLI invocation and store it in the runtime context.
  - Update instruction helper functions to accept a resolved config value instead of reloading it.

## Alternatives Considered

- Background fetch (spawn and do not await): avoids blocking but adds complexity and non-determinism.
- Add a short timeout to fetch: still blocks and is brittle across environments.
- Disable coordination branch feature globally by default: too broad; keep behavior scoped to instruction generation.

## Risks / Trade-offs

- [Risk] Users expect apply instructions to pre-sync coordination state.
  -> Mitigation: provide an explicit opt-in, document it in the apply instructions artifact, and keep stateful operations syncing as needed.

- [Risk] Config caching could accidentally change merge semantics if consumers previously loaded with different contexts.
  -> Mitigation: cache the same merged config that would have been produced; add tests asserting one-load behavior and consistency.
<!-- ITO:END -->
