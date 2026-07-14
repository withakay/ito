# Core-reset release verification

Date: 2026-07-14

Branch: `031-06_migrate-ito-authority-and-release`

Main base: `813a8d0ac50d1c7b1ee5f592933f59037de60693`

This is readiness evidence for reviewed integration. No tag, push, GitHub
release, package publication, archive, or external-coordination mutation was
performed.

## Clean feature and quality matrix

Before the final matrix, 44.9 GiB of disposable Cargo and Showboat build
artifacts were removed from this worktree. The source was then frozen while an
independent test runner executed each gate sequentially. All eight commands
exited zero:

| Gate | Command | Result |
| ---: | --- | --- |
| 1 | `make feature-matrix-check` | PASS; default, backend-only, coordination-only, and all-feature combinations |
| 2 | `make check` | PASS; every shipping hook, affected test, and coverage floor |
| 3 | `make check-experimental` | PASS |
| 4 | `cargo test --workspace` | PASS |
| 5 | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| 6 | `cargo test --workspace --all-features` | PASS |
| 7 | `make config-schema-check` | PASS |
| 8 | `make docs-site-check` | PASS |

Experimental coverage was 80.52% of regions (62,307 total, 12,135 missed),
75.91% of functions (4,014 total, 967 missed), and 80.66% of lines (42,917
total, 8,301 missed). The passing shipping hook intentionally suppresses its
exact percentage while enforcing the configured hard floors.

The final run created no source diff, `.snap.new` file, or hook normalization.
The earlier mixed-workspace failure is covered by the executable-capability
regression: `ito-cli` now reports its own feature set instead of inheriting an
experimental `ito-core` feature unified by another workspace member.

## Generated and distributed surfaces

- Two consecutive `make config-schema` plus
  `target/debug/ito init --upgrade --tools all` passes produced the same Git
  diff hash before, after the first pass, and after the second pass:
  `5ecb4049dd500beac9754cae0212d45f31d4ddc6b762795d18b7a34492522486`.
- The only cleanup warning was the intentionally retained historical
  `.ito/planning` directory; no retired asset was regenerated.
- Canonical Ito skills and every Ito-managed harness expose exactly seven
  lifecycle skills: `ito`, `ito-proposal`, `ito-research`, `ito-apply`,
  `ito-review`, `ito-archive`, and `ito-loop`.
- Backend and coordination implementation code is absent from the standard
  feature selection and remains independently buildable behind explicit
  Cargo features. Iteration/Ralph/loop remains in the standard CLI.
- Tmux configuration, runtime code, canonical assets, and generated Ito
  surfaces are absent. Known legacy managed assets remain cleanup candidates,
  not install outputs.
- `docs/ito` and the configurable publication contract are absent. Tracked
  `.ito` is the direct review and checkout surface.

## Executable demonstrations and Ito validation

All eight Showboat documents under changes `031-01` through `031-05` were
re-executed successfully with `uvx showboat verify`:

1. coordination migration fixture;
2. legacy detection and command policy;
3. main-first lifecycle;
4. proposal-integration configuration;
5. immutable authority/readiness;
6. standard versus experimental feature boundary;
7. tmux removal; and
8. seven-skill lifecycle.

`target/debug/ito validate --all --strict` passed with 256 items checked.
Strict validation passed for every change from `031-01` through `031-06`.
Traceability was ready with no uncovered requirements for `031-01` (7/7),
`031-02` (7/7), `031-03` (11/11), and `031-06` (11/11). Changes `031-04` and
`031-05` predate requirement IDs in their delta specs, so `ito trace` correctly
reported traceability as unavailable while strict validation and their
Showboat/tests passed.

## Non-publishing release plan

`dist` is `cargo-dist 0.30.3`. `dist plan --output-format=json` succeeded with:

- announcement tag `v0.1.32`;
- one release, `ito-cli` version `0.1.32`; and
- 16 approved archives, checksums/source artifacts, installers, and formula
  artifacts.

`dist-workspace.toml` selects only `ito-cli`, sets `default-features = false`,
sets `features = ["web"]`, and sets `all-features = false`. The plan contains
no backend service release and does not opt into backend or coordination.

## Authority and rollback preservation

The retained source checkout remains:

- path: retained external checkout resolved at verification time (the
  normalized managed targets are recorded in `source-links.txt`);
- branch: `ito/internal/changes`;
- commit: `e2f023435cfa8f07fc96a3bbaf897528c235af25`; and
- status: the same single pre-existing modification to
  `.ito/changes/031-03_gate-experimental-backend-coordination/demos/feature-boundary.md`.

On the final verification pass, all 1,999 entries in `source-files.sha256`
passed `shasum -a 256 --check`. No sync, commit, reset, clean, delete, push, or
checkout command was run in the retained source.

## Independent review

The Rust/config/template/release review is recorded in
`reviews/rust-release-review.md` with no remaining blocking or non-blocking
findings. The independent migration/parity/fresh-checkout review is the next
ordered gate and will be recorded before the requirement audit is finalized.
