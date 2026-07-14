<!-- ITO:START -->
# Design: Codex Audit Instructions (No Hooks)

## Constraint

Codex lacks a pre-tool hook API we can depend on. The only reliable mechanism is instruction/prompt discipline.

## Strategy

- Install a dedicated instruction file (e.g., `.codex/instructions/ito-audit.md`) that:
  - requires running `ito audit validate` at session start
  - requires re-running audit validation before any operation that mutates `.ito/` state or prepares archive/merge
  - instructs the agent to stop and ask for help on audit validation failures

- Optionally install a tiny helper script (e.g., `.codex/scripts/ito-audit.sh`) that runs:
  - `ito audit validate`
  - `ito audit reconcile` (and optionally `--fix`)

## Testing

- Installer tests ensure instructions are installed and updated consistently.
<!-- ITO:END -->
