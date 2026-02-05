# Rust CLI UX Parity (TTY, prompts, spinners, colors)

## Goals

- Match the TypeScript CLI user-facing output and interaction patterns.
- Ensure `--no-color` and `NO_COLOR` behave identically.
- Ensure `--no-interactive` and `ITO_INTERACTIVE=0` disable prompts.

## TTY + Interactivity

Recommended building blocks:

- Detect TTY: `std::io::IsTerminal` (stable) or `atty`.
- Centralize an `Interactivity` policy:
  - `interactive = stdin_is_tty && env(ITO_INTERACTIVE) != "0" && !--no-interactive`

For interactive selection (multi-select, list prompts):

- Prefer `inquire` (multi-select, text input, theming) or `dialoguer`.
- Requirement: the prompt library must support:
  - Multi-select with stable ordering
  - Cancellation detection (to return correct exit codes)
  - Non-interactive fallback paths

## Spinners / Progress

The TypeScript CLI uses spinners for longer operations (notably validation and
some workflow commands). In Rust:

- Prefer `indicatif` for spinners/progress bars.
- Disable spinners automatically when not a TTY.
- Ensure spinner output does not pollute `--json` mode.

## Color Output

Requirements:

- Honor `--no-color`.
- Honor `NO_COLOR`.
- In `--json` mode, emit JSON only.

Implementation approach:

- Use `anstream` for stdout/stderr writers with `ColorChoice` derived from flags
  and environment.
- Use `anstyle` / `owo-colors` only behind a single rendering layer so that text
  snapshots are stable.

## Error Messages and Exit Codes

- Use a stable, string-first error surface. Even if using `miette` for
  diagnostics, the final user-facing messages must match the TypeScript output.
- Preserve exit code conventions from specs and current help behavior.

## JSON vs Text Mode

- `--json` must be strict: no spinners, no prefixes, no extra lines.
- Prefer rendering output structures into a serializable Rust struct and then
  printing via `serde_json`.
