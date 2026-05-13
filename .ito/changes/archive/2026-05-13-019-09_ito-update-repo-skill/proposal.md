<!-- ITO:START -->
## Why

`ito init --update` (and its alias `ito update`) refreshes Ito-managed template files in a project, but it is strictly additive: it never deletes skills, commands, or prompts installed by an older version of the CLI. Every rename (e.g. `ito-write-change-proposal` → `ito-proposal`, `ito-tmux` → `tmux`) or deprecation leaves stale assets behind in `.claude/skills/`, `.codex/skills/`, `.github/skills/`, `.opencode/skills/`, and `.pi/skills/`, alongside their matching commands/prompts. Over time a project accumulates dozens of orphans that still appear to agents as valid skills and point at commands that no longer exist.

Users currently need tribal knowledge to clean this up manually. An official skill + command shipped with Ito codifies the workflow: run the non-destructive update, audit for orphans, and remove them with explicit approval.

## What Changes

- Add a new shared skill `ito-update-repo` to the Ito templates bundle. It runs `ito init --update --tools all` non-interactively, diffs the installed harness directories against the templates currently shipped with the CLI, reports orphans (with a rename-hint table), and deletes them after user approval.
- Add a matching shared command `ito-update-repo.md` to the Ito templates bundle. It loads the skill the same way other `ito-*` commands do, including the audit guardrail preamble and the `$ARGUMENTS` untrusted-input contract.
- The skill supports `--dry-run`, `--yes`/`-y`, `--tools <list>`, and `--keep <name>[,<name>]`.
- **Codify the `ito-` prefix rule.** Every Ito-managed skill, command, prompt, and agent in the templates bundle MUST have a basename starting with `ito-` (with the sole exception of the root entrypoint `ito`). Rename the remaining unprefixed assets in-tree: `skills/tmux/` → `skills/ito-tmux/`, `skills/test-with-subagent/` → `skills/ito-test-with-subagent/`, `skills/using-ito-skills/` → `skills/ito-using-ito-skills/`, `agents/opencode/test-runner.md` → `agents/opencode/ito-test-runner.md`. Update internal references (script paths, `test-runner` subagent references) accordingly. Add a test guard that fails CI if an unprefixed asset is introduced.
- **Every Ito-produced markdown file has a managed block.** Retrofit `<!-- ITO:START -->` / `<!-- ITO:END -->` around the Ito-owned body of every markdown asset in the templates bundle (skills, commands, agents, instructions, schema templates, default project files). YAML frontmatter stays above the start marker so parsers are undisturbed. A CI test enforces the pair is present in every shipped markdown file.
- **Stamp every managed-block file with the CLI version.** `ito init` and `ito init --update` MUST inject `<!-- ITO:VERSION: <semver> -->` on the line immediately after `<!-- ITO:START -->` for every managed-block file. The stamp is refreshed idempotently on every update and consumed by the orphan audit to distinguish stale (older-version) assets from orphans (removed-upstream) assets. Non-markdown managed files (YAML/JSON configuration) are out of scope for this change.
- Because skills in `assets/skills/` are distributed by Ito to every harness automatically, no per-harness duplication of the skill body is required.

## Capabilities

### New Capabilities

- `ito-update-repo-skill`: The `/ito-update-repo` skill and its command wrapper — documents the "update then prune" workflow, enumerates harness roots (`.claude/skills/`, `.codex/skills/`, `.github/skills/`, `.opencode/skills/`, `.pi/skills/` and their command/prompt siblings), encodes the rename table for known orphans, and requires user approval before deletion.
- `ito-managed-asset-naming`: The naming rule for Ito-owned template assets (`ito-` prefix, with the bare `ito` root as the only exception). Drives orphan ownership detection — anything without the prefix is treated as user- or third-party-owned and left alone.
- `ito-managed-asset-versioning`: Every managed file emitted by `ito init` / `ito init --update` carries a CLI-version stamp inside its managed region. Stamp format is stable and machine-parseable so tooling can detect staleness cheaply; stamp content is limited to the semver string so no user metadata leaks.

## Impact

- **Templates bundle (new files)**:
  - `ito-rs/crates/ito-templates/assets/skills/ito-update-repo/SKILL.md`
  - `ito-rs/crates/ito-templates/assets/commands/ito-update-repo.md`
- **Templates bundle (renames)**:
  - `skills/tmux/` → `skills/ito-tmux/`
  - `skills/test-with-subagent/` → `skills/ito-test-with-subagent/`
  - `skills/using-ito-skills/` → `skills/ito-using-ito-skills/`
  - `agents/opencode/test-runner.md` → `agents/opencode/ito-test-runner.md`
  - Internal content references updated (helper-script paths, agent name references, `name:` frontmatter fields)
- **Templates bundle (new enforcement)**: a unit test or compile-time check in `ito-templates` fails if any asset under `skills/`, `commands/`, or `agents/<harness>/` violates the prefix rule.
- **CLI code (version stamping)**: `ito-templates` or `ito-cli` gains a render-time shim that injects the current CLI version into managed-markdown markers and into managed YAML/JSON regions before they are written to disk. `ito init --upgrade` is extended so the version comment is always refreshed even when other marker content is unchanged.
- **Behavioural expectation**: the skill defaults to non-destructive `--update` semantics and never passes `--force`. Deletion is always approval-gated unless the user passes `--yes`. Staleness (older version stamp) is reported but never auto-deleted — users rerun `ito init --update` to refresh.
- **Backwards compatibility**: existing projects do not have stamps. On first run after upgrading, Ito treats missing stamps as stale and stamps them during the next update. No content loss.
- **Breaking change for downstream consumers**: scripts that hard-code the paths `.opencode/skills/tmux/`, `.opencode/skills/test-with-subagent/`, `.opencode/skills/using-ito-skills/`, or `agents/opencode/test-runner` will need to switch to the `ito-` prefixed paths. These paths are only referenced inside Ito's own templates; the `ito-update-repo` skill's rename table covers the transition.
<!-- ITO:END -->
