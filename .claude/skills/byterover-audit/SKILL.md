---
name: byterover-audit
description: "Audit knowledge freshness and coverage. Checks what's documented against the current codebase, identifies stale or outdated knowledge, finds gaps, and provides targeted brv curate commands to fix them."
---

# ByteRover Knowledge Audit

A structured workflow for auditing the health of your ByteRover knowledge base. Finds stale entries, missing coverage, and provides actionable remediation commands.

## When to Use

- After major refactors or dependency upgrades
- Periodically (monthly or per sprint) to maintain knowledge quality
- When suspecting knowledge drift (documented patterns no longer match code)
- Before starting a new feature in an area with existing documentation

## Prerequisites

Run `brv status` first. If errors occur, instruct the user to resolve them in the brv terminal. See the byterover skill's TROUBLESHOOTING.md for details.

## Process

### Phase 1: Inventory Existing Knowledge

Query every major knowledge domain to understand what ByteRover currently knows:

```bash
brv query "What is documented about the technology stack and dependencies?"
brv query "What is documented about the architecture and project structure?"
brv query "What conventions and coding patterns are documented?"
brv query "What testing approaches and frameworks are documented?"
brv query "What integrations and external services are documented?"
brv query "What concerns, tech debt, and known issues are documented?"
```

For each query, record:
- Whether ByteRover returned substantive knowledge or nothing
- The specificity of the response (file paths referenced? concrete patterns?)
- The approximate age indicators (does it mention current dependencies?)

If ByteRover returns nothing for most domains, stop and recommend running `byterover-explore` first — auditing requires an existing knowledge base.

### Phase 2: Cross-Reference Against Codebase

For each piece of documented knowledge, verify it matches the current codebase:

**Technology Stack:**
- Read `package.json`, `requirements.txt`, or equivalent — do documented dependencies still exist? Are versions accurate?
- Check for new major dependencies not yet documented

**Architecture:**
- Verify documented directory structure matches reality
- Check if documented entry points still exist
- Confirm documented layer boundaries are still respected

**Conventions:**
- Read linting/formatting configs — do documented conventions match current rules?
- Sample 2-3 source files — do naming patterns match what's documented?

**Testing:**
- Verify test framework config matches documentation
- Check if test file patterns match documented conventions

**Integrations:**
- Read `.env.example` — are documented integrations still present? Any new ones?
- Check for new SDK imports not yet documented

### Phase 3: Staleness Detection

Flag knowledge entries as stale when:
- Referenced files no longer exist (deleted or renamed)
- Referenced functions/classes have been renamed or removed
- Documented patterns contradict current linting rules
- Documented dependencies have been replaced (e.g., "uses moment.js" but codebase has date-fns)
- Documented architecture no longer matches directory structure

For each stale entry, note:
- What the knowledge says
- What the codebase actually shows
- Severity: **critical** (misleading), **moderate** (outdated but not harmful), **minor** (cosmetic)

### Phase 4: Gap Analysis

Identify areas with zero or insufficient coverage:

- Scan top-level directories — are all major modules documented?
- Check for new files/directories not mentioned in any knowledge entry
- Look for undocumented environment variables, API endpoints, or configuration
- Check if error handling patterns are documented
- Verify deployment/build process is documented

### Phase 5: Coverage Report and Remediation

Present a structured report:

**Coverage Summary:**
| Domain | Status | Issues |
|--------|--------|--------|
| Technology Stack | Current / Stale / Missing | Count of issues |
| Architecture | Current / Stale / Missing | Count of issues |
| Conventions | Current / Stale / Missing | Count of issues |
| Testing | Current / Stale / Missing | Count of issues |
| Integrations | Current / Stale / Missing | Count of issues |
| Concerns | Current / Stale / Missing | Count of issues |

**Stale Entries (fix first — wrong knowledge is worse than missing):**

For each stale entry, provide the exact `brv curate` command to fix it:

```bash
brv curate "OUTDATED: [old knowledge]. NEW: [current state]. Clean up old context." -f [relevant files]
```

**Gap Entries (fill after fixing stale):**

For each gap, provide the exact `brv curate` command to fill it:

```bash
brv curate "[domain]: [what needs documenting]" -f [relevant files]
```

### Completion

After presenting the report:
- Prioritize: fix stale entries first, then fill gaps
- Offer to execute the remediation commands
- Query the knowledge base again to verify fixes took effect

## Important Rules

1. **Never read secrets** — Skip `.env`, `.key`, `credentials.json`, and similar files
2. **Wrong knowledge > missing knowledge** — Prioritize fixing stale entries over filling gaps
3. **Provide actionable commands** — Every finding must include a concrete `brv curate` command
4. **Reference specific file paths** — Every claim must reference the actual file checked
5. **Be honest about coverage** — Don't inflate coverage ratings; if knowledge is vague, rate it as partial
6. **Max 5 files per curate** — Break down large updates into multiple `brv curate` commands
7. **Verify curations** — After storing critical context, run `brv curate view <logId>` to confirm what was stored (logId is printed by `brv curate` on completion). Run `brv curate view --help` to see all options.
