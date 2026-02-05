## Context

The Ito Rust implementation (`ito-rs`) uses canonical module and change IDs for on-disk organization, CLI routing, and artifact discovery. Canonicalization currently forces 2-digit change numbers and rejects values greater than 99, which blocks growth in a single module.

Padding has value for alphabetical sorting, but correctness and removing hard limits is more important.

## Goals / Non-Goals

**Goals:**

- Allow any (practical) non-negative integer change number (e.g. 100, 1234) in `ito-rs` without parse/validation failures.
- Preserve existing IDs and behavior for change numbers 0-99.
- Keep canonicalization predictable: minimum 2-digit padding, no truncation.
- Add Rust tests to cover 3+ digit change numbers.

**Non-Goals:**

- Renaming existing on-disk change directories to a new fixed width.
- Guaranteeing lexicographic sort order matches numeric order once change numbers exceed 99.
- Expanding module IDs beyond 3 digits (999) in this change.
- Updating the TypeScript implementation in this change.

## Decisions

### Decision: Remove the 99 hard cap

The Rust parser currently enforces `changeNum <= 99`. We will remove that check.

### Decision: Keep minimum padding, allow overflow

Canonical change number formatting will remain:

- `pad to at least 2 digits`, but
- do not truncate numbers that exceed 2 digits.

Examples:

- `1-2_name` -> `001-02_name`
- `1-100_name` -> `001-100_name`

This meets the functional requirement while keeping the original intent of padding.

### Decision: Specs and docs describe the canonical format as `NNN-<change>_name`

This change adds a `ito-rs`-scoped spec for the overflow behavior. Project-wide docs can be updated in a follow-up once TypeScript parity is established.

## Risks / Trade-offs

- \[Lexicographic ordering\] `001-100_*` may not sort after `001-99_*` in all listings.
  -> Mitigation: treat ordering as best-effort; tooling should not rely on directory sorting for correctness.
- \[Drift between TS and Rust\] Updating `ito-rs` only means behavior diverges.
  -> Mitigation: keep this change explicitly scoped to `ito-rs` and follow up with TS parity when ready.
