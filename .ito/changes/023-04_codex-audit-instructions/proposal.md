<!-- ITO:START -->
## Why

Codex does not provide a reliable pre-tool hook surface for running policy checks. Without explicit, consistently installed instructions, Codex-driven changes can bypass Ito audit validation and drift can go unnoticed.

## What Changes

- Extend `ito init` / `ito update` to install Codex-specific instructions that make Ito audit validation a mandatory precondition for stateful work.
- Keep the solution prompt-first: do not pretend hooks exist; provide clear, enforceable instructions and minimal helper scripts where appropriate.
- Add tests to ensure `.codex/` instruction assets install/update consistently.

## Capabilities

### New Capabilities

- `harness-audit-hooks`: Deterministic audit validation guidance for harnesses that lack hook APIs.

### Modified Capabilities

- `tool-adapters`: Add Codex-specific audit guidance and installation wiring.
- `rust-installers`: Ensure `.codex/` assets are installed/updated consistently.

## Impact

- New/updated files under `.codex/instructions/` and/or `.codex/prompts/`.
- Installer tests for Codex assets.
<!-- ITO:END -->
