## Context

Installers must preserve user-owned content and only replace managed blocks. OpenCode uses singular path conventions (`.opencode/skill/`, `.opencode/command/`, `.opencode/plugin/`). Codex honors `CODEX_HOME` when set, else defaults to `~/.codex/prompts`.

## Goals / Non-Goals

**Goals:**

- Implement installers and marker-managed editing in Rust.
- Ensure outputs match TypeScript exactly in non-interactive mode.
- Ensure interactive selections behave equivalently (PTY tests as needed).

**Non-Goals:**

- Redesign templates or file layouts.

## Decisions

### Decision: Templates embedded in `ito-templates`

Render templates from embedded sources and apply `.ito/` path normalization (custom ito dir support).

### Decision: Marker editing lives in `ito-fs`

Implement idempotent marker replacement with strict preservation of unmanaged content.

### Decision: Tree-diff parity tests

For non-interactive runs, compare output directory trees (relative paths + bytes) between TS and Rust.

## Testing Strategy

- Unit tests: marker parsing/replacement, template rendering
- Integration tests: installer runs in temp dirs
- Parity tests: TS vs Rust tree diffs
