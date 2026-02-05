# Change: Bootstrap Artifact for CLI Instruction Generator

## Why

Tool adapters (OpenCode plugin, Claude Code hook, Codex bootstrap) should be extremely small. All workflow content should come from the Ito CLI's instruction generator. Adding a `bootstrap` artifact to `ito agent instruction` provides tool-specific preambles that adapters can invoke, keeping the adapters thin and the content centralized.

## What Changes

- Add new `bootstrap` (or `preamble`) artifact to `ito agent instruction`:
  - `ito agent instruction bootstrap --tool opencode|claude|codex`
  - Contains: tool mapping notes + "how to get workflow bodies" pointers
  - Does NOT contain full workflow text (delegates to other artifacts)
- Update instruction generator to support `--tool` flag for tool-specific content
- Define tool-specific content:
  - OpenCode: MCP tool names, parallel invocation patterns
  - Claude Code: Task tool delegation, Read/Write/Edit tool usage
  - Codex: Available commands, shell execution patterns

## Capabilities

### New Capabilities

- `bootstrap-artifact`: CLI instruction generator for tool-specific bootstrap content

### Modified Capabilities

- `agent-instructions`: Extended to support `bootstrap` artifact with `--tool` flag

## Impact

- Affected specs: `agent-instructions` (modified)
- Affected code:
  - `ito-rs/crates/ito-core/src/ralph/` (instruction generator)
  - Instruction templates
- This is a prerequisite for 013-01, 013-02, 013-03 (they consume this artifact)
- Parallelization: Can start immediately; other tracks can stub the expected output

## Parallel Execution Notes

This change is a soft dependency for:
- 013-01 (OpenCode adapter)
- 013-02 (Claude Code integration)
- 013-03 (Codex bootstrap)

All three adapter tracks can proceed in parallel by:
1. Defining the expected output format of `ito agent instruction bootstrap --tool <tool>`
2. Implementing the adapters to call this command
3. This change delivers the actual content

No hard blockers - adapters can be developed with placeholder content.
