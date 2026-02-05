# Documentation Drift Analysis: Ito vs Original OpenSpec

**Date**: 2026-01-18
**Purpose**: Identify and document areas where Ito documentation has drifted from the original OpenSpec spec

______________________________________________________________________

## Executive Summary

Ito is a fork of OpenSpec that has added significant features for project-centric planning and long-running, multi-agent workflows. This document identifies the key areas of documentation drift between the original OpenSpec conventions and the current Ito implementation.

**Key Finding**: The primary drift is not in documentation, but in **feature additions** and **directory structure changes** that extend beyond the original OpenSpec scope.

______________________________________________________________________

## 1. Directory Structure Changes

### Original OpenSpec

```
ito/
├── AGENTS.md
├── project.md
├── specs/
└── changes/
    ├── <change-id>/
    │   ├── proposal.md
    │   ├── design.md
    │   ├── tasks.md
    │   └── specs/
    └── archive/
```

### Current Ito

```
.ito/                          # ← Hidden directory (changed from ito/)
├── AGENTS.md                         # Root-level instructions (was in ito/)
├── project.md                        # Minimal project overview
├── planning/                         # NEW: Project-level planning artifacts
│   ├── PROJECT.md                    # NEW: Project vision, constraints
│   ├── ROADMAP.md                    # NEW: Phased milestones
│   └── STATE.md                      # NEW: Session state persistence
├── research/                         # NEW: Domain research artifacts
│   ├── SUMMARY.md
│   └── investigations/
│       ├── stack-analysis.md
│       ├── feature-landscape.md
│       ├── architecture.md
│       └── pitfalls.md
├── changes/
│   ├── <change-id>/
│   │   ├── proposal.md
│   │   ├── design.md
│   │   ├── tasks.md                  # ENHANCED: Structured with waves
│   │   ├── specs/
│   │   ├── reviews/                  # NEW: Adversarial review outputs
│   │   └── change.yaml               # NEW: Schema metadata (proposed)
│   └── archive/
├── workflows/                        # NEW: YAML workflow definitions
│   ├── research.yaml
│   ├── execute.yaml
│   ├── review.yaml
│   └── .state/                       # NEW: Workflow execution state
├── commands/                         # NEW: AI tool slash command templates
│   ├── research-*.md
│   ├── plan-*.md
│   ├── execute.md
│   └── review-*.md
└── config.yaml                       # NEW: Agent configuration (proposed)
```

### Documentation Inconsistencies

| Document | Uses | Should Use |
|----------|------|------------|
| `README.md` | `.ito/` | ✅ Correct |
| `AGENTS.md` | `.ito/` | ✅ Correct |
| `cli-init/spec.md:28-34` | `ito/` | ❌ Should be `.ito/` |
| `cli-validate/spec.md:13` | `ito/` | ❌ Should be `.ito/` |
| `schema-customization.md` | `ito/` | ⚠️ Context-dependent |
| `schema-workflow-gaps.md` | `ito/` | ⚠️ Context-dependent |

**Critical Issue**: The `cli-init/spec.md` spec explicitly creates a `ito/` directory, but the README says the default is `.ito/`. This is a **documentation vs implementation inconsistency**.

______________________________________________________________________

## 2. Spec Format Drift

### Original OpenSpec Spec Format

```markdown
# Spec Name

## Purpose
[Brief description of what this spec defines]

## Requirements

### Requirement: [Title]
Descriptive text explaining the requirement.

#### Scenario: [Short name]
- **WHEN** [precondition or action]
- **THEN** [expected result]
- **AND** [additional outcomes]
```

**Key Conventions**:

- `### Requirement:` headers with SHALL statements
- `#### Scenario:` headers with structured WHEN/THEN/AND format
- Bold keywords: `**WHEN**`, `**THEN**`, `**AND**`
- Descriptive text must follow requirement header before scenarios

### Current Ito Spec Format

**Same as original OpenSpec** - ✅ No drift detected.

The `ito-conventions/spec.md` explicitly defines and maintains the original format:

- `### Requirement:` + descriptive text
- `#### Scenario:` + bold WHEN/THEN/AND
- Non-breaking gradual migration support
- Allows alternative formats (OpenAPI, JSON Schema)

**Documentation Accuracy**: ✅ Spec format documentation is consistent.

______________________________________________________________________

## 3. Change Proposal Format Drift

### Original OpenSpec Change Format

```markdown
## Why
[Reason for the change]

## What Changes
- Bulleted list of changes
```

### Current Ito Change Format

**Enhanced format** (from `schemas/spec-driven/templates/proposal.md`):

```markdown
## Why
[Reason for the change]

## What Changes
- Bulleted list of changes

## Capabilities

### New Capabilities
List of new capabilities being added.

### Modified Capabilities
List of existing capabilities being modified.

## Impact
Description of impact on existing functionality.
```

**Key Enhancements**:

- Explicit `## Capabilities` section
- Separate `New` and `Modified` capabilities
- Structured `## Impact` section
- Delta storage format: `## ADDED`, `## MODIFIED`, `## REMOVED`, `## RENAMED`

**Documentation Accuracy**: ✅ Templates match documentation.

______________________________________________________________________

## 4. Features Added Beyond OpenSpec

### 4.1 Project Planning (NEW)

**Files**: `planning/PROJECT.md`, `planning/ROADMAP.md`, `planning/STATE.md`

**Purpose**: Multi-session project context, milestone tracking, state persistence.

**Status**: ✅ Documented in README and `project-planning-research-proposal.md`

______________________________________________________________________

### 4.2 Research Phase (NEW)

**Files**: `research/SUMMARY.md`, `research/investigations/*.md`

**Purpose**: Pre-proposal domain investigation (stack analysis, feature landscape, architecture, pitfalls).

**Status**: ✅ Documented in README and `project-planning-research-proposal.md`

______________________________________________________________________

### 4.3 Enhanced Tasks Format (ENHANCED)

**File**: `changes/<id>/tasks.md`

**Changes from original**:

- Waves (grouping and parallelizable chunks)
- Explicit `Verify` commands
- `Done When` acceptance criteria
- Task status tracking (pending/in-progress/complete)
- Checkpoint tasks for human approval

**Status**: ✅ Documented in `project-planning-research-proposal.md` but **not reflected in spec template** (`schemas/spec-driven/templates/tasks.md` is just a placeholder).

**Gap**: The enhanced tasks format is documented but the template hasn't been updated.

______________________________________________________________________

### 4.4 Adversarial Review (NEW)

**Files**: `changes/<id>/reviews/`, command templates

**Purpose**: Systematic multi-perspective review (security, scale, edge cases).

**Status**: ✅ Documented in `project-planning-research-proposal.md`

______________________________________________________________________

### 4.5 Workflow Orchestration (NEW)

**Files**: `workflows/*.yaml`, `workflows/.state/*.json`

**Purpose**: YAML-defined workflows with waves, tasks, and checkpoints.

**Commands**:

- `ito workflow init`
- `ito workflow list`
- `ito workflow show <workflow>`
- `ito workflow run <workflow> --tool <tool> -v topic="..."`
- `ito workflow status <workflow>`

**Status**: ✅ Documented in README

______________________________________________________________________

### 4.6 Agent Configuration (NEW)

**File**: `config.yaml`

**Purpose**: Per-tool model selection and context budgets.

**Commands**:

- `ito agent-config init`
- `ito agent-config summary`
- `ito agent-config get <path>`
- `ito agent-config set <path> <value>`

**Status**: ⚠️ Documented in README but **not implemented** in specs (no `agent-config` spec exists).

**Gap**: Feature is documented but not fully specified.

______________________________________________________________________

### 4.7 Schema Customization (ENHANCED)

**Feature**: 2-level schema resolution (XDG user override → package built-in).

**Resolution Order**:

1. `./ito/schemas/<name>/` (NEW: Project-local)
1. `~/.local/share/ito/schemas/<name>/` (User global)
1. `<npm-package>/schemas/<name>/` (Built-in)

**Status**: ✅ Documented in `schema-customization.md` and `schema-workflow-gaps.md`

**Gap**: Schema management CLI (`ito schema list/copy/diff/reset`) is **proposed but not implemented**.

______________________________________________________________________

### 4.8 Change Metadata (PROPOSED)

**File**: `changes/<id>/change.yaml`

**Purpose**: Bind schema to change, store metadata.

```yaml
schema: tdd
created: 2025-01-15T10:30:00Z
description: Add user authentication system
```

**Status**: ⚠️ Proposed in `schema-workflow-gaps.md` but **not implemented**.

**Gap**: Feature is proposed but not specified or implemented.

______________________________________________________________________

## 5. CLI Command Extensions

### 5.1 New Commands

| Command | Status | Documentation |
|---------|--------|----------------|
| `ito plan init/status` | ✅ Implemented | README |
| `ito research init/status` | ⚠️ Proposed | `project-planning-research-proposal.md` |
| `ito tasks init/status/start/complete/next` | ⚠️ Proposed | `project-planning-research-proposal.md` |
| `ito workflow init/list/show/run/status` | ✅ Implemented | README |
| `ito agent-config init/summary/get/set` | ⚠️ Proposed | README |
| `ito schema list/which/copy/diff/reset/validate` | ⚠️ Proposed | `schema-customization.md` |
| `ito state` | ⚠️ Proposed | `project-planning-research-proposal.md` |

### 5.2 Enhanced Commands

| Command | Enhancement | Status |
|---------|-------------|--------|
| `ito init` | AI tool selection, progress indicators | ✅ Implemented |
| `ito list` | `--specs` flag, interactive selection | ✅ Implemented |
| `ito show` | `--json`, `--deltas-only`, `--type` flags | ✅ Implemented |
| `ito validate` | `--all`, `--changes`, `--specs`, `--strict`, `--type`, `--no-interactive` | ✅ Implemented |
| `ito change` | `show`, `list`, `validate` subcommands | ✅ Implemented |
| `ito archive` | Change arguments, dry-run | ✅ Implemented |

______________________________________________________________________

## 6. Documentation Quality Issues

### 6.1 Inconsistent Directory References

**Issue**: Mixed use of `ito/` and `.ito/` across documentation.

**Examples**:

- `cli-init/spec.md:28-34` creates `ito/` directory
- `cli-validate/spec.md:13` references `ito/changes/`
- `README.md` consistently uses `.ito/`
- `schema-customization.md` uses `ito/schemas/` (project-local context)

**Recommendation**: Audit all documentation and standardize on `.ito/` for the working directory. Update `cli-init/spec.md` to reflect the actual implementation.

______________________________________________________________________

### 6.2 Missing Implementation Specs

**Issue**: Features are documented in README or proposals but lack corresponding spec files.

**Examples**:

- `agent-config` commands: No spec in `.ito/specs/`
- `plan` commands: No spec in `.ito/specs/`
- `research` commands: No spec in `.ito/specs/`
- `tasks` commands: No spec in `.ito/specs/`
- `workflow` commands: No spec in `.ito/specs/`

**Impact**: Features are described but not formally specified, leading to implementation ambiguity.

**Recommendation**: Create spec files for each command group following the established `cli-*` spec pattern.

______________________________________________________________________

### 6.3 Template Drift

**Issue**: The `tasks.md` template doesn't reflect the enhanced format documented in proposals.

**Current Template** (`schemas/spec-driven/templates/tasks.md`):

```markdown
## Tasks
- [ ] Task 1
- [ ] Task 2
```

**Documented Format** (`project-planning-research-proposal.md`):

```markdown
## Wave 1

### Task 1.1: [Title]
- **Files**: [...]
- **Dependencies**: [...]
- **Action**: [...]
- **Verify**: [...]
- **Done When**: [...]
- **Status**: [ ] pending / [ ] in-progress / [x] complete
```

**Recommendation**: Update the `tasks.md` template to match the documented enhanced format.

______________________________________________________________________

### 6.4 Deprecated Documentation

**Issue**: Some archived changes contain outdated directory references.

**Example**: `2025-08-19-structured-spec-format/proposal.md` may contain outdated paths.

**Impact**: Archived documentation can confuse users who reference it for historical context.

**Recommendation**: Add migration notes or deprecation headers to archived documents.

______________________________________________________________________

## 7. Spec Compliance Analysis

### 7.1 Spec Format Compliance

| Spec File | Format Compliant | Issues |
|-----------|------------------|--------|
| `cli-change/spec.md` | ✅ | None |
| `cli-init/spec.md` | ✅ | Directory name inconsistency |
| `cli-list/spec.md` | ✅ | None |
| `cli-show/spec.md` | ✅ | None |
| `cli-validate/spec.md` | ✅ | Directory name inconsistency |
| `artifact-graph/spec.md` | ✅ | None |
| `ito-conventions/spec.md` | ✅ | None |

**Overall**: ✅ Spec format is consistent and follows original OpenSpec conventions.

______________________________________________________________________

### 7.2 Change Proposal Compliance

| Change | Format Compliant | Issues |
|--------|------------------|--------|
| `2025-12-25-add-change-manager` | ✅ | None |
| `2025-08-19-structured-spec-format` | ✅ | None |
| `2025-10-14-add-non-interactive-init-options` | ✅ | None |

**Overall**: ✅ Change proposals follow the enhanced Ito format.

______________________________________________________________________

## 8. Recommendations

### 8.1 High Priority

1. **Fix directory name inconsistency**:

   - Update `cli-init/spec.md` to use `.ito/`
   - Update `cli-validate/spec.md` to use `.ito/`
   - Add a migration note in README explaining the change from `ito/` to `.ito/`

1. **Create missing spec files**:

   - `cli-plan/spec.md` for `ito plan` commands
   - `cli-research/spec.md` for `ito research` commands
   - `cli-tasks/spec.md` for `ito tasks` commands
   - `cli-workflow/spec.md` for `ito workflow` commands
   - `cli-agent-config/spec.md` for `ito agent-config` commands

1. **Update tasks.md template**:

   - Reflect the enhanced format with waves, verify commands, and status tracking

______________________________________________________________________

### 8.2 Medium Priority

4. **Implement proposed features**:

   - `change.yaml` metadata (as proposed in `schema-workflow-gaps.md`)
   - Schema management CLI (`ito schema list/copy/diff/reset`)
   - Project-local schema resolution

1. **Document migration path**:

   - Add migration guide for users with `ito/` directories
   - Document how to upgrade from older Ito versions

______________________________________________________________________

### 8.3 Low Priority

6. **Clean up archived documentation**:

   - Add deprecation headers to archived changes
   - Update outdated path references in archive

1. **Improve documentation cross-references**:

   - Add links between related documentation files
   - Create a comprehensive index of all Ito features

______________________________________________________________________

## 9. Conclusion

**Summary**:

- ✅ **Core spec format** is fully compliant with original OpenSpec
- ✅ **Change proposal format** follows documented conventions
- ⚠️ **Directory structure** has inconsistencies between documentation and specs
- ⚠️ **Feature completeness**: Many features are documented but not specified or implemented
- ⚠️ **Template drift**: Tasks template doesn't match documented enhancements

**Assessment**: Ito has successfully maintained the original OpenSpec core conventions while significantly extending the feature set. The primary documentation drift issues are:

1. Inconsistent directory naming (ito/ vs .ito/)
1. Missing spec files for documented features
1. Outdated templates that don't reflect enhanced formats

**Next Steps**: Address high-priority recommendations to align documentation, specs, and implementation.

______________________________________________________________________

## Appendix A: File Inventory

### Documentation Files

- `README.md` - Main project documentation
- `AGENTS.md` - AI assistant instructions (root level)
- `.ito/AGENTS.md` - AI assistant instructions (ito level)
- `docs/schema-customization.md` - Schema customization guide
- `docs/project-planning-research-proposal.md` - Planning and research extension proposal
- `docs/schema-workflow-gaps.md` - Schema workflow analysis
- `CHANGELOG.md` - Version history

### Template Files

- `schemas/spec-driven/templates/proposal.md`
- `schemas/spec-driven/templates/spec.md`
- `schemas/spec-driven/templates/design.md`
- `schemas/spec-driven/templates/tasks.md`
- `schemas/tdd/templates/*.md`

### Spec Files

- `.ito/specs/cli-change/spec.md`
- `.ito/specs/cli-init/spec.md`
- `.ito/specs/cli-list/spec.md`
- `.ito/specs/cli-show/spec.md`
- `.ito/specs/cli-validate/spec.md`
- `.ito/specs/artifact-graph/spec.md`
- `.ito/specs/ito-conventions/spec.md`

### Archived Changes

- `.ito/changes/archive/` - Historical change proposals

______________________________________________________________________

**Document Version**: 1.0
**Last Updated**: 2026-01-18
**Maintainer**: Ito Team
