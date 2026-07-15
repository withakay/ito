# Materialized authority parity

## Copy and verification boundary

The five legacy link targets were copied to an isolated staging area, compared
with the frozen source inventory, and then materialized as real directories.
Commit `a650b52a` records the cutover before later spec convergence and mirror
retirement.

Comparing the retained source checkout with the `a650b52a` Git tree produces
exactly four intentional content differences:

- `.ito/changes/000-16_fix-opencode-agents-path/tasks.md` was normalized to the
  enhanced task format so strict validation can parse it.
- `.ito/specs/ito-managed-asset-naming/spec.md` gained its missing Purpose.
- `.ito/specs/ito-managed-asset-versioning/spec.md` gained its missing Purpose.
- `.ito/specs/lifecycle-skill-profile/spec.md` gained its missing Purpose.

The source also contains three empty directories that Git cannot represent:
the empty audit root, one empty historical demo directory, and one empty
historical module directory. The two historical empty directories remain an
accepted inventory normalization because they carry no content. The required
top-level audit authority contains a tracked, empty `.gitkeep` sentinel so a
fresh checkout materializes all five authority roots. That non-semantic
sentinel is the sole destination-only file excluded from source hash parity.
None of the five top-level destinations is a symlink.

After the byte-preserving cutover, reviewed changes intentionally advanced the
tracked authority: current specs were reconciled, accepted deltas from
`031-01` through `031-05` were promoted, and the published mirror contract was
retired. Those reviewed changes are not expected to remain byte-identical to
the rollback source.

## External preservation

The retained checkout remains on commit
`e2f023435cfa8f07fc96a3bbaf897528c235af25`. The checksum manifest continues
to verify against its working-tree snapshot. No sync, commit, reset, push,
delete, or cleanup command was run in that checkout.
