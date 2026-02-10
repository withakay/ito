## Context

Current workflow schema loading relies on filesystem package paths (for example, repository `schemas/` during local builds). That approach is fragile across install channels and fails when schemas are not colocated with the runtime binary. Users also lack a first-class way to materialize built-ins into project-local override directories for customization.

## Goals / Non-Goals

**Goals:**

- Embed default workflow schemas in `ito-templates/assets/schemas` so they are always available with the binary.
- Add project-local schema override support at `.ito/templates/schemas/<name>/`.
- Preserve user-level override support at `${XDG_DATA_HOME}/ito/schemas/<name>/`.
- Add `ito templates schemas export` so users can write embedded schemas to local disk.

**Non-Goals:**

- Redesign workflow schema format (`schema.yaml`) or artifact semantics.
- Remove user-level override support.
- Build a full schema marketplace/remote registry.

## Decisions

- **Decision: Move built-ins to `ito-templates` embedded assets.**
  - Rationale: `ito-templates` already owns embedded install assets and provides a stable include-dir pattern.
  - Alternative: keep filesystem lookup in repo/package paths only; rejected as brittle.

- **Decision: Resolution precedence becomes project-local -> user -> embedded built-in -> optional legacy fallback.**
  - Rationale: project-local overrides should be reproducible in-repo; user overrides remain global customizations.
  - Alternative: user over project; rejected because project should define collaborative defaults.

- **Decision: Add explicit export command under `ito templates schemas export`.**
  - Rationale: users need a clear command to bootstrap local overrides from built-ins.
  - Alternative: require manual copy from install paths; rejected due to poor discoverability and portability.

## Risks / Trade-offs

- **[Risk] Duplicate source of truth during migration** -> Keep a short compatibility fallback and phase out legacy `schemas/` once tests pass across installers.
- **[Risk] Command surface confusion (`templates` vs existing experimental commands)** -> Document canonical command path and provide help text aliases if needed.
- **[Risk] Overwrite surprises during export** -> Require explicit force mode for destructive writes and print per-file actions.

## Migration Plan

1. Add embedded schema assets under `ito-rs/crates/ito-templates/assets/schemas`.
2. Update schema resolver in `ito-core` to consult project-local and user override paths, then embedded built-ins.
3. Add `ito templates schemas export` CLI path with optional force behavior.
4. Add tests for precedence, export behavior, and deterministic output.
5. Optionally keep legacy package `schemas/` fallback temporarily, then deprecate/remove in a follow-up.

Rollback: keep legacy filesystem schema loading as primary and disable export command wiring.

## Open Questions

- Should `ito templates schemas export` default target be `.ito/templates/schemas` when `-f` is omitted?
- Should exported files include provenance comments or remain exact copies for minimal diff churn?
