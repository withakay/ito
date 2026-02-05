# Tasks for: 013-05_distribution-fetch-mechanics

## Execution Notes

- **Tool**: Any
- **Mode**: Sequential
- **Created**: 2026-01-31
- **Rust**: Implementation MUST follow the `rust-style` skill

```bash
ito tasks status 013-05_distribution-fetch-mechanics
ito tasks next 013-05_distribution-fetch-mechanics
ito tasks start 013-05_distribution-fetch-mechanics 1.1
ito tasks complete 013-05_distribution-fetch-mechanics 1.1
ito tasks show 013-05_distribution-fetch-mechanics
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Implement ito-skills fetch + cache with local-dev fallback

- **Files**: `ito-rs/crates/ito-core/src/installers/`, `.ito/changes/013-05_distribution-fetch-mechanics/design.md`
- **Dependencies**: None
- **Action**:
  - Implement a fetcher that can retrieve raw files from GitHub:
    - Tagged: `https://raw.githubusercontent.com/withakay/ito/<tag>/ito-skills/<path>`
    - Fallback: `https://raw.githubusercontent.com/withakay/ito/main/ito-skills/<path>`
  - Add per-user cache:
    - `~/.config/ito/cache/ito-skills/<tag>/<path>`
  - Add dev-mode source:
    - If `./ito-skills/` exists in repo root, copy from there instead of HTTP.
  - Encode the per-tool file manifests (OpenCode/Claude/Codex) as data (not ad-hoc logic).
  - Apply the `rust-style` skill for all Rust changes (formatting, structure, naming).
- **Verify**: `make test`
- **Done When**: Fetcher can source from local repo or remote, with caching
- **Updated At**: 2026-01-31
- **Status**: [x] complete

### Task 1.2: Wire install into `ito init` and refresh into `ito update`

- **Files**: `ito-rs/crates/ito-cli/`, `ito-rs/crates/ito-core/src/installers/`
- **Dependencies**: Task 1.1
- **Action**:
  - Extend `ito init` to accept `--tools opencode,claude,codex` and install selected adapter files.
  - Extend `ito update` to refresh the managed adapter files.
  - Ensure both are idempotent and safe.
- **Verify**: `make test`
- **Done When**: `ito init --tools ...` and `ito update` install/refresh adapters consistently
- **Updated At**: 2026-01-31
- **Status**: [x] complete

______________________________________________________________________

## Checkpoints

### Checkpoint: Review Implementation

- **Type**: checkpoint (requires human approval)
- **Files**: `.ito/changes/013-05_distribution-fetch-mechanics/proposal.md`, `.ito/changes/013-05_distribution-fetch-mechanics/design.md`
- **Dependencies**: None
- **Action**: Review cache location, URL scheme, and tool-specific destinations
- **Done When**: User confirms implementation is correct
- **Updated At**: 2026-01-31
- **Status**: [x] completed
