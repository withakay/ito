## Implementation Tasks

### Phase 1: Update Source Code

- \[x\] Update `src/commands/validate.ts:179` - Replace hint with verb-first command
- \[x\] Update `src/commands/validate.ts:266` - Replace debug hint with verb-first command
- \[x\] Update `src/commands/show.ts:102` - Replace hint with verb-first command
- \[x\] Update `src/commands/show.ts:120` - Replace hint with verb-first command
- \[x\] Update `src/core/validation/constants.ts:61` - Update error message hint
- \[x\] Update `src/core/templates/agents-template.ts:88` - Update command reference in generated agent instructions

### Phase 2: Update Tests

- \[x\] Update `test/commands/show.test.ts:47` - Expect verb-first hint
- \[x\] Update `test/commands/change.interactive-show.test.ts:38` - Expect verb-first hint
- \[x\] Update `test/commands/validate.enriched-output.test.ts:43` - Expect verb-first hint
- \[x\] Update `test/commands/change.interactive-validate.test.ts:41` - Expect verb-first hint

### Phase 3: Update Documentation

- \[x\] Update `.ito/specs/cli-change/spec.md` - Replace all `ito change` references
- \[x\] Update `.ito/specs/cli-show/spec.md` - Replace all `ito change` references
- \[x\] Update `.ito/specs/cli-validate/spec.md` - Replace all `ito change` references
- \[x\] Update `.ito/specs/projector-conventions/spec.md` - Update command pattern description
- \[x\] Update `.ito/AGENTS.md:84` - Update enumeration command reference

### Phase 4: Validation and Testing

- \[x\] Run unit tests to verify test assertions pass
- \[x\] Run `ito validate` on a change to verify error messages
- \[x\] Run `ito show` on non-existent change to verify hints
- \[x\] Verify deprecation warnings still appear in `src/cli/index.ts`
- \[x\] Run full test suite: `make test`
