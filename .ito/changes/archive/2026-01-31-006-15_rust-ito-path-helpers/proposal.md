## Why

Rust code across `ito-core`, `ito-workflow`, and `ito-cli` repeatedly constructs `.ito/` paths and scans directories using ad-hoc joins and string formatting. This causes:

- duplicated logic (changes/modules/specs paths constructed in multiple places)
- inconsistent handling of special directories (like `.ito/changes/archive`)
- harder refactors when `.ito/` layout or rules evolve

Centralizing path construction reduces repetition and prevents inconsistencies.

## What Changes

- Add a single `ito-core` path helper module (or struct) that provides canonical path construction for:
  - `.ito/` root
  - changes directory and per-change paths
  - modules directory
  - spec paths
- Refactor call sites in `ito-core` and `ito-cli` to use this helper rather than duplicating `.join("changes")`, `.join("modules")`, or `format!("{}/...", ...)`.

## Capabilities

### New Capabilities

- `rust-ito-path-helpers`

### Modified Capabilities

(none)

## Impact

- No user-facing behavior change expected.
- Makes future work safer: path rules live in one place.
