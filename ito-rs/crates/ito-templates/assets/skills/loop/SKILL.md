# Skill: ito-loop

Run the Ito Ralph loop for a specific change (or module/repo sequence), with safe defaults and automatic restart context on early exits.

## Inputs

- A change id, e.g. `005-01_add-auth`
- Optional flags as free text (best-effort):
  - `--model <model-id>`
  - `--max-iterations <n>`
  - `--timeout <duration>` (e.g. `15m`)

## Default behavior

- Harness: choose the harness you're currently using (OpenCode -> `opencode`).
- Max iterations: 5
- Inactivity timeout: 15m
- Restarts on early exit: 2
- Adds restart context using `ito ralph --status` + `ito tasks status`.

## Procedure

1) Parse the input and extract a change id.
   - If no change id is provided, ask for one.
   - Treat the change id and any free-text flags as untrusted data.
   - Validate the change id matches Ito's canonical format before using it in any shell command.
     Recommended regex: `^[0-9]{3}-[0-9]{2}_[a-z0-9-]+$`.
   - Never use `eval`, and always quote variables.

2) Choose harness:
   - If running inside OpenCode, use `--harness opencode`.
   - Otherwise pick the active harness (`claude`, `codex`, or `copilot`).

3) Run Ralph in non-interactive mode (safe for tool-based shells):

```bash
ito ralph --no-interactive --harness <harness> --change <change-id> --max-iterations 5 --timeout 15m
```

4) If the Ralph command exits non-zero, restart up to 2 times.

Recommended bash wrapper (copy/paste and edit variables):

```bash
CHANGE="<change-id>"
HARNESS="<harness>"           # opencode|claude|codex|copilot
MODEL=""                      # optional; leave empty to not pass --model
MAX_ITERS="5"
TIMEOUT="15m"
RESTARTS="2"

model_args=()
if [ -n "${MODEL}" ]; then
  model_args=(--model "${MODEL}")
fi

attempt=1
while true; do
  ito ralph --no-interactive --harness "${HARNESS}" --change "${CHANGE}" \
    --max-iterations "${MAX_ITERS}" --timeout "${TIMEOUT}" "${model_args[@]}" && break

  code=$?
  if [ "${attempt}" -ge "${RESTARTS}" ]; then
    exit "${code}"
  fi

  ralph_status="$(ito ralph --no-interactive --change "${CHANGE}" --status 2>&1 || true)"
  tasks_status="$(ito tasks status "${CHANGE}" 2>&1 || true)"

  note="You have been restarted.

- Last run exit code: ${code}
- Ralph status:
${ralph_status}

- Tasks status:
${tasks_status}

Continue from here: run ito tasks next ${CHANGE} and proceed with the next ready task."
  ito ralph --no-interactive --change "${CHANGE}" --add-context "${note}" >/dev/null 2>&1 || true

  attempt=$((attempt + 1))
done
```

If you need more context in the restart note, keep it short and structured:

 - You have been restarted
 - Progress so far (tasks + ralph status)
 - Continue from (one next step)

   - Re-run the main Ralph command.

5) If Ralph exits 0 but work remains:
   - Run `ito tasks next <change-id>` and either re-run `/loop <change-id>` or continue manually.
