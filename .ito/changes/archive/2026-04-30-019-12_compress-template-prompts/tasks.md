## 1. Scope and inventory
- [x] 1.1 Confirm which markdown assets under `ito-rs/crates/ito-templates/{AGENTS.md,assets/{skills,agents,commands,instructions,default/project/AGENTS.md}}` are eligible for compaction and identify any protected change-proposal templates.
- [x] 1.2 Encode or document the exclusion list so change-proposal templates named `spec.md`, `design.md`, `proposal.md`, and `tasks.md` are not compacted.

## 2. Template compaction updates
- [x] 2.1 Compact eligible skill markdown assets while preserving behavior, managed markers, and helper prompts.
- [x] 2.2 Compact eligible AGENTS, agent, command, and instruction markdown assets while preserving frontmatter, Jinja placeholders, and harness-specific semantics.

## 3. Verification
- [x] 3.1 Review the touched template assets to confirm excluded basenames and `.autopilot` were not modified.
- [x] 3.2 Run `ito validate 019-12_compress-template-prompts --strict`.
