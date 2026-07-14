<!-- ITO:START -->
## Context

The repository already uses slash-command automation for targeted maintainer workflows. CI failures on pull requests currently require manual triage, reproduction, and branch updates, which can delay merge flow. This change adds a small, self-contained workflow that activates only on explicit PR-scoped triggers.

## Goals / Non-Goals

**Goals:**

- Add an on-demand `pr-fix` workflow that can inspect PR failures, attempt remediation, and report outcomes.
- Keep execution bounded with explicit permissions, timeout, safe outputs, and default network profile.
- Preserve operator control by requiring command/reaction invocation rather than always-on behavior.

**Non-Goals:**

- Building a general autonomous refactoring system outside PR-fix scope.
- Changing repository CI definitions or branch protection policies.
- Introducing new long-running infrastructure or external services.

## Decisions

- Define a dedicated workflow entry with `slash_command: pr-fix` and optional `reaction: eyes` to align with existing command-driven operator experience.
- Use a constrained toolset (`web-fetch`, `bash`) and read-all baseline permissions with explicit safe outputs to reduce blast radius while still enabling practical fixes.
- Encode a deterministic execution sequence in the workflow prompt: gather PR context, interpret optional user instructions, diagnose failing checks, apply fixes, verify via tests/formatters, push if progress, and comment summary.
- Keep implementation as a single workflow artifact for minimal rollout complexity and straightforward future iteration.

## Risks / Trade-offs

- [Automated fix attempts may choose an incorrect remediation] -> Mitigation: require explicit PR context, run validation checks, and provide transparent PR comment summary.
- [Workflow could spend time on non-actionable CI failures] -> Mitigation: enforce a 20-minute timeout and allow maintainers to re-run with specific instructions.
- [Push permissions could be overused if prompt scope drifts] -> Mitigation: limit allowed outputs/actions and keep task scope restricted to PR branch fixes.

## Migration Plan

1. Add the new PR-fix workflow definition to repository automation.
2. Validate syntax and trigger wiring in CI workflow checks.
3. Test on a non-critical PR by issuing `/pr-fix` with and without extra instructions.
4. Monitor first runs and tune prompt wording if repeated misdiagnosis appears.
5. Rollback by removing the workflow file if behavior is undesirable.

## Open Questions

- Should `/pr-fix` be restricted to maintainers only via an allowlist in a follow-up hardening change?
- Should future versions attach structured diagnostics (check names, log excerpts) to the PR comment for auditability?
<!-- ITO:END -->
