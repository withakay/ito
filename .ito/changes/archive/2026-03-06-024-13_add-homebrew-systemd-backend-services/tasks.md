# Tasks for: 024-13_add-homebrew-systemd-backend-services

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 024-13_add-homebrew-systemd-backend-services
ito tasks next 024-13_add-homebrew-systemd-backend-services
ito tasks start 024-13_add-homebrew-systemd-backend-services 1.1
ito tasks complete 024-13_add-homebrew-systemd-backend-services 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add Homebrew service runtime artifact and wiring

- **Files**: Homebrew packaging/runtime files (final paths determined during implementation), backend startup docs
- **Dependencies**: None
- **Action**: Add or update Homebrew artifacts so Ito backend can be started/stopped as a Homebrew-managed service using documented defaults.
- **Verify**: `brew services list | rg -n "ito|backend"`
- **Done When**: Homebrew service workflow is implemented and documented with start/stop/status commands.
- **Updated At**: 2026-03-02
- **Status**: [x] complete

### Task 1.2: Add systemd service unit/runtime artifact and wiring

- **Files**: systemd unit template/location for backend runtime (path determined during implementation), backend startup docs
- **Dependencies**: Task 1.1
- **Action**: Add systemd service artifacts and usage instructions so Ito backend can be managed with standard systemd lifecycle commands.
- **Verify**: `systemd-analyze verify <path-to-ito-backend.service>`
- **Done When**: systemd workflow is implemented and documented with start/stop/status/log commands.
- **Updated At**: 2026-03-02
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Align backend runtime docs across Compose, Homebrew, and systemd

- **Files**: `docs/backend-client-mode.md`, `README.md` (if linking runtime entry points)
- **Dependencies**: None
- **Action**: Update docs to describe when to use each runtime path (Compose/Homebrew/systemd), required config, and health/log verification commands.
- **Verify**: `rg -n "docker compose|brew services|systemctl|journalctl|serve-api" docs README.md`
- **Done When**: Runtime docs provide a coherent multi-platform workflow with clear scope boundaries.
- **Updated At**: 2026-03-02
- **Status**: [x] complete

### Task 2.2: Validate proposal package integrity

- **Files**: `.ito/changes/024-13_add-homebrew-systemd-backend-services/**`
- **Dependencies**: Task 2.1
- **Action**: Run strict Ito validation and resolve any proposal/spec/tasks format or scope issues.
- **Verify**: `ito validate 024-13_add-homebrew-systemd-backend-services --strict`
- **Done When**: The change validates successfully with strict checks.
- **Updated At**: 2026-03-02
- **Status**: [x] complete

______________________________________________________________________

## Checkpoints

### Checkpoint: Proposal Review

- **Type**: checkpoint (requires human approval)
- **Dependencies**: None
- **Action**: Review proposal, design, and spec deltas before implementation.
- **Done When**: Stakeholders approve this follow-up change for implementation.
- **Updated At**: 2026-03-06
- **Status**: [x] complete
