---
name: ito-orchestrator-workflow
description: Repo-specific workflow conventions consumed by the orchestrator.
---

<!-- ITO:START -->
<!--ITO:VERSION:0.1.29-->


This is a living-document skill: keep it updated as your repo's tooling and conventions evolve.

## Goal

Provide repo-specific workflow guidance that the orchestrator can load by convention.

This skill SHOULD include:

- The commands to run for format/lint/tests
- Any preconditions (services to start, env vars, etc)
- What constitutes "done" for reviewer gates
- Remediation expectations (what to include in a remediation packet)

## Suggested Content

### Gate Commands

- format: `<repo command>`
- lint: `<repo command>`
- tests: `<repo command>`

### Review Guidelines

- Code-review: prioritize correctness and regressions
- Security-review: prioritize secrets, injection, unsafe deserialization, auth boundaries

### Project Notes

- Add any repo-specific gotchas that help orchestrator coordination.

<!-- ITO:END -->
