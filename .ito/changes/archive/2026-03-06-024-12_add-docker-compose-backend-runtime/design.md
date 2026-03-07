<!-- ITO:START -->
## Context

The backend can be launched via `ito serve-api`, but local testing currently depends on each contributor wiring their own command, environment, and lifecycle management. A repository-managed Docker Compose definition provides a repeatable runtime boundary that is easy to start, stop, and troubleshoot.

## Goals / Non-Goals

**Goals:**

- Provide a documented and reproducible Docker Compose runtime for local backend testing.
- Keep startup and teardown simple enough for day-to-day developer workflows.
- Ensure the runtime exposes a clear health-check path so developers can quickly confirm readiness.

**Non-Goals:**

- Shipping Homebrew service definitions.
- Shipping systemd unit files.
- Production deployment hardening or orchestration beyond local testing needs.

## Decisions

- Use Docker Compose as the first supported local runtime wrapper around the existing backend binary/entrypoint.
  - Rationale: low setup overhead, widely available developer tooling, and easy lifecycle commands (`up`, `down`, logs).
- Keep runtime configuration minimal and local-testing oriented, with documented defaults and optional overrides.
  - Rationale: prioritize fast feedback loops over broad deployment flexibility in this phase.
- Treat Homebrew/systemd integrations as explicit follow-up changes instead of bundling multiple service-manager paths now.
  - Rationale: keeps scope focused and reduces review/maintenance overhead for this initial runtime path.

## Alternatives Considered

- Homebrew service first: good macOS UX, but not cross-platform and does not help Linux CI/local parity.
- systemd unit first: useful on Linux hosts, but higher operational complexity and poor portability for contributors.
- No managed runtime artifacts: preserves status quo but continues inconsistent setup and onboarding friction.

## Risks / Trade-offs

- [Risk] Developers without Docker cannot use the compose workflow.
  -> Mitigation: retain existing non-compose backend startup path and document compose as an additional supported option.

- [Risk] Compose defaults may diverge from expected backend config over time.
  -> Mitigation: keep config surface small and validate runtime docs/config together in change review.

- [Risk] Runtime expectations expand to production use unintentionally.
  -> Mitigation: document local-testing scope and defer production/service-manager concerns to separate proposals.

## Migration Plan

1. Add compose runtime assets and local testing documentation.
2. Validate compose configuration and basic backend health flow.
3. Keep existing backend startup methods intact as fallback.

Rollback: remove compose artifacts and docs; backend startup via existing methods remains available.

## Open Questions

- Should the compose file live at repository root or under `ito-rs/` as backend-adjacent tooling?
- Should follow-up service-manager work target Homebrew and systemd in one change or split by platform?
<!-- ITO:END -->
