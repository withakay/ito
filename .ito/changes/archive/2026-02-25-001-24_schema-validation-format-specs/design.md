<!-- ITO:START -->
## Context

Ito already parses and validates delta spec markdown (change delta specs) and tasks tracking markdown (`tasks.md`). Those validations are currently defined by code behavior, without a versioned, normative specification that validators can cite.

## Goals / Non-Goals

**Goals:**

- Define and version the delta spec markdown format as a first-class spec with a stable validator id.
- Define and version the tasks tracking markdown format as a first-class spec with a stable validator id.
- Ensure validation issues can cite the validator id to route authors to the right spec.

**Non-Goals:**

- Changing the delta spec or tasks tracking formats in breaking ways.
- Rewriting parsers; v1 reflects current behavior.
- Making schema validation depend on network or external documentation.

## Decisions

- **Spec locations**: Add new capabilities under `.ito/specs/delta-specs/spec.md` and `.ito/specs/tasks-tracking/spec.md` when the change is archived.
- **Validator ids**: Use `ito.delta-specs.v1` and `ito.tasks-tracking.v1` as stable identifiers that can be surfaced in issues and referenced by schemas.
- **Error messaging**: Any validation failure attributable to one of these formats SHALL include the relevant validator id so authors can locate the matching v1 spec.
- **Versioning policy**: v1 documents "as implemented" behavior; future changes that intentionally alter accepted syntax create `v2` validator ids and specs.

## Risks / Trade-offs

- **Risk**: Specs drift from actual parser behavior.
  - Mitigation: Keep v1 narrowly scoped to existing behavior and add tests that tie validator behavior to the documented ids.
<!-- ITO:END -->
