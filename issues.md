# Worktree Symlink Recovery Notes

Date: 2026-04-30

## Summary

While creating a new Ito change in a fresh worktree, `ito create change` failed before it could scaffold the change. The first failure mode was clearly caused by missing `.ito` coordination symlinks in the new worktree. After wiring those symlinks with `ito init --update --tools none`, `ito create change` still failed with a generic `I/O error: No such file or directory`, so a second issue likely exists beyond the missing symlinks.

This file records the exact commands and observations from the session so future agents can recover more quickly and so Ito can be improved to self-heal or auto-wire worktrees.

## Expected Baseline

In the main worktree, these paths are symlinks:

- `.ito/audit -> /Users/jack/.local/share/ito/withakay/ito/.ito/audit`
- `.ito/changes -> /Users/jack/.local/share/ito/withakay/ito/.ito/changes`
- `.ito/modules -> /Users/jack/.local/share/ito/withakay/ito/.ito/modules`
- `.ito/specs -> /Users/jack/.local/share/ito/withakay/ito/.ito/specs`
- `.ito/workflows -> /Users/jack/.local/share/ito/withakay/ito/.ito/workflows`

Those symlinks were missing in newly created worktrees until `ito init --update --tools none` was run inside each worktree.

## Commands Tried

### 1. Confirm worktree layout guidance

Read guidance from:

- `AGENTS.md`
- `.ito/AGENTS.md`

Observed rule: create a temporary proposal worktree first when no final change ID exists yet, then create the final change worktree before editing generated artifacts.

### 2. Create a temporary proposal worktree manually

Workdir:

- `<repo-root>`

Command:

```bash
git worktree add "<repo-root>/ito-worktrees/proposal-ddd-workflow" -b proposal-ddd-workflow main
```

Result:

- Worktree was created successfully.
- No `.ito` coordination symlink wiring happened automatically.

### 3. Try to create the change immediately in the temporary worktree

Workdir:

- `<repo-root>/ito-worktrees/proposal-ddd-workflow`

Command:

```bash
ito create change "add-ddd-discovery-workflow" --module 001 --schema spec-driven
```

Result:

- Ito printed warnings that the following were regular directories rather than symlinks to the coordination worktree:
  - `.ito/changes`
  - `.ito/specs`
  - `.ito/modules`
  - `.ito/workflows`
  - `.ito/audit`
- Ito then failed with:

```text
Error: Module '001' not found
```

Conclusion:

- Missing coordination symlinks prevented the worktree from seeing shared module/change/spec state.

### 4. Ask Ito how worktree setup is supposed to work

Workdir:

- `<repo-root>/ito-worktrees/proposal-ddd-workflow`

Commands:

```bash
ito worktree --help
ito agent instruction worktree-init
```

Result:

- `ito worktree` advertised `ensure`, `setup`, and `validate`.
- `ito agent instruction worktree-init` documented how to use `ito worktree ensure --change <change-id>`.
- The instruction did **not** explain how to wire `.ito` coordination symlinks in a temporary proposal worktree created via raw `git worktree add`.

Conclusion:

- The documented worktree path does not appear to cover the temporary proposal worktree recovery case.

### 5. Compare main worktree vs temporary worktree symlink state

Workdirs:

- Main: `<repo-root>/main`
- Temp: `<repo-root>/ito-worktrees/proposal-ddd-workflow`

Command used in both places:

```bash
ls -ld ".ito/changes" ".ito/specs" ".ito/modules" ".ito/workflows" ".ito/audit"
```

Result:

- Main worktree: all five paths were symlinks.
- Temporary proposal worktree: most of the paths were absent, and `.ito/modules` existed only as a regular directory.

Conclusion:

- Fresh worktrees were not inheriting or creating the expected coordination links.

### 6. Try Ito init as a recovery step inside the temporary worktree

Workdir:

- `<repo-root>/ito-worktrees/proposal-ddd-workflow`

Command:

```bash
ito init --update --tools none
```

Result:

- Ito initialized the worktree.
- Afterward, the same `ls -ld` check showed all expected `.ito/*` coordination symlinks were present.

Conclusion:

- `ito init --update --tools none` is an effective manual recovery step for missing `.ito` coordination symlinks in a fresh worktree.

### 7. Re-sync and retry after symlink recovery in the temporary worktree

Workdir:

- `<repo-root>/ito-worktrees/proposal-ddd-workflow`

Commands:

```bash
ito sync
ito list --modules
ito create change "add-ddd-discovery-workflow" --module 001 --schema spec-driven
```

Result:

- `ito sync` succeeded.
- `ito list --modules` showed module `001_workflow-enhancements` correctly.
- `ito create change` still failed, but now with a generic error instead of the symlink/module warning:

```text
Error: I/O error: No such file or directory (os error 2)
```

Conclusion:

- Fixing the symlinks solved the first problem, but there is likely a second bug in `ito create change` for this workflow.

### 8. Try Ito path helpers to inspect configured locations

Workdir:

- `/Users/jack/Code/withakay/ito/main`

Command:

```bash
ito path project-root && ito path worktrees-root && ito path worktree-root
```

Result:

- `project-root` -> `<repo-root>`
- `worktrees-root` -> `<repo-root>/ito-worktrees`
- `worktree-root` -> `<repo-root>/main`

Conclusion:

- Path configuration matched the documented bare/control layout.

### 9. Try Ito-managed final change worktree creation

Workdir:

- `<repo-root>/ito-worktrees/proposal-ddd-workflow`

Command:

```bash
ito worktree ensure --change "001-34_add-ddd-discovery-workflow"
```

Result:

- Ito created the final worktree path:

```text
<repo-root>/ito-worktrees/001-34_add-ddd-discovery-workflow
```

- But immediately after creation, the new final worktree still had no `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, or `.ito/audit` paths.

Conclusion:

- `ito worktree ensure` appears to create the worktree directory/branch, but it did **not** wire the expected coordination symlinks in this run.

### 10. Try Ito init as a recovery step inside the final change worktree

Workdir:

- `<repo-root>/ito-worktrees/001-34_add-ddd-discovery-workflow`

Command:

```bash
ito init --update --tools none
```

Result:

- Ito initialized the worktree.
- The expected `.ito/*` coordination symlinks were created successfully.

Conclusion:

- The same recovery step works in both temporary and final worktrees.

### 11. Retry change creation inside the final change worktree after recovery

Workdir:

- `<repo-root>/ito-worktrees/001-34_add-ddd-discovery-workflow`

Command:

```bash
ito create change "add-ddd-discovery-workflow" --module 001 --schema spec-driven
```

Result:

- Same generic error remained:

```text
Error: I/O error: No such file or directory (os error 2)
```

Conclusion:

- Even with symlinks fixed, `ito create change` still needs deeper debugging.

### 12. Workaround used to proceed

Workaround:

- Manually scaffolded `.ito/changes/001-34_add-ddd-discovery-workflow/...`
- Updated `.ito/modules/001_workflow-enhancements/module.md`
- Ran:

```bash
ito validate 001-34_add-ddd-discovery-workflow --strict
ito audit reconcile --change 001-34_add-ddd-discovery-workflow
ito audit validate --change 001-34_add-ddd-discovery-workflow
```

Result:

- Validation and audit passed.

## What We Learned

### Reliable manual recovery step

If a new worktree is missing `.ito` coordination symlinks, this recovered them in both test cases:

```bash
ito init --update --tools none
```

### Important distinction

- Raw `git worktree add ...` created a usable Git worktree but not a fully wired Ito worktree.
- `ito worktree ensure --change <id>` also did not appear to wire the `.ito` coordination symlinks in this session.
- `ito create change ...` could not recover from the missing symlinks automatically.

## Product Gaps To Fix

### Gap 1: Fresh worktrees are not Ito-ready

Ideal behavior:

- A worktree created for Ito work should already contain the expected `.ito/*` coordination symlinks.

Potential fixes:

- Make `ito worktree ensure` run the equivalent of the symlink-wiring part of `ito init --update --tools none` automatically.
- Add a dedicated `ito worktree repair` or `ito worktree init` command that only wires coordination state for the current worktree.

### Gap 2: Temporary proposal worktree path is underspecified

Ideal behavior:

- The documented temp-worktree flow should include the exact recovery or setup command needed after raw `git worktree add`.

Potential fixes:

- Update `ito agent instruction worktree-init` to document the required follow-up step for temporary proposal worktrees.
- If raw `git worktree add` remains the recommended temp-worktree path, explicitly mention running:

```bash
ito init --update --tools none
```

### Gap 3: `ito create change` error is not actionable

Ideal behavior:

- If `.ito` symlinks are missing, `ito create change` should either repair them or fail with a direct hint.

Potential fixes:

- Detect missing coordination symlinks and print:

```text
This worktree is missing Ito coordination symlinks. Run `ito init --update --tools none` in this worktree, then retry.
```

- Better: offer automatic repair before proceeding.

### Gap 4: Generic `os error 2` remains after symlink recovery

Ideal behavior:

- `ito create change` should report the exact missing path and operation.

Potential fixes:

- Improve the error context in the create-change path so the message answers:
  - what path was missing
  - during which step
  - how to fix it

## Suggested Future Repro Checklist

For the next debugging pass, try this exact sequence and capture the first failing path in Rust logs if available:

```bash
git worktree add "<temp-worktree>" -b "<temp-branch>" main
cd "<temp-worktree>"
ls -ld .ito/changes .ito/specs .ito/modules .ito/workflows .ito/audit
ito init --update --tools none
ls -ld .ito/changes .ito/specs .ito/modules .ito/workflows .ito/audit
ito sync
ito list --modules
ito create change "<name>" --module 001 --schema spec-driven
```

Then separately:

```bash
ito worktree ensure --change "<change-id>"
cd "$(ito path worktree --change <change-id>)"
ls -ld .ito/changes .ito/specs .ito/modules .ito/workflows .ito/audit
ito init --update --tools none
ito create change "<name>" --module 001 --schema spec-driven
```

## Short Recommendation

The most useful short-term fix for agents is:

1. After any new worktree is created, run `ito init --update --tools none` in that worktree.
2. If `ito create change` still fails, improve its error reporting to expose the missing path.
3. Long term, make `ito worktree ensure` create a fully wired Ito worktree by default.
