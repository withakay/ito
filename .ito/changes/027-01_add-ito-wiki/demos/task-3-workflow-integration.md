# Task 3: Wiki Workflow Integration

*2026-04-27T08:23:10Z by Showboat 0.6.1*
<!-- showboat-id: beee44f9-718d-4706-b7bd-c577fc03f235 -->

Integrated warn-and-update wiki guidance into proposal, research, and archive workflows. Proposal guidance consults .ito/wiki/index.md early, research keeps source artifacts separate from wiki synthesis, and archive guidance refreshes topic pages after successful archive/spec sync.

```bash
rtk cargo test -p ito-templates new_proposal_template_moves_to_worktree_after_create
```

```output
cargo test: 1 passed, 113 filtered out (8 suites, 0.00s)
```

```bash
rtk cargo test -p ito-templates archive_template_renders_targeted_instruction_with_change
```

```output
cargo test: 1 passed, 113 filtered out (8 suites, 0.00s)
```

```bash
rtk cargo test -p ito-templates research_and_archive_skills_include_wiki_follow_up
```

```output
cargo test: 1 passed, 113 filtered out (8 suites, 0.00s)
```

```bash
rtk cargo test -p ito-templates default_project_agents_mentions_fix_feature_and_wiki_guidance
```

```output
cargo test: 1 passed, 113 filtered out (8 suites, 0.00s)
```

```bash
rtk cargo test -p ito-templates
```

```output
cargo test: 121 passed (9 suites, 0.00s)
```

```bash
rtk cargo test -p ito-cli
```

```output
cargo test: 374 passed, 3 ignored (53 suites, 20.79s)
```

Normalized Wiki Follow-Up headings across research templates after review feedback.

```bash
rtk cargo test -p ito-templates
```

```output
cargo test: 121 passed (9 suites, 0.01s)
```

```bash
rtk cargo test -p ito-cli
```

```output
cargo test: 374 passed, 3 ignored (53 suites, 23.11s)
```
