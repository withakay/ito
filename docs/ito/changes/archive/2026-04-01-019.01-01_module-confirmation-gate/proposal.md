<!-- ITO:START -->
## Why

When agents create change proposals, they often pick a module automatically (defaulting to `000`) without confirming with the user. This skips an important decision point — the user may want to place the change in a specific module or create a new sub-module under an existing one. The module selection step needs to be an explicit confirmation gate that the agent cannot bypass, ensuring the user always has the opportunity to review, confirm, or redirect the module choice before any scaffolding is created.

## What Changes

- The `new-proposal.md.j2` instruction template is updated to make module selection an explicit confirmation gate with a mandatory user response before proceeding to `ito create change`
- The confirmation flow presents the user with clear options: use an existing module, create a new module, or create a new sub-module under an existing module
- The agent is instructed to run `ito list --modules` to show the full module tree (including sub-modules) and present it to the user
- Sub-module awareness is added: the prompt explains how sub-module IDs work (`NNN.SS` format) and how to create/use them
- The `ito-proposal` skill SKILL.md is updated to reinforce the confirmation gate in its step-by-step flow

## Capabilities

### New Capabilities

_(none — this modifies an existing capability)_

### Modified Capabilities

- `interactive-module-selection`: Adding mandatory confirmation gate behavior and sub-module awareness to the module selection step in proposal prompts

## Impact

- Affected templates: `ito-rs/crates/ito-templates/assets/instructions/agent/new-proposal.md.j2`
- Affected skills: `ito-rs/crates/ito-templates/assets/skills/ito-proposal/SKILL.md`
- No Rust code changes required — this is a prompt/instruction-only change
- All projects using Ito will get the updated prompts on next `ito init` or `ito update`
<!-- ITO:END -->
