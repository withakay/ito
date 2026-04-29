## Key points
- The document describes cleanup behavior for renamed **ito-orchestrator specialist assets** during install/init flows.
- Cleanup now runs not only on **update** paths, but also on **forceful reinstall/init** paths.
- The installer performs a **pre-pass removal** of obsolete legacy assets before writing new assets.
- Cleanup uses **`symlink_metadata`** so **broken symlinks** are detected and removed.
- Legacy specialist assets renamed from **`ito-orchestrator-*`** to **`ito-*`** are targeted for deletion.
- **Coordinator assets** like `ito-orchestrator.md` and `ito-orchestrator-workflow` are explicitly preserved.
- After removal, **empty legacy directories** are pruned.

## Structure / sections summary
- **Reason**: States the purpose is to document cleanup rules for renamed orchestrator assets during install and init flows.
- **Raw Concept**: Summarizes the task, changes, affected files, the high-level flow, and a timestamp.
- **Narrative**
  - **Structure**: Identifies the main functions involved: `install_agent_templates()`, `remove_obsolete_specialist_agent()`, and `prune_empty_agent_dirs()`.
  - **Dependencies**: Notes reliance on `symlink_metadata` and installer mode/options to control when cleanup happens.
  - **Highlights**: Lists the specific legacy assets removed and clarifies that coordinator assets are excluded.
  - **Rules**: Defines when cleanup triggers and states that plain init preserves user files.
  - **Examples**: Mentions tests validating `init --update --tools all` and `init --force --tools all`.
- **Facts**: Condenses the main behavioral rules into four project facts.

## Notable entities, patterns, or decisions
- **Functions**: `install_agent_templates()`, `remove_obsolete_specialist_agent()`, `prune_empty_agent_dirs()`
- **Files**:
  - `ito-rs/crates/ito-cli/tests/init_obsolete_cleanup.rs`
  - `ito-rs/crates/ito-core/src/installers/agents_cleanup.rs`
- **Modes/options**:
  - Cleanup triggers when `mode == InstallMode::Update`
  - Also when `opts.update` or `opts.force` is set
- **Deletion targets**:
  - `ito-orchestrator-planner`
  - `ito-orchestrator-researcher`
  - `ito-orchestrator-reviewer`
  - `ito-orchestrator-worker`
  - associated `*.md` and `SKILL.md` files
- **Design decision**: Preserve coordinator assets while removing only obsolete specialist assets.
- **Testing pattern**: Coverage focuses on update/force paths, preservation of installed assets, and retention of coordinator files.