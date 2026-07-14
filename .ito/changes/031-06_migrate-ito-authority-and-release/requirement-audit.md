# Requirement audit: 031-06 migrate Ito authority and release

Date: 2026-07-14

Status: final — all requirements and ordered review gates pass

The implementation and automated evidence cover every declared requirement.
The independent migration reviewer verified committed cutover `0b02a8c3` in a
disposable legacy-main merge and a separate fresh checkout.

| Requirement ID | Scenario evidence | Implementation and verification evidence | Result |
| --- | --- | --- | --- |
| `ito-authority-cutover:dependency-gated-cutover` | Complete and incomplete dependency gates | `evidence/dependencies.md`; five reviewed implementations are ancestors of this main-bound branch; all six changes pass strict validation | PASS |
| `ito-authority-cutover:external-state-preservation` | Healthy snapshot and ambiguous/conflicting stop | `evidence/source-manifest.md`, `source-links.txt`, `source-files.sha256`, `materialized-parity.md`; final 1,999-file checksum pass; detector collision/ambiguity tests; retained checkout identity/status unchanged | PASS |
| `ito-authority-cutover:tracked-main-authority` | Materialized state and main-compatible config | Real tracked `.ito/{changes,specs,modules,workflows,audit}`; documented empty `.ito/audit/.gitkeep` sentinel; embedded/disabled coordination, backend false, no tmux; migration and config tests | PASS |
| `ito-authority-cutover:mirror-parity-before-retirement` | Complete parity and unmatched-content stop | `evidence/mirror-parity.md`; 1,814 mapped mirror artifacts, no mirror-only authority path after normalization, reviewed semantic reconciliation, strict specs pass, `docs/ito` retired only afterward | PASS |
| `ito-authority-cutover:guidance-and-asset-convergence` | Managed regeneration and authority-reference audit | Current specs/wiki/project guidance/templates point to tracked `.ito` on main; exactly seven Ito lifecycle skills; second generation diff hash stable; schema/docs/template gates pass | PASS |
| `ito-authority-cutover:dual-lane-release-verification` | Passing lanes/reviews and failed-gate visibility | `evidence/release-verification.md`; clean 8/8 matrix; experimental coverage; standard cargo-dist selection; both ordered reviews have no remaining finding | PASS |
| `published-ito-mirror:plain-checkout-visibility` | Plain checkout reads tracked authority | Tracked `.ito/changes` and `.ito/specs`; `docs/ito` absent; `reviews/migration-requirements-review.md` proves the legacy-main merge and clean-checkout cases | PASS |
| `published-ito-mirror:default-and-configurable-path` | No mirror path is resolved | Published-mirror DTO/default/schema/code removed; obsolete-value regression covered by config tests and schema gate | PASS |
| `published-ito-mirror:generated-read-only-output` | Ito mutations update canonical files | Mirror generator/publication code absent; tracked `.ito` changes are the reviewed diff; regeneration produces no mirror | PASS |
| `published-ito-mirror:main-publication-workflow` | Canonical state is published once | No mirror workflow or follow-up publication surface; main-bound merge carries canonical `.ito` directly | PASS |
| `ito-config-crate:published-mirror-path` | Config resolves without output path | Config crate and generated schema omit the setting; permissive obsolete-value regression passes | PASS |

## Scenario-level negative evidence

- Missing/unintegrated dependencies, ambiguous links, wrong targets,
  non-empty destination collisions, hash drift, unsafe Git paths, missing
  authoritative proposal blobs, and failed readiness all stop before mutation
  in focused tests and the feature-neutral migration prompt.
- Standard builds return typed feature-unavailable errors before experimental
  repository/fetch behavior. Explicit experimental builds remain testable but
  do not activate either feature from defaults.
- A failed release gate remains visible and blocks completion; this run retained
  the earlier feature-unification failure until the executable boundary was
  repaired and the clean matrix passed 8/8.
- No command in this implementation tagged, pushed, published, archived the
  changes, or cleaned the external rollback source.

## Task/audit bootstrap note

The main-first preflight intentionally rejects Ito task mutations for a
proposal that has not yet reached `main`. This repository cutover is the
bootstrap batch that introduces that rule and materializes its own authority,
so task status is recorded directly in this reviewed `tasks.md` diff. The
read-only `ito audit reconcile` reported the corresponding missing audit events;
`--fix` was not used to bypass the new readiness boundary. The final main merge
is the authority transition, and future changes use the ordinary task/audit
path after their reviewed proposals are on main.

## Ordered review and completion evidence

1. Integration state `0b02a8c3` committed Tasks 1.1 through 6.1 complete.
2. `reviews/migration-requirements-review.md` independently reproduced both
   the legacy-main merge and clean-checkout cases with no blocking or
   non-blocking finding.
3. Task 6.2 was marked complete only after that review.
4. Final strict validation, 11/11 traceability, source checksums, review checks,
   codemap freshness, and diff checks were rerun before Task 6.3 completion.

No release, tag, push, archive, mirror publication, or external-source cleanup
was performed. The branch is ready for reviewed integration into `main`.
