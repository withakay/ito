# 019-08 Routing Assets

*2026-04-01T16:29:26Z by Showboat 0.6.1*
<!-- showboat-id: 7a4d21ed-2c17-4221-8f0f-933682cf0eb8 -->

Added a Stage 0 proposal intake flow, fix and feature entrypoints, richer schema guidance, and updated generated project guidance.

```bash
rtk cargo test -p ito-templates --quiet
```

```output
cargo test: 64 passed (5 suites, 0.00s)
```

```bash
ito validate 019-08_proposal-intake-and-schema-routing --strict
```

```output
Change '019-08_proposal-intake-and-schema-routing' is valid
```

```bash
rtk ls ito-rs/crates/ito-templates/assets/commands && rtk ls ito-rs/crates/ito-templates/assets/skills
```

```output
ito-apply.md  660B
ito-archive.md  562B
ito-feature.md  618B
ito-fix.md  598B
ito-list.md  576B
ito-loop.md  750B
ito-proposal-intake.md  602B
ito-proposal.md  672B
ito-research.md  574B
ito-review.md  560B
ito.md  561B

11 files, 0 dirs (11 .md)
ito/
ito-apply/
ito-archive/
ito-brainstorming/
ito-commit/
ito-feature/
ito-finish/
ito-fix/
ito-list/
ito-loop/
ito-path/
ito-proposal/
ito-proposal-intake/
ito-research/
ito-review/
ito-subagent-driven-development/
ito-tasks/
ito-using-git-worktrees/
ito-verification-before-completion/
ito-workflow/
test-with-subagent/
tmux/
using-ito-skills/

0 files, 23 dirs
```
