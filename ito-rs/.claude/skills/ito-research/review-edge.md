# Edge Case Review

## Objective

Find edge cases and unexpected behaviors in: **{{change_id}}**

## Perspective

Think like a chaos monkey. What happens when:

- Inputs are at boundaries (empty, null, huge, unicode)
- Operations fail partway through
- Timing is unexpected (slow, fast, concurrent)
- Users do unexpected things

## Process

1. Map all inputs and their valid ranges
1. Test boundary conditions
1. Consider partial failures
1. Think about concurrency
1. Check error handling paths

## Output Format

# Edge Case Review: {{change_id}}

## Input Boundaries

| Input | Valid Range | Edge Cases to Test |
|-------|-------------|-------------------|
| ... | ... | empty, max, special chars |

## Findings

### \[HIGH/MEDIUM/LOW\]: Edge Case Title

- **Trigger**: How to reproduce
- **Current Behavior**: What happens
- **Expected Behavior**: What should happen
- **Impact**: User experience / data integrity effect
- **Fix**: How to handle properly
- **Status**: [ ] Not addressed

## Concurrency Scenarios

- Race condition 1: Description and mitigation
- Race condition 2: Description and mitigation

## Failure Modes

| Operation | Failure Mode | Current Handling | Recommended |
|-----------|-------------|------------------|-------------|
| ... | ... | ... | ... |

## Verdict

- [ ] Edge cases adequately handled
- [ ] Minor edge case improvements needed
- [ ] Significant gaps in error handling
