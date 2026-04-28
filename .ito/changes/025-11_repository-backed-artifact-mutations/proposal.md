## Why

Ito already tells agents that active-work mutations should go through CLI/repository-backed flows, but that contract breaks down in practice because the CLI still lacks a general write/patch surface for change artifacts and promoted specs. Agents can read the rules, then immediately fall back to editing `proposal.md`, `tasks.md`, or spec markdown directly because that is still the most ergonomic path exposed by common harnesses.

That leaves Ito in an awkward middle state: reads are increasingly repository-backed, task mutations have a dedicated CLI path, but broader active-work authoring is still file-shaped. If we want filesystem, SQLite, and remote modes to be truly interchangeable, artifact mutation has to move behind the same runtime-selected abstraction, and the generated instructions/harness assets need to point agents at that interface everywhere Ito teaches them how to work.

## What Changes

- Add Ito-native artifact mutation commands for active-work authoring, with a targeted patch surface and a whole-artifact write surface for change/spec artifacts.
- Introduce repository-runtime-selected artifact mutation services so filesystem, SQLite, and remote modes share one mutation contract instead of falling back to local markdown editing.
- Extend the remote/backend client runtime to support artifact mutation operations with revision-aware responses.
- Update generated instruction artifacts and installed harness guidance so active-work Ito artifacts are mutated through `ito` commands rather than direct markdown edits.
- Add comprehensive behavior and instruction-output tests covering the command surface, runtime selection, remote-mode messaging, and harness/instruction parity.

## Capabilities

### New Capabilities

<!-- None. -->

### Modified Capabilities

- `repository-runtime-selection`: runtime composition must include artifact mutation services in addition to read repositories and task mutation services.
- `backend-client-runtime`: remote runtime must be sufficient to build artifact mutation clients alongside repository readers.
- `agent-instructions`: generated active-work guidance must point agents at Ito artifact mutation commands when they need to change proposal/tasks/spec artifacts.
- `backend-agent-instructions`: backend-aware guidance must explicitly describe `ito` artifact mutation commands as the authoritative active-work write path and distinguish them from read-only Git projections.

## Impact

- Affected code: `ito-domain` mutation traits/DTOs, `ito-core` repository runtime and backend clients, `ito-cli` command surface, instruction rendering, embedded harness/template assets, and associated tests.
- Affected behavior: agents gain a first-class Ito write/patch workflow for stateful artifacts; generated guidance becomes stricter and more explicit about using it.
- Persistence impact: filesystem, SQLite, and remote modes must all support the same artifact mutation contract.
- Test impact: instruction rendering and installed harness outputs become first-class acceptance surfaces and need exhaustive coverage.

## Execution Guidance

- Build the mutation abstraction first, then attach CLI commands, then update instructions once the final command shape is stable.
- Prefer additive behavior: keep `ito tasks ...` as the semantic task workflow, and add broader artifact mutation commands rather than forcing task transitions through generic patching.
- Treat the generated instruction surfaces as product behavior, not just docs. The tests should prove that every relevant output path tells agents where to mutate Ito artifacts safely.
