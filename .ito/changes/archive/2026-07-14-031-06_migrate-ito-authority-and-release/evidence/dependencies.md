# Core-reset dependency evidence

The cutover branch was created from main commit
`813a8d0ac50d1c7b1ee5f592933f59037de60693` and integrates the five reviewed
implementation branches:

| Change | Implementation commit | Integration commit |
| --- | --- | --- |
| `031-01_migrate-coordination-state-to-main` | `17a28227e1442d8eabe29bc3e7654d13cabbca6a` | `59d7f8e9` |
| `031-02_enforce-main-first-implementation` | `4f039291c5bdb63f55a6306f6e91994bda605b02` | `58408923` |
| `031-03_gate-experimental-backend-coordination` | `6ba48b4c5b5b65bd6ab295642e35731db6089c34` | `6093a276` |
| `031-04_remove-tmux-integration` | `21604d9ad9e9425d860c87582b7f62a1848add6a` | `52232932` |
| `031-05_consolidate-seven-lifecycle-skills` | `79f6b840e28ded9c265abac1e3e7c0901b443a4d` | `9e3467d0` |

The integration branch is the reviewed main-bound unit; none of these commits
was pushed, tagged, released, or merged into the protected main checkout by
this implementation run.

Verification recorded before cutover:

- All five Ito change packages passed strict validation.
- `031-03` passed the default, backend-only, coordination-only, and all-feature
  check/test/Clippy matrix.
- `031-03` passed `make check` and `make check-experimental` with the unchanged
  80 percent line and region coverage floors.
- `031-04` removed the tmux runtime, config, templates, scripts, and generated
  distribution surfaces.
- `031-05` reduced the canonical lifecycle profile to seven skills and kept
  iteration under `ito-loop`.
