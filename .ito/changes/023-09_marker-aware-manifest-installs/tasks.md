<!-- ITO:START -->
# Tasks for: 023-09_marker-aware-manifest-installs

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 023-09_marker-aware-manifest-installs
ito tasks next 023-09_marker-aware-manifest-installs
ito tasks start 023-09_marker-aware-manifest-installs 1.1
ito tasks complete 023-09_marker-aware-manifest-installs 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Extract a shared marker-aware writer helper

- **Files**: `ito-rs/crates/ito-core/src/installers/mod.rs`, possibly a new `ito-rs/crates/ito-core/src/installers/marker_writer.rs` module
- **Dependencies**: None
- **Action**: Lift the marker-scoped write logic out of `write_one` (lines around 460-620 in the post-019-09 mod.rs) into a small reusable helper, e.g. `pub(crate) fn write_marker_aware(target: &Path, rendered_bytes: &[u8], mode: InstallMode, opts: &InitOptions, ownership: FileOwnership) -> CoreResult<()>`. Keep `write_one`'s signature intact by delegating. Public surface inside the crate only.
- **Verify**: `cargo test -p ito-core` (existing project-template tests still pass with the refactor).
- **Done When**: `write_one` calls the shared helper for the marker-managed branch and existing behaviour is preserved.
- **Requirements**: cli-update:idempotent-non-destructive
- **Updated At**: 2026-04-25
- **Status**: [x] complete

### Task 1.2: Plumb mode + opts into install_manifests

- **Files**: `ito-rs/crates/ito-core/src/distribution.rs`, `ito-rs/crates/ito-core/src/installers/mod.rs` (caller), `ito-rs/crates/ito-cli/src/app/init.rs` (and any other caller surfaced by `cargo check`)
- **Dependencies**: None
- **Action**: Extend `install_manifests` signature with `mode: InstallMode` and `opts: &InitOptions` (or wrap in a small struct if the parameter list gets ugly). Forward them from every caller. Default behaviour for callers that previously passed nothing should match `InstallMode::Init` + default `InitOptions` so existing tests still pass.
- **Verify**: `cargo build --workspace --exclude ito-web`; existing harness install tests still pass.
- **Done When**: `install_manifests` knows whether it is running in init/update mode and whether `--force` / `--update` / `--upgrade` were requested.
- **Requirements**: cli-update:harness-manifest-marker-scoped
- **Updated At**: 2026-04-25
- **Status**: [x] complete

### Task 1.3: Route managed-marker manifest entries through the marker-aware writer

- **Files**: `ito-rs/crates/ito-core/src/distribution.rs`
- **Dependencies**: None
- **Action**: After rendering and stamping a manifest entry, decide the write strategy. If the rendered bytes contain `<!-- ITO:START -->` AND the asset path ends in `.md` (excluding `.md.j2`), call the shared marker-aware writer from Task 1.1. Otherwise fall back to the existing wholesale `ito_common::io::write_std`. Preserve `--force` short-circuit. Preserve script-executable post-step (`ensure_manifest_script_is_executable`).
- **Verify**: `cargo test --workspace --exclude ito-web` is green.
- **Done When**: A managed-marker harness skill installed by the manifest installer is updated marker-scoped, while non-markdown manifest entries continue to be wholesale-written.
- **Requirements**: cli-update:harness-manifest-marker-scoped
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Integration tests for user-edit survival

- **Files**: `ito-rs/crates/ito-cli/tests/update_smoke.rs` (or a sibling `update_harness_marker_scoped.rs`)
- **Dependencies**: None
- **Action**: Add tests proving that:
  1. A user-appended section after `<!-- ITO:END -->` in `.opencode/skills/ito-feature/SKILL.md` survives `ito update`.
  2. A user-appended section after `<!-- ITO:END -->` in `.opencode/commands/ito-loop.md` survives `ito update`.
  3. The same files have their managed block content refreshed (e.g., the `<!--ITO:VERSION:...-->` stamp matches the current CLI version).
  4. Running `ito update` a second time produces zero file modifications (idempotence).
  5. A non-markdown manifest entry (e.g. `.opencode/skills/ito-tmux/scripts/wait-for-text.sh`) is still refreshed to match the bundle.
- **Verify**: `cargo test -p ito-cli update_` passes.
- **Done When**: All five scenarios are covered by passing tests.
- **Requirements**: cli-update:harness-manifest-marker-scoped, cli-update:idempotent-non-destructive
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 2.2: Update SKILL.md doc for ito-update-repo

- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-update-repo/SKILL.md`
- **Dependencies**: None
- **Action**: Tighten the skill's documentation to reflect that the harness install path is now marker-scoped (remove or soften the prior caveat that user edits to harness skills/commands could be overwritten). Add a brief note that user edits OUTSIDE the managed block survive `ito update`, while edits INSIDE the managed block are still overwritten.
- **Verify**: `ito-templates` tests still pass; `rg -n 'wholesale' ito-rs/crates/ito-templates/assets/skills/ito-update-repo/SKILL.md` returns nothing misleading.
- **Done When**: The skill's mental model matches the new install behaviour.
- **Requirements**: cli-update:harness-manifest-marker-scoped
- **Updated At**: 2026-04-25
- **Status**: [ ] pending
<!-- ITO:END -->
