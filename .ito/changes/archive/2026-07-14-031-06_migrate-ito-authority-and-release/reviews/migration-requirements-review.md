# Independent Migration, Parity, Documentation, and Requirements Review

Date: 2026-07-14

Reviewer: independent migration/parity/documentation pass

Comparison base: `813a8d0ac50d1c7b1ee5f592933f59037de60693`

Reviewed integration commit: `0b02a8c325bf90029fdac12ff5409e0a903a8999`

## Verdict

Task 6.2 passes against the committed integration state.

- **Blocking findings:** none.
- **Non-blocking findings:** none.
- The retained external coordination checkout was read only throughout this
  review. No tag, push, publication, archive, cleanup, source mutation, or
  commit was performed.

## Dependency and immutable-source evidence

All implementation and integration commits recorded for `031-01` through
`031-05` are ancestors of the reviewed commit, and the complete repository
passes strict validation with 256 items checked. The dependency table is
recorded in `evidence/dependencies.md:3`.

The retained source independently reproduced the identity and state recorded in
`evidence/source-manifest.md:5`:

- branch `ito/internal/changes`;
- commit `e2f023435cfa8f07fc96a3bbaf897528c235af25`;
- exactly the documented pre-existing modification to the `031-03`
  feature-boundary demo; and
- all 1,999 entries in `source-files.sha256` passed strict SHA-256 checking.

An independent type/mode scan found only regular, non-executable files under
the five source roots and no unexpected filesystem entry type. Comparing the
source with materialization commit `a650b52a` produced exactly the four
documented validation repairs in `evidence/materialized-parity.md:10` and the
two content-free historical directories that Git cannot represent. The empty
top-level audit root is handled separately by the documented, non-semantic
`.ito/audit/.gitkeep` exception at `evidence/materialized-parity.md:19`.

## Disposable merge and fresh-checkout proof

Two disposable checkouts were created from the local object store and deleted
after verification.

First, a checkout at the base commit was given the real legacy shape: all five
`.ito/{changes,specs,modules,workflows,audit}` entries were symlinks, including
an empty audit-authority target. A fast-forward to the reviewed commit:

- replaced all five symlinks with real directories;
- retained all five roots as unignored repository authority;
- materialized and tracked `.ito/audit/.gitkeep`;
- removed `docs/ito`;
- resolved backend disabled and coordination disabled with embedded storage;
  and
- exposed exactly the seven approved Ito lifecycle skills in each generated
  Claude, Codex, GitHub, OpenCode, and Pi harness.

Second, a clean detached checkout of the reviewed commit reproduced the same
five real directories, tracked audit sentinel, absent mirror, configuration,
and five-by-seven generated skill inventory. This closes the clean-clone and
existing-main cases required by `ito-authority-cutover:tracked-main-authority`
and the removed plain-checkout mirror requirement.

The committed configuration is explicit at `.ito/config.json:4` and
`.ito/config.json:12`; the canonical lifecycle inventory is the seven-element
constant at `ito-rs/crates/ito-templates/src/lib.rs:43`.

## Mirror parity and retirement

The pre-retirement Git tree at `a650b52a` independently contains 1,815
`docs/ito` files and 1,965 authoritative change/spec files, matching the
baseline in `evidence/mirror-parity.md:29`. The evidence supplies the ordinary
and five cross-state path mappings at `evidence/mirror-parity.md:9`, records no
mirror-only path after normalization, and documents the semantic reconciliation
of the 126 content differences at `evidence/mirror-parity.md:42`.

The reviewed commit contains zero `docs/ito` files and no live published-mirror
DTO, schema field, generator, or workflow. Historical references consistently
label the mirror retired. Current specs pass strict validation with 201 items
checked, including the accepted mirror semantics reconciled before retirement.

## Guidance, workflow, and generated-surface convergence

Current guidance is internally consistent:

- `docs/config.md:95` makes proposal review and integration precede
  implementation, defaults to `pull_request`, permits explicit `direct_merge`,
  and requires captured-authority prepare/execute checks.
- `.ito/wiki/_meta/config.yaml:4` indexes tracked `.ito` specs, changes,
  research, modules, project guidance, and instructions rather than `docs/ito`.
- `.brv/context-tree/development/release_workflow/release_plz_guardrails.md:46`
  keeps the five authority roots tracked, forbids hiding them from release
  checks, retires `docs/ito`, and records the standard `web`-only release
  feature set.
- Generated harness checks found no Ito tmux skill or retired lifecycle skill;
  each supported harness contains exactly `ito`, `ito-proposal`, `ito-research`,
  `ito-apply`, `ito-review`, `ito-archive`, and `ito-loop`.

The stable double-generation hash and the clean eight-gate feature/release
matrix are recorded at `evidence/release-verification.md:13` and
`evidence/release-verification.md:41`. The first ordered independent review is
present at `reviews/rust-release-review.md` and records the final feature,
configuration, template, and release boundary checks.

## Requirements and final-gate handoff

`ito trace 031-06_migrate-ito-authority-and-release` reports 11/11 declared
requirements covered with none uncovered. `git diff --check` from the base to
the reviewed commit passes. The draft mapping in `requirement-audit.md:11`
accounts for every added and removed requirement and correctly leaves only the
Task 6.2-dependent rows pending this review.

The following independent checks passed during this review:

1. dependency-commit ancestry to the reviewed integration commit;
2. strict validation of all 256 Ito items and all 201 current specs;
3. 11/11 requirement trace coverage;
4. strict verification of all 1,999 retained-source checksums;
5. source file-type/mode inspection and source-to-materialization comparison;
6. disposable legacy-main fast-forward and clean-checkout simulations;
7. five real, unignored authority roots with a tracked audit sentinel;
8. absent `docs/ito` and absent live published-mirror contract;
9. embedded/disabled coordination, disabled backend, and absent tmux config;
10. exactly seven lifecycle skills in each of five generated harnesses; and
11. base-to-integration `git diff --check`.

Task 6.2 may now be marked complete. Task 6.3 should finalize the requirement
audit, rerun its declared read-only checks, and record final readiness without
tagging, publishing, archiving, or deleting the retained rollback source.

## Workspace boundary

Three pending ByteRover draft files are untracked and are not part of reviewed
commit `0b02a8c3`. This review does not approve them for the follow-up commit;
the review/task/audit closure must keep those unrelated drafts excluded.
