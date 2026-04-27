## Key points
- `ito worktree validate --change <id> [--json]` now supports a dedicated **read-only worktree validation flow** for change work.
- Validation outputs **machine-readable status** so OpenCode pre-tool hooks can make correct gating decisions.
- **Main/control checkouts** are treated as **hard failures** and block execution.
- **Non-main mismatches** are treated as **advisory**, with recovery guidance instead of a hard stop.
- Change matching uses **exact change-id prefixes** to avoid false positives from arbitrary substring matching.
- The prefix-matching approach covers suffix worktrees such as **`<change>-review`** without misclassifying unrelated entries.

## Structure / sections summary
- **Metadata**: title, summary, tags, related, keywords, timestamps.
- **Reason**: states the purpose—documenting dedicated read-only worktree validation behavior for change work.
- **Raw Concept**:
  - **Task**: describes the documentation goal.
  - **Changes**: lists the four core behavioral updates.
  - **Flow**: outlines the validation pipeline from validation to status emission and policy handling.
  - **Timestamp**: records when the concept was captured.
- **Narrative**:
  - **Structure**: explains the rationale for separating unsafe main/control cases from recoverable mismatches.
  - **Dependencies**: notes OpenCode pre-tool hooks rely on machine-readable status.
  - **Highlights**: emphasizes exact prefix matching to prevent false positives.
- **Facts**: enumerates the specific documented behaviors as named project facts.

## Notable entities, patterns, or decisions
- **CLI command**: `ito worktree validate --change <id> [--json]`
- **OpenCode pre-tool hooks**: consume machine-readable status to gate execution.
- **Policy split**:
  - **Hard-fail**: main/control checkout
  - **Warn/advisory**: non-main mismatch with guidance
- **Matching rule**: exact **change-id prefixes**, not arbitrary substrings.
- **False-positive avoidance**: explicitly prevents mistaken matches for suffix worktrees like **`<change>-review`**.