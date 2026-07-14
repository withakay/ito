## Why

The Ito Rust implementation (`ito-rs`) currently hard-caps the change number at 99 (two-digit padding), which prevents larger modules from resolving and validating changes like `001-100_example`. As projects grow, this becomes a hard workflow blocker.

## What Changes

- Remove the `> 99` hard limit from change ID parsing/normalization in `ito-rs`.
- Keep alphabetical-friendly padding as a best-effort:
  - Change numbers are canonicalized with **minimum** 2-digit zero padding (`1` -> `01`).
  - Numbers that exceed 2 digits are preserved without truncation (`100` -> `100`).
- Update error messages to describe the new behavior (no hard cap).
- Add Rust tests covering 3+ digit change numbers (e.g. `001-100_name`).

## Capabilities

### New Capabilities

- `ito-rs-change-id-overflow`: Change ID parsing in `ito-rs` supports change numbers larger than 99 (minimum 2-digit padding, allow overflow).

### Modified Capabilities

(none)

## Impact

- Existing change IDs remain valid and unchanged.
- `ito-rs` may accept and validate change directories with 3+ digit change numbers (e.g. `.ito/changes/001-100_some-change/`).
- Alphabetical sorting of change directories remains best-effort. Once change numbers exceed 99, lexicographic ordering may no longer strictly match numeric order; functionality is prioritized over sorting.
