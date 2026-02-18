<!-- ITO:START -->
# Design: GitHub Copilot Audit Hooks

## Constraint

GitHub Copilot's repo agent does not provide a local "pre-tool" hook surface equivalent to Claude/OpenCode. The deterministic surface we control is the Copilot setup steps workflow.

## Strategy

- Add audit validation to `.github/workflows/copilot-setup-steps.yml`.
  - Run `ito audit validate`.
  - Optionally run `ito audit reconcile` (warn-only) or `ito audit reconcile --fix` (policy-dependent).
  - Fail the job on hard audit validation failure.

## Prompt Guidance

- Update `.github/prompts/*.prompt.md` (Ito commands) to include a short, strongly worded rule:
  - run audit validate before any stateful operation (tasks status changes, archive, etc.)

## Testing

- Add installer tests verifying `.github/workflows/copilot-setup-steps.yml` is installed by init and updated by update.
<!-- ITO:END -->
