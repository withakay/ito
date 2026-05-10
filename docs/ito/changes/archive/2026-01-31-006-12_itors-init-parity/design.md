## Context

The TypeScript CLI’s `init` behavior is the de-facto reference implementation: it prompts for tool selection when `--tools` is not provided, and supports explicit non-interactive configuration via `--tools`. The Rust port (`itors`) currently diverges by behaving as a non-interactive “install everything” initializer, which is inconsistent with the porting goals and user expectations.

## Current Behavior (Rust)

- `itors init` previously defaulted to configuring all supported tools when `--tools` was omitted (no prompt).
- Tool installation is driven by a `BTreeSet<String>` of tool IDs passed into the installer layer.

## Relevant Rust Entry Points

- CLI command parsing/dispatch: `ito-rs/crates/ito-cli/src/main.rs`
- Init orchestration + template installation: `ito-rs/crates/ito-core/src/installers/mod.rs`
- Embedded templates (project + home): `ito-rs/crates/ito-templates/src/lib.rs`
- Shared interactive-mode resolution: `ito-rs/crates/ito-core/src/output/mod.rs`

## Goals / Non-Goals

**Goals:**

- Make `itors init` match the TypeScript CLI interaction model and flag semantics for tool selection.
- Keep behavior deterministic for CI/non-interactive usage using `--tools`.
- Add parity harness coverage for both interactive and non-interactive init flows.

**Non-Goals:**

- Introduce new tool installation behaviors not present in the TypeScript CLI.
- Add Taskwarrior or other new tool support as part of this change.
- Build a new configuration format; this is strictly parity work.

## Decisions

- **Decision: Interactive by default when tools are omitted (TTY only)**

  - Rationale: Mirrors the TypeScript CLI’s default path and avoids surprise “install all tools” behavior.
  - Alternative: keep non-interactive default and add an `--interactive` flag (rejected; increases divergence).

- **Decision: `--tools` is the single non-interactive control surface**

  - Rationale: Matches TypeScript CLI semantics and keeps CI usage explicit.
  - Alternative: add separate flags per tool (rejected; not present in TS version).

- **Decision: PTY-driven tests are required for interactive parity**

  - Rationale: Prevents regressions and anchors UX parity to a runnable harness.

## Implementation Notes

- Prefer well-maintained, cross-platform crates for the interactive path:
  - Prompts/wizard UI: `dialoguer`
  - Terminal control (TTY detection, raw mode if needed): `crossterm`
  - Spinners/progress: `indicatif`
- Avoid introducing a full-screen TUI unless parity demands it; if it does, use `ratatui` (with `crossterm` backend) and keep the surface area limited.
- Keep prompt logic behind a small interface so parity tests can exercise non-interactive logic without PTYs, and reserve PTY tests for end-to-end coverage.

## Risks / Trade-offs

- Prompt UX drift across platforms → Mitigation: keep prompts minimal, match TS option labels/ordering where possible, and test via PTY.
- Output differences between CLIs → Mitigation: parity harness normalizes known differences and focuses on installed artifacts as the primary oracle.

## Migration Plan

- Users currently depending on `itors init` installing everything without prompts can switch to `itors init --tools all`.

## Open Questions

- Do we want a dedicated flag (e.g. `--yes`) that selects a recommended default tool set, or is `--tools all|none|...` sufficient for parity?
