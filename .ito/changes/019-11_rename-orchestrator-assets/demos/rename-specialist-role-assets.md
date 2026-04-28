# Rename scoped orchestrator specialist assets

*2026-04-28T19:12:25Z by Showboat 0.6.1*
<!-- showboat-id: fbf9c422-330b-4935-b76b-3f85fb72505a -->

This demo shows that only the specialist role assets were renamed from ito-orchestrator-* to concise ito-* names while top-level ito-orchestrator and ito-orchestrator-workflow assets remain intact.

```bash
rg -n 'ito-orchestrator-(planner|researcher|reviewer|worker)' ito-rs/crates/ito-templates ito-rs/crates/ito-cli/tests .agents .opencode .claude .github .pi
```

```output
```
