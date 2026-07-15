# Published Mirror Parity and Reconciliation Evidence

## Scope

This evidence compares the materialized tracked authority under `.ito/changes` and `.ito/specs` with the generated `docs/ito` mirror before mirror retirement. It records the reviewed path normalization, inventory and content-hash results, semantic reconciliations, and historical material retained in tracked authority.

The baseline was captured after authority materialization and before deleting or modifying `docs/ito`. The mirror remains present during this reconciliation wave.

## Path normalization

The ordinary one-to-one mappings are:

- `docs/ito/changes/active/<change-id>/<relative-path>` -> `.ito/changes/<change-id>/<relative-path>`
- `docs/ito/changes/archive/<dated-change-id>/<relative-path>` -> `.ito/changes/archive/<dated-change-id>/<relative-path>`
- `docs/ito/specs/<capability>/<relative-path>` -> `.ito/specs/<capability>/<relative-path>`

Five mirror directories were stale active snapshots of changes already archived in authority. Their reviewed cross-state mappings are:

| Mirror path | Authoritative path |
| --- | --- |
| `docs/ito/changes/active/000-11_normalize-main-spec-formatting` | `.ito/changes/archive/2026-05-12-000-11_normalize-main-spec-formatting` |
| `docs/ito/changes/active/001-32_add-planning-workflow` | `.ito/changes/archive/2026-05-12-001-32_add-planning-workflow` |
| `docs/ito/changes/active/001-33_enhance-spec-driven-workflow-validation` | `.ito/changes/archive/2026-05-12-001-33_enhance-spec-driven-workflow-validation` |
| `docs/ito/changes/active/001-34_add-ddd-discovery-workflow` | `.ito/changes/archive/2026-05-12-001-34_add-ddd-discovery-workflow` |
| `docs/ito/changes/active/019-09_ito-update-repo-skill` | `.ito/changes/archive/2026-05-13-019-09_ito-update-repo-skill` |

`docs/ito/README.md` is a generated landing page, not an Ito authority artifact, and therefore has no authority counterpart.

## Baseline inventory and hash results

The materialized authority contained 1,965 files. The mirror contained 1,815 files: 1,814 mapped authority artifacts and the generated-only README.

| Logical area | Common mapped files | Byte-equal | Different content | Authority-only | Mirror-only paths |
| --- | ---: | ---: | ---: | ---: | ---: |
| Active changes | 63 | 55 | 8 | 109 | 0 |
| Archived changes, including cross-state mappings | 1,552 | 1,537 | 15 | 39 | 0 |
| Current specs | 199 | 96 | 103 | 3 | 0 |
| **Total** | **1,814** | **1,688** | **126** | **151** | **0** |

There were no mirror-only file paths after normalization. Authority-only active files are later work in `016-18`, `025-12`, `030-01` through `030-06`, and `031-01` through `031-06`. The 39 authority-only archive files are later evidence under the five relocated archives. The three authority-only spec paths were `domain-discovery-workflow`, `spec-formatting`, and `tools-config`.

## Content-difference disposition

The 23 non-current-spec mismatches comprise 12 task/status-format updates and 11 small formatting, evidence, or correction changes. The materialized authority versions are newer or more complete; no requirement block or unique mirror file is discarded by retaining them.

The 103 differing current spec paths required a semantic comparison rather than an authority-wins assumption. Requirement headings, scenario headings, requirement identifiers, and normative `SHALL`/`MUST` lines were compared. That review found 12 accepted mirror requirements, 20 scenarios, and 23 normative lines missing from materialized authority across seven specs.

The reconciled disposition is:

- Preserve authority-newer requirements and merge three accepted export/discoverability requirements from the mirror into `cli-templates-schemas`.
- Preserve authority-newer domain validation requirements and merge four accepted requirement-traceability requirements into `cli-validate`.
- Preserve authority-newer domain-discovery requirements and merge three accepted `validation.yaml` requirements into `ito-schemas`.
- Preserve authority-newer task-quality requirements and merge the accepted enhanced-task requirement-reference contract into `tasks-tracking`.
- Preserve authority-newer domain-discovery workflow requirements and merge the accepted unified-workflow requirement into `workflow-convergence`.
- Prefer the accepted `031-04_remove-tmux-integration` mirror semantics in `config-schema` and `global-config`; the materialized versions incorrectly restored `tools.tmux.enabled`.
- Remove authority-only `tools-config`, which is stale state that accepted change `031-04` deleted. Preserve authority-only `domain-discovery-workflow` and `spec-formatting`.

The five merged requirement groups remain traceable to archived changes `001-23_embed-and-export-workflow-schemas`, `001-27_add-requirement-traceability`, `019-04_schema-driven-validation`, and `001-22_sunset-legacy-workflow-command`. The tmux-removal truth remains traceable to active change `031-04_remove-tmux-integration` and its integrated cutover-branch history.

## Current-truth promotion

Because changes `031-01`, `031-02`, and `031-03` have been accepted into the main-first reset but remain active until implementation is complete, their deltas were promoted without archiving the changes:

- Added the `coordination-main-migration` current spec and merged the new agent-instruction and reverse-migration requirements into `agent-instructions` and `coordination-worktree-migration`.
- Added the `main-first-implementation` current spec.
- Merged the feature-gating requirements into `backend-client-runtime`, `cascading-config`, `change-coordination-branch`, `release-automation`, and `rust-workspace`.

This keeps current specs aligned with the accepted workflow while retaining the active change packages for their remaining implementation and verification work.

## Retired mirror contract

The current `published-ito-mirror` capability was removed. The sole current `ito-config-crate` requirement described the same published-mirror path, so the now-empty current capability file was removed as well. No live Rust DTO, default, renderer, `ito publish` command, generated-schema property, test, or GitHub publication workflow exists in the repository; the implementation claimed by archived change `000-15_publish-ito-state-mirror` was not present in live code or its Git history.

The following historical evidence is intentionally retained:

- `.ito/changes/archive/2026-04-30-000-15_publish-ito-state-mirror/**`
- the corresponding completed-change entry in `.ito/modules/000_ungrouped/module.md`
- all other archived proposals, deltas, task records, and demos represented by the mirror

Unrelated audit-branch mirroring and backend archive/spec mirroring are separate capabilities and are not part of published-mirror retirement.

## Post-reconciliation checks

After reconciliation, the mirror-to-authority semantic comparison was repeated:

- All 12 previously missing accepted requirement headings are present once in current authority.
- All 25 requirement identifiers introduced by `031-01`, `031-02`, and `031-03` are present once in current authority.
- The only mirror normative lines absent from current authority are the five statements that require the retired published mirror or its configurable path.
- The only mirror scenario headings absent from current authority are the eleven scenarios belonging to the retired `published-ito-mirror` capability and published-mirror-path requirement.
- `ito validate --specs --strict` reports `All items valid (201 checked)`.
- `ito validate 031-06_migrate-ito-authority-and-release --strict` reports the change as valid.
- `ito validate --all --strict` reports `All items valid (256 checked)`.

## Retirement decision

The mirror has no unique file path to preserve. Its accepted semantic content has been merged or explicitly preferred where materialized authority was stale, later authority-only content remains tracked, and historical mirror-design evidence remains available in the archive. After the reconciled specs pass validation and current wiki/template/documentation references are redirected to `.ito`, `docs/ito` may be removed without losing reviewed Ito state.
