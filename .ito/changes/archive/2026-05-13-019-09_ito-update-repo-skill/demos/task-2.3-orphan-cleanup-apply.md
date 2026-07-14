# Task 2.3: Orphan Cleanup Apply

*2026-05-12T16:36:24Z by Showboat 0.6.1*
<!-- showboat-id: 97337ee4-9e12-411f-8c60-73361e22737d -->

Rebuilt and installed the Ito CLI from this change, reran ito init --update --tools all, audited Ito-owned harness assets against current templates plus default project command seeds, and confirmed zero orphan cleanup candidates.

```bash
ito --version
```

```output
0.1.31-local.202605121732
```

```bash
node - <<'NODE'
const fs = require('fs');
const path = require('path');
const root = process.cwd();
const namesIn = (dir, opts={}) => fs.existsSync(dir) ? fs.readdirSync(dir).filter(n => opts.dirs ? fs.statSync(path.join(dir,n)).isDirectory() : n.endsWith('.md')).map(n => opts.dirs ? n : path.basename(n, '.md')) : [];
const skillDir = path.join(root, 'ito-rs/crates/ito-templates/assets/skills');
const commandDir = path.join(root, 'ito-rs/crates/ito-templates/assets/commands');
const defaultDir = path.join(root, 'ito-rs/crates/ito-templates/assets/default/project');
const agentDir = path.join(root, 'ito-rs/crates/ito-templates/assets/agents');
const expectedSkills = new Set(namesIn(skillDir, {dirs:true}));
const sharedCommands = new Set(namesIn(commandDir));
const roots = new Map([
  ['.claude/skills', expectedSkills], ['.codex/skills', expectedSkills], ['.github/skills', expectedSkills], ['.opencode/skills', expectedSkills], ['.pi/skills', expectedSkills],
  ['.claude/commands', new Set([...sharedCommands, ...namesIn(path.join(defaultDir, '.claude/commands'))])],
  ['.codex/commands', new Set(namesIn(path.join(defaultDir, '.codex/commands')))],
  ['.codex/prompts', sharedCommands],
  ['.github/prompts', new Set([...sharedCommands].map(n => n + '.prompt'))],
  ['.opencode/commands', new Set([...sharedCommands, ...namesIn(path.join(defaultDir, '.opencode/commands'))])],
  ['.pi/commands', new Set([...sharedCommands, ...namesIn(path.join(defaultDir, '.pi/commands'))])],
]);
for (const [rootRel, tpl] of [['.claude/agents','claude-code'], ['.github/agents','github-copilot'], ['.opencode/agents','opencode'], ['.pi/agents','pi']]) roots.set(rootRel, new Set(namesIn(path.join(agentDir, tpl))));
const orphans = [];
for (const [rel, expected] of roots) {
  const dir = path.join(root, rel);
  if (!fs.existsSync(dir)) continue;
  for (const entry of fs.readdirSync(dir)) {
    const full = path.join(dir, entry);
    const base = fs.statSync(full).isDirectory() ? entry : path.basename(entry, '.md');
    if ((base === 'ito' || base.startsWith('ito-')) && !expected.has(base)) orphans.push(path.relative(root, full));
  }
}
console.log(JSON.stringify({orphanCount: orphans.length, orphans}, null, 2));
NODE
```

```output
{
  "orphanCount": 0,
  "orphans": []
}
```

```bash
before=$(git diff --binary | shasum); ito init --update --tools all >/tmp/ito-update-demo.log 2>&1; after=$(git diff --binary | shasum); if [ "$before" = "$after" ]; then printf 'idempotent: yes\n'; else printf 'idempotent: no\nbefore=%s\nafter=%s\n' "$before" "$after"; exit 1; fi
```

```output
idempotent: yes
```

After tightening the skill manifest guidance to include default-project command seeds, rebuilt and reinstalled Ito again, refreshed generated assets, and rechecked orphan/idempotence behavior.

```bash
ito --version
```

```output
0.1.31-local.202605121737
```

```bash
before=$(git diff --binary | shasum); ito init --update --tools all >/tmp/ito-update-demo-final.log 2>&1; after=$(git diff --binary | shasum); if [ "$before" = "$after" ]; then printf 'idempotent: yes\n'; else printf 'idempotent: no\nbefore=%s\nafter=%s\n' "$before" "$after"; exit 1; fi
```

```output
idempotent: yes
```

Aligned the template skill frontmatter description with generated harness outputs, rebuilt/installed Ito, and refreshed generated assets again.

```bash
ito --version
```

```output
0.1.31-local.202605121744
```

```bash
before=$(git diff --binary | shasum); ito init --update --tools all >/tmp/ito-update-demo-post-review.log 2>&1; after=$(git diff --binary | shasum); if [ "$before" = "$after" ]; then printf 'idempotent: yes\n'; else printf 'idempotent: no\nbefore=%s\nafter=%s\n' "$before" "$after"; exit 1; fi
```

```output
idempotent: yes
```

Aligned the command template frontmatter with generated command/prompt wrappers, rebuilt/installed Ito, and refreshed generated assets.

```bash
ito --version
```

```output
0.1.31-local.202605121749
```

```bash
before=$(git diff --binary | shasum); ito init --update --tools all >/tmp/ito-update-demo-command-fix.log 2>&1; after=$(git diff --binary | shasum); if [ "$before" = "$after" ]; then printf 'idempotent: yes\n'; else printf 'idempotent: no\nbefore=%s\nafter=%s\n' "$before" "$after"; exit 1; fi
```

```output
idempotent: yes
```
