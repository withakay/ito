## Context

The vendored OpenCode plugin (`ito-skills/.opencode/plugins/ito-skills.js`) assumes it runs inside the repo and uses relative paths like `../../skills`. When the plugin is copy-installed into `${OPENCODE_CONFIG_DIR}/plugins/`, those paths break.

This change introduces a Ito-owned plugin designed for copy-install.

## Goals / Non-Goals

- Goals:
  - Inject a minimal bootstrap into OpenCode conversations.
  - Delegate all workflow bodies to `ito agent instruction` artifacts.
  - Resolve skills from a stable location under `${OPENCODE_CONFIG_DIR}`.
- Non-Goals:
  - Tool interception / lifecycle hooks beyond the system prompt transform.
  - Re-implementing Ito instructions inside the plugin.

## Contracts

### CLI Contract

The plugin assumes this command exists and returns a short, tool-specific preamble:

`ito agent instruction bootstrap --tool opencode`

The output should be safe to paste into a system prompt.

### Install Contract

The distribution/install layer will ensure these destinations exist:

- Plugin destination: `${OPENCODE_CONFIG_DIR}/plugins/ito-skills.js`
- Skill destination root: `${OPENCODE_CONFIG_DIR}/skills/ito-skills/`

## Implementation Notes

- Use `experimental.chat.system.transform` for prompt injection.
- Never resolve skill paths relative to the plugin file.
- Keep the injected preamble short; it should primarily point to `ito agent instruction bootstrap --tool opencode`.

## Rust Style

If this change requires Rust updates (e.g., template embedding or installer plumbing), follow the `rust-style` skill.

## Open Questions

- Should the plugin also inject a single-line hint for `ito agent instruction apply --change <id>` (in addition to bootstrap)?
