# Removing Ito-owned tmux integration

*2026-07-13T19:06:09Z by Showboat 0.6.1*
<!-- showboat-id: 80ee2296-c4fb-4ca8-a61a-159a8cefbe90 -->

Ito now keeps terminal multiplexing outside its spec-driven lifecycle. This demo proves the removed runtime, config, and bundled-asset surfaces while preserving the remaining proposal viewers.

```bash
set -e
for path in ito-rs/crates/ito-core/src/viewer/tmux_nvim.rs ito-rs/crates/ito-templates/assets/skills/ito-tmux/SKILL.md .claude/skills/ito-tmux/SKILL.md .codex/skills/ito-tmux/SKILL.md .github/skills/ito-tmux/SKILL.md .opencode/skills/ito-tmux/SKILL.md .pi/skills/ito-tmux/SKILL.md; do
  test ! -e "$path"
done
echo "Removed viewer and tmux skill assets are absent from source and all five harnesses."
```

```output
Removed viewer and tmux skill assets are absent from source and all five harnesses.
```

```bash
set -e
test -f schemas/ito-config.schema.json
if jq -e ".properties.tools or .definitions.ToolsConfig or .definitions.TmuxConfig" schemas/ito-config.schema.json >/dev/null; then
  exit 1
fi
echo "Generated config schema contains no tmux-only tools namespace or DTOs."
```

```output
Generated config schema contains no tmux-only tools namespace or DTOs.
```

```bash
set -e
test -x target/debug/ito
if target/debug/ito init --help | rg -q -- "--no-tmux"; then
  exit 1
fi
echo "Init help no longer exposes the removed tmux flag."
```

```output
Init help no longer exposes the removed tmux flag.
```

```bash
set -e
cargo test -q -p ito-cli --test view_proposal view_proposal_removed_tmux_viewer_is_unknown >/dev/null 2>&1
echo "Removed tmux viewer is rejected as an unknown viewer; remaining viewer tests still compile."
```

```output
Removed tmux viewer is rejected as an unknown viewer; remaining viewer tests still compile.
```
