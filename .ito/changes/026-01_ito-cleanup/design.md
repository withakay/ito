<!-- ITO:START -->
## Context

Ito's installer (`ito init`, `ito update`, `ito init --upgrade`) writes files from embedded template assets to the project directory. Over multiple Ito versions, skills have been renamed (e.g., `ito-apply-change-proposal` -> `ito-apply`), removed entirely (e.g., `ito-dispatching-parallel-agents`), and directories restructured (e.g., `.opencode/command/` -> `.opencode/commands/`). The installer never deletes files — it only creates and updates. This means repos accumulate orphaned files that can confuse agents loading stale skill instructions.

Currently, the only related feature is `repo-sweep`, which scans for sub-module ID format assumptions in file *content*. There is no mechanism for detecting or removing orphaned *files*.

## Goals / Non-Goals

**Goals:**

- Provide agents with a complete manifest of what Ito should have installed, so they can compare against what's on disk
- Enumerate all known legacy/deprecated file paths from previous Ito versions
- Give agents a structured workflow to detect and remove orphans with user confirmation
- Integrate cleanup detection into `ito init --upgrade` so users are notified during upgrades
- Keep the legacy registry maintainable — adding new entries when future renames/removals happen should be trivial

**Non-Goals:**

- Automatic silent deletion of files (always require confirmation)
- Tracking user-created files that happen to be in Ito-managed directories
- Migrating file *content* (that's `repo-sweep`'s domain)
- Cleaning up files from non-Ito tools (e.g., user-created skills)

## Decisions

### Decision 1: Legacy registry as a Rust data structure in `ito-templates`

**Choice**: Define the legacy file registry as a `const` array of `LegacyEntry` structs in `ito-templates/src/legacy.rs`, embedded at compile time.

**Alternatives considered**:
- *YAML/JSON file in assets/*: Would require runtime parsing and a serde dependency in the hot path. The data is static and known at compile time.
- *Generated from git history*: Too fragile — depends on having the full git history available, which isn't the case in installed binaries.

**Rationale**: A Rust data structure is type-safe, zero-cost at runtime, and easy to extend — just add a new entry to the array. It also enables the CLI to use the same registry without file I/O.

### Decision 2: Manifest generation reuses existing `distribution.rs` manifest functions

**Choice**: The cleanup instruction generates the "expected files" list by calling the same `*_manifests()` functions in `distribution.rs` that the installer uses, ensuring perfect sync.

**Alternatives considered**:
- *Separate manifest definition*: Would drift from the installer over time.
- *Scanning the embedded assets directory*: Less precise — doesn't account for per-harness path mapping.

**Rationale**: Single source of truth. If a new skill is added to the templates, it automatically appears in the cleanup manifest.

### Decision 3: Agent instruction as the primary interface, CLI cleanup as convenience

**Choice**: The `ito agent instruction cleanup` artifact is the primary mechanism. It outputs a comprehensive guide that agents follow. The `ito init --upgrade --cleanup` flag is a convenience wrapper that runs the same detection logic but handles removal directly in the CLI.

**Rationale**: Agents are the primary consumers of Ito instructions. The CLI flag is useful for non-agent workflows (e.g., CI pipelines, manual cleanup).

### Decision 4: Instruction template uses Jinja2 with dynamic context

**Choice**: The `cleanup.md.j2` template receives a context struct containing the manifest and legacy entries, rendered dynamically per project configuration.

**Rationale**: Follows the established pattern for all other instruction artifacts (`repo-sweep`, `apply`, etc.).

### Decision 5: Skill is a thin wrapper

**Choice**: The `ito-cleanup` skill SKILL.md simply instructs the agent to run `ito agent instruction cleanup` and follow the output. No complex logic in the skill itself.

**Rationale**: Keeps the skill maintainable and ensures the instruction artifact is the single source of truth for cleanup logic.

## Risks / Trade-offs

- **[Risk] Legacy registry becomes stale** -> Mitigated by making it trivial to add entries (just append to the array). Add a comment in the registry file reminding developers to update it when renaming/removing template files.
- **[Risk] False positives in orphan detection** -> Mitigated by only flagging files that match known legacy paths, not arbitrary files in Ito directories. User-created files are never flagged.
- **[Risk] Agent removes files without confirmation** -> Mitigated by the instruction explicitly requiring user confirmation before any deletion. The skill reinforces this gate.
- **[Risk] Manifest doesn't reflect actual installed state** -> The manifest shows what *should* be installed based on current templates and configured tools. Files that were never installed (because the tool wasn't configured at the time) won't be flagged as missing — this is acceptable since the goal is orphan removal, not completeness verification.
<!-- ITO:END -->
