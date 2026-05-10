<!-- ITO:START -->
## Why

`cli-update` already promises that `ito update` "SHALL only change files that are Ito-managed or marker-managed" (see `specs/cli-update/spec.md` → "Update does not require force"). The project-template installer (`install_project_templates` in `ito-rs/crates/ito-core/src/installers/mod.rs`) routes every write through `write_one`, which honours that contract by extracting the managed block and only updating content between `<!-- ITO:START -->` and `<!-- ITO:END -->`.

The harness manifest installer (`install_manifests` in `ito-rs/crates/ito-core/src/distribution.rs`) does **not** honour that contract. It renders, stamps, and then writes the rendered bytes wholesale via `ito_common::io::write_std`, regardless of whether the user has edited content **outside** the managed block in the installed file. That means today, on every `ito update`, any user edits to:

- `.opencode/skills/<name>/SKILL.md`
- `.claude/skills/<name>/SKILL.md`
- `.codex/skills/<name>/SKILL.md`
- `.github/skills/<name>/SKILL.md`
- `.pi/skills/<name>/SKILL.md`
- the corresponding `.../commands/<name>.md` and `.../prompts/<name>.prompt.md` siblings
- adapter scripts that ship as `.md`

…are silently overwritten. This was surfaced by the codex review of `019-09_ito-update-repo-skill` as a known follow-up.

Now that every shipped harness markdown asset carries an `<!-- ITO:START -->` / `<!-- ITO:END -->` pair, we can finally extend the `cli-update` non-destructive guarantee to the harness install path: write the managed block on update, but leave any user content outside the markers untouched.

## What Changes

- Route harness manifest writes through marker-aware update logic for every markdown asset in the bundle (skills, commands, prompts, agents — everything that already has managed markers after `019-09`).
- Non-markdown manifest entries (shell scripts, JS/TS adapter glue, JSON/YAML configuration) keep the existing wholesale-write behaviour, since they do not (yet) carry managed markers and tooling does not parse them as managed blocks.
- Plumb `InstallMode` (`Init` / `Update`) and `InitOptions` (`force` / `update` / `upgrade`) into `install_manifests` so it can honour the same `--force` / `--update` / `--upgrade` semantics as `install_project_templates`. Without `--force`, an existing managed-marker file is updated marker-scoped only; with `--force`, the file is rewritten wholesale.
- Preserve idempotence: a second `ito update` against an unchanged tree SHALL produce no further changes (managed block stable, version stamp byte-identical, content outside markers untouched).
- Refresh the `<!-- ITO:VERSION:... -->` stamp inside the managed block as part of the marker-scoped update, exactly as `write_one` already does for project templates.
- Add focused tests in `ito-cli/tests/` (or `ito-core/tests/`) that exercise the user-edit-survives-update scenario for at least one harness skill and one harness command.

## Capabilities

### Modified Capabilities

- `cli-update`: extend the existing requirement "Update does not require force" so it explicitly applies to harness manifest installs as well as project templates. Add a new requirement that harness manifest writes preserve user-edited content outside the managed block.

## Impact

- **Code**: `ito-rs/crates/ito-core/src/distribution.rs` (`install_manifests` signature gains mode + opts; new marker-aware write path for managed-marker markdown). Possibly a small refactor that lifts the marker-aware write logic out of `write_one` into a shared helper so both installers can share it without duplicating logic.
- **Callers of `install_manifests`**: `ito-cli/src/app/init.rs` and any other caller pass through the new mode/opts arguments.
- **Tests**: new integration tests proving user content outside the managed block survives `ito update` for harness skills and commands; expanded coverage in `ito-cli/tests/update_smoke.rs` for the harness paths.
- **No template authoring changes** — the managed markers retrofitted in `019-09` are exactly what this change relies on.
- **No spec for skills bundle changes** — only `cli-update` gains a delta.
- **Behavioural change for users**: edits made to installed harness skill/command markdown files outside the managed block now survive `ito update`. This is a fix, not a breaking change. Edits made *inside* the managed block continue to be overwritten on update (as documented).

## Out of Scope

- Adding managed markers to non-markdown harness assets (shell scripts, JS adapters). They are still wholesale-overwritten by manifest install. Tracked separately if needed.
- Stamping non-markdown managed configuration with `ito_version`. Same reason; spec scope is markdown-only for now.
- Changing the rendered template output for any harness asset.
<!-- ITO:END -->
