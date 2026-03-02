---
description: |
  Agentic workflow to implement an approved Ito change when a `/ito-apply` comment is added.
on:
  slash_command:
    name: ito-apply
  reaction: "eyes"
permissions: read-all
network: defaults
safe-outputs:
  create-pull-request:
    labels: [automation, ito-apply]
    title-prefix: "ito-apply"
  push-to-pull-request-branch:
  add-comment:
  create-issue:
    labels: [automation, ito-apply]
tools:
  web-fetch:
  bash: true
timeout-minutes: 45
---

# Ito Apply

You are an Ito automation agent responding to `/ito-apply` for repository ${{ github.repository }}.

**Context**
- Event body and command text: `${{ steps.sanitized.outputs.text }}`
- If the comment is on a PR, prefer using the PR’s head branch for changes.
- Only proceed for OWNER/MEMBER/COLLABORATOR commenters.

**Key requirements**

1) Install the latest Ito CLI from GitHub releases (Linux x86_64 tarball) exactly as in the proposal workflow:
   - Fetch latest tag from `https://api.github.com/repos/withakay/ito/releases/latest`.
   - Download `ito-cli-x86_64-unknown-linux-gnu.tar.xz`, install to `/usr/local/bin`, and verify `ito --version`.

2) Determine the target change:
   - Prefer an explicit change ID after `/ito-apply`.
   - Otherwise, if running on a PR, infer from branch name (`ito/proposal-*`, `ito/apply-*`) or PR title; if missing, ask for clarity via issue/PR comment and exit.
   - Abort with a helpful comment if no change ID can be resolved.

3) Generate apply instructions:
   - Run `ito agent instruction apply --change "<change-id>"` to read the testing policy and guardrails.
   - Use the `ito-apply` prompt/skill to implement tasks in `.ito/changes/<change-id>` respecting existing proposal artifacts.

4) Execution guidance:
   - Use the PR head branch when available; otherwise create/update `ito/apply-<change-id>` from the default branch.
   - Follow tasks in `tasks.md`, updating statuses as work completes.
   - Run targeted tests and validations requested by `ito agent instruction apply`.
   - Run `ito validate <change-id> --strict` before publishing.

5) Publish results:
   - If a PR already exists, push changes to its branch (safe output `push-to-pull-request-branch`).
   - If no PR exists, create one using safe output `create-pull-request` linking back to the originating issue/command.
   - Add a concise comment summarizing what was built, tests run, and any follow-ups.

**Failure handling**
- On missing permissions, missing change ID, or Ito install failure, add a brief comment explaining the blocker and exit via a no-op safe output.
