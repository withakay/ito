<!-- ITO:START -->
# Tasks for: 001-28_tmux-skill-integration

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 001-28_tmux-skill-integration
ito tasks next 001-28_tmux-skill-integration
ito tasks start 001-28_tmux-skill-integration 1.1
ito tasks complete 001-28_tmux-skill-integration 1.1
```

______________________________________________________________________

## Wave 1: Embed tmux skill assets

### Task 1.1: Copy SKILL.md into ito-templates assets

- **Files**: `ito-rs/crates/ito-templates/assets/skills/tmux/SKILL.md`
- **Dependencies**: None
- **Action**: Create the `tmux/` skill directory under `assets/skills/` and write the SKILL.md, adapting frontmatter to include `name`, `description`, and `metadata.upstream` fields referencing the OpenCode tmux skill origin
- **Verify**: `cat ito-rs/crates/ito-templates/assets/skills/tmux/SKILL.md | head -10` confirms frontmatter present
- **Done When**: `SKILL.md` exists with valid frontmatter; `name: tmux` present
- **Status**: [ ] pending

### Task 1.2: Copy helper scripts into assets

- **Files**: `ito-rs/crates/ito-templates/assets/skills/tmux/scripts/wait-for-text.sh`, `ito-rs/crates/ito-templates/assets/skills/tmux/scripts/find-sessions.sh`
- **Dependencies**: Task 1.1
- **Action**: Copy `wait-for-text.sh` and `find-sessions.sh` from the global OpenCode tmux skill (`~/.config/opencode/skills/tmux/scripts/`) into the assets directory
- **Verify**: Both script files exist under `assets/skills/tmux/scripts/`
- **Done When**: Both scripts are present in the assets tree
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2: Verify installer picks up scripts

- **Depends On**: Wave 1

### Task 2.1: Confirm include_dir! embeds scripts

- **Files**: `ito-rs/crates/ito-templates/src/lib.rs`
- **Dependencies**: None
- **Action**: Verify that the existing `include_dir!` macro in `lib.rs` recursively embeds subdirectories (including `scripts/`); add or update any install logic if scripts need executable permissions set at write time
- **Verify**: `cargo build -p ito-templates 2>&1 | grep -c error` returns 0; inspect embedded asset listing in a test or `ito agent instruction` output for tmux skill
- **Done When**: Build passes; tmux skill directory with scripts appears in the embedded asset tree
- **Status**: [ ] pending

### Task 2.2: Write installer test for skill-with-scripts

- **Files**: `ito-rs/crates/ito-templates/tests/` or relevant test module
- **Dependencies**: Task 2.1
- **Action**: Add a test asserting that after `ito init` (or the installer function), the tmux skill directory contains both `SKILL.md` and `scripts/wait-for-text.sh` with executable permissions
- **Verify**: `cargo test -p ito-templates 2>&1 | grep -E "PASSED|ok"` for the new test
- **Done When**: Test passes; script permissions verified
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3: Update documentation

- **Depends On**: Wave 2

### Task 3.1: Update ito-templates AGENTS.md

- **Files**: `ito-rs/crates/ito-templates/AGENTS.md`
- **Dependencies**: None
- **Action**: Document that skills may include a `scripts/` subdirectory alongside `SKILL.md`, and that scripts are installed with executable permissions; note the tmux skill as an example
- **Verify**: File reads correctly; no stale references
- **Done When**: AGENTS.md updated with script-bundling pattern guidance
- **Status**: [ ] pending

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave
- Wave N depends on all prior waves completing
- Task dependencies within a wave are fine; cross-wave deps use the wave dependency
<!-- ITO:END -->
