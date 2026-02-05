## Why

The `ito change` noun-based commands were deprecated in favor of verb-first commands (`ito list`, `ito show`, `ito validate`) in change `2025-08-19-adopt-verb-noun-cli-structure`. However, many references to these deprecated commands still exist throughout the codebase in error messages, hints, test assertions, and documentation. This creates confusion for users and contributors who encounter outdated command references.

## What Changes

### Code Updates

**Source files** - Update error messages and hints:

- `src/commands/validate.ts:179` - Replace hint with verb-first command
- `src/commands/validate.ts:266` - Replace debug hint with verb-first command
- `src/commands/show.ts:102` - Replace hint with verb-first command
- `src/commands/show.ts:120` - Replace hint with verb-first command

**Template files**:

- `src/core/templates/agents-template.ts:88` - Update command reference in generated agent instructions

### Test Updates

Update test assertions to check for verb-first command references instead of deprecated ones:

- `test/commands/show.test.ts:47` - Expect verb-first hint
- `test/commands/change.interactive-show.test.ts:38` - Expect verb-first hint
- `test/commands/validate.enriched-output.test.ts:43` - Expect verb-first hint
- `test/commands/change.interactive-validate.test.ts:41` - Expect verb-first hint

### Documentation Updates

**Core documentation**:

- `.ito/AGENTS.md:84` - Update enumeration command reference

**Validation constants**:

- `src/core/validation/constants.ts:61` - Update error message hint

**Archived changes**:

- `.ito/changes/archive/2025-08-19-add-change-commands/` specs - These reference the deprecated commands in historical context, should be left as-is
- `.ito/changes/archive/2025-08-19-add-interactive-show-command/` specs - Same
- `.ito/changes/archive/2025-08-19-bulk-validation-interactive-selection/` specs - Same
- `.ito/changes/archive/2025-08-19-improve-validate-error-messages/` specs - Same
- `.ito/changes/archive/2025-10-14-enhance-validation-error-messages/` specs - Same

**Other documentation**:

- `.ito/specs/cli-change/spec.md` - Update all `ito change` references
- `.ito/specs/cli-show/spec.md` - Update all `ito change` references
- `.ito/specs/cli-validate/spec.md` - Update all `ito change` references
- `.ito/specs/projector-conventions/spec.md` - Update command pattern description

### Deprecation Warnings

Keep deprecation warnings in `src/cli/index.ts:173,200` as these are part of the implementation that shows warnings to users of deprecated commands. These should remain until the deprecated commands are fully removed.

## Capabilities

### Modified Capabilities

- `cli-change`: Update spec to use verb-first command examples
- `cli-show`: Update spec to use verb-first command examples
- `cli-validate`: Update spec to use verb-first command examples
- `projector-conventions`: Update to document verb-first CLI structure
- `agent-workflow-docs`: Update command references in agent instructions

## Impact

**Affected code**:

- `src/commands/validate.ts` - Update error message hints
- `src/commands/show.ts` - Update error message hints
- `src/core/validation/constants.ts` - Update validation error hints
- `src/core/templates/agents-template.ts` - Update template for generated instructions

**Affected tests**:

- `test/commands/show.test.ts`
- `test/commands/change.interactive-show.test.ts`
- `test/commands/validate.enriched-output.test.ts`
- `test/commands/change.interactive-validate.test.ts`

**Affected documentation**:

- `.ito/AGENTS.md`
- `.ito/specs/cli-change/spec.md`
- `.ito/specs/cli-show/spec.md`
- `.ito/specs/cli-validate/spec.md`
- `.ito/specs/projector-conventions/spec.md`

**No breaking changes**: This only updates text in error messages, hints, tests, and documentation. The deprecated commands still function and show warnings.

## Note

When archiving this change, use `--skip-specs` since this modifies existing spec files rather than adding new capabilities.
