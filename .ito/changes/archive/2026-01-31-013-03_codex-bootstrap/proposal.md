# Change: Codex Bootstrap for Ito Skills

## Why

Codex has no reliable lifecycle hooks. The vendored `ito-skills` uses a Node CLI runner for bootstrap/skill lookup, but Ito should avoid this complexity. A simple, always-on bootstrap snippet in Codex's agents/prompt layer is more durable and delegates to `ito agent instruction <artifact>`.

## What Changes

- Create a minimal Codex bootstrap snippet (`ito-skills-bootstrap.md`) that:
  - Points to `ito agent instruction <artifact>` for all workflow content
  - Does not embed long workflow text
  - Provides a quick reference of available instruction artifacts
- Deprecate or remove the Node CLI skill runner (`ito-skills/.codex/ito-skills-codex`)
- Update `.codex/INSTALL.md` to document the simplified approach

## Capabilities

### New Capabilities

- `codex-adapter`: Codex bootstrap integration for Ito workflows

### Modified Capabilities

None

## Impact

- Affected specs: `tool-adapters` (new)
- Affected code:
  - Updated: `ito-skills/.codex/ito-skills-bootstrap.md`
  - Updated: `ito-skills/.codex/INSTALL.md`
  - Deprecate: `ito-skills/.codex/ito-skills-codex` (Node CLI)
- Embedded in: `ito-rs/crates/ito-templates/assets/`
- Parallelization: Can be developed in parallel with 013-01, 013-02

## Parallel Execution Notes

This change can be implemented in parallel with:
- 013-01 (OpenCode adapter) - no shared code paths
- 013-02 (Claude Code integration) - no shared code paths

Soft dependency on:
- 013-04 (bootstrap artifact CLI) - for the `ito agent instruction bootstrap --tool codex` content
- 013-05 (distribution) - for install/fetch mechanics

## Example Bootstrap Snippet

```text
When starting work on a Ito change, get the canonical workflow instructions from the CLI:

- Proposal/specs/design/tasks: run `ito agent instruction proposal|specs|design|tasks --change "<id>"`
- Implementation: run `ito agent instruction apply --change "<id>"`
- Review: run `ito agent instruction review --change "<id>"`
- Archive: run `ito agent instruction archive --change "<id>"`

Do not embed long, duplicated workflows here.
```
