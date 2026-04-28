<!-- ITO:START -->

Run `ito agent instruction project-setup` and follow the guide to configure this project for Ito (stack detection, dev command scaffolding, and marking `.ito/project.md` setup as complete).

OpenCode installs the Ito audit hook plugin at `.opencode/plugins/ito-skills.js`.
The plugin runs `ito audit validate` and `ito audit reconcile` before tool execution.
When the active Ito target is a change and worktrees are enabled, it also runs `ito worktree validate --change <id> --json` before relevant change-work tools so agents do not keep editing the main/control checkout by mistake.

Optional environment flags:
- `ITO_OPENCODE_AUDIT_DISABLED=1` disables the pre-tool audit hook.
- `ITO_OPENCODE_AUDIT_FIX=1` enables `ito audit reconcile --fix` when drift is detected.
- `ITO_OPENCODE_AUDIT_TTL_MS=<milliseconds>` overrides the short audit cache TTL.
- `ITO_OPENCODE_WORKTREE_GUARD_DISABLED=1` disables the pre-tool change-worktree guard.
- `ITO_OPENCODE_WORKTREE_GUARD_TTL_MS=<milliseconds>` overrides the short success-cache TTL for worktree validation.

If the guard blocks you because OpenCode is still running from the main/control checkout, switch into the dedicated change worktree first. If you must debug around the guard temporarily, set `ITO_OPENCODE_WORKTREE_GUARD_DISABLED=1` for that session and re-enable it afterward.

Other guard outcomes:
- A **mismatch** notice means you are outside main/control, but the current branch or worktree path does not start with the full change ID. Switch to the right change worktree or run `ito worktree ensure --change <id>`.
- An **active-change advisory** means OpenCode could not infer the current change. If you are about to do change work, validate explicitly with `ito worktree validate --change <id>`.
- A **validation warning** means the plugin could not parse the validator response. Treat it as a signal to check your current worktree manually before continuing.

<!-- ITO:END -->
