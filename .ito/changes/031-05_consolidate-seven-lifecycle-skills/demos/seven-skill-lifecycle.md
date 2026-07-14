# Seven-skill lifecycle consolidation

*2026-07-13T21:12:41Z by Showboat 0.6.1*
<!-- showboat-id: 93d4c327-f299-4a38-8260-095f052dfe40 -->

Ito now installs one compact lifecycle: route, propose, research, apply, review, archive, and iterate. The checks below prove the exact inventory, removal of superseded surfaces, requirement-level archive reconciliation, and ownership-safe upgrade cleanup.

```bash
actual=$(find ito-rs/crates/ito-templates/assets/skills -mindepth 2 -maxdepth 2 -name SKILL.md -print | sed 's|.*/assets/skills/||; s|/SKILL.md||' | sort | paste -sd, -)
expected=ito,ito-apply,ito-archive,ito-loop,ito-proposal,ito-research,ito-review
test "$actual" = "$expected"
printf 'seven lifecycle skills: %s\n' "$actual"

```

```output
seven lifecycle skills: ito,ito-apply,ito-archive,ito-loop,ito-proposal,ito-research,ito-review
```

```bash
actual=$(find ito-rs/crates/ito-templates/assets/commands -maxdepth 1 -type f -name '*.md' -print | sed 's|.*/||; s|\.md$||' | sort | paste -sd, -)
expected=ito,ito-apply,ito-archive,ito-loop,ito-proposal,ito-research,ito-review
test "$actual" = "$expected"
printf 'seven matching command wrappers: %s\n' "$actual"

```

```output
seven matching command wrappers: ito,ito-apply,ito-archive,ito-loop,ito-proposal,ito-research,ito-review
```

```bash
test ! -e ito-rs/crates/ito-templates/assets/skills/ito-tmux
test ! -e ito-rs/crates/ito-templates/assets/agents/codex
test ! -e ito-rs/crates/ito-templates/assets/default/project/.claude/commands/ito-project-setup.md
test ! -e ito-rs/crates/ito-templates/assets/default/project/.codex/commands/ito-project-setup.md
test ! -e ito-rs/crates/ito-templates/assets/default/project/.opencode/commands/ito-project-setup.md
test ! -e ito-rs/crates/ito-templates/assets/default/project/.pi/commands/ito-project-setup.md
printf '%s\n' 'tmux, Codex role-skills, and project-setup wrappers are absent'

```

```output
tmux, Codex role-skills, and project-setup wrappers are absent
```

```bash
if cargo test -q -p ito-core archive_specs >/dev/null 2>&1; then
  printf '%s\n' 'requirement-level archive reconciliation: passed'
else
  exit 1
fi

```

```output
requirement-level archive reconciliation: passed
```

```bash
if cargo test -q -p ito-core retired_cleanup >/dev/null 2>&1; then
  printf '%s\n' 'ownership-safe retired-surface cleanup: passed'
else
  exit 1
fi

```

```output
ownership-safe retired-surface cleanup: passed
```

```bash
for cap in experimental-workflow-commands ito-sync-specs-skill ito-tmux-skill ito-update-repo-skill orchestrate-workflow-skill; do
  test ! -e ".ito/specs/$cap"
done
test -f .ito/specs/lifecycle-skill-profile/spec.md
test -f ito-rs/crates/ito-templates/assets/skills/ito-loop/SKILL.md
printf '%s\n' 'retired capabilities removed; lifecycle profile and iteration remain'

```

```output
retired capabilities removed; lifecycle profile and iteration remain
```
