## Key points
- Repo-level Ito refreshes manage harness assets from `ito-rs/crates/ito-templates/assets/skills`, `ito-rs/crates/ito-templates/assets/commands`, and the default project command `ito-project-setup`.
- The audit confirmed there were **no remaining `ito-*` orphan skills or commands** after running `ito init --update --tools all`.
- Non-Ito entries, such as `.claude/skills/byterover*` and `.opencode/commands/compare-workflow-tool.md`, are treated as **user-owned** and must be skipped.
- The refresh flow is: refresh harness assets → audit for `ito-*` orphans → skip user-owned entries → rerun `ito init --update --tools all` → verify unchanged git diff hash.
- Re-running the update was **idempotent**, with the git diff hash staying unchanged.
- The document frames the audit as a repo-level scope and stability check for Ito refresh operations.

## Structure / sections summary
- **Metadata**: Title, summary, tags, and timestamps.
- **Reason**: Explains the purpose of documenting harness asset scope, orphan checks, and idempotence.
- **Raw Concept**:
  - **Task**: States the audit objective.
  - **Changes**: Lists the validated outcomes of the audit.
  - **Files**: Names the relevant asset paths and command file.
  - **Flow**: Describes the refresh-and-verify sequence.
  - **Timestamp / Author**: Records when and by whom the audit was created.
- **Narrative**:
  - **Structure**: Clarifies managed vs. user-owned asset separation.
  - **Dependencies**: Notes reliance on refreshed harness state and git diff comparison.
  - **Highlights**: Summarizes the no-orphans and idempotence result.
  - **Rules**: Specifies what must be skipped during audits.
  - **Examples**: Gives concrete locations for managed assets.
- **Facts**: Consolidated bullet facts about asset scope, orphan status, user-owned exclusions, idempotence, and diff stability.

## Notable entities, patterns, or decisions
- **Entities**:
  - `ito-rs/crates/ito-templates/assets/skills`
  - `ito-rs/crates/ito-templates/assets/commands`
  - `ito-project-setup`
  - `.claude/skills/byterover*`
  - `.opencode/commands/compare-workflow-tool.md`
  - `ito init --update --tools all`
- **Pattern**: Clear separation between **managed harness assets** and **user-owned entries**.
- **Decision/Rule**: Only Ito-managed assets are refreshed; non-Ito files are explicitly excluded from modification.
- **Verification pattern**: Idempotence is validated through **git diff hash stability** after rerunning the refresh command.