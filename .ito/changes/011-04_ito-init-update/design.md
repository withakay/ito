<!-- ITO:START -->
## Context

`ito init` is both a first-run setup command and a rerunnable project setup wizard. The current behavior does not consistently treat existing config as the source of defaults, so rerunning the wizard can present generic defaults instead of the user's chosen values. Separately, the config model has grown faster than the init/update surfaces, which makes it easy for new settings to be omitted from setup and update flows.

## Goals / Non-Goals

**Goals:**

- Make `ito init` load existing config before asking wizard questions.
- Use explicit existing config values as selected defaults in the TUI.
- Add a config gap analysis mechanism so init/update coverage stays aligned with the config model.
- Render worktree-enabled project instructions that make dedicated change worktrees mandatory before write operations.
- Preserve explicit config values unless the user changes them or passes an overriding flag.
- Add regression tests for tmux, worktrees, and bare sibling strategy defaults.

**Non-Goals:**

- Redesign the full config schema or rename existing config keys.
- Add prompts for runtime-only settings that do not belong in setup.
- Change the managed-block update semantics for installed instruction assets.

## Decisions

- **Decision: Treat resolved existing config as the wizard default source.** Interactive init should build prompt defaults from loaded config rather than hard-coded defaults, while still falling back to current defaults for missing values. Alternative considered: only prefill a few known prompts. That would fix the immediate tmux/worktree issue but keep the UX surprising as new prompts are added.
- **Decision: Separate config coverage classification from prompt rendering.** The implementation should maintain a small table or equivalent metadata that classifies config fields as init-managed, update-refreshable, runtime-only, or excluded. Alternative considered: infer coverage from schema names. That is brittle and makes intentional exclusions hard to audit.
- **Decision: Flags override config, absence of flags preserves config.** Non-interactive flags should be explicit user intent; not passing a flag should not erase configured values. Alternative considered: always rewrite defaults on update. That would be simple but destructive and surprising.
- **Decision: Put the worktree rule in generated instructions, not only runtime behavior.** Worktree-enabled repos should receive explicit, portable instructions that main/control is read-only for writes and that agents must create or use the change worktree before any write operation. Alternative considered: rely only on OpenCode hook enforcement. Hooks help, but generated instructions are tool-agnostic and apply to every repo initialized by Ito.

## Risks / Trade-offs

- **Coverage list can become stale** -> Add tests that fail when a config field lacks a classification.
- **Resolved config may hide whether a value was explicit or defaulted** -> Prefer config loading APIs that preserve source/provenance where available; otherwise use raw project config for “explicitly set” decisions and resolved config for fallback defaults.
- **Interactive tests can be brittle** -> Keep prompt-level tests focused on selected defaults and resulting config, not terminal rendering details.
- **Instruction-only protection can still be ignored** -> Keep this change focused on generated instructions and coverage; pair with the separate OpenCode guard proposal for runtime enforcement.

## Migration Plan

No data migration is required. Existing configs remain valid; rerunning `ito init` should preserve explicit settings by default and only write changes when the user changes a selection or passes an overriding flag.

## Open Questions

- Which settings are classified as update-refreshable versus init-only after the gap analysis?
- Should the coverage classification be generated from schema metadata eventually, or remain a code-owned audit table for clarity?
<!-- ITO:END -->
