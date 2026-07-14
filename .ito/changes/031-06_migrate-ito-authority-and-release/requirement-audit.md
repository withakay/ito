# Requirement audit: 031-06 migrate Ito authority and release

Date: 2026-07-14

Status: draft pending the ordered Task 6.2 migration review

The implementation and automated evidence cover every declared requirement.
This audit becomes final only after the independent migration reviewer verifies
the committed cutover in a disposable merge/fresh-checkout simulation.

| Requirement ID | Scenario evidence | Implementation and verification evidence | Result |
| --- | --- | --- | --- |
| `ito-authority-cutover:dependency-gated-cutover` | Complete and incomplete dependency gates | `evidence/dependencies.md`; five reviewed implementations are ancestors of this main-bound branch; all six changes pass strict validation | PASS |
| `ito-authority-cutover:external-state-preservation` | Healthy snapshot and ambiguous/conflicting stop | `evidence/source-manifest.md`, `source-links.txt`, `source-files.sha256`, `materialized-parity.md`; final 1,999-file checksum pass; detector collision/ambiguity tests; retained checkout identity/status unchanged | PASS |
| `ito-authority-cutover:tracked-main-authority` | Materialized state and main-compatible config | Real tracked `.ito/{changes,specs,modules,workflows,audit}`; documented empty `.ito/audit/.gitkeep` sentinel; embedded/disabled coordination, backend false, no tmux; migration and config tests | PASS |
| `ito-authority-cutover:mirror-parity-before-retirement` | Complete parity and unmatched-content stop | `evidence/mirror-parity.md`; 1,814 mapped mirror artifacts, no mirror-only authority path after normalization, reviewed semantic reconciliation, strict specs pass, `docs/ito` retired only afterward | PASS |
| `ito-authority-cutover:guidance-and-asset-convergence` | Managed regeneration and authority-reference audit | Current specs/wiki/project guidance/templates point to tracked `.ito` on main; exactly seven Ito lifecycle skills; second generation diff hash stable; schema/docs/template gates pass | PASS |
| `ito-authority-cutover:dual-lane-release-verification` | Passing lanes/reviews and failed-gate visibility | `evidence/release-verification.md`; clean 8/8 matrix; experimental coverage; standard cargo-dist selection; Rust review complete; migration review remains the ordered final gate | PENDING REVIEW |
| `published-ito-mirror:plain-checkout-visibility` | Plain checkout reads tracked authority | Tracked `.ito/changes` and `.ito/specs`; `docs/ito` absent; fresh-checkout proof pending Task 6.2 | PENDING REVIEW |
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

## Ordered completion gate

Before this audit is marked final:

1. commit the integration state with Tasks 1.1 through 6.1 complete;
2. run the independent disposable merge and fresh-checkout review;
3. record `reviews/migration-requirements-review.md` with no blocking finding;
4. mark Task 6.2 complete; and
5. rerun strict validation, traceability, source checksums, diff checks, and
   this mapping before marking Task 6.3 complete.
