## Why

`ito init` installs useful scaffolding, but projects still need a deliberate setup step (project context, dev commands, and toolchain preferences). Today that setup is ad-hoc and inconsistent across repos and harnesses. Ito already has a `project-setup` instruction artifact, but it does not yet produce consistent, stack-appropriate dev command scaffolding or a machine-checkable “setup complete” signal.

## What Changes

- Add a machine-checkable project setup marker to the installed `.ito/project.md` template:
  - `<!-- ITO:PROJECT_SETUP:INCOMPLETE -->` (default)
  - `<!-- ITO:PROJECT_SETUP:COMPLETE -->`
- Upgrade the `ito agent instruction project-setup` workflow to:
  - Do best-effort stack detection (Cargo/package.json/pyproject/go.mod) and confirm with the user.
  - Produce dev command scaffolding that matches the detected stack.
  - Generate a `Makefile` with targets `help`, `build`, `test`, `lint`/`check` (stack-specific) without overwriting an existing `Makefile` unless explicitly confirmed.
  - Provide a Windows-friendly alternative (PowerShell entrypoint) when appropriate.
  - Flip the marker in `.ito/project.md` from INCOMPLETE -> COMPLETE when setup is finished.
- Update `ito init` post-init messaging to be marker-aware:
  - Print the “Run /ito-project-setup” nudge only when `.ito/project.md` indicates setup is incomplete.
  - Keep init non-interactive and non-fatal.
- Update installed docs/bootstrap snippets to mention `project-setup` and how to run it.

## Capabilities

### New Capabilities

- `project-setup`: wizard-style project setup guidance that detects stack, interviews for preferences, and outputs dev command scaffolding (Makefile and/or Windows alternative).

### Modified Capabilities

- `cli-init`: init hints when project setup is incomplete.
- `tool-adapters`: bootstrap/help content lists the new artifact and how to run it.
- `docs-agent-instructions`: documentation references the new setup workflow and how to run it.

## Impact

- Update installed project templates:
  - Add setup marker to `ito-rs/crates/ito-templates/assets/default/project/.ito/project.md`.
  - Ensure the command stubs exist and remain consistent across harnesses:
    - `ito-rs/crates/ito-templates/assets/default/project/.opencode/commands/ito-project-setup.md`
    - `ito-rs/crates/ito-templates/assets/default/project/.claude/commands/ito-project-setup.md`
    - `ito-rs/crates/ito-templates/assets/default/project/.codex/commands/ito-project-setup.md`
- Update the instruction artifact content at `ito-rs/crates/ito-templates/assets/instructions/agent/project-setup.md.j2`.
- Update CLI behavior:
  - `ito-rs/crates/ito-cli/src/app/init.rs` prints the setup nudge only when the marker indicates INCOMPLETE.
- Update adapter/bootstrap docs:
  - Codex bootstrap snippet: `ito-rs/crates/ito-templates/assets/adapters/codex/ito-skills-bootstrap.md`.
- No breaking CLI flags; the setup workflow remains additive and opt-in.
