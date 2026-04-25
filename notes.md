### We should think about dating the skills for creating a proposal such that it respects the sch...

We should think about dating the skills for creating a proposal such that it respects the schemas and suggests an appropriate change schema, so a change type, I guess, from those that are available.


---

### Let's see how fast this is. Maybe we need to think about which model it uses. If it's fast, I...

Let's see how fast this is. Maybe we need to think about which model it uses. If it's fast, I would like it to actually possibly improve what's been said.


---

---
## 2026-02-25 07:17:32 UTC - Reply with exactly noted

After the tool succeeds, reply with exactly: noted


---

---
## 2026-02-25 07:21:21 UTC - One more check

Okay, one more check. It's probably something that I could have said here, but I'm going to.

If empty or whitespace-only, output exactly: no text
If not empty, use the `note_append` tool and output exactly the tool result.


---

---
## 2026-02-25 07:21:46 UTC - user-note

Is this quicker? I think this is quicker.


---

### agent-instruction perf proposal

Change idea: 016-13_optimize-agent-instructions
- Goal: speed up `ito agent instruction apply` by removing default blocking `git fetch` and caching cascading config per invocation.
- Specs: agent-instructions, change-coordination-branch, cascading-config.
- Measured: apply ~1.35s vs others ~15ms; fetch is main cost.


---

---
## DAW config schema idea
## 2026-03-07 06:24:06 UTC

Let's think about adding a new schema for a DAW type, like a super minimal schema for fixes related to configuration of the repository and CI/CD that aren't strictly features of the application.


---

---
## Coding convention test file separation
## 2026-03-07 12:57:50 UTC

If a code file is more than 300 lines long, ALWAYS put the tests in a separate file (e.g., `foo_tests.rs` or `tests/foo.rs`). Do not inline `#[cfg(test)] mod tests` in files exceeding 300 lines.


---

---
## I want to explore extending the OpenCode plugin
## 2026-04-24 08:07:23 UTC

I want to explore extending the OpenCode plugin. In particular, I want to make it Ito change-aware and Ito worktree-aware. I would like to be able to update the OpenCode session title based on the change we're working on, such as the module number or change ID. I also want to explore injecting continuation prompts to help the orchestrator and the apply agents keep things ticking.

There's some prior art to look at here in the Oh My OpenCode Slim project.


---

---
## OpenCode Ito integration ideas
## 2026-04-24 08:07:31 UTC

I want to explore extending the OpenCode plugin. In particular, I want to make it Ito change-aware and Ito worktree-aware. I would like to be able to update the OpenCode session title based on the change we're working on, such as the module number or change ID. I also want to explore injecting continuation prompts to help the orchestrator and the apply agents keep things ticking.

There's some prior art to look at here in the Oh My OpenCode Slim project.


---

---
## Ito worktree configuration distinctions
## 2026-04-24 20:34:59 UTC

Ito worktree configuration distinction for this repo:

Regular change/feature worktree placement is controlled by `worktrees.*`, especially `worktrees.strategy`, `worktrees.layout.base_dir`, and `worktrees.layout.dir_name`. This repo uses bare/control siblings with change worktrees under `ito-worktrees/`.

Creation-time copy/setup for `ito worktree ensure` is `worktrees.init.include` and `worktrees.init.setup` (for example: copy `.env`, `.envrc`, `.mise.local.toml`; run `make init`). Manual apply fallback instructions use separate `worktrees.apply.copy_from_main` and `worktrees.apply.setup_commands`, which should generally mirror `worktrees.init.*`.

Coordination worktree storage is separate: `changes.coordination_branch.storage = "worktree"` and optional `changes.coordination_branch.worktree_path` points to the shared coordination worktree that stores `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, and `.ito/audit` symlink targets. Do not confuse `changes.coordination_branch.worktree_path` with the feature worktree path prefix.

Machine-specific coordination worktree absolute paths belong in ignored local config such as `.ito/config.local.json` or `.local/ito/config.json`, not committed `.ito/config.json`.


---

---
## ByteRover Autonomous Approval Follow-Up
## 2026-04-25 15:50:34 UTC

Follow-up idea from `016-17_add-list-archive`: create a separate change for autonomous ByteRover approval review. Proposed shape: add an explicit repo policy for which ByteRover pending review operations may be agent-approved, then add a reviewer workflow/subagent that inspects `brv review pending --format json`, validates UPSERTs against touched files and existing memory, and only runs `brv review approve <taskId>` when the policy permits. Default should remain human approval for deletes, broad architecture/security guidance, conflicts, or unsupported claims.
