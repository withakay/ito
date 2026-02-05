# Change: Emphasize TDD (RED/GREEN) and configurable coverage targets

## Why

Ito's proposal/apply templates and instructions do not consistently steer agents toward a disciplined RED/GREEN/REFACTOR loop or a measurable coverage target. Making this workflow explicit, and configurable per project, improves correctness and reduces regressions without requiring every repository to reinvent guidance.

## What Changes

- Update the proposal + apply instruction templates to explicitly direct a TDD RED/GREEN (and REFACTOR) workflow.
- Add a per-project testing policy configuration surface for:
  - TDD guidance mode / workflow (default: RED/GREEN/REFACTOR)
  - Coverage target percent (default: 80)
- Ensure instruction generation and docs mention the configured policy and show how to override it per project.

## Capabilities

### New Capabilities

- (none)

### Modified Capabilities

- `stable-instruction-generation`: instruction artifacts (proposal/apply) include TDD + coverage guidance and honor configured defaults.
- `cli-agent-config`: agent config supports storing testing policy defaults used by instruction generation.
- `docs-agent-instructions`: update AI-facing docs/templates to highlight RED/GREEN and coverage targets.
- `agent-workflow-docs`: update workflow docs to reinforce the TDD loop and coverage expectations.

## Impact

- Affected templates/docs: `ito-rs/crates/ito-templates/assets/default/project/` (prompts, skills, `.ito/*`).
- Affected behavior: `ito agent instruction proposal|apply` content becomes more explicit about TDD and coverage.
- Config: new optional keys under the existing cascading JSON config model; older versions ignore unknown keys.
- No breaking CLI flags; this is guidance/config expansion only.
