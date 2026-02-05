# Ito Ralph (Loop) Implementation Plan

Migration of iterative AI loop functionality from `ralph.ts` to `ito ralph`.

## 1. Core Architecture (`src/core/ralph/`)

### A. Interfaces (`src/core/ralph/types.ts`)

```typescript
import { EventEmitter } from "events";

export interface RalphRunConfig {
  prompt: string;
  model?: string;
  cwd: string;
  env?: NodeJS.ProcessEnv;
  interactive?: boolean;
}

export interface AgentHarness extends EventEmitter {
  name: string;
  run(config: RalphRunConfig): Promise<void>;
  stop(): void;
}

export interface RalphState {
  iteration: number;
  history: Array<{
    timestamp: number;
    duration: number;
    completionPromiseFound: boolean;
  }>;
  contextFile: string;
}
```

### B. OpenCode Harness (`src/core/ralph/harnesses/opencode.ts`)

- Extends `EventEmitter` implements `AgentHarness`.
- Spawns `opencode run`.
- Maps Ito flags to OpenCode flags.
- Captures stdout/stderr for promise checking.

### C. The Runner (`src/core/ralph/runner.ts`)

- **Class**: `RalphRunner`
- **Dependencies**: `AgentHarness`, `FileSystemUtils`.
- **Logic**:
  - `start()`:
    - Load/Init state.
    - Loop while `iteration < maxIterations`.
    - Build prompt (incorporating Context).
    - `harness.run()`.
    - Check completion promise.
    - Commit changes (if `!noCommit`).
    - Update state.

### D. Context & Prompt (`src/core/ralph/context.ts`)

- `buildPrompt(basePrompt, options)`:
  - If `options.changeId`: Read `proposal.md` from Change.
  - If `options.moduleId`: Read module info.
  - Append `ralph-context.md` content.

## 2. CLI Command (`src/commands/ralph.ts`)

```typescript
import { Command } from "commander";

export function register(program: Command) {
  program
    .command("ralph [prompt]")
    .alias("loop")
    .description("Run iterative AI loop")
    .option("-c, --change <id>", "Target a specific change proposal")
    .option("-m, --module <id>", "Target a specific module")
    .option("--harness <agent>", "Agent harness to use", "opencode")
    .option("--min-iterations <n>", "Min iterations", "1")
    .option("--max-iterations <n>", "Max iterations")
    .option("--no-commit", "Disable auto-commit")
    .action(async (prompt, options) => {
      // Initialize Runner and execute
    });
}
```

## 3. Migration & State

- State Directory: `.ito/ralph/`
- Context File: `.ito/ralph/context.md`
- Config Generation: Reuse `ralph.ts` logic to generate ephemeral `opencode` config (permissions).

## 4. Dependencies

- `cross-spawn` (or internal Ito shell util).
- `fs-extra` (or internal fs util).
