<!-- ITO:START -->
## Context

`ito-config` already exposes a fully-typed `ItoConfig` (`ito-rs/crates/ito-config/src/config/types.rs`), and `ito-core::coordination` already implements a structured health check (`check_coordination_health` returning a `CoordinationHealthStatus` enum) plus an actionable message formatter (`format_health_message`). `ito sync` runs that check before pushing, but only as a precondition for a mutating operation. `ito worktree validate --change <id>` is a per-change read-only check used by OpenCode pre-tool hooks. Neither is a general-purpose configuration-aware repository guard, and there is currently no callable surface for a pre-commit hook to ask "does my checkout match what `.ito/config.json` says it should?".

The `ito validate` subcommand owns artifact-content validation today (spec deltas, change tracking, requirement traceability) via `ito-core::validate`. Issues are emitted using a stable `ValidationIssue` shape with optional `rule_id` and `metadata` fields — the same shape the new repository rules will reuse so JSON consumers (CI, hooks, dashboards) see one schema across all `ito validate` surfaces.

## Goals / Non-Goals

**Goals:**

- A new subcommand `ito validate repo` that loads `ItoConfig`, derives a set of active rules from its values, runs them, and prints a What/Why/Fix report.
- Reuse the existing `ValidationIssue` / `ValidationReport` envelope. No new JSON schema.
- A `--staged` mode that integrates with git's index for use in pre-commit hooks.
- An extension to the existing `ito-update-repo` skill (canonical source in `ito-templates`) that detects which pre-commit framework the project uses and writes the appropriate hook entry.
- A post-install advisory in `ito init` / `ito init --upgrade` that fires only when `ito validate repo` would activate at least one rule and points the user at `ito-update-repo`.

**Non-Goals:**

- Auto-fixing repository drift (defer; `ito sync` and `ito init` already cover the symlink case).
- A user-defined rule API (defer; v1 ships built-in rules only).
- Suppression files (`.ito/validate-repo-suppressions.toml`); deferred — `--no-rule <id>` is the v1 escape hatch.
- Replacing `ito worktree validate --change` (kept; it serves a narrower purpose for harness pre-tool hooks).
- Scope creep into audit / repository / backend rules — those land in change `011-06`.

## Approach

The work splits cleanly across the existing layering:

- **Layer 0 (`ito-config`)**: no changes. The typed `ItoConfig` is the single configuration source the engine reads.
- **Layer 2 (`ito-core::validate_repo`, new module)**: defines a `Rule` trait, a `RuleRegistry` of built-in rules, and a `RepoValidator` that filters rules by their config gates and runs them. Each rule produces zero or more `ValidationIssue`s with a stable `rule_id`. Coordination + worktree rules are implemented here. The module also owns the `--staged` plumbing: it accepts an optional `StagedFiles` snapshot (a thin newtype around the output of `git diff --cached --name-only -z`) and only activates `*-staged-*` rules when one is provided.
- **Layer 3 (`ito-cli`)**: a new `Repo(RepoValidateArgs)` variant on `ValidateCommand`, a new adapter at `app/validate_repo.rs`, and the human / JSON renderers. The renderer reuses the existing helpers in `app/validate.rs` for issue formatting where possible.
- **Layer 1 (`ito-templates`)**: extend the canonical `ito-update-repo` skill at `assets/skills/ito-update-repo/SKILL.md` with a "Pre-commit hook setup" section. The pre-commit detection algorithm is documented in the skill prose so the agent (any harness) follows it. The same content is reflected in the harness command shells (`assets/commands/ito-update-repo.md` and the per-harness equivalents under `.opencode/`, `.claude/`, `.codex/`, `.github/`, `.pi/` — kept in sync per `ito-templates/AGENTS.md`).

The `ito init` advisory is emitted from `ito-cli/src/app/init.rs` (or the post-install summary site, TBD during implementation). It calls `validate_repo::list_active_rules(config)` (read-only) and the pre-commit-detection helper (also exposed from `ito-core::validate_repo`) so the message states the detected system. This keeps the detection logic in one place and shared between the skill's instructional prose and the init advisory's runtime behaviour.

## Contracts / Interfaces

### CLI

```
ito validate repo [OPTIONS]

Options:
  --staged                Read git index and enable staged-only rules.
  --strict                Treat warnings as errors.
  --json                  Machine-readable JSON envelope (matches `ito validate <item> --json`).
  --rule <id>             Run only this rule (repeatable).
  --no-rule <id>          Skip this rule (repeatable). Mutually exclusive with --rule.
  --list-rules            Print active rules and skipped rules with their gating config values, then exit 0.
  --explain <id>          Print the gating config values for a single rule, then exit 0.
```

Exit codes:

| Code | Meaning |
|---|---|
| 0 | All active rules passed (or warnings without `--strict`). |
| 1 | One or more rules emitted an error (or warning under `--strict`). |
| 2 | Usage error or unloadable config. |

### `ito-core::validate_repo` public API (new)

| Item | Shape |
|---|---|
| `RuleId` | newtype around `&'static str` (e.g. `"coordination/symlinks-wired"`). |
| `RuleSeverity` | enum: `Error`, `Warning`, `Info`. Maps to existing `ValidationLevel`. |
| `Rule` (trait) | `fn id(&self) -> RuleId; fn severity(&self) -> RuleSeverity; fn is_active(&self, ctx: &RuleContext) -> bool; fn check(&self, ctx: &RuleContext) -> Vec<ValidationIssue>;` |
| `RuleContext` | bundle: `&ItoConfig`, `&Path` (project_root), `&Path` (ito_path), `Option<&StagedFiles>`. |
| `RuleRegistry::built_in()` | returns `Vec<Box<dyn Rule>>` containing all built-in rules. |
| `run_repo_validation(config, project_root, ito_path, options) -> ValidationReport` | drives the pipeline. |
| `list_active_rules(config) -> Vec<ActiveRule>` | read-only introspection used by `--list-rules` and `ito init`. |
| `detect_pre_commit_system(project_root) -> PreCommitSystem` | enum: `Prek`, `PreCommit`, `Husky`, `Lefthook`, `None`. |

`ValidationIssue.rule_id` and `ValidationIssue.metadata` are already optional today — the engine sets them on every issue it emits.

### Pre-commit-system detection (single source of truth)

Probed in this order; first match wins:

| System | Marker(s) |
|---|---|
| `Prek` | `.pre-commit-config.yaml` AND any of: `prek` on `PATH`, `mise.toml` mentioning `prek`, or `.pre-commit-config.yaml` containing a `prek:` toolchain hint. |
| `PreCommit` | `.pre-commit-config.yaml` (without prek markers). |
| `Husky` | `.husky/` directory OR `package.json` with a `husky` key. |
| `Lefthook` | `lefthook.yml`, `lefthook.yaml`, `.lefthook.yml`, or `.lefthook.yaml` at repo root. |
| `None` | none of the above. |

The skill applies the appropriate edit per system; the engine only **detects**, it does not write hook entries. Writing is the skill's job (so the user gets a dry-run + approval step).

## Data / State

The engine is **stateless** between invocations. Inputs:

- `ItoConfig` from `load_cascading_project_config(...).merged`.
- Filesystem reads under the project root (existence checks, symlink resolution, `.gitignore` content).
- For `--staged`: `git diff --cached --name-only -z` output, parsed into a sorted set of project-relative paths.

No new on-disk state is introduced. No audit events are written by validation runs (read-only).

## Decisions

| # | Decision | Rationale |
|---|---|---|
| 1 | Place the subcommand under `ito validate repo`, not `ito check`/`ito doctor`. | One verb, one envelope, less to teach; user explicitly chose this option. |
| 2 | Reuse `ValidationIssue` / `ValidationReport`. | JSON consumers already parse this shape. `rule_id` and `metadata` already exist. |
| 3 | `pre-commit` is the default hook stage for this repo. | User explicit; the existing `pre-push` quality gate stays. The `ito-rs/tools/hooks/pre-commit` no-op stub is replaced and `ito-rs/AGENTS.md` is updated. |
| 4 | The engine **detects** the pre-commit system but **never writes** hook entries. | Writing is left to the `ito-update-repo` skill so the agent can present a diff + approval step. Keeps the engine read-only and side-effect-free. |
| 5 | Extend the existing `ito-update-repo` skill instead of creating a new slash command. | The skill is already the documented entry point for "fix up my Ito repo"; adding a step is cheaper and discovers naturally. |
| 6 | `backend/token-not-committed`-style rules live in change `011-06`. | Keeps `011-05` reviewable; B builds on the engine without engine churn. |
| 7 | `--staged` reads the git index directly via `git diff --cached --name-only -z`. | No new dependencies. Output is null-byte-delimited so newlines in paths are safe. |
| 8 | `list_active_rules` is exposed publicly so `ito init` can render the advisory without re-implementing config gating. | Single source of truth for "would the engine do anything here?" |

## Risks / Trade-offs

- **Pre-commit performance**: must complete under ~150 ms cold-cache to avoid annoying developers. Mitigation: read config once, cache symlink resolution, run rules sequentially (the rule set is small and IO-bound — parallelism not needed in v1).
- **Over-fitting the detection table**: `prek` and `pre-commit` share `.pre-commit-config.yaml`. False positives on prek would write a prek-shaped hook into a plain `pre-commit` repo. Mitigation: prek detection requires an explicit secondary marker; ship targeted unit tests for each combination.
- **`ito init` advisory noise**: if every init prints the message, users tune it out. Mitigation: gate on `list_active_rules(config).len() > 0` AND "no pre-commit hook for `ito validate repo` is already installed" so a fully-set-up project stays quiet.
- **Convention break for pre-commit**: the project's pre-commit was intentionally a no-op. The override is documented in `ito-rs/AGENTS.md` as part of this change so future agents know.
- **Cross-platform symlinks**: Windows uses NTFS junctions via `junction::create` (already in use by `coordination::create_dir_link`). The rules read symlinks via `read_link` which works on both. No new platform code is added.

## Verification Strategy

- **Per-rule unit tests** in `ito-core::validate_repo` (one fixture per rule covering active/inactive and pass/fail).
- **Engine integration test** with a temp-dir project plus a fabricated `ItoConfig`; asserts the active-rule set across config permutations.
- **CLI snapshot tests** in `ito-cli` (using `ito-test-support` PTY helpers and snapshot normalisation): human output, `--json`, `--list-rules`, `--explain`, exit codes.
- **`--staged` test** with a temp git repo, a synthetic index, and assertions against `coordination/staged-symlinked-paths` and `worktrees/no-write-on-control`.
- **Pre-commit detection test**: each of `Prek`, `PreCommit`, `Husky`, `Lefthook`, `None` is exercised via fixture project layouts.
- **Self-test on this repo**: after wiring the new pre-commit hook, run `prek run --all-files --hook-stage pre-commit ito-validate-repo` and confirm exit 0 in CI.
- **Coverage policy**: per `ito-rs/AGENTS.md`, hard floor 80%, target 90%.

## Migration / Rollback

- **Migration**: zero data migration. Existing `ito validate <item>` invocations are unchanged. The `pre-commit` hook is added but the existing `pre-push` gate is preserved. The `ito-rs/tools/hooks/pre-commit` no-op stub is replaced; if the change is reverted, the stub is restored verbatim from git history.
- **Rollback**: revert the merge commit. The new module is fully additive; the only edits to existing files are: the `ValidateCommand` enum (one new variant), `.pre-commit-config.yaml` (one new hook), `ito-rs/tools/hooks/pre-commit` (one-line replacement), `ito-rs/AGENTS.md` text, and the `coordination.rs` minor refactor exposing `gitignore_entries()` as pure. All revert cleanly.
- **Backwards compat**: agents and humans on older Ito CLIs are unaffected — they simply do not have the `ito validate repo` subcommand. The pre-commit hook entry refers to the local `ito` binary; if missing, `prek` reports the failure clearly.

## Open Questions

- Should `--list-rules` also print the **rule severity** alongside the gate value? (Lean yes; cheap to add.)
- Where does the `ito init` advisory live: directly in `app/init.rs` or in a dedicated `app/init_advisory.rs` helper? Decide during implementation; keep `init.rs` thin.
- Does `ito-update-repo`'s pre-commit-setup step belong in a sub-skill (`ito-update-repo/pre-commit-setup/SKILL.md`) or remain inline in the main `SKILL.md`? Inline keeps discoverability higher; revisit if the section exceeds 60 lines.
<!-- ITO:END -->
