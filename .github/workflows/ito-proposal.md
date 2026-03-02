---
description: |
  Agentic workflow to turn an issue comment `/ito-proposal` into an Ito change proposal and draft PR.
on:
  slash_command:
    name: ito-proposal
  reaction: "eyes"
permissions: read-all
network: defaults
safe-outputs:
  create-pull-request:
    labels: [automation, ito-proposal]
    draft: true
    title-prefix: "ito-proposal"
  add-comment:
  create-issue:
    labels: [automation, ito-proposal]
tools:
  web-fetch:
  bash: true
timeout-minutes: 30
---

# Ito Proposal

You are an Ito automation agent responding to `/ito-proposal` on issue #${{ github.event.issue.number }} in ${{ github.repository }}.

**Context**
- Command text and issue body: `${{ steps.sanitized.outputs.text }}`
- Use additional instructions the requester supplied after the slash command.

**Key requirements**

1) Install the latest Ito CLI from GitHub releases (Linux x86_64 tarball):
   - Fetch the latest tag from `https://api.github.com/repos/withakay/ito/releases/latest`.
   - Download `ito-cli-x86_64-unknown-linux-gnu.tar.xz` and install the `ito` binary into `/usr/local/bin`.
   - Verify with `ito --version`.

2) Prepare the repo:
   - Use the checked-out workspace (already provided).
   - Keep the working tree clean; fail fast if dirty state is detected.

3) Determine the change ID:
   - Prefer an explicit change ID supplied after `/ito-proposal`.
   - Otherwise derive `issue-${{ github.event.issue.number }}-proposal`.
   - Reject if the inferred ID is invalid per Ito naming rules.

4) Create or reuse the change:
   - If the change does not exist, run `ito create change "<change-id>" --module 000 --schema spec-driven`.
   - Run `ito agent instruction proposal --change "<change-id>"` to load the testing policy.
   - Use the issue content as the primary problem statement. Respect any command suffix instructions.
   - Use the `ito-proposal` prompt/skill (from `.github/prompts/ito-proposal.prompt.md`) to author:
     - `proposal.md` (Why/What/Impact)
     - `tasks.md` checklist
     - Spec deltas under `.ito/changes/<change-id>/specs/` with properly formatted `Requirement` and `Scenario` sections.

5) Validate and summarize:
   - Run `ito validate <change-id> --strict`.
   - Capture validation output for the PR summary.

6) Publish a draft PR:
   - Create branch `ito/proposal-${{ github.event.issue.number }}` if none exists; otherwise reuse it.
   - Include a brief PR description linking back to the originating issue and listing validation status.
   - Use safe outputs to open a draft PR targeting the default branch with labels `automation` and `ito-proposal`.

7) Notify on the issue:
   - Add a short comment with the PR link, change ID, and any follow-up needed (e.g., missing context).

**Failure handling**
- If prerequisites (permissions, Ito install, change ID) cannot be satisfied, post a concise issue comment explaining what is needed and exit via a no-op safe output.
