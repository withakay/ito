<!-- ITO:START -->
# Tasks

## Execution Notes

- Keep the fix limited to OpenCode agent asset install paths and related tests.
- Do not rename unrelated OpenCode directories such as commands or skills.

## Wave 1

- **Depends On**: None

- [ ] 1.1 Update the OpenCode agent template target path from `.opencode/agent` to `.opencode/agents`
- [ ] 1.2 Adjust init/update tests that assert OpenCode agent asset paths
- [ ] 1.3 Verify the targeted CLI/template test coverage

## Wave 2

- **Depends On**: Wave 1

- [ ] 2.1 Run strict Ito validation for `000-16_fix-opencode-agents-path`
<!-- ITO:END -->
