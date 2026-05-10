## Why

The Ito CLI has drifted into a mixed and overly broad command surface (top-level verbs, noun-group commands, and experimental `x-*` commands visible in help). This makes the CLI harder to learn, harder to document, and easy for shell completion to fall out of sync.

We want a small, intentional stable CLI surface that is the only supported UX shown in `ito --help`, while keeping hidden deprecated compatibility shims callable.

## What Changes

- Lock the stable help surface to a small set of intentional top-level commands:
  - `init`, `update`
  - `dashboard`
  - `status`, `ralph`
  - `create`, `list`, `show`, `validate`, `archive`, `split`
  - `config`, `completions`
- Keep only two experimental commands visible in help:
  - `x-templates`, `x-schemas`
- Keep deprecated and internal commands callable (hidden from help and omitted from completions) with clear warnings:
  - legacy noun groups: `change`, `spec`, `module`, `completion`, `skills`, `view`
  - legacy verb shims: `get`, `set`, `unset`, `reset`, `edit`, `path`, `generate`, `install`, `uninstall`
  - all other `x-*` commands
- Remove skills as a user-facing CLI surface; skills are refreshed via `ito init` and `ito update`.
- Align shell completion generation with the visible stable surface.

## Capabilities

### New Capabilities

- `cli-surface`: codify the stable CLI help surface and shim policy

### Modified Capabilities

- `cli-artifact-workflow`: expose artifact workflow commands under the `x-*` experimental naming convention; define compatibility/alias expectations for the old names.
- `cli-research`: align the research CLI entrypoint with the experimental naming convention (`x-research`) instead of a bespoke `ito-research` command.
- `cli-completion`: ensure completions are generated for the preferred visible command surface.
- `experimental-workflow-commands`: align any references to `artifact-experimental-setup` with `x-artifact-experimental-setup` (and document compatibility behavior).
- `qa-testing-area`: update any documented invocations of `ito ralph` to reflect the supported experimental naming and/or alias behavior.
- `projector-conventions`: codify the stable-first CLI policy and deprecation rules.
- `cli-config`: keep config operations under `ito config ...` and deprecate old verb shims.
- `cli-skills`: confirm skills are not part of the supported CLI UX.
- `cli-change`: keep `ito change ...` callable but hidden and deprecated.
- `cli-spec`: keep `ito spec ...` callable but hidden and deprecated.
- `cli-module`: keep `ito module ...` callable but hidden and deprecated; modules remain accessible via stable verbs/flags where supported.

## Impact

- CLI UX: `ito --help` becomes small and stable; experimental commands no longer pollute the primary help surface.
- Compatibility: existing invocations continue to work during a deprecation window (warnings + migration hints); help + completions prioritize the preferred surface.
- Code: command registration and completion registry are updated to match the preferred surface; hidden shims remain callable.
