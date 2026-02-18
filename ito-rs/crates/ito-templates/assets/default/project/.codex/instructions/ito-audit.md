# Ito Audit Guardrails (Codex)

Codex does not provide a reliable pre-tool hook API for policy checks.

Follow these rules before any stateful work:

1. Run `ito audit validate` at session start.
2. If validation fails, stop and request user guidance before continuing.
3. Before mutating `.ito/` state (tasks, specs, archive prep, merge prep), run:
   - `ito audit validate`
   - `ito audit reconcile`
4. If reconcile reports drift and repair is appropriate, run `ito audit reconcile --fix` and then re-run `ito audit validate`.
5. Never continue stateful work while audit validation is failing.
