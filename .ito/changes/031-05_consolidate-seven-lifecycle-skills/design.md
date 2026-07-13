<!-- ITO:START -->
## Context

Ito embeds every directory under `assets/skills` and adapts that tree into multiple harness destinations. Additional role templates—especially Codex role `SKILL.md` files—also become discoverable skills. Command/prompt wrappers mirror much of the same surface. As features accumulated, narrow helpers for intake, planning, worktrees, tasks, verification, memory, wiki, orchestration, cleanup, commits, finish, and updates duplicated policy and pushed the installed count far beyond the core lifecycle.

The authoritative behavior already increasingly lives in `ito agent instruction ...` templates and CLI operations. Consolidation can therefore reduce activation names without discarding essential policy: retained skills become stable phase entrypoints, and detailed rules remain emitted from one instruction source.

## Goals / Non-Goals

**Goals:**

- Make the default Ito-managed skill inventory exactly seven for every harness.
- Give each helper concern one lifecycle owner and eliminate duplicate top-level activation names.
- Keep iteration/Ralph available through `ito-loop` by default.
- Keep delegated roles separate from the installed skill inventory.
- Remove obsolete managed assets safely and idempotently during upgrade/update.
- Preserve direct CLI functionality and authoritative instruction artifacts needed by retained skills.

**Non-Goals:**

- Prevent projects or users from installing their own skills.
- Add a configurable profile-selection subsystem or optional Ito skill packs.
- Remove all CLI commands whose helper skill disappears.
- Remove harness-native sub-agent support where it does not create discoverable skills.
- Reimplement memory, wiki, orchestration, worktree, or verification policy inside every retained skill.

## Approach

Introduce one canonical lifecycle inventory in `ito-templates`, represented as stable skill names in lifecycle order. Shared manifest generation selects only these names and errors/tests if a retained asset is absent. All harness adapters consume the same selection; no adapter may append a harness-only skill.

Reduce the shared skill asset tree to the seven retained directories. Rewrite their managed sections as phase-oriented entrypoints:

| Retained skill | Consolidated responsibilities |
| --- | --- |
| `ito` | Router, setup orientation, direct CLI fallback, list/path/config/update/cleanup discovery |
| `ito-proposal` | Intake, feature/fix framing, brainstorming, research handoff, planning, proposal/spec/design/task scaffolding |
| `ito-research` | Structured research, wiki/search navigation, memory query/search, synthesis |
| `ito-apply` | Main-first preflight, worktree setup, task execution, sub-agent development, commits and progress tracking |
| `ito-review` | Proposal/code/spec review, tests, verification, quality gates, completion evidence |
| `ito-archive` | Archive, finish, spec promotion, wiki/memory capture, cleanup follow-through |
| `ito-loop` | Ralph iteration plus instruction-backed multi-change/orchestrated execution |

Retained skills reference CLI instruction artifacts for detailed policy. Resource files may remain inside a retained skill directory when phase-specific, but cannot introduce another discoverable `SKILL.md`.

Remove retired shared skill directories and helper command/prompt wrappers. Preserve direct CLI commands where still useful. Simplify the `ito` router to a fixed six-destination lifecycle table plus CLI fallback; remove wildcard discovery/cache behavior.

For agent roles, keep native agent files only for harnesses with a separate agent mechanism. Do not install specialist roles under `.agents/skills`, `.codex/skills`, or another skill discovery path. Where a harness cannot represent roles without skills, rely on retained instructions and the harness's ordinary delegation features rather than synthesizing role skills.

Expand the existing legacy manifest/cleanup pre-pass with every retired managed path. Managed-only assets and broken symlinks are removed; Markdown containing user content outside the managed block is preserved and reported. Cleanup runs before writing the retained inventory and is idempotent.

## Contracts / Interfaces

Canonical default skill names, in order:

1. `ito`
2. `ito-proposal`
3. `ito-research`
4. `ito-apply`
5. `ito-review`
6. `ito-archive`
7. `ito-loop`

The `ito` routing contract recognizes the six phase intents and otherwise invokes the CLI. An explicit retired skill request receives a replacement-phase explanation rather than dynamic wildcard activation.

Fresh harness installs expose only wrappers corresponding to retained lifecycle entrypoints. Direct CLI subcommands remain available through the binary and CLI help even when a similarly named skill/command wrapper is removed.

## Data / State

No domain data changes. Installed managed assets transition as follows:

| Asset state | Cleanup action |
| --- | --- |
| Retained lifecycle asset | Marker-scoped refresh |
| Retired, managed-only Markdown | Remove file and empty managed directories |
| Retired Markdown with user content outside marker | Preserve and warn with replacement phase |
| Retired non-Markdown generated asset | Remove only at exact known managed path |
| Retired broken symlink | Remove through `symlink_metadata` |
| User/project skill not in Ito legacy manifest | Preserve |

The canonical inventory is code, not user configuration. That avoids a new profile schema and ensures the default remains deterministic.

## Decisions

- **Seven names are an invariant, not a recommendation.** Exact-set tests catch accidental re-expansion.
- **Phase ownership replaces helper routing.** Agents reason about lifecycle state rather than selecting among dozens of mechanisms.
- **CLI instructions remain authoritative.** Consolidation removes duplicate activation surfaces, not the policy needed to perform work safely.
- **No optional Ito profiles in this change.** A profile subsystem would reintroduce configuration and testing complexity before a proven need exists.
- **Native agents are not skills.** Delegation can remain available without polluting user-facing skill discovery.
- **Preserve modified retired assets.** User content outranks cleanup neatness; reports make residual extensions explicit.

## Risks / Trade-offs

- Users may rely on retired names in prompts or automation. Retained skills include a migration map, cleanup reports replacement phases, and release notes list removals.
- Folding too much prose into seven files could create large prompts. Skills stay thin and call instruction artifacts/resources on demand.
- Harness capabilities differ. Exact logical-inventory tests are paired with harness-specific native-agent assertions.
- Leaving modified obsolete skill files can make an upgraded project show more than seven skills. This is treated as a preserved user extension, not part of the Ito-managed default; cleanup reports it clearly.
- Removing command wrappers can reduce palette discoverability. The retained `ito` entrypoint and CLI help become the supported discovery surface.

## Verification Strategy

- Unit tests assert the canonical inventory value and ensure each retained asset exists exactly once.
- Manifest tests compare exact logical skill sets for OpenCode, Claude, Codex, Pi, and GitHub Copilot.
- Fresh-install integration tests inspect actual harness directories and assert exactly seven Ito-managed `SKILL.md` entrypoints.
- Upgrade fixtures start with the current broad surface and prove managed-only helpers are pruned, user content survives, and a second update is byte-stable.
- Router tests cover every lifecycle destination, helper-to-phase migration, CLI fallback, argument preservation, and absence of wildcard discovery.
- Content tests prove retained skills link to all necessary instruction artifacts and do not duplicate canonical policy.
- Agent-surface tests prove native roles never create extra skill directories.
- Default build smoke tests prove `ito-loop` and Ralph remain installed and callable.

## Migration / Rollback

Release cleanup and retained skill rewrites atomically. Before deletion, test the current installed inventory from every harness fixture. Upgrade output lists retired paths that could not be removed due to user content and the retained phase that replaces each helper.

Rollback restores deleted embedded assets and manifests; the next update reinstalls managed copies. User content was preserved, so rollback does not require data recovery.

## Open Questions

None. The approved default is exactly seven lifecycle skills with no optional Ito profile subsystem in this reset.
<!-- ITO:END -->
