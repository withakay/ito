---
children_hash: 9659a1e58d7070f64328ba0f4b990c0efd78450c4395589d5e8f8faa9b496fab
compression_ratio: 0.4890572390572391
condensation_order: 1
covers: [context.md, published_ito_mirror.md, worktree_validation_flow.md]
covers_token_total: 1188
summary_level: d1
token_count: 581
type: summary
---
## ito_workflow

This topic covers how Ito publishes a read-only mirror of coordination-backed state and how it validates worktrees safely during change work. The two child entries define complementary parts of the workflow: `published_ito_mirror.md` describes mirror generation and synchronization, while `worktree_validation_flow.md` defines the read-only validation gate used by tooling.

### Core workflow themes
- **Coordination-backed source of truth**: Writable state remains in coordination storage; `docs/ito` is generated as readable output for plain GitHub/main checkouts.
- **Safety-first path handling**: Mirror paths are project-relative, configurable, and strictly validated before generation.
- **Read-only publication**: The published mirror is deterministic and skips symlinks, making it safe for consumption without exposing writable state.
- **Change work validation**: Worktree validation distinguishes unsafe main/control checkouts from advisory mismatches elsewhere and emits machine-readable status for hooks.

### Related child entries
- **`published_ito_mirror.md`**
  - Configures the mirror via `changes.published_mirror.path` with default `docs/ito`.
  - Validates paths by rejecting empty, absolute, parent-traversal, and project-root-only inputs.
  - Generates a read-only layout under `README.md`, `changes/active`, `changes/archive`, and `specs`.
  - The `ito publish` CLI loads cascading config, detects drift by comparing generated output to the existing mirror, and replaces the mirror from coordination-backed state.

- **`worktree_validation_flow.md`**
  - `ito worktree validate --change <id> [--json]` provides a dedicated read-only validation flow.
  - Main/control checkouts are hard failures.
  - Non-main mismatches are advisory and include recovery guidance.
  - Matching uses exact change-id prefixes, including suffix worktrees like `<change>-review`, to avoid false positives.

### Structural relationship
- `published_ito_mirror.md` is about **publishing** state outward into a safe read-only tree.
- `worktree_validation_flow.md` is about **guarding** change work and ensuring tooling reacts correctly to checkout state.
- Together they support a workflow where coordination state is authoritative, published docs are consumable, and validation prevents unsafe operations.
