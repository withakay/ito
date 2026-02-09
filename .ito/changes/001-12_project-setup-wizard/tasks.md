# Tasks for: 001-12_project-setup-wizard

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 001-12_project-setup-wizard
ito tasks next 001-12_project-setup-wizard
ito tasks start 001-12_project-setup-wizard 1.1
ito tasks complete 001-12_project-setup-wizard 1.1
ito tasks show 001-12_project-setup-wizard
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add project setup marker to .ito/project.md template

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.ito/project.md`
- **Dependencies**: None
- **Action**:
  - Add `<!-- ITO:PROJECT_SETUP:INCOMPLETE -->` to the default installed `.ito/project.md`.
  - Document (briefly) that the marker is flipped by project setup.
- **Verify**: `make test`
- **Done When**: freshly initialized projects include the INCOMPLETE marker.
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 1.2: Expand the project-setup instruction to generate dev scaffolding and flip marker

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/project-setup.md.j2`
- **Dependencies**: Task 1.1
- **Action**:
  - Add best-effort stack detection prompts (Cargo/package.json/pyproject/go.mod) and confirm with the user.
  - Add guidance to generate a `Makefile` with `help`, `build`, `test`, `lint`/`check` targets (stack-specific), without overwriting an existing Makefile unless confirmed.
  - Add guidance for a Windows-friendly PowerShell entrypoint mirroring those tasks when appropriate.
  - Add explicit step to change `.ito/project.md` marker from INCOMPLETE -> COMPLETE once setup is done.
- **Verify**: `make test`
- **Done When**: `ito agent instruction project-setup` includes the scaffolding + marker flip steps.
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 1.3: Make `ito init` post-init guidance marker-aware

- **Files**: `ito-rs/crates/ito-cli/src/app/init.rs`
- **Dependencies**: Task 1.1
- **Action**:
  - After template install, read `.ito/project.md`.
  - Print the “Next step: Run /ito-project-setup…” nudge only when the marker indicates INCOMPLETE.
  - Keep behavior non-fatal and non-interactive.
- **Verify**: `make test`
- **Done When**: tests cover both marker-present and marker-absent behavior.
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Update docs/bootstrap snippets to mention project-setup

- **Files**:
  - `ito-rs/crates/ito-templates/assets/adapters/codex/ito-skills-bootstrap.md`
  - `ito-rs/crates/ito-templates/assets/default/project/AGENTS.md`
  - `ito-rs/crates/ito-templates/assets/default/project/CLAUDE.md`
- **Dependencies**: None
- **Action**:
  - Mention `/ito-project-setup` and `ito agent instruction project-setup`.
  - For Codex bootstrap snippet, include `project-setup` in the list of available artifacts without exceeding the snippet size constraint.
- **Verify**: `make test`
- **Done When**: installed docs/snippets reference the setup flow and how to run it.
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3 (Checkpoint)

- **Depends On**: Wave 2

### Task 3.1: Human review of default interview + generated Makefile shape

- **Type**: checkpoint (requires human approval before proceeding)
- **Files**:
  - `ito-rs/crates/ito-templates/assets/instructions/agent/project-setup.md.j2`
- **Dependencies**: Task 2.1
- **Action**: Validate that the interview is short, the Makefile targets match expectations, and Windows guidance is reasonable.
- **Done When**: reviewer approves the template content.
- **Updated At**: 2026-02-08
- **Status**: [ ] pending
