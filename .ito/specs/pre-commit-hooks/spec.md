<!-- ITO:START -->
# Pre Commit Hooks

## Purpose

This spec defines the current behavior and requirements for pre commit hooks.

## Requirements

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
The pre-commit hook entry documented by Ito SHALL be opt-in for downstream projects and SHALL NOT be installed or edited automatically by `ito init`, `ito update`, or a managed helper skill. Projects MAY copy the documented entry into their chosen hook framework and SHALL verify it with `ito validate repo --staged --strict`.

#### Scenario: Ito init does not write hook configuration
- **WHEN** the user runs `ito init` on a fresh project
- **THEN** the command SHALL NOT modify `.pre-commit-config.yaml`, Husky scripts, or other hook framework files
- **AND** it does not delegate that edit to a retired helper skill

#### Scenario: Project adopts hook explicitly
- **WHEN** a project owner copies or authors an Ito validation hook entry
- **THEN** the edit is reviewed through the project's normal workflow
- **AND** direct `ito validate repo --staged --strict` verification remains available
<!-- ITO:END -->
