## Context

Ito already supports managed markers and non-destructive updates, but users still need a clear, reliable path for upgrading prompt/template content to newer embedded versions without losing local customizations. The request specifically calls out preserving everything outside Ito marker blocks and making upgrades easy to run.

## Goals / Non-Goals

**Goals:**

- Provide an explicit upgrade mode in `ito init` for prompt/template refresh.
- Ensure upgrade behavior is marker-scoped: update only between Ito markers and preserve everything else.
- Keep behavior deterministic and test-covered across `init`/`update` installer paths.

**Non-Goals:**

- Introduce a separate top-level `ito upgrade` command in this change.
- Reformat or rewrite user-owned files outside Ito-managed blocks.
- Replace human review for major upgrade diffs.

## Decisions

- Decision: Add `--upgrade` to `ito init` as the explicit user-facing upgrade mode, wired to existing managed update flow.
  - Alternatives considered:
    - Reuse `--update` only: rejected because users explicitly ask for an upgrade-oriented workflow and discoverability is weaker.
    - Add new top-level `ito upgrade`: deferred because it increases command surface and duplicates init/update plumbing.
- Decision: Use marker-scoped merge as the default for prompt/template files that contain Ito managed markers.
  - Alternatives considered:
    - Full-file overwrite for templates: rejected because it destroys local customizations.
    - Always skip changed files: rejected because it blocks adoption of embedded template improvements.
- Decision: For files expected to be marker-managed but missing markers, fail safe by preserving file content and emitting actionable guidance.
  - Alternatives considered:
    - Attempt heuristic merge without markers: rejected due high risk of corrupting user-authored content.
- Decision: Defer a dedicated `ito-upgrade` agent skill to a follow-up unless needed after CLI upgrade behavior lands.
  - Alternatives considered:
    - Add skill now: possible, but not required for core correctness; CLI behavior is the source of truth.

## Risks / Trade-offs

- Marker mismatch in legacy files could prevent upgrades from applying -> Mitigation: emit clear warnings and remediation steps.
- Alias behavior (`--update` vs `--upgrade`) can confuse users -> Mitigation: document precedence and keep both flags consistent.
- Broader marker coverage may expose edge cases in templates -> Mitigation: add focused tests for prompt/template fixtures.

## Migration Plan

1. Add `--upgrade` CLI handling and route it through managed installer update mode.
2. Extend marker-aware merge coverage for prompt/template assets that carry Ito markers.
3. Add tests for marker-only replacement and preservation outside markers.
4. Update docs/help text to steer users toward the explicit upgrade workflow.
5. Keep `--update` working for compatibility; consider deprecation messaging in a later change.

## Open Questions

- Should `--update` become a pure alias for `--upgrade`, or should one become preferred and the other deprecated?
- Should a follow-up change add an `ito-upgrade` agent skill that wraps validation plus `ito init --upgrade` execution?
