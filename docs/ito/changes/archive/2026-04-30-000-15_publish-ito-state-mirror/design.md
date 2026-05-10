<!-- ITO:START -->
## Context

Ito currently uses worktree-backed coordination state, with `.ito/changes`, `.ito/specs`, and related directories wired through symlinks into a separate coordination worktree. This makes active state invisible to plain GitHub browsing and to agents operating in a checkout that has not been Ito-wired.

## Goals / Non-Goals

**Goals:**

- Preserve the coordination branch as the only writable source of truth for live Ito state.
- Publish a committed, read-only mirror to `docs/ito` by default so plain `main` checkouts can inspect active changes and specs.
- Support a configurable published path instead of hardcoding `docs/ito`.
- Make drift behavior explicit when someone edits the published mirror directly.

**Non-Goals:**

- Replace coordination-worktree storage with committed in-tree `.ito` authoring.
- Make the published mirror itself an editable source of truth.
- Finalize every command/UI surface in this proposal; implementation may choose the exact publication trigger as long as the published-on-main behavior is satisfied.

## Decisions

- Decision: use a separate published path rather than committing live coordination state into canonical `.ito/...` paths on `main`.
- Decision: default the published mirror path to `docs/ito`, but expose a config override so repos can relocate the mirror.
- Decision: include active changes in the published mirror, not just archived changes and canonical specs, because the user needs in-flight state visible from plain checkouts.
- Decision: treat the published mirror as generated output and define direct edits as drift to be regenerated or rejected.

## Risks / Trade-offs

- Risk: the published mirror can lag behind coordination state if publication is not refreshed. Mitigation: specify refresh semantics and drift behavior explicitly.
- Risk: readers may confuse published mirror files with writable `.ito` source files. Mitigation: keep the mirror in a separate path and document it as generated/read-only.
- Risk: active change publication onto `main` may require additional integration workflow beyond simple coordination sync. Mitigation: make the publication lifecycle a first-class requirement rather than an accidental side effect of symlink wiring.
<!-- ITO:END -->
