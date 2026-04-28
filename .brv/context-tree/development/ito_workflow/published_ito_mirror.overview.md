## Key points
- The published Ito mirror is a **read-only generated docs tree** intended for consumption from plain GitHub/main checkouts.
- Mirror location is configurable via **`changes.published_mirror.path`**, with **`docs/ito`** as the default.
- Path resolution is hardened with **safe validation**: it rejects empty paths, absolute paths, parent traversal, and project-root-only paths.
- The renderer produces a deterministic mirror layout under **`README.md`**, **`changes/active`**, **`changes/archive`**, and **`specs`**.
- The generator **skips symlinks** while building the mirror.
- The **`ito publish` CLI** loads cascading configuration, compares generated output to the existing mirror for **drift detection**, and replaces the mirror from coordination-backed state when needed.
- **Coordination state remains the writable source of truth**; the mirror is only a published read view.

## Structure / sections summary
- **Reason**: States the purpose—document the published mirror implementation and its safety constraints.
- **Raw Concept**: Summarizes the task, the changes made, the files involved (`docs/ito`, `ito publish CLI`), and the operational flow from configuration to replacement.
- **Narrative**:
  - **Structure**: Describes the system as a configurable path + renderer + publish CLI reconciliation loop.
  - **Dependencies**: Notes reliance on cascading config, coordination-backed Ito state, and the publish command.
  - **Highlights**: Emphasizes safety, determinism, and compatibility with standard GitHub checkouts.
- **Facts**: Enumerates concrete implementation details, including path defaults, validation rules, output layout, symlink behavior, CLI drift detection, and source-of-truth semantics.

## Notable entities, patterns, or decisions
- **Entities**: `changes.published_mirror.path`, `docs/ito`, `README.md`, `changes/active`, `changes/archive`, `specs`, `ito publish`.
- **Pattern**: A **generate-compare-replace** publish workflow:
  1. configure mirror path  
  2. validate path  
  3. generate read-only mirror  
  4. compare for drift  
  5. replace from coordination state
- **Decision**: Keep the coordination-backed store writable and the published mirror read-only.
- **Decision**: Enforce safe path resolution to prevent filesystem escape or unsafe target selection.
- **Decision**: Exclude symlinks from generated output to avoid unsafe or unstable mirror contents.