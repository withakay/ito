# Iteration 6: Validation Documentation Alignment

*2026-05-11T06:47:34Z by Showboat 0.6.1*
<!-- showboat-id: 2b4c3607-ba2d-48a5-b97d-85dbb73f7e8f -->

Aligned the public schema customization docs and active cli-validate spec language with the implemented proposal behavior: ui_mechanics is gated by an explicit ui tag, and task_quality validates resolvable Requirements entries when present.

```bash
ito validate --changes 001-33_enhance-spec-driven-workflow-validation && ito audit reconcile --change 001-33_enhance-spec-driven-workflow-validation && git diff --check
```

```output
All items valid (14 checked)
Reconcile: 001-33_enhance-spec-driven-workflow-validation
──────────────────────────────────────────────────
No drift detected. Audit log and files are in sync.
```
