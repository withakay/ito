<!-- ITO:START -->
# Tasks

## Execution Notes

- Keep the fix limited to OpenCode agent asset install paths and related tests.
- Do not rename unrelated OpenCode directories such as commands or skills.

## Wave 1

- **Depends On**: None

### Task 1.1: Correct the OpenCode agent template path

- **Action**: Update the OpenCode agent template target path from `.opencode/agent` to `.opencode/agents`.
- **Status**: [x] complete
- **Updated At**: 2026-07-13

### Task 1.2: Update path assertions

- **Action**: Adjust init/update tests that assert OpenCode agent asset paths.
- **Status**: [x] complete
- **Updated At**: 2026-07-13

### Task 1.3: Verify targeted coverage

- **Action**: Run the targeted CLI and template test coverage.
- **Status**: [x] complete
- **Updated At**: 2026-07-13

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Validate the change

- **Action**: Run strict Ito validation for `000-16_fix-opencode-agents-path`.
- **Status**: [x] complete
- **Updated At**: 2026-07-13
<!-- ITO:END -->
