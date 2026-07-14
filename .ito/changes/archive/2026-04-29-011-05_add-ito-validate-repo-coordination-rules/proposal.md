<!-- ITO:START -->
## Why

Ito's repository structure is implicitly defined by `.ito/config.json`: when `changes.coordination_branch.storage = "worktree"` the canonical artifact directories under `.ito/` MUST be symlinks into the coordination worktree, and the gitignore MUST exclude them; when `worktrees.enabled = true` the user MUST never commit changes from the main/control checkout. Today this is enforced only as a side effect of `ito sync` (which mutates) or `ito worktree validate --change` (single-change scope). Configuration drift therefore lands silently — the very symptom motivating this proposal: in this checkout, `.ito/{changes,specs,modules,workflows,audit}` are real directories, not symlinks, despite the config saying otherwise.

Agents and humans need a non-mutating, configuration-aware **repository guard** that derives a rule set from `ItoConfig`, runs ad hoc, and is callable from a `pre-commit` hook (or a harness pre-tool hook) so structurally invalid commits never land. The check must be fast, machine-readable, and produce What/Why/Fix messages so a stuck agent can self-recover.

## What Changes

- **New CLI subcommand** `ito validate repo` under the existing `ito validate` slice. Existing artifact-content validation (`ito validate <change-id>`, `ito validate --all`, `ito validate module …`) is unchanged; `repo` is added alongside.
- **New rule engine** in `ito-core::validate_repo` (Layer 2) that loads `ItoConfig` once, filters built-in rules by their config gates, and emits `ValidationIssue`s. Rules carry `rule_id`, severity, an explanation tying the issue to its config gate, and a concrete `fix` command. The CLI envelope reuses the existing `ValidationIssue` / `ValidationReport` shape so JSON consumers see one schema across all `ito validate` surfaces.
- **CLI flags** for the `repo` subcommand: `--staged` (read `git diff --cached --name-only` and enable staged-only rules), `--strict` (treat warnings as errors), `--json`, `--rule <id>` / `--no-rule <id>` (repeatable), `--list-rules` (show which rules activate for the current config and which are gated off, with the gating value), `--explain <id>` (show why a single rule activates).
- **Initial rule set (coordination + worktrees only)**:
  - `coordination/symlinks-wired` — when `changes.coordination_branch.storage = "worktree"`: every directory in `coordination::COORDINATION_DIRS` under `.ito/` is a symlink resolving to the coordination worktree. Reuses `coordination::check_coordination_health` and `format_health_message`.
  - `coordination/gitignore-entries` — when storage is `worktree`: `.gitignore` lists each `.ito/<dir>` entry. Source of truth for the desired list is the existing `update_gitignore_for_symlinks` helper, refactored to expose the desired-entries list as a pure function.
  - `coordination/staged-symlinked-paths` — when storage is `worktree` AND `--staged`: no staged files under `.ito/{changes,specs,modules,workflows,audit}` (those belong to the coordination branch, never to main).
  - `coordination/branch-name-set` — always: `changes.coordination_branch.name` is non-empty and matches the `ito/internal/*` naming convention.
  - `worktrees/no-write-on-control` — when `worktrees.enabled = true` AND `--staged`: refuses staged files when the current checkout is the configured `default_branch` / control worktree. Reuses the branch / worktree detection plumbing from `ito-core::worktree_validate`.
  - `worktrees/layout-consistent` — when `worktrees.enabled = true`: `worktrees.layout.dir_name` is non-empty; for `WorktreeStrategy::CheckoutSubdir` the directory is also gitignored.
- **Pre-commit hook integration**:
  - Replace the no-op stub at `ito-rs/tools/hooks/pre-commit` with a real call to `ito validate repo --staged --strict`.
  - Add a matching `pre-commit` (not `pre-push`) entry to `.pre-commit-config.yaml` that runs the same command via `prek`.
  - Update `ito-rs/AGENTS.md` "Git Hooks (prek)" section to reflect that `pre-commit` is no longer a no-op for this repo and to document the override.
- **`ito init` advisory**: after `ito init` (and `ito init --upgrade`) finishes, emit a post-install message **only when** `ito validate repo` would produce non-trivial work (i.e. at least one rule activates from the resolved config). The message points the user at the existing `ito-update-repo` skill / slash command and tells them to invoke it from their harness so the agent finishes setup. The message lists the detected pre-commit system if any (`prek`, `pre-commit`, Husky, lefthook), or notes that none was detected so the user can pick one.
- **Extend the existing `ito-update-repo` skill** (canonical source: `ito-rs/crates/ito-templates/assets/skills/ito-update-repo/SKILL.md`) to cover pre-commit hook setup. The extension:
  - Adds a new "pre-commit hook setup" step after the templates refresh and orphan cleanup.
  - Detects the active pre-commit system by inspecting repo-root markers, in this order: `prek` (`.pre-commit-config.yaml` + a `prek` toolchain installer such as `mise.toml` or environment hint), `pre-commit` (the standard `pre-commit` framework, signaled by `.pre-commit-config.yaml` without prek), Husky (`.husky/` directory or `husky` field in `package.json`), lefthook (`lefthook.yml` / `.lefthook.yml`), or "none detected".
  - For the detected system, applies the appropriate edit:
    - prek / pre-commit: append a `local` hook entry running `ito validate repo --staged --strict` at `pre-commit` stage; replace `ito-rs/tools/hooks/pre-commit` no-op when present.
    - Husky: write a `.husky/pre-commit` script invoking `ito validate repo --staged --strict`.
    - lefthook: add a `pre-commit.commands.ito-validate-repo` entry running the same command.
    - none detected: print a short list of the supported systems and ask the user to pick one (do not auto-install a framework).
  - Always shows a dry-run / approval step before writing.
  - Verifies by running `ito validate repo --staged --strict` once after install and reporting the exit code.
- **Update the matching slash command** (`ito-rs/crates/ito-templates/assets/commands/ito-update-repo.md` / harness equivalents) to mention pre-commit setup as part of the skill's scope so users discover it from the command palette.

<!-- Allowed vocabulary:
  - Type: feature | fix | refactor | migration | contract | event-driven
  - Risk: low | medium | high
  - Stateful: yes | no
  - Public Contract: none | openapi | jsonschema | asyncapi | cli | config (comma-separated when needed)
  - Design Needed: yes | no
  - Design Reason: free text
-->
## Change Shape

- **Type**: feature
- **Risk**: medium
- **Stateful**: yes
- **Public Contract**: cli, config
- **Design Needed**: yes
- **Design Reason**: Crosses CLI surface, a new rule engine in `ito-core`, the templates crate (skill + slash command + project-setup adapter), pre-commit hook conventions, and `ito init` post-install messaging. A short `design.md` is needed to lock down the rule trait, registry filtering, JSON envelope, and the multi-system pre-commit detection algorithm before implementation.

## Capabilities

### New Capabilities

- `validate-repo-engine`: Configuration-aware rule trait, registry, runner, and JSON envelope. Loads `ItoConfig` once, filters rules by their config gates, and emits issues using the existing `ValidationIssue` shape with `rule_id`, `metadata.config_gate`, and `metadata.fix`.
- `validate-repo-coordination-rules`: `coordination/symlinks-wired`, `coordination/gitignore-entries`, `coordination/staged-symlinked-paths`, and `coordination/branch-name-set`. Wraps existing `coordination::check_coordination_health` plumbing as the source of truth.
- `validate-repo-worktrees-rules`: `worktrees/no-write-on-control` and `worktrees/layout-consistent`. Reuses branch / worktree detection from `ito-core::worktree_validate`.
- `validate-repo-cli-surface`: `ito validate repo` subcommand and its flags (`--staged`, `--strict`, `--json`, `--rule`, `--no-rule`, `--list-rules`, `--explain`).
- `pre-commit-hook-detection`: Multi-system pre-commit framework detection (`prek`, `pre-commit`, Husky, lefthook, none) used by the extended `ito-update-repo` skill.

### Modified Capabilities

- `ito-validate-cli`: Adds a `repo` subcommand to the existing `ito validate` clap surface. No change to existing `ito validate <item>`, `ito validate --all`, or `ito validate module …` behaviour.
- `ito-init-post-install`: After `ito init` / `ito init --upgrade`, emits an advisory (only when at least one repo rule would activate) that names the detected pre-commit system and tells the user to invoke the `ito-update-repo` skill in their harness to finish setup.
- `ito-update-repo` (skill + slash command + canonical templates): Adds a "pre-commit hook setup" step with auto-detection of `prek`, `pre-commit`, Husky, and lefthook; writes the appropriate hook entry; verifies with `ito validate repo`. The orphan-cleanup behaviour and templates-refresh behaviour are unchanged.
- `pre-commit-hooks` (`ito-rs/tools/hooks/pre-commit`, `.pre-commit-config.yaml`, `ito-rs/AGENTS.md` text): The `pre-commit` stage is no longer a no-op for this repo. The `pre-push` stage continues to run the full quality gate.

## Impact

- **CLI**: `ito-rs/crates/ito-cli/src/cli/validate.rs` (new `Repo(RepoValidateArgs)` variant on `ValidateCommand`), `ito-rs/crates/ito-cli/src/app/validate_repo.rs` (new adapter), help-text updates in `ito-rs/crates/ito-cli/src/cli.rs`.
- **Core**: `ito-rs/crates/ito-core/src/validate_repo/` (new module tree: `mod.rs`, `rule.rs`, `registry.rs`, `coordination_rules.rs`, `worktrees_rules.rs`, `tests.rs`). Minor refactor of `ito-rs/crates/ito-core/src/coordination.rs` to expose the canonical gitignore-entries list as a pure function.
- **Templates** (Layer 1, the canonical source for installed assets):
  - `ito-rs/crates/ito-templates/assets/skills/ito-update-repo/SKILL.md` — extend with the pre-commit hook setup step and detection table.
  - `ito-rs/crates/ito-templates/assets/commands/ito-update-repo.md` — surface pre-commit setup in the description / Notes block.
  - Harness-equivalent variants under `.opencode/`, `.claude/`, `.codex/`, `.github/`, `.pi/` skills/commands/prompts MUST stay functionally equivalent (per `ito-templates/AGENTS.md`).
  - `ito-rs/crates/ito-templates/assets/instructions/agent/project-setup.md.j2` — add a step pointing at `ito-update-repo` for pre-commit setup.
  - Optionally: `ito-rs/crates/ito-templates/assets/instructions/agent/repo-sweep.md.j2` (or similar) — note `ito validate repo` as a check to run as part of agent housekeeping.
- **Hook plumbing**: `ito-rs/tools/hooks/pre-commit` (replace no-op stub), `.pre-commit-config.yaml` (new `local` hook), `ito-rs/AGENTS.md` (update "Git Hooks (prek)" section).
- **Init flow**: `ito-rs/crates/ito-cli/src/app/init.rs` (or wherever the post-install summary lives) — add the advisory call site that consults the new `validate_repo` engine in "list rules" mode and the pre-commit detection logic.
- **Schemas**: No `ItoConfig` JSON schema change. The rule engine is config-readers only.
- **Docs**: Short addition under `.ito/architecture.md` describing the repository validation rule layer (one paragraph + a link to the `validate_repo` module).
- **Tests**: New unit tests in `ito-core::validate_repo`, integration tests in `ito-cli` for human/JSON output, exit codes, `--list-rules`, `--explain`, and a `--staged` test using `ito-test-support` helpers. The pre-commit detection is unit-tested with synthetic project layouts.
- **Out of scope**: rule suppression files (`.ito/validate-repo-suppressions.toml`), auto-fix (`--fix`), and user-defined custom rules. Reserved for follow-ups.
<!-- ITO:END -->
