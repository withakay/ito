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

### Task 1.1: Add new instruction artifact template (project-setup)

- **Files**:
  - `ito-rs/crates/ito-schemas/schemas/spec-driven/` (artifact graph + template)
  - `ito-rs/crates/ito-schemas/schemas/spec-driven/templates/project-setup.md` (new)
- **Dependencies**: None
- **Action**:
  - Define a new instruction artifact `project-setup` in the spec-driven schema.
  - Add a template that guides the agent through:
    - Stack detection (Cargo/package.json/pyproject/go.mod)
    - A short interview for runtime/package manager/version manager
    - Generating a Makefile (help/build/test/lint) without overwriting existing files
    - Generating a Windows alternative when appropriate
    - Updating `.ito/project.md` marker from INCOMPLETE -> COMPLETE
- **Verify**: `make test`
- **Done When**: `ito agent instruction project-setup` renders and includes an output path.
- **Updated At**: 2026-02-01
- **Status**: [ ] pending

### Task 1.2: Add harness command /ito-project-setup (all harnesses)

- **Files**:
  - `ito-rs/crates/ito-templates/assets/default/project/.opencode/commands/ito-project-setup.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.claude/commands/ito/project-setup.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.codex/prompts/ito-project-setup.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.github/prompts/ito-project-setup.prompt.md`
- **Dependencies**: Task 1.1
- **Action**:
  - Add a command prompt that delegates workflow content to:
    - `ito agent instruction project-setup`
  - Ensure wording is consistent across harnesses and matches existing Ito command style.
- **Verify**: `make test`
- **Done When**: `ito init` installs `/ito-project-setup` command for each harness.
- **Updated At**: 2026-02-01
- **Status**: [ ] pending

### Task 1.3: Update `ito init` to hint when setup is incomplete

- **Files**: `ito-rs/` (`ito-cli` init command + project.md installer)
- **Dependencies**: Task 1.1
- **Action**:
  - After init, read `.ito/project.md` (respecting configured itoDir).
  - If it contains `<!-- ITO:PROJECT_SETUP:INCOMPLETE -->`, print a hint:
    - “Run `/ito-project-setup` (or `ito agent instruction project-setup`) to generate your Makefile/dev commands.”
  - Keep behavior non-fatal and non-interactive.
- **Verify**: `make test`
- **Done When**: an integration test asserts the hint is printed only when marker is present.
- **Updated At**: 2026-02-01
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Update docs/bootstrap listings to include project-setup

- **Files**:
  - `ito-rs/crates/ito-templates/assets/default/project/AGENTS.md`
  - `ito-rs/crates/ito-templates/assets/default/project/CLAUDE.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.codex/instructions/*` (if needed)
- **Dependencies**: None
- **Action**:
  - Mention `/ito-project-setup` and `ito agent instruction project-setup`.
  - Briefly describe what outputs it produces (Makefile/PowerShell script + project marker).
- **Verify**: `make test`
- **Done When**: docs in installed templates reference the new setup flow.
- **Updated At**: 2026-02-01
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3 (Checkpoint)

- **Depends On**: Wave 2

### Task 3.1: Human review of default interview + generated Makefile shape

- **Type**: checkpoint (requires human approval before proceeding)
- **Files**:
  - `ito-rs/crates/ito-schemas/schemas/spec-driven/templates/project-setup.md`
- **Dependencies**: Task 2.1
- **Action**: Validate that the interview is short, the Makefile targets match expectations, and Windows guidance is reasonable.
- **Done When**: reviewer approves the template content.
- **Updated At**: 2026-02-01
- **Status**: [ ] pending
