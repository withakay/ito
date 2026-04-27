---
children_hash: 38404905b38580e4f234bd3c73a78549167c24824ddea9ce6e88005137f16d39
compression_ratio: 0.6504854368932039
condensation_order: 1
covers: [worktree_validation_flow.md]
covers_token_total: 515
summary_level: d1
token_count: 335
type: summary
---
## Worktree Validation Flow

The worktree validation knowledge centers on a dedicated read-only validation path for change work, documented in **worktree_validation_flow.md**. The key design shift is that `ito worktree validate --change <id> [--json]` now emits machine-readable status for OpenCode pre-tool hooks, enabling them to distinguish unsafe states from recoverable ones.

### Core behavior
- **Hard-fail policy:** main/control checkouts are treated as hard failures.
- **Advisory policy:** mismatches outside main are not fatal; they return guidance and recovery instructions.
- **Matching rule:** validation uses exact change-id prefixes to avoid false positives, including suffix worktrees such as `<change>-review`.

### Structural relationship
- The validation command produces status output consumed by **OpenCode pre-tool hooks**, which rely on the machine-readable format to gate execution correctly.
- The flow is explicitly separated into:
  1. validate worktree
  2. emit machine-readable status
  3. hard-fail on main/control checkout
  4. otherwise return advisory mismatch guidance
  5. match exact change-id prefixes

### Key takeaway
This entry documents a safer validation model that blocks only dangerous main/control cases while reducing false positives and preserving actionable guidance for non-main mismatches.
