---
name: byterover-review
description: "Review code changes against stored conventions, patterns, and architecture decisions. Checks staged changes or specific files for convention violations, pattern mismatches, missing tests, and security concerns. Curates newly discovered patterns."
---

# ByteRover Code Review

A structured workflow for reviewing code changes against the project's documented conventions, patterns, and architecture decisions stored in ByteRover's knowledge base.

## When to Use

- Before committing changes
- During pull request reviews
- After receiving code from other agents or developers
- When unsure if changes follow project conventions

## Prerequisites

Run `brv status` first. If errors occur, instruct the user to resolve them in the brv terminal. See the byterover skill's TROUBLESHOOTING.md for details.

The review needs something to review. Ask the user for one of:
- Staged changes (default: `git diff --cached`)
- Unstaged changes (`git diff`)
- Specific file paths to review

## Process

### Phase 1: Load Project Context

Query the knowledge base for all relevant standards:

```bash
brv query "What coding conventions and naming patterns are used in this project?"
brv query "What architecture patterns and layer boundaries are documented?"
brv query "What testing patterns and requirements are documented?"
brv query "What error handling and security patterns are documented?"
```

If ByteRover returns little or no knowledge, note this and recommend running `byterover-explore` first. Proceed with best-effort review based on what's available.

### Phase 2: Identify Changes

Determine what to review:

```bash
# For staged changes (most common)
git diff --cached --name-only
git diff --cached

# For unstaged changes
git diff --name-only
git diff

# For specific files - read them directly
```

Categorize changed files by type:
- **Source code** — primary review focus
- **Test files** — check test quality and coverage
- **Config files** — check for unintended changes
- **Documentation** — check accuracy

### Phase 3: Convention Compliance

Compare each changed file against documented conventions:

**Naming conventions:**
- Variable/function names follow documented patterns (camelCase, snake_case, etc.)
- File naming matches project conventions
- Export naming is consistent

**Import patterns:**
- Import ordering matches documented rules
- No circular dependencies introduced
- Proper use of type imports where applicable

**Code style:**
- Error handling follows documented patterns
- Logging follows documented conventions
- Comments are meaningful (not redundant)

For each violation, note:
- File and line number
- The convention being violated
- The documented standard (from knowledge base)
- Severity: **blocking** (must fix) or **suggestion** (nice to have)

### Phase 4: Architecture and Pattern Check

Verify changes respect documented architecture:

**Layer boundaries:**
- Do changes maintain proper layer separation?
- Are dependencies flowing in the documented direction?
- No business logic in presentation layer (or vice versa)

**Pattern compliance:**
- Do new components/modules follow existing patterns?
- Are established abstractions reused rather than reinvented?
- Does data flow match documented architecture?

**API contracts:**
- Do interface changes maintain backward compatibility?
- Are new APIs consistent with existing ones?

### Phase 5: Test and Security Check

**Test coverage:**
- Do changed source files have corresponding test files?
- Are new functions/components tested?
- Do test patterns match documented testing approach?

**Security concerns:**
- No hardcoded secrets, API keys, or credentials
- Input validation for user-facing endpoints
- No SQL injection, XSS, or command injection vectors
- Proper authentication/authorization checks
- No sensitive data logged or exposed

### Phase 6: Curate New Patterns

If the review reveals patterns not yet documented:

```bash
# New convention discovered
brv curate "Convention: [new pattern observed across multiple files]" -f [example files]

# New architectural decision
brv curate "Architecture decision: [what was decided and why]" -f [relevant files]

# New concern identified
brv curate "Concern: [potential issue found during review]" -f [affected files]
```

### Completion

Present a structured review:

**Summary:** X blocking issues, Y suggestions, Z new patterns found

**Blocking Issues (must fix):**
- [File:line] Issue description | Convention: [source]

**Suggestions (nice to have):**
- [File:line] Suggestion | Rationale

**New Patterns Curated:**
- [Pattern description] → stored via `brv curate`

**Missing Coverage:**
- Areas where knowledge base had no guidance (recommend `byterover-explore`)

## Important Rules

1. **Never read secrets** — Skip `.env`, credential files, and similar
2. **Knowledge-first** — Base reviews on documented standards, not personal preference
3. **Distinguish severity** — Clearly separate blocking issues from suggestions
4. **Be specific** — Reference file paths, line numbers, and the exact convention violated
5. **Curate discoveries** — Store genuinely new patterns, not noise
6. **Max 5 files per curate** — Break down large curate operations
7. **No false positives** — Only flag issues where a documented standard exists or a clear security concern is present
8. **Verify curations** — After storing critical context, run `brv curate view <logId>` to confirm what was stored (logId is printed by `brv curate` on completion). Run `brv curate view --help` to see all options.
