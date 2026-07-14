# Tasks for: 001-26_add-pr-fix-workflow

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status changes and next-task selection.
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 001-26_add-pr-fix-workflow
ito tasks next 001-26_add-pr-fix-workflow
ito tasks start 001-26_add-pr-fix-workflow 1.1
ito tasks complete 001-26_add-pr-fix-workflow 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add PR-fix slash-command workflow file

- **Files**: `.github/workflows/pr-fix.md`
- **Dependencies**: None
- **Action**: Add a minimal workflow definition triggered by `/pr-fix` and `eyes` reaction, with declared permissions, safe outputs, tool access, timeout, and prompt body for PR-fix automation.
- **Verify**: `rg -n "^description:|^on:|slash_command:|reaction:|safe-outputs:|tools:|timeout-minutes:" .github/workflows/pr-fix.md`
- **Done When**: Workflow file exists and includes required trigger/action sections from the approved proposal.
- **Updated At**: 2026-03-02
- **Status**: [x] complete

### Task 1.2: Align workflow prompt with CI-remediation sequence

- **Files**: `.github/workflows/pr-fix.md`
- **Dependencies**: Task 1.1
- **Action**: Ensure the embedded PR-fix instructions include PR/context collection, CI failure analysis, root-cause diagnosis, fix implementation, verification, formatting/linting, push, and PR comment summary steps.
- **Verify**: `rg -n "Read the pull request|failing CI checks|Run any necessary tests|Run any code formatters|Add a comment" .github/workflows/pr-fix.md`
- **Done When**: Prompt includes all required ordered steps and fallback behavior when explicit instructions are absent.
- **Updated At**: 2026-03-02
- **Status**: [x] complete

### Task 1.3: Validate change package and repository checks

- **Files**: `.ito/changes/001-26_add-pr-fix-workflow/proposal.md`, `.ito/changes/001-26_add-pr-fix-workflow/specs/pr-fix-workflow/spec.md`, `.ito/changes/001-26_add-pr-fix-workflow/tasks.md`, `.github/workflows/pr-fix.md`
- **Dependencies**: Task 1.1, Task 1.2
- **Action**: Run Ito strict validation for the change and run repository-level checks relevant to workflow syntax/quality.
- **Verify**: `ito validate 001-26_add-pr-fix-workflow --strict`
- **Done When**: Strict validation passes and no blocking workflow quality issues remain.
- **Updated At**: 2026-03-02
- **Status**: [x] complete

______________________________________________________________________

## Checkpoints

### Checkpoint: Approval before implementation

- **Type**: checkpoint (requires human approval)
- **Dependencies**: None
- **Action**: Obtain approval to proceed from planning artifacts to implementation tasks.
- **Done When**: User confirms the change proposal package is approved.
- **Updated At**: 2026-03-02
- **Status**: [x] complete
