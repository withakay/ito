<!-- ITO:START -->
# Tasks for: 019-09_ito-update-repo-skill

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 019-09_ito-update-repo-skill
ito tasks next 019-09_ito-update-repo-skill
ito tasks start 019-09_ito-update-repo-skill 1.1
ito tasks complete 019-09_ito-update-repo-skill 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add `ito-update-repo` skill template

- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-update-repo/SKILL.md`
- **Dependencies**: None
- **Action**: Author the SKILL.md using the skill-coach progressive-disclosure pattern. Include the "when to use", "NOT for", the core shibboleth (update is additive, cleanup is separate), stepwise workflow, anti-patterns, and verification section. Frontmatter limited to `name` + `description`.
- **Verify**: File exists; `ito-templates` builds (`cargo build -p ito-templates`). Manual smoke: `grep -q 'ito-update-repo' ito-rs/crates/ito-templates/assets/skills/ito-update-repo/SKILL.md`.
- **Done When**: SKILL.md exists and documents update + orphan audit + approval workflow.
- **Requirements**: ito-update-repo-skill:skill-entrypoint, ito-update-repo-skill:non-destructive, ito-update-repo-skill:orphan-audit, ito-update-repo-skill:approval-gate, ito-update-repo-skill:rerun-idempotent
- **Updated At**: 2026-04-24
- **Status**: [x] complete

### Task 1.2: Add `/ito-update-repo` command template

- **Files**: `ito-rs/crates/ito-templates/assets/commands/ito-update-repo.md`
- **Dependencies**: None
- **Action**: Author the command wrapper following the existing pattern (frontmatter with `name`, `description`, `category: Ito`, `tags`; `<UserRequest>$ARGUMENTS</UserRequest>` block; ITO-managed body that loads the skill; audit guardrail preamble).
- **Verify**: File exists; `ito-templates` builds cleanly.
- **Done When**: Command file parses and matches the shape of sibling `ito-*.md` commands.
- **Requirements**: ito-update-repo-skill:skill-entrypoint, ito-update-repo-skill:distribution
- **Updated At**: 2026-04-24
- **Status**: [x] complete

### Task 1.3: Rename unprefixed template assets to `ito-` prefix

- **Files**: `ito-rs/crates/ito-templates/assets/skills/tmux/` → `.../ito-tmux/`; `assets/skills/test-with-subagent/` → `.../ito-test-with-subagent/`; `assets/skills/using-ito-skills/` → `.../ito-using-ito-skills/`; `assets/agents/opencode/test-runner.md` → `.../ito-test-runner.md`
- **Dependencies**: None
- **Action**: Rename the three unprefixed skill directories and the unprefixed agent file. Update each file's `name:` frontmatter, internal self-references (helper-script paths such as `.opencode/skills/ito-tmux/scripts/...`), references to the `test-runner` subagent (now `ito-test-runner`) in sibling skills/templates, and any mention in `assets/default/project/AGENTS.md`. Run `rg` to catch stragglers.
- **Verify**: `rg -n "(?<!ito-)(^|/)(tmux|test-with-subagent|using-ito-skills|test-runner)(/|\.md|\"|\b)" ito-rs/crates/ito-templates/assets` returns no semantic hits.
- **Done When**: Every asset basename under `assets/{skills,commands,agents}/` satisfies the prefix rule (`ito` or `ito-<suffix>`).
- **Requirements**: ito-managed-asset-naming:prefix-rule, ito-managed-asset-naming:templates-enforce
- **Updated At**: 2026-04-25
- **Status**: [x] complete

### Task 1.4: Add CI guard for the prefix rule

- **Files**: `ito-rs/crates/ito-templates/tests/prefix_rule.rs` (or a unit test inside `src/lib.rs`); possibly `ito-rs/tools/check_prefix.py` if a pre-commit check is preferred
- **Dependencies**: None
- **Action**: Add a test that enumerates every file/directory under `assets/skills/`, `assets/commands/`, and `assets/agents/<harness>/` (via `include_dir!` or a build-time walk) and fails if any basename is not `ito` or does not start with `ito-`. Wire it into `make check`.
- **Verify**: `cargo test -p ito-templates prefix_rule` passes against the renamed tree and fails if a test fixture introduces an unprefixed asset.
- **Done When**: Contributors cannot land an unprefixed Ito asset without updating the allow-list.
- **Requirements**: ito-managed-asset-naming:templates-enforce
- **Updated At**: 2026-04-25
- **Status**: [x] complete

### Task 1.7: Retrofit managed blocks into every templated markdown file

- **Files**: all `*.md` under `ito-rs/crates/ito-templates/assets/{skills,commands,agents,default/project,instructions,adapters,schemas}/` that do not already contain `<!-- ITO:START -->` / `<!-- ITO:END -->`
- **Dependencies**: None
- **Action**: For every shipped markdown file that lacks managed markers, wrap its Ito-owned body in `<!-- ITO:START -->` / `<!-- ITO:END -->`. YAML frontmatter (if present) stays above the start marker. The `<!-- ITO:END -->` sits at the very end of the file (trailing newline preserved). No content re-ordering beyond the wrap. Add a unit test in `ito-templates` that iterates every `*.md` embedded asset and asserts each has exactly one `ITO:START` and one `ITO:END` on their own lines.
- **Verify**: `cargo test -p ito-templates managed_markers_present` passes. `rg -L 'ITO:START' ito-rs/crates/ito-templates/assets --type md` returns zero lines.
- **Done When**: Every markdown file in the templates bundle contains the managed-block pair; the CI test enforces this for new files.
- **Requirements**: ito-managed-asset-versioning:managed-block-everywhere
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 1.5: Implement version stamping in the install pipeline

- **Files**: `ito-rs/crates/ito-templates/src/lib.rs`, `ito-rs/crates/ito-templates/src/project_templates.rs`, `ito-rs/crates/ito-core/src/templates/**`, relevant unit tests
- **Dependencies**: None
- **Action**: Extend the template render/install pipeline so every managed markdown file gets `<!-- ITO:VERSION: <semver> -->` injected immediately after `<!-- ITO:START -->` (idempotent when the version is unchanged), and every managed YAML/JSON region gains an `ito_version` field. Source the version from `env!("CARGO_PKG_VERSION")` (or the existing CLI version helper). Extend `ito init --upgrade` so version stamps are refreshed even when the rest of the managed block is unchanged. Ensure the stamp contains only the semver string — no usernames, hostnames, or timestamps.
- **Verify**: New unit tests assert: (a) a freshly rendered skill contains the stamp, (b) a second render produces a byte-identical file, (c) a managed file whose only drift is an older stamp is updated by `--upgrade`, (d) files outside managed markers are never stamped.
- **Done When**: Every managed file emitted by `ito init` / `ito init --update` is version-stamped and the stamp round-trips cleanly.
- **Requirements**: ito-managed-asset-versioning:stamp-every-output, ito-managed-asset-versioning:stamp-format, ito-managed-asset-versioning:privacy
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 1.6: Teach `ito-update-repo` to report staleness from stamps

- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-update-repo/SKILL.md` (already drafted); any CLI helper the skill delegates to for cheap stamp reads
- **Dependencies**: None
- **Action**: Ensure the skill's audit step classifies files as `current` / `stale` / `missing-stamp` / `orphan` using the stamp. If a programmatic helper is worth adding (e.g., `ito audit stamps` that prints `<path>\t<version>` for every managed file), add it; otherwise have the skill read the first ~20 lines of each managed file directly. Document the `--include-stale` / `--stale-only` toggle if the skill supports narrowing the report.
- **Verify**: Invoking the skill against a project that contains one stale file and one orphan skill produces a report that distinguishes them.
- **Done When**: Stamp-based classification drives the skill's report and no stale file is ever proposed for deletion.
- **Requirements**: ito-managed-asset-versioning:stamp-readable, ito-update-repo-skill:orphan-audit
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Rebuild the CLI and install into a scratch project

- **Files**: none (build + smoke test)
- **Dependencies**: None
- **Action**: `make install`, then in a temp directory run `ito init --tools all` and confirm the new skill + command appear in every harness directory. Then run `ito init --update --tools all` against a project that previously lacked the skill and confirm the skill/command are added.
- **Verify**: New files present under each of `.claude/`, `.codex/`, `.github/`, `.opencode/`, `.pi/`.
- **Done When**: Fresh-init and update paths both install the new assets.
- **Requirements**: ito-update-repo-skill:distribution
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 2.2: End-to-end orphan + staleness dry-run

- **Files**: none (behavioural verification)
- **Dependencies**: None
- **Action**: In this repository (which has known orphans such as `ito-write-change-proposal`, `ito-finishing-a-development-branch`, plus pre-stamp files from before version stamping existed), invoke `/ito-update-repo --dry-run` and confirm: (a) the report covers every harness root, (b) orphans are classified as such with rename hints, (c) pre-stamp files are reported as `missing-stamp` stale rather than orphans, (d) any unprefixed user-owned file is ignored, (e) nothing is deleted.
- **Verify**: Dry-run output clearly distinguishes orphan / stale / missing-stamp / current; `git status` shows no deletions.
- **Done When**: Dry-run output matches the orphan-audit and stamp-readable requirements simultaneously.
- **Requirements**: ito-update-repo-skill:orphan-audit, ito-update-repo-skill:approval-gate, ito-managed-asset-versioning:stamp-readable, ito-managed-asset-naming:prefix-drives-ownership
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 2.3: End-to-end orphan cleanup apply

- **Files**: potentially many orphan files across harness directories (deletions only)
- **Dependencies**: Task 2.2
- **Action**: Invoke `/ito-update-repo` (non-dry-run) on a throwaway branch in this repo, approve the cleanup, and confirm that a rerun reports zero orphans and zero file modifications.
- **Verify**: Second invocation is a no-op; `git status` only shows the deletions the user approved.
- **Done When**: Re-run idempotence holds.
- **Requirements**: ito-update-repo-skill:rerun-idempotent
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Mention the new skill in user-facing docs

- **Files**: `docs/` (wherever the command catalogue lives), `README.md` if applicable
- **Dependencies**: None
- **Action**: Add a one-line entry for `/ito-update-repo` alongside the existing `/ito-*` commands.
- **Verify**: `rg -n "ito-update-repo" docs/` returns at least one hit.
- **Done When**: Command is discoverable in documentation.
- **Requirements**: ito-update-repo-skill:distribution
- **Updated At**: 2026-04-24
- **Status**: [ ] pending
<!-- ITO:END -->
