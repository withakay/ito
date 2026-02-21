# Tasks for: <!-- CHANGE_ID -->

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status <!-- CHANGE_ID -->
ito tasks next <!-- CHANGE_ID -->
ito tasks start <!-- CHANGE_ID --> 1.1
ito tasks complete <!-- CHANGE_ID --> 1.1
```

______________________________________________________________________

## Wave 1

### Task 1.1: <!-- Task Name -->

- **Files**: <!-- file paths -->
- **Dependencies**: None
- **Action**: <!-- what to implement -->
- **Verify**: <!-- command to verify -->
- **Done When**: <!-- acceptance criteria -->
- **Status**: [ ] pending

### Task 1.2: <!-- Task Name -->

- **Files**: <!-- file paths -->
- **Dependencies**: Task 1.1
- **Action**: <!-- what to implement -->
- **Verify**: <!-- command to verify -->
- **Done When**: <!-- acceptance criteria -->
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: <!-- Task Name -->

- **Files**: <!-- file paths -->
- **Dependencies**: Task 1.1, Task 1.2
- **Action**: <!-- what to implement -->
- **Verify**: <!-- command to verify -->
- **Done When**: <!-- acceptance criteria -->
- **Status**: [ ] pending

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave
- Wave N depends on all prior waves completing
- Task dependencies within a wave are fine; cross-wave deps use the wave dependency
- Checkpoint waves require human approval before proceeding
