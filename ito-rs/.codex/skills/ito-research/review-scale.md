# Scale Review

## Objective

Identify performance bottlenecks and scaling issues in: **{{change_id}}**

## Perspective

What breaks at 10x, 100x, 1000x scale? Think about:

- Request volume
- Data volume
- User concurrency
- Geographic distribution

## Process

1. Review data access patterns
1. Identify N+1 query problems
1. Check for missing indexes
1. Find memory-intensive operations
1. Look for blocking calls in hot paths
1. Evaluate caching opportunities
1. Consider horizontal scaling implications

## Output Format

# Scale Review: {{change_id}}

## Current Design Analysis

Brief summary of the proposed architecture from a scaling perspective.

## Findings

### \[HIGH/MEDIUM/LOW\]: Finding Title

- **Component**: What's affected
- **Current Behavior**: What happens now
- **At Scale**: What breaks and when
- **Impact**: Performance/cost/reliability effect
- **Mitigation**: Optimization strategy
- **Status**: [ ] Not addressed

## Scaling Recommendations

- Caching strategy
- Database optimization
- Async processing opportunities
- CDN/edge considerations

## Load Estimates

| Scenario | Requests/sec | Data Size | Expected Latency |
|----------|-------------|-----------|------------------|
| Current | ... | ... | ... |
| 10x | ... | ... | ... |
| 100x | ... | ... | ... |

## Verdict

- [ ] Scales adequately for expected load
- [ ] Needs optimization before launch
- [ ] Requires architectural changes
