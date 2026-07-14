# Legacy coordination source manifest

Captured on 2026-07-14 before final cutover verification.

## Git identity

- Branch: `ito/internal/changes`
- Commit: `e2f023435cfa8f07fc96a3bbaf897528c235af25`
- The source checkout commit and branch were not changed by this migration.
- The frozen working tree contains one tracked modification:
  `.ito/changes/031-03_gate-experimental-backend-coordination/demos/feature-boundary.md`.
  This in-flight Showboat wording was present in the materialized snapshot and
  is covered by `source-files.sha256`; it was not cleaned, committed, reset, or
  otherwise mutated in the retained source checkout.

## Inventory

| Directory | Files | Entries including directories |
| --- | ---: | ---: |
| `.ito/changes` | 1,763 | 3,016 |
| `.ito/specs` | 202 | 404 |
| `.ito/modules` | 33 | 68 |
| `.ito/workflows` | 1 | 2 |
| `.ito/audit` | 0 | 0 |

The sorted relative file checksum list contains 1,999 files and has SHA-256
`b60ee69e93eeeb161439dee32af735cda3b149224dfa392bae0f401ed56697df`.
Per-directory checksum-list hashes are:

- changes: `7d7a09b7f00415fbc5e300eebf0901044078cade787995784a65adf32adcbc59`
- specs: `9f38e833ffb8c9dd1c3c80c521c87fbd5d8b97d0c1280124ae610a8ce03b1786`
- modules: `b2a40f27b741d0a324dcc3afd7ad2d787f080e5a65843600906700006f6c6161`
- workflows: `53f681bc2114c022bcaf78db9fa884aa20e76f6d949d68c1c323f480eae25129`
- audit: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`

## Reproduction

Set `LEGACY_CHECKOUT` to the retained coordination checkout, then run:

```bash
cd "$LEGACY_CHECKOUT"
shasum -a 256 --check \
  "$CUTOVER_WORKTREE/.ito/changes/031-06_migrate-ito-authority-and-release/evidence/source-files.sha256"
git rev-parse HEAD
git status --short --branch
```

The actual pre-cutover link targets are recorded in normalized form in
`source-links.txt`; no machine-specific absolute path is committed.
