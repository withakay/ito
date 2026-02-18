<!-- ITO:START -->
## Why

GitHub Copilot's repository-scoped coding agent runs in a GitHub Actions environment. Today, nothing guarantees that Ito's audit log is validated (or drift reconciled) before the agent begins making changes, so audit state can silently diverge and only surface later.

## What Changes

- Extend `ito init` / `ito update` to install Copilot-specific preflight wiring that runs Ito audit validation before the agent starts.
- Add Copilot prompt/instruction guidance so the agent consistently treats audit checks as mandatory guardrails.
- Add tests to validate installation/update behavior for `.github/` Copilot assets.

## Capabilities

### New Capabilities

- `harness-audit-hooks`: Deterministic audit validation triggers for supported harnesses.

### Modified Capabilities

- `tool-adapters`: Add GitHub Copilot harness guidance and preflight wiring for audit validation.
- `rust-installers`: Ensure `.github/` assets are installed/updated consistently.

## Impact

- `.github/workflows/copilot-setup-steps.yml` (and possibly additional prompt files under `.github/prompts/`) updated/installed.
- Rust installer/update tests expanded to cover `.github/` assets.
<!-- ITO:END -->
