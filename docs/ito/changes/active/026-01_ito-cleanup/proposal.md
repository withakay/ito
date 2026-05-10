<!-- ITO:START -->
## Why

When Ito upgrades (`ito init --upgrade`, `ito update`), it installs new and updated files but **never removes files that were deleted or renamed in newer versions**. Over time, repos accumulate orphaned skills, commands, agents, planning directories, and other artifacts from previous Ito versions. These stale files confuse agents (which may load outdated skill instructions), bloat the repo, and create ambiguity about which files are authoritative. There is currently no way for a user or agent to know what Ito *should* have installed vs what is *actually* on disk, nor any mechanism to clean up the difference.

## What Changes

- **New `ito agent instruction cleanup` artifact**: A Jinja2-rendered instruction that provides agents with:
  - The complete manifest of files Ito currently installs (dynamically generated from embedded templates, per configured harness tools)
  - A list of known legacy/deprecated file paths from previous Ito versions (skills renamed, commands removed, planning directories deleted, singular-to-plural directory migrations, etc.)
  - Step-by-step instructions for the agent to: (1) scan the repo for orphaned files, (2) report findings, (3) optionally remove them with user confirmation
- **New `ito-cleanup` skill**: An installable skill that tells the agent to run `ito agent instruction cleanup` and follow the returned instructions. This is the user-facing entry point.
- **Enhance `ito init --upgrade` with cleanup capability**: Extend the upgrade flow to detect and report (and optionally remove) orphaned files from previous versions. This makes cleanup a first-class part of the upgrade process rather than a separate manual step.
- **Legacy file registry in `ito-templates`**: A structured data source (embedded in the templates crate) that enumerates known legacy paths — files that were renamed, removed, or relocated across Ito versions. This registry powers both the agent instruction and the CLI cleanup.

## Capabilities

### New Capabilities

- `cleanup-instruction`: Agent instruction artifact (`ito agent instruction cleanup`) that generates a manifest of expected vs actual Ito-managed files and legacy orphan detection guidance for agents.
- `cleanup-skill`: Installable skill (`ito-cleanup`) that wraps the cleanup instruction into an agent-invocable workflow with interactive confirmation.
- `cleanup-cli`: CLI-level cleanup during `ito init --upgrade` that detects and optionally removes orphaned files from previous Ito versions.

### Modified Capabilities

_(none — this is purely additive)_

## Impact

- **Crates affected**: `ito-templates` (legacy registry, instruction template, new skill asset), `ito-core` (installer cleanup logic), `ito-cli` (new instruction artifact handler, upgrade flow enhancement)
- **Installed files**: New skill directory (`ito-cleanup/SKILL.md`) added to all harness skill directories
- **User-facing**: New `ito agent instruction cleanup` command; enhanced `ito init --upgrade` output with orphan detection
- **Risk**: Low — purely additive. Cleanup removals require user confirmation. No breaking changes.
<!-- ITO:END -->
