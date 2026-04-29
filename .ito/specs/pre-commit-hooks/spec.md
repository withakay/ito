<!-- ITO:START -->
## ADDED Requirements

### Requirement: Repository pre-commit stage runs ito validate repo

The repository's pre-commit hook plumbing SHALL invoke `ito validate repo --staged --strict` at the `pre-commit` stage. The existing `pre-push` quality gate (format, clippy, docs, tests, coverage, guardrails, etc.) SHALL remain unchanged.

- **Requirement ID**: pre-commit-hooks:repo-pre-commit-stage

#### Scenario: prek runs the new hook at pre-commit stage

- **GIVEN** prek hooks are installed (`prek install -t pre-commit`)
- **WHEN** the user runs `git commit`
- **THEN** prek SHALL execute the `ito-validate-repo` hook
- **AND** the hook SHALL invoke `ito validate repo --staged --strict`

#### Scenario: pre-push gate is unaffected

- **WHEN** the user runs `git push`
- **THEN** the existing pre-push hooks (cargo fmt, clippy, docs, test-coverage, arch-guardrails, etc.) SHALL still run

### Requirement: Pre-commit hook stub is replaced with a real call

The previous no-op stub at `ito-rs/tools/hooks/pre-commit` SHALL be replaced with a real invocation of `ito validate repo --staged --strict` when present. The replacement SHALL preserve the script's exit-on-error semantics so a non-zero exit aborts the commit.

- **Requirement ID**: pre-commit-hooks:replace-no-op-stub

#### Scenario: Stub now invokes ito validate repo

- **WHEN** `ito-rs/tools/hooks/pre-commit` is invoked manually
- **THEN** the script SHALL run `ito validate repo --staged --strict`
- **AND** the script's exit code SHALL match `ito validate repo`'s exit code

#### Scenario: AGENTS.md documents the convention change

- **WHEN** an agent reads `ito-rs/AGENTS.md`
- **THEN** the "Git Hooks (prek)" section SHALL state that the `pre-commit` stage now runs `ito validate repo --staged --strict`
- **AND** the section SHALL note that the previous "pre-commit is a no-op" guidance has been superseded for this repo

### Requirement: Pre-commit hook entry is opt-in for downstream projects

The pre-commit hook entry installed by Ito templates SHALL be opt-in for projects that consume Ito; it SHALL NOT be force-installed by `ito init` without an explicit user step (such as the `ito-update-repo` skill running its pre-commit setup step). Other projects SHOULD be able to copy the example into their own pre-commit configuration without taking on Ito's full templates bundle.

- **Requirement ID**: pre-commit-hooks:opt-in-downstream

#### Scenario: ito init does not write to .pre-commit-config.yaml

- **WHEN** the user runs `ito init` on a fresh project
- **THEN** the command SHALL NOT modify `.pre-commit-config.yaml` automatically
- **AND** the command SHALL only print the advisory described by the `ito-init` capability

#### Scenario: ito-update-repo writes the entry after approval

- **WHEN** the user runs the `ito-update-repo` skill and approves the proposed edit
- **THEN** the skill SHALL append the `ito-validate-repo` hook entry to the project's pre-commit configuration
- **AND** the skill SHALL run the verification step described by `ito-update-repo-skill:verify-after-install`
<!-- ITO:END -->
