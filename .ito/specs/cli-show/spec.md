# Spec: cli-show

## Purpose

Define the `cli-show` capability and its current-truth behavior. This spec captures requirements and scenarios (for example: Deterministic ID ordering for selection lists).

## Requirements

### Requirement: `ito show specs` renders a bundled truth-spec prompt

The `ito show specs` command SHALL render a single stream of markdown by concatenating all main spec documents under `.ito/specs/*/spec.md`.

Specs MUST be ordered in ascending spec id order (where spec id is the directory name under `.ito/specs/`).

For each spec, the output MUST include a metadata comment line immediately before the spec content in the following form:

`<!-- spec-id: <id>; source: <absolute-path-to-spec.md> -->`

The spec markdown content MUST be emitted verbatim (no rewriting of headings or sections).

The bundled output MUST include main specs only (it MUST NOT include change delta specs under `.ito/changes/**/specs/**`).

#### Scenario: User bundles all truth specs as markdown

- **WHEN** a user runs `ito show specs`
- **THEN** the output contains one metadata comment per spec
- **AND** each metadata comment contains the spec id (folder name)
- **AND** each metadata comment contains an absolute `spec.md` path
- **AND** specs appear in ascending spec id order

#### Scenario: Bundled output excludes change deltas

- **GIVEN** one or more change delta spec files exist under `.ito/changes/**/specs/**`
- **WHEN** a user runs `ito show specs`
- **THEN** the output only includes specs sourced from `.ito/specs/**/spec.md`

### Requirement: `ito show specs --json` emits machine-readable bundled specs

When invoked with `--json`, `ito show specs` SHALL output a single JSON object containing:

- `specCount`: total number of bundled specs
- `specs`: an array of objects with:
  - `id`: spec id (folder name)
  - `path`: absolute path to `.ito/specs/<id>/spec.md`
  - `markdown`: raw markdown contents of that spec

All JSON filesystem path fields MUST be absolute.

#### Scenario: User bundles truth specs as JSON

- **WHEN** a user runs `ito show specs --json`
- **THEN** the output is valid JSON
- **AND** each spec entry includes an absolute `path`
