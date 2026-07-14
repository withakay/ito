<!-- ITO:START -->
# Tasks for: 002-17_opencode-loop-command

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates

```bash
ito tasks status 002-17_opencode-loop-command
ito tasks next 002-17_opencode-loop-command
ito tasks start 002-17_opencode-loop-command 1.1
ito tasks complete 002-17_opencode-loop-command 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add OpenCode /loop command wrapper

- **Files**: `ito-rs/crates/ito-templates/assets/commands/loop.md`
- **Dependencies**: None
- **Action**: Add an OpenCode command that loads a dedicated Ito skill (or contains the wrapper instructions) and accepts a change id argument.
- **Verify**: `make test`
- **Done When**: `ito init` installs `.opencode/commands/loop.md` and `/loop <change-id>` is usable.
- **Updated At**: 2026-02-28
- **Status**: [x] complete

### Task 1.2: Add ito-loop skill for standardized wrapper behavior

- **Files**: `ito-rs/crates/ito-templates/assets/skills/loop/SKILL.md`
- **Dependencies**: Task 1.1
- **Action**: Create a skill that:
  - Requests/infers a change id (or errors if missing).
  - Runs `ito ralph` non-interactively with stable defaults (`--harness opencode`, `--timeout`, `--max-iterations`).
  - On early exit, appends a restart note via `ito ralph --add-context` and retries up to a small cap.
- **Verify**: `make test`
- **Done When**: Wrapper behavior is documented and consistent.
- **Updated At**: 2026-02-28
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Add minimal test coverage

- **Files**: `ito-rs/crates/ito-core/tests/**`, `ito-rs/crates/ito-cli/tests/**`, `ito-rs/crates/ito-templates/tests/**`
- **Dependencies**: None
- **Action**: Add tests that verify:
  - command distribution includes `loop.md` for OpenCode
  - loop wrapper guidance includes `--add-context` and references `--status`/tasks progress
- **Verify**: `make test`
- **Done When**: Tests catch regressions in install + wrapper behavior.
- **Updated At**: 2026-02-28
- **Status**: [x] complete

<!-- ITO:END -->
