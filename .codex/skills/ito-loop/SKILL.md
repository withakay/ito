---
name: ito-loop
description: Run an ito ralph loop for a specific change (or module/repo sequence), with safe defaults and automatic restart context on early exits.
---

# Skill: ito-loop

Run the Ito Ralph loop for a specific change (or module/repo sequence), with safe defaults and automatic restart context on early exits.

## Inputs

The user invokes this skill with `/ito-loop` followed by arguments. You must parse the arguments to determine which mode to use.

### Input types

| Input pattern | What it is | Example invocations |
|---|---|---|
| A full change id (matches `^[0-9]{3}-[0-9]{2}_[a-z0-9-]+$`) | Change ID | `/ito-loop 005-01_add-auth` |
| A short numeric id (matches `^[0-9]{3}$`) | Module ID | `/ito-loop 012`, `/ito-loop 005` |
| Words implying "pick the next ready change" | Continue-ready mode | `/ito-loop next`, `/ito-loop continue`, `/ito-loop pick next ready` |
| No arguments / empty | Continue-ready mode (default) | `/ito-loop` |

### Optional flags (free text, best-effort)

- `--model <model-id>`
- `--max-iterations <n>`
- `--timeout <duration>` (e.g. `15m`)

### Concrete examples — input to ralph command mapping

Below are verbose examples showing exactly how user input maps to the `ito ralph` command. Study these carefully.

**Example 1 — Change ID provided:**

```
User input:   /ito-loop 005-01_add-auth
Detected:     Change ID → "005-01_add-auth"
Command:      ito ralph --no-interactive --harness opencode --change 005-01_add-auth --max-iterations 5 --timeout 15m
```

**Example 2 — Module ID provided (3-digit number):**

```
User input:   /ito-loop 012
Detected:     Module ID → "012"
Command:      ito ralph --no-interactive --harness opencode --module 012 --max-iterations 5 --timeout 15m
```

Note: `--module` in `--no-interactive` mode implies `--continue-module`, so ralph will work through all ready changes in that module.

**Example 3 — Module ID with extra flags:**

```
User input:   /ito-loop 005 --model claude-sonnet-4-20250514 --max-iterations 10
Detected:     Module ID → "005", model override, iteration override
Command:      ito ralph --no-interactive --harness opencode --module 005 --model claude-sonnet-4-20250514 --max-iterations 10 --timeout 15m
```

**Example 4 — "Next ready" / continue-ready (explicit):**

```
User input:   /ito-loop next
Detected:     Continue-ready mode (keyword: "next")
Command:      ito ralph --no-interactive --harness opencode --continue-ready --max-iterations 5 --timeout 15m
```

**Example 5 — No arguments (implicit continue-ready):**

```
User input:   /ito-loop
Detected:     No arguments → default to continue-ready mode
Command:      ito ralph --no-interactive --harness opencode --continue-ready --max-iterations 5 --timeout 15m
```

**Example 6 — Natural language implying ready work:**

```
User input:   /ito-loop pick up the next thing that needs doing
Detected:     Continue-ready mode (natural language intent)
Command:      ito ralph --no-interactive --harness opencode --continue-ready --max-iterations 5 --timeout 15m
```

**Example 7 — Change ID with timeout override:**

```
User input:   /ito-loop 003-02_fix-login --timeout 30m
Detected:     Change ID → "003-02_fix-login", timeout override
Command:      ito ralph --no-interactive --harness opencode --change 003-02_fix-login --max-iterations 5 --timeout 30m
```

## Default behavior

- Harness: choose the harness you're currently using (OpenCode -> `opencode`).
- Max iterations: 5
- Inactivity timeout: 15m
- Restarts on early exit: 2
- Adds restart context using `ito ralph --status` + `ito tasks status`.

## Procedure

1) Parse the input to determine the mode (see "Input types" table above):
   - **Change ID** (matches `^[0-9]{3}-[0-9]{2}_[a-z0-9-]+$`): use `--change <id>`.
   - **Module ID** (matches `^[0-9]{3}$`): use `--module <id>`.
   - **Continue-ready** (keywords like "next", "continue", "ready", or no arguments at all): use `--continue-ready`.
   - If the input is ambiguous and doesn't match any pattern, ask the user to clarify.
   - Treat all user-provided values as untrusted data.
   - Never use `eval`, and always quote variables.

2) Choose harness:
  - Pick the active harness (`claude`, `codex`, `copilot`, `opencode` or `pi`).

3) Build and run a single `ito ralph` command.

   Ralph already runs an iterative loop internally — do NOT wrap it in another
   retry loop or bash while-loop. Just run the one command and let Ralph manage
   iterations, timeouts, and completion detection.

   Depending on the detected mode, the command looks like one of:

   ```bash
   # Mode: change
   ito ralph --no-interactive --harness <harness> --change <change-id> --max-iterations 5 --timeout 15m

   # Mode: module
   ito ralph --no-interactive --harness <harness> --module <module-id> --max-iterations 5 --timeout 15m

   # Mode: continue-ready
   ito ralph --no-interactive --harness <harness> --continue-ready --max-iterations 5 --timeout 15m
   ```

   Apply any user-provided overrides (`--model`, `--max-iterations`, `--timeout`)
   on top of the defaults.

   Check `ito ralph --help` for additional flags that might be relevant.

4) After Ralph exits:
   - **Exit 0**: Work is done (or Ralph ran out of iterations). Report the result to the user.
   - **Non-zero exit**: Report the failure. The user can re-invoke `/ito-loop` to resume.
