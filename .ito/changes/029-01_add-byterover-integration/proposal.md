<!-- ITO:START -->
## Why

Ito does not yet integrate with any agent-memory / knowledge store. Information
captured during work (decisions, design rationale, gotchas, learnings) lives
in chat transcripts and archived change folders, but is not indexed, searchable,
or reusable by the next session. We want persistent, queryable project memory
for agents operating on this repo.

ByteRover (`brv`) is the first concrete provider we want to adopt. Getting
it wired into the repo is a prerequisite for the follow-on change
(`029-02_agent-memory-abstraction`) which makes memory a first-class,
provider-agnostic concept inside Ito itself.

## What Changes

- **New**: Install the ByteRover agent-skill connector for Claude Code via
  `brv connectors install "Claude Code" --type skill`. Claude Code is the only
  connector installed here because shared skills for other agents already live
  under `.agents/skills/` (which Claude Code does not consume).
- **New**: Install the following ByteRover hub skills into
  `.agents/skills/byterover/` via `brv hub install`, so every agent harness
  that reads `.agents/skills/` gets them:
  - `byterover-review`
  - `byterover-plan`
  - `byterover-explore`
  - `byterover-audit`
- **New**: Curate the following repo content into the local ByteRover
  context tree. Use folder-pack curation (`brv curate --folder <path>`) for
  directories and file-mode curation for the top-level agent docs. This is a
  one-off bootstrap pass; the follow-on change introduces the ongoing capture
  workflow.
  - `.ito/specs/`
  - `docs/`
  - `AGENTS.md`
  - `.ito/AGENTS.md`
  - `.ito/architecture.md`
  - `.ito/changes/archive/`
  - `.ito/modules/` (follows the symlink to the coordination-branch store)
- **New**: `.agents/skills/byterover/README.md` — short index explaining
  which ByteRover skills are installed, what they do, and how to invoke `brv`
  locally.
- **Explicitly excluded (out of scope for this change)**:
  - No cloud sync (`brv login`, `brv push`, `brv pull`) or version-control
    integration (`brv vc …`). The context tree stays local to the developer's
    machine.
  - No changes to Ito CLI, instruction templates, or config. Those belong to
    `029-02_agent-memory-abstraction`.
  - No default memory provider is wired into Ito by this change.

## Capabilities

### New Capabilities

- `agent-memory-byterover`: Concrete agent-memory integration for ByteRover.
  Specifies where ByteRover-specific skills, connectors, and curated content
  live inside this repo, and the invariants that make the integration
  reproducible for a new contributor.

### Modified Capabilities

- _None._ This change only adds files under `.agents/skills/byterover/` and
  a local (git-ignored) ByteRover context tree. It does not modify any
  existing Ito spec.

## Impact

- Workspace: adds `.agents/skills/byterover/byterover-{review,plan,explore,audit}/`
  skill folders and a Claude Code connector artifact (e.g. under `.claude/`).
- Developer machine: populates a local ByteRover context tree (cache location
  managed by `brv`; the project root is registered via `brv status`).
- No Rust source changes. No breaking CLI or schema changes. No changes to
  existing specs.
<!-- ITO:END -->
