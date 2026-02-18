<!-- ITO:START -->
# Design: Init/Update Installer Parity

## Problem Summary

- Marker-managed files have a merge pathway (managed block updates), but non-marker files behave differently depending on init/update mode.
- `ito update` currently runs with a configuration that can overwrite more than intended.

## Target Semantics

### Ownership Classes

1. **Ito-managed (safe to overwrite on update)**
   - Harness wiring files: `.opencode/`, `.claude/`, `.github/`, `.codex/`
   - Installed commands/prompts/agents/skills that are generated/managed by Ito

2. **Marker-managed (merge managed block)**
   - Files containing `<!-- ITO:START -->` / `<!-- ITO:END -->` that are intended for user extension
   - Update behavior: replace managed block, preserve user content outside block

3. **User-owned (never clobber on update)**
   - `.ito/project.md`, `.ito/config.json` (and other explicitly documented user-edit files)

### Init vs Update

- `ito init`:
  - default: do not overwrite existing files
  - `--force`: overwrite all files
  - `--update`: attempt marker-merge; skip user-owned files; overwrite Ito-managed adapter assets

- `ito update`:
  - overwrite Ito-managed assets
  - marker-merge marker-managed files
  - never clobber user-owned files

## Implementation Sketch

- In `ito-core` installer:
  - Add an explicit policy decision per file path (ownership classification)
  - Apply consistent logic in both InstallMode::Init and InstallMode::Update

- In `ito-cli`:
  - Ensure `ito update` passes options that reflect update semantics (do not rely on `force=true` as a proxy).

## Tests

- Add tests that seed a project directory with:
  - existing non-marker file content
  - marker-managed file missing one/both markers
  - user-owned files with edits
  - harness adapter assets with expected updates
and verify init/update outcomes.
<!-- ITO:END -->
