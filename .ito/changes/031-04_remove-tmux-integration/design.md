<!-- ITO:START -->
## Context

Tmux entered Ito through three independent paths: a bundled agent skill and shell helpers, a `tools.tmux.enabled` preference exposed through config/schema/init, and a `tmux-nvim` proposal viewer. These paths are joined only by tests and conditional suggestions; no proposal, task, validation, archive, or iteration capability depends on them. Because template embedding enumerates whole asset trees, merely hiding the viewer or changing a default would still ship the skill and scripts.

Removal must also account for already-installed managed assets. Ito update preserves user files by default, so deletion from the embedded source is insufficient; the managed legacy manifest must identify obsolete Ito-owned tmux paths and prune them only in update/upgrade cleanup modes.

## Goals / Non-Goals

**Goals:**

- Remove all tmux-owned code, configuration, schema, prompts, skills, scripts, and current documentation.
- Keep non-tmux proposal viewers and external user tmux usage unaffected.
- Remove obsolete Ito-managed tmux assets safely during update/upgrade.
- Prove that standard and all-feature builds contain no tmux integration.

**Non-Goals:**

- Prohibit users or agents from running tmux independently.
- Replace tmux with another terminal multiplexer or interactive session system.
- Change Ralph/loop iteration behavior.
- Rewrite archived changes or changelog history that records the former feature.

## Approach

Remove tmux from the leaves inward so compile failures expose every live dependency. Delete the skill directory and scripts, then remove the viewer module/registry variant, then remove CLI init/view flags and configuration types. Regenerate the schema only after Rust config types are final.

Template and distribution tests will switch from positive tmux assertions to an exact absence contract. Add legacy entries for every formerly installed tmux skill/script/command location across harness adapters. Cleanup continues to use managed-path ownership and `symlink_metadata`, preserving non-Ito paths and user content outside the obsolete managed directory.

The public config deserializer no longer models `tools.tmux.enabled`. Unknown-key behavior follows existing config validation policy: it may surface a removed/unknown key diagnostic, but must never continue to advertise the value as supported or use it to select behavior. Project config cleanup for Ito itself occurs in the final self-migration change.

Current specs are updated through delta specs and the tracked schema is regenerated. Searches treat archived changes and changelog entries as immutable history but require zero live references in Rust source, embedded current assets, generated schema, reference docs, CI, and project configuration after the dependent self-migration batch.

## Contracts / Interfaces

Removed public interfaces:

- Config: `tools.tmux.enabled` and the tmux-only `tools` namespace.
- CLI/init: tmux enable/disable prompt and tmux-specific init flag(s).
- CLI/view: `--viewer tmux-nvim` and any interactive tmux viewer option.
- Installed asset: `ito-tmux/SKILL.md` plus `scripts/wait-for-text.sh` and `scripts/find-sessions.sh` in each harness skill destination.

Unchanged interfaces:

- `ito view proposal` with the remaining editor, pager, HTML/browser, and other registered viewers.
- Worktree, proposal, apply, archive, review, and Ralph/loop commands.
- General user-owned tmux configuration outside Ito-managed paths.

## Data / State

No application data is migrated. The only persistent state is configuration and installed managed files.

| Existing state | Upgrade result |
| --- | --- |
| `tools.tmux.enabled` in config | Reported as removed/unknown; no runtime effect |
| Ito-managed `ito-tmux` directory matching managed ownership | Removed during update/upgrade cleanup |
| Broken symlink at an obsolete managed tmux path | Removed through symlink-aware cleanup |
| User tmux files outside Ito-managed destination | Preserved |
| Archived proposal/spec mentioning tmux | Preserved as history |

## Decisions

- **Delete rather than feature-flag tmux.** It is not experimental Ito functionality and retaining dormant code would not simplify the product.
- **Remove the config namespace when empty.** An empty `tools` contract would imply extension policy Ito no longer needs.
- **Use existing legacy cleanup ownership rules.** Removal must be safe and idempotent across all harness destinations.
- **Keep historical records.** Archives and changelog entries explain past releases and are not active product surfaces.
- **Do not touch iteration.** Ralph/loop stays in the standard product and is verified explicitly.

## Risks / Trade-offs

- Users referencing the removed viewer or config key receive a breaking diagnostic. Release notes and upgrade guidance will name the removal.
- Deleting an embedded directory does not delete installed copies by itself. Exact legacy manifest entries and cross-harness upgrade tests mitigate this.
- Broad text searches can mistake history for live code. Verification scopes active source/config/assets separately and permits references only in immutable history.
- Some generic viewer tests may assume enum ordering. Tests should assert names/capabilities, not incidental indices.

## Verification Strategy

- Config tests prove the tmux type/default is gone and the generated schema has no tmux key.
- CLI init/update tests prove there is no tmux prompt or flag and upgrades handle removed config deterministically.
- Viewer tests prove `tmux-nvim` is not registered or accepted and remaining viewers still probe/open correctly.
- Template/distribution tests assert no tmux skill or script is embedded or installed for any harness.
- Legacy cleanup fixtures cover files, directories, executable scripts, and broken symlinks while preserving user paths.
- Scoped source searches produce zero live tmux references; default and all-feature check/test lanes pass; Ralph/loop smoke tests remain green.

## Migration / Rollback

Ship removal with managed cleanup and release notes in the same release. Users who still want tmux automation install it independently before upgrading. Ito's own config key is removed in `031-06` after the new schema exists.

Rollback is a normal code revert that restores config DTOs, viewer code, assets, and specs. Removed managed assets would be reinstalled on the next Ito update; user-owned tmux state was never modified.

## Open Questions

None. Complete removal, historical preservation, and no replacement integration are approved.
<!-- ITO:END -->
