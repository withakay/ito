# Fuzzy Change Matching For Change Flags

## Why

Many Ito CLI commands accept `--change` / `--change-id`, but today they require users to type the full canonical change directory name. This is slow, error-prone, and inconsistent with other parts of the CLI that already do intelligent selection and suggestions.

## What Changes

- Add a shared, deterministic change-target resolution behavior for all commands that accept `--change` / `--change-id`.
- Allow partial inputs to resolve to a change when (and only when) they produce a unique match.
- Explicitly support dropping leading zeros in module and change numbers (e.g., `1-12` resolves to `001-12`).

## Capabilities

### New

- `cli-change-targets` (change target resolution for `--change` / `--change-id`)

### Modified

- None

## Impact

- Improves CLI ergonomics without changing canonical on-disk naming rules.
- Makes ambiguity/not-found behavior consistent and actionable across commands.
