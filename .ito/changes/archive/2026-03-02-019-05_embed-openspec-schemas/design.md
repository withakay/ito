<!-- ITO:START -->
## Context

The upstream repository `intent-driven-dev/openspec-schemas` provides a set of schemas and templates that are useful as real-world, non-Ito-native workflows. Ito can already load embedded schemas (via `ito-templates` assets) and can export them via `ito templates schemas export`.

To make these schemas usable out-of-the-box, Ito must embed a curated subset, ensure users can discover and select them, and include unambiguous attribution plus license compliance.

To avoid misleading validation outcomes, Ito should also ship Ito-authored `validation.yaml` files alongside these embedded schemas. The initial validation policy should prioritize correctness and clarity over deep semantic validation.

## Goals / Non-Goals

**Goals:**

- Embed `minimalist` and `event-driven` schemas as built-in assets.
- Add repository-tracked attribution and include upstream license text/references as required.
- Provide `validation.yaml` for each embedded OpenSpec schema that:
  - validates required artifact presence
  - emits an explicit informational issue stating semantic validation is manual (until a semantic validator exists)

**Non-Goals:**

- Implementing a semantic OpenSpec validator (for example `openspec.user-stories.v1`) in this change.
- Making Ito depend on upstream at runtime (no git submodules or runtime fetch).
- Supporting arbitrary upstream schema sets; this change is intentionally curated.

## Decisions

- **Vendoring strategy**: Copy upstream schema directories into `ito-rs/crates/ito-templates/assets/schemas/<name>/` and record the upstream repo URL and pinned commit hash in a small metadata file (for example `UPSTREAM.md`) to make future updates traceable.

- **Attribution location**: Add a single repository-level attribution file (prefer `THIRD_PARTY_NOTICES.md`) that:
  - names the upstream project and URL
  - lists which schemas were vendored
  - includes the required license text or required references

- **Validation policy (v1)**: For each embedded OpenSpec schema, ship an Ito-authored `validation.yaml` that performs presence checks for expected artifacts and emits an explicit `INFO` issue indicating semantic validation is not configured and must be performed manually.

## Risks / Trade-offs

- [Risk] License or attribution requirements could be missed.
  -> Mitigation: treat attribution as a first-class acceptance criterion; include upstream license text and verify compliance before merge.

- [Risk] Users expect semantic validation.
  -> Mitigation: emit an explicit informational issue indicating validation coverage and the manual validation expectation.

- [Risk] Schema names could conflict with user/project schemas.
  -> Mitigation: rely on the existing override precedence (project-local, then user, then built-in) and document it.
<!-- ITO:END -->
