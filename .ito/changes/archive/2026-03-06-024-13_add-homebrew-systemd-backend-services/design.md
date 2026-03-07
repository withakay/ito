<!-- ITO:START -->
## Context

Change `024-12_add-docker-compose-backend-runtime` introduces a containerized path for backend testing, but not every environment uses containers for long-lived local services. For day-to-day development and lightweight self-hosting, service-manager integrations provide simpler lifecycle control, restart behavior, and log access.

## Goals / Non-Goals

**Goals:**

- Define a Homebrew service workflow for backend runtime on macOS.
- Define a systemd service workflow for backend runtime on Linux.
- Ensure both service-manager paths invoke the same backend entrypoint with documented configuration.

**Non-Goals:**

- Replacing Docker Compose as the container runtime option.
- Supporting additional init/service systems in this change (launchd beyond Homebrew, OpenRC, Windows services).
- Hardening this flow for managed production orchestration.

## Decisions

- Keep `ito serve-api` as the single process entrypoint for all service-manager wrappers.
  - Rationale: avoids divergence in backend startup semantics across runtime options.
- Add service-manager-specific artifacts (Homebrew service definition and systemd unit template) with clear, documented defaults.
  - Rationale: explicit artifacts are easier to review and troubleshoot than generated ad-hoc commands.
- Document both workflows in backend runtime docs with platform-specific command examples and log/status checks.
  - Rationale: operational usability depends on discoverable lifecycle commands, not just shipped files.

## Alternatives Considered

- Keep Docker Compose as the only managed runtime: simpler scope, but does not address host-native service needs.
- Implement only Homebrew or only systemd first: faster initial delivery, but leaves one major platform without parity.
- Add a custom internal service supervisor: higher maintenance burden and duplicates mature platform primitives.

## Risks / Trade-offs

- [Risk] Homebrew and systemd behaviors diverge in restart/logging semantics.
  -> Mitigation: constrain both paths to the same backend command/config and document expected differences.

- [Risk] Platform-specific setup introduces additional support surface.
  -> Mitigation: keep interfaces narrow, provide explicit troubleshooting commands, and defer non-core platforms.

- [Risk] Users treat these flows as production deployment guidance.
  -> Mitigation: label scope as development/self-hosted runtime support and defer production deployment guidance.

## Migration Plan

1. Add service-manager artifacts and docs for Homebrew and systemd.
2. Validate each flow can start, report healthy backend state, and stop cleanly.
3. Keep Docker Compose and direct CLI startup paths unchanged.

Rollback: remove service-manager artifacts/docs and continue using existing compose/direct runtime flows.

## Open Questions

- Should service-manager artifacts be generated from shared templates or maintained as static files?
- Should systemd support user units only, or both user and system units in initial scope?
<!-- ITO:END -->
