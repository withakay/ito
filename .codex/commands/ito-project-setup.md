<!-- ITO:START -->
<!--ITO:VERSION:0.1.30-->

Run `ito agent instruction project-setup` and follow the guide to configure this project for Ito (stack detection, dev command scaffolding, and marking `.ito/project.md` setup as complete).

Audit guardrail for Codex sessions:

- Follow `.codex/instructions/ito-audit.md` before stateful work.

When your harness is working on an Ito change from a dedicated worktree, validate the current checkout before change work and use the same recovery flow across sessions:
- Validate explicitly with `ito worktree validate --change <id>` when you need to confirm the active change worktree.
- If you are in the wrong checkout, create or move into the dedicated change worktree with `ito worktree ensure --change <id>`.
- A **mismatch** means you are outside main/control, but the current branch or worktree path does not start with the full change ID.
- An **active-change advisory** means the harness could not infer the current change; validate explicitly before proceeding with change work.
- A **validation warning** means the validator response could not be interpreted cleanly; check the current worktree manually before continuing.

If your harness exposes a temporary session-level guard disable for debugging, use it only while diagnosing the issue and re-enable it immediately afterward.

<!-- ITO:END -->
