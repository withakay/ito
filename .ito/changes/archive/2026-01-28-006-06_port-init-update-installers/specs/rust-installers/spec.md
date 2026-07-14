# Spec Delta: rust-installers

## Purpose

Port `ito init` and `ito update` installers to Rust, preserving byte-for-byte output and marker-managed editing semantics.

## ADDED Requirements

### Requirement: Marker-managed edits preserve unmanaged content

The Rust implementation MUST only replace managed blocks and MUST preserve user-owned content outside markers.

#### Scenario: Update preserves user edits

- GIVEN a file containing a managed marker block and user edits outside the block
- WHEN `ito update` is run
- THEN only the managed block content is replaced
- AND user edits outside the block remain unchanged

### Requirement: Non-interactive installers match TypeScript byte-for-byte

When run in non-interactive mode, Rust MUST produce the same files and bytes as TypeScript.

#### Scenario: `init` output tree matches

- GIVEN a clean repository
- WHEN `ito init` is run with non-interactive flags
- THEN Rust produces the same file tree as TypeScript
- AND every file byte sequence matches

### Requirement: Path conventions match existing behavior

Installer outputs MUST use the correct path conventions.

#### Scenario: OpenCode singular directories

- GIVEN OpenCode installation selected
- WHEN `ito init` installs skills/commands/plugins
- THEN it writes under `.opencode/skill/`, `.opencode/command/`, and `.opencode/plugin/`
