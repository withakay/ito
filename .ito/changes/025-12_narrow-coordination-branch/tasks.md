<!-- ITO:START -->
# Tasks for: 025-12_narrow-coordination-branch

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 025-12_narrow-coordination-branch
ito tasks next 025-12_narrow-coordination-branch
ito tasks start 025-12_narrow-coordination-branch 1.1
ito tasks complete 025-12_narrow-coordination-branch 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Initialize missing coordination branches from empty history

- **Files**: `ito-rs/crates/ito-core/src/git.rs`
- **Dependencies**: None
- **Action**: Replace the missing-remote-branch setup path so it creates an empty root commit and pushes that commit instead of pushing `HEAD`.
- **Verify**: `cargo test -p ito-core coordination_branch --lib`
- **Done When**: A missing coordination branch is created from an empty-tree commit, existing remote branches still return ready, and setup failures are reported clearly.
- **Requirements**: change-coordination-branch:empty-history-initialization
- **Updated At**: 2026-05-13
- **Status**: [x] complete
<!-- ITO:END -->
