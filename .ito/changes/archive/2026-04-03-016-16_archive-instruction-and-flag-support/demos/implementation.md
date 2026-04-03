# 016-16: archive instruction support

*2026-04-02T00:53:42Z by Showboat 0.6.1*
<!-- showboat-id: 18023758-f114-4c68-9c68-0bc46083b807 -->

Added agent/archive.md.j2 template and wired the archive artifact handler in instructions.rs. The handler supports --change (optional) and falls back to generic guidance when omitted.

```bash
ito agent instruction archive 2>&1 | head -20
```

````output
# Archive Instruction

## Archive a Change

`ito archive` moves a completed change to the archive and merges its spec deltas into the main specs.

### When to archive

- All tasks in `tasks.md` are complete
- The PR has been merged (or the change has been integrated into `main`)
- You want to record the change as done and update the canonical specs

### Commands

```bash
# Archive a specific change
ito archive <change-id> --yes

# Pre-archive audit reconcile (recommended)
ito audit reconcile --change <change-id>
````

```bash
ito agent instruction archive --change 016-16_archive-instruction-and-flag-support 2>&1
```

````output
# Archive Instruction

## Archive: `016-16_archive-instruction-and-flag-support`

Run the following to archive this change:

```bash
# 1. Ensure audit log is in sync before archiving
ito audit reconcile --change 016-16_archive-instruction-and-flag-support

# 2. Archive the change (merges spec deltas into main specs)
ito archive 016-16_archive-instruction-and-flag-support --yes
```

If the reconcile reports drift, fix it first:

```bash
ito audit reconcile --change 016-16_archive-instruction-and-flag-support --fix
```

### Other available changes

- `000-11_normalize-main-spec-formatting`
- `001-25_tracking-file-support`
- `001-30_proposal-viewer-html`
- `009-02_event-sourced-audit-log`
- `016-13_optimize-agent-instructions`
- `019-05_embed-openspec-schemas`
- `019-07_embedded-schema-validation`
- `019-08_proposal-intake-and-schema-routing`
- `022-01_separate-tests-into-foo-tests`
- `024-01_add-shared-state-api`
- `026-01_ito-cleanup`
````
