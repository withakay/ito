# Tasks: 026-01_ito-cleanup

## Wave 1: Legacy Registry and Manifest Infrastructure
- **Depends On**: none

### Task 1.1: Create legacy file registry in ito-templates
- **Status**: [ ] pending
- **Updated At**: 2026-03-24
- **Description**: Create a `legacy.rs` module in `ito-templates` that defines a `LegacyEntry` struct (with `old_path`, `new_path: Option`, `entry_type` enum: Renamed/Removed/Relocated, `description`) and a `const` array of all known legacy entries. Include entries for: renamed skills (`ito-apply-change-proposal` -> `ito-apply`, `ito-write-change-proposal` -> `ito-proposal`, etc.), removed skills (`ito-dispatching-parallel-agents`, `ito-finishing-a-development-branch`, `ito-receiving-code-review`, `ito-requesting-code-review`, `ito-systematic-debugging`, `ito-test-driven-development`, `ito-writing-skills`), removed planning directory (`.ito/planning/`), directory renames (`.opencode/command/` -> `.opencode/commands/`, `.opencode/agent/` -> `.opencode/agents/`), and removed commands (`loop.md`). Export from `lib.rs`.
- **Files**: `ito-rs/crates/ito-templates/src/legacy.rs`, `ito-rs/crates/ito-templates/src/lib.rs`
- **Verify**: `cargo build -p ito-templates && cargo test -p ito-templates`
- **Done When**: `LegacyEntry` struct and `LEGACY_ENTRIES` constant compile and are exported. Unit test verifies entry count and key entries.
- **Requirements**: cleanup-instruction:legacy-registry

### Task 1.2: Add manifest generation function to ito-templates
- **Status**: [ ] pending
- **Updated At**: 2026-03-24
- **Description**: Create a `manifest.rs` module that exposes a function to generate the complete list of files Ito would install for a given set of configured harness tools. Reuse existing `*_files()` functions and path-mapping logic from `distribution.rs` in `ito-core`. Return a `Vec<ManifestEntry>` with `relative_path`, `source` (skill/command/adapter/project), and `harness` (which tool it belongs to). Export from `lib.rs`.
- **Files**: `ito-rs/crates/ito-templates/src/manifest.rs`, `ito-rs/crates/ito-templates/src/lib.rs`
- **Verify**: `cargo build -p ito-templates && cargo test -p ito-templates`
- **Done When**: `generate_manifest()` function compiles and returns correct entries for a given tool set. Unit test verifies expected entries for at least one harness.
- **Requirements**: cleanup-instruction:dynamic-manifest

## Wave 2: Agent Instruction Artifact
- **Depends On**: Wave 1

### Task 2.1: Create cleanup instruction Jinja2 template
- **Status**: [ ] pending
- **Updated At**: 2026-03-24
- **Description**: Create the Jinja2 template for the cleanup instruction. Render: (1) a section listing all expected Ito-managed files (from manifest), (2) a section listing all known legacy/deprecated files (from registry), (3) step-by-step instructions for the agent to scan the repo, compare against the manifest, detect orphans, report findings, and remove with user confirmation. Include concrete shell commands for scanning.
- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/cleanup.md.j2`
- **Verify**: Template renders without errors via `ito-templates` instruction rendering (unit test)
- **Done When**: Template file exists and renders successfully with sample context data
- **Requirements**: cleanup-instruction:agent-instruction-artifact, cleanup-instruction:output-format

### Task 2.2: Add cleanup artifact handler in ito-cli
- **Status**: [ ] pending
- **Updated At**: 2026-03-24
- **Description**: Add the `cleanup` artifact to the instruction dispatch in `handle_agent_instruction()`. Create a `generate_cleanup_instruction()` function that reads the project's configured tools from `.ito/config.json`, generates the manifest and legacy entries, and renders the `cleanup.md.j2` template. Update the `after_help` text in `cli.rs` to include `cleanup` in the artifact list and examples.
- **Files**: `ito-rs/crates/ito-cli/src/app/instructions.rs`, `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: Task 2.1
- **Verify**: `cargo build -p ito-cli && ito agent instruction cleanup` produces valid output
- **Done When**: `ito agent instruction cleanup` renders the full cleanup instruction. `ito agent instruction cleanup --json` produces valid JSON.
- **Requirements**: cleanup-instruction:agent-instruction-artifact, cleanup-instruction:output-format

## Wave 3: Skill and CLI Integration
- **Depends On**: Wave 2

### Task 3.1: Create ito-cleanup skill asset
- **Status**: [ ] pending
- **Updated At**: 2026-03-24
- **Description**: Create the `ito-cleanup` skill SKILL.md in the templates assets. The skill should describe itself as a cleanup/migration tool for repos with legacy Ito files, instruct the agent to run `ito agent instruction cleanup`, instruct the agent to follow the returned instructions step-by-step, and emphasize the confirmation gate before any deletions.
- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-cleanup/SKILL.md`
- **Verify**: `cargo build -p ito-templates` (skill is embedded via `include_dir!`)
- **Done When**: Skill file exists in assets, compiles into the binary, and is installed by `ito init` to all harness skill directories.
- **Requirements**: cleanup-skill:skill-definition, cleanup-skill:trigger-matching

### Task 3.2: Add cleanup detection to ito init --upgrade
- **Status**: [ ] pending
- **Updated At**: 2026-03-24
- **Description**: After the existing upgrade logic in `install_default_templates()`, add a cleanup detection pass that iterates the legacy registry, checks if each legacy path exists on disk, and collects found orphans. In the CLI layer, if orphans are detected during `--upgrade`, print a warning listing them and suggest `--cleanup`. Add a `--cleanup` flag to `InitArgs`. When `--cleanup` is set with `--upgrade`, prompt for confirmation (or skip if `--force`) and remove the orphaned files.
- **Files**: `ito-rs/crates/ito-core/src/installers/mod.rs`, `ito-rs/crates/ito-cli/src/app/init.rs`, `ito-rs/crates/ito-cli/src/cli.rs`
- **Verify**: `cargo build -p ito-cli && cargo test -p ito-cli`
- **Done When**: `ito init --upgrade` detects and reports orphaned files. `ito init --upgrade --cleanup` removes them with confirmation. `ito init --upgrade --cleanup --force` removes without prompting.
- **Requirements**: cleanup-cli:upgrade-detection, cleanup-cli:upgrade-removal

## Wave 4: Testing and Validation
- **Depends On**: Wave 3

### Task 4.1: Integration tests for cleanup instruction
- **Status**: [ ] pending
- **Updated At**: 2026-03-24
- **Description**: Write integration tests that verify `ito agent instruction cleanup` produces valid output, `--json` mode produces valid JSON with manifest and legacy entries, and the manifest reflects configured tools.
- **Files**: `ito-rs/crates/ito-cli/tests/agent_instruction_cleanup.rs`
- **Verify**: `cargo test -p ito-cli --test agent_instruction_cleanup`
- **Done When**: All integration tests pass
- **Requirements**: cleanup-instruction:agent-instruction-artifact, cleanup-instruction:dynamic-manifest, cleanup-instruction:output-format

### Task 4.2: Integration tests for upgrade cleanup
- **Status**: [ ] pending
- **Updated At**: 2026-03-24
- **Description**: Write integration tests that set up a temp project with legacy files, run `ito init --upgrade` and verify orphan detection output, run `ito init --upgrade --cleanup --force` and verify orphan removal, and verify user-owned files are never removed.
- **Files**: `ito-rs/crates/ito-cli/tests/init_cleanup.rs`
- **Verify**: `cargo test -p ito-cli --test init_cleanup`
- **Done When**: All integration tests pass
- **Requirements**: cleanup-cli:upgrade-detection, cleanup-cli:upgrade-removal

### Task 4.3: Validate full change
- **Status**: [ ] pending
- **Updated At**: 2026-03-24
- **Description**: Run `ito validate 026-01_ito-cleanup --strict`, `make check`, and `make test` to ensure everything passes.
- **Files**: N/A
- **Dependencies**: Task 4.1, Task 4.2
- **Verify**: `ito validate 026-01_ito-cleanup --strict && make check && make test`
- **Done When**: All validations pass with no errors
- **Requirements**: cleanup-instruction:agent-instruction-artifact, cleanup-cli:upgrade-detection
