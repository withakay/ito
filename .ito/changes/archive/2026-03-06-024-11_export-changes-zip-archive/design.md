## Context

Backend mode is becoming the primary system of record for change artifacts and lifecycle state. Teams need a portable, canonical export artifact before migrations, cleanup operations, or environment handoffs. Today there is no single command that packages backend change history into a validated bundle.

## Goals / Non-Goals

**Goals:**

- Add `ito backend export` to produce a zip archive containing active and archived changes.
- Define a canonical archive layout with manifest metadata so downstream tooling can consume exports consistently.
- Include integrity metadata for exported files.
- Keep export deterministic for unchanged backend state.

**Non-Goals:**

- Implementing zip import in this change.
- Capturing non-change project files outside change artifacts.
- Providing incremental/delta-only exports.

## Decisions

- Decision: Expose export as `ito backend export`.
  - Rationale: keeps backend migration/backup operations under the backend namespace.
  - Alternative considered: `ito tasks export`. Rejected to avoid overloading task workflows with backend lifecycle operations.

- Decision: Archive layout uses explicit lifecycle roots (`changes/active/`, `changes/archived/`) plus `manifest.json`.
  - Rationale: makes lifecycle semantics unambiguous and easy to validate.
  - Alternative considered: mirroring `.ito/changes/` tree exactly. Rejected because archived path conventions are harder to normalize across systems.

- Decision: Manifest includes format version, counts, and per-file checksums.
  - Rationale: supports integrity verification and forward-compatible parsing.
  - Alternative considered: no checksums. Rejected because portability without integrity checks creates silent corruption risk.

- Decision: Export ordering is deterministic by canonical change ID and normalized path ordering.
  - Rationale: improves reproducibility and testability.
  - Alternative considered: backend-return order. Rejected because unstable order makes validation and diffs noisy.

## Risks / Trade-offs

- [Large exports consume memory/disk] -> Stream files into zip and surface archive size in summary output.
- [Checksum generation increases runtime] -> Use efficient hashing and bounded buffering.
- [Schema evolution breaks consumers] -> Version `manifest.json` and keep backward-compatible parsing guidance.
- [Archived lifecycle mapping bugs] -> Add integration tests with mixed active/archived data.

## Migration Plan

1. Add core export orchestration to fetch active and archived backend changes.
2. Implement canonical zip writer and manifest generator with checksums.
3. Add CLI command `ito backend export` with optional output path.
4. Add tests for layout, deterministic ordering, integrity metadata, and backend-mode gating.
5. Document export workflow as the canonical backup step for backend deployments.

## Open Questions

- Should the default filename include project slug plus timestamp, or only timestamp?
- Should the command expose `--include-archived=false` now or in a follow-up?
