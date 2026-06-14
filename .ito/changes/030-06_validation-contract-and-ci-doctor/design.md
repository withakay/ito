# Design: Validation Contract And CI Doctor

## Overview

Create a validation planner and reporter that gives agents one deterministic place to ask: what should be run, what ran, and what failed?

## Commands

```bash
ito check --json
ito test affected --json
ito doctor ci --json
```

## Validation Plan Sources

Order of precedence:

1. Explicit Ito config validation commands.
2. Project setup metadata recorded by Ito.
3. Makefile targets such as `check` and `test`.
4. Language/package manifests such as Cargo, npm, or Python tooling.
5. Agent guidance as advisory text only, not the primary machine contract.

## Output Model

Validation output should include:

- `result`
- `command_source`
- `commands`
- `duration_ms`
- `failure_excerpt`
- `artifacts`
- `fallback_reason`
- `next_steps`

## CI Doctor

Start with GitHub Actions because existing workflows use `gh`. Keep provider-specific code behind a small trait so other CI providers can be added later.

## Risks

Running validation can be expensive. Support dry-run or plan-only modes if needed, but keep `ito check --json` as the simplest default for agents.
