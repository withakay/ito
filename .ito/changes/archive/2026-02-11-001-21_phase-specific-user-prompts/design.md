## Context

Instruction output currently supports one shared guidance file: `.ito/user-guidance.md`. That works for broad policy, but it is too coarse for workflows where proposal writing and apply execution need different constraints. Users currently workaround this by editing guidance between phases, which is brittle and error-prone.

## Goals / Non-Goals

**Goals:**

- Add artifact-scoped user guidance files for proposal/apply and future artifacts.
- Keep existing `.ito/user-guidance.md` behavior fully backward compatible.
- Keep schema instructions authoritative and treat user guidance as additive.
- Use a simple naming convention that scales without new config knobs.

**Non-Goals:**

- Introducing per-change guidance files in this change.
- Adding a new DSL for prompt merging/ranking.
- Replacing schema-defined instructions with user-provided prompts.

## Decisions

- **Directory + naming:** Use `.ito/user-prompts/<artifact-id>.md` to map guidance to artifact IDs (`proposal`, `apply`, etc.).
- **Composition model:** Inject scoped guidance when present, then include shared guidance from `.ito/user-prompts/guidance.md`, falling back to `.ito/user-guidance.md` for compatibility.
- **Failure behavior:** Missing scoped files are non-errors; instruction generation proceeds normally.
- **Compatibility:** Keep existing managed header and update semantics for `.ito/user-guidance.md`; scoped files remain fully user-owned.

## Risks / Trade-offs

- **[Risk] Guidance conflicts** -> Schema requirements stay authoritative and unchanged.
- **[Risk] Path confusion** -> Document canonical location and examples in project docs.
- **[Risk] Prompt bloat** -> Keep output format compact and deterministic for both guidance sources.

## Migration Plan

1. Add loader support for `.ito/user-prompts/<artifact-id>.md`.
2. Wire instruction rendering to compose scoped + shared guidance.
3. Add tests for proposal/apply scoped guidance and fallback behavior.
4. Update docs with recommended file layout and examples.

## Open Questions

- Should `ito init` scaffold `.ito/user-prompts/` with sample files, or remain opt-in?
- Should future support include wildcard or group-level guidance (for example all write artifacts)?
