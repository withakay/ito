# Design: Managed File Ownership

## Overview

Add a managed-file registry and dry-run planner for Ito install/update outputs. The registry should describe generated paths, marker-managed paths, and durable user guidance locations.

## Ownership Types

- `generated`: fully owned by Ito and normally overwritten by update.
- `marker_managed`: file contains Ito start/end markers and only the managed block is owned by Ito.
- `user_owned`: expected to be edited by users or agents.
- `unknown`: not recognized by the registry.

## Commands

```bash
ito managed status --json
ito managed diff --json
ito update --dry-run --json
```

## Implementation Notes

- Reuse installer/template metadata where possible.
- Avoid reading remote resources during dry-run unless the normal update would do so and can report that dependency explicitly.
- Diff output should be compact and path-oriented, with optional text hunks for human review.

## Guidance Integration

Agent bootstrap and project guidance should state: before editing `.ito/`, `.opencode/`, `.github/`, `.codex/`, or marker-managed files, inspect managed status or choose a documented user guidance path.

## Risks

The registry can drift from installer behavior. Add tests that compare installer planned outputs with managed registry entries.
