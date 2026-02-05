## Why

`ito init` installs useful scaffolding, but projects still need a deliberate setup step (project context, dev commands, and toolchain preferences). Today that setup is ad-hoc and inconsistent across repos and agent harnesses. A wizard-style “project setup” flow would make Ito onboarding faster, more repeatable, and less error-prone.

## What Changes

- Add a new instruction artifact: `ito agent instruction project-setup`.
- Install a harness command + skill (`/ito-project-setup`) that runs the setup workflow using the agent.
- Update `ito init` to detect “project setup incomplete” from `.ito/project.md` and print a hint to run project setup (without making init interactive).
- Provide Makefile scaffolding as an output of project setup:
  - Targets: `help`, `build`, `test`, `lint`/`check` (stack-specific)
  - Keep `make` defaulting to `help`.
- Include stack detection + interview prompts (runtime, package manager, version manager/environment tooling) and produce stack-appropriate commands.
- Provide a Windows-friendly alternative when `make` is not expected (PowerShell script or equivalent task runner entrypoint).

## Capabilities

### New Capabilities

- `project-setup`: wizard-style project setup guidance that detects stack, interviews for preferences, and outputs dev command scaffolding (Makefile and/or Windows alternative).

### Modified Capabilities

- `agent-instructions`: add the `project-setup` instruction artifact.
- `cli-init`: init hints when project setup is incomplete.
- `tool-adapters`: bootstrap/help content lists the new artifact and the new harness command.
- `docs-agent-instructions`: documentation references the new setup workflow and how to run it.

## Impact

- New schema template for `project-setup` under the `spec-driven` schema templates.
- Template asset updates under `ito-rs/crates/ito-templates/assets/default/project/`:
  - OpenCode: `.../.opencode/commands/ito-project-setup.md`
  - Claude: `.../.claude/commands/ito/project-setup.md`
  - Codex: `.../.codex/prompts/ito-project-setup.md`
  - GitHub: `.../.github/prompts/ito-project-setup.prompt.md`
- CLI behavior change: `ito init` reads `.ito/project.md` for an “incomplete setup” marker and prints a non-fatal hint.
- No breaking CLI flags; the setup workflow is additive and opt-in.
