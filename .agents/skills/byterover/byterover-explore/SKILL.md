---
name: byterover-explore
description: "Systematically explore a codebase, documentation, and knowledge gaps. Maps technology stack, architecture, conventions, testing, integrations, and concerns into the ByteRover context tree via brv curate."
---

# ByteRover Explore

A structured workflow for systematically exploring a codebase and curating findings into ByteRover's context tree. Covers code structure, documentation, and knowledge gaps.

## When to Use

- Starting work on an unfamiliar codebase
- Onboarding to a new project
- Auditing knowledge coverage after major changes
- Building a comprehensive project knowledge base from scratch

## Prerequisites

Run `brv status` first. If errors occur, instruct the user to resolve them in the brv terminal. See the byterover skill's TROUBLESHOOTING.md for details.

## Process

### Phase 1: Check Existing Knowledge

Before exploring, query what ByteRover already knows to avoid duplicate work:

```bash
brv query "What is documented about the codebase architecture and structure?"
brv query "What conventions, patterns, and testing approaches are documented?"
brv query "What concerns, tech debt, or known issues are documented?"
```

If ByteRover already has comprehensive coverage, report what exists and ask the user which areas to refresh.

### Phase 2: Codebase Mapping

Explore each area systematically. For each area, read actual files, then curate findings.

#### 2.1 Technology Stack

Identify languages, runtime, frameworks, and key dependencies.

**What to examine:**
- Package manifests: `package.json`, `requirements.txt`, `Cargo.toml`, `go.mod`, `Gemfile`
- Config files: `tsconfig.json`, `.nvmrc`, `.python-version`, `Dockerfile`
- SDK/API imports in source files

```bash
brv curate "Technology stack: [languages, runtime, frameworks, key dependencies]" -f package.json
```

#### 2.2 Architecture & Structure

Map directory layout, module boundaries, entry points, layers, and data flow.

**What to examine:**
- Top-level directory structure (excluding `node_modules`, `.git`, `dist`)
- Entry points: `src/index.*`, `src/main.*`, `app/page.*`
- Import patterns to understand layer dependencies

```bash
brv curate "Architecture: [directory layout, layers, data flow, entry points, module boundaries]" -f [key structural files]
```

#### 2.3 Conventions & Patterns

Analyze coding style, naming conventions, import ordering, error handling patterns.

**What to examine:**
- Linting/formatting config: `.eslintrc*`, `.prettierrc*`, `biome.json`
- Sample source files for naming patterns (camelCase, snake_case, etc.)
- Import ordering and module resolution patterns

```bash
brv curate "Conventions: [naming, code style, import patterns, error handling]" -f [config files]
```

#### 2.4 Testing Patterns

Identify test framework, file organization, mocking approaches, fixtures.

**What to examine:**
- Test config: `jest.config.*`, `vitest.config.*`, `pytest.ini`, `.mocharc.*`
- Test file locations and naming conventions
- Sample test files for patterns (mocking, fixtures, assertions)

```bash
brv curate "Testing: [framework, organization, mocking patterns, fixtures]" -f [test config]
```

#### 2.5 Integrations

Document external APIs, databases, auth providers, webhooks, third-party services.

**What to examine:**
- Environment variables (`.env.example`, not `.env`)
- SDK imports (stripe, supabase, aws, firebase, etc.)
- Database config, ORM setup, migration files

```bash
brv curate "Integrations: [external APIs, databases, auth, third-party services]" -f [relevant files]
```

#### 2.6 Concerns & Technical Debt

Find TODOs, FIXMEs, large files, known issues, security risks, fragile areas.

**What to examine:**
- `TODO`, `FIXME`, `HACK`, `XXX` comments in source
- Large files (potential complexity hotspots)
- Empty stubs, incomplete implementations

```bash
brv curate "Concerns: [tech debt, TODOs, known issues, fragile areas, security risks]" -f [relevant files]
```

### Phase 3: Documentation & Knowledge Gaps

#### 3.1 Documentation Coverage

Assess existing documentation quality and coverage.

**What to examine:**
- `README.md` completeness (setup instructions, architecture overview, contribution guide)
- `docs/` directory existence and contents
- API documentation (OpenAPI specs, JSDoc, docstrings)
- Architectural decision records (ADRs)
- Inline code comments quality

```bash
brv curate "Documentation coverage: [what exists, quality assessment, what's missing]"
```

#### 3.2 Knowledge Gap Analysis

Query ByteRover for each major area and identify what's still missing.

```bash
brv query "What is documented about the technology stack?"
brv query "What is documented about the architecture?"
brv query "What is documented about conventions?"
brv query "What is documented about testing?"
brv query "What is documented about integrations?"
brv query "What is documented about concerns and tech debt?"
```

For any area with insufficient coverage, curate the missing knowledge:

```bash
brv curate "Knowledge gaps: [areas with missing or incomplete documentation]"
```

### Completion

After exploration, verify overall coverage:

```bash
brv query "Summarize all documented knowledge about this project"
```

Report to the user:
- What areas were explored and curated
- Key findings (notable patterns, concerns, interesting architecture decisions)
- Remaining gaps that need deeper investigation
- Suggested next steps

## Important Rules

1. **Never read secrets** — Skip `.env`, `.key`, `credentials.json`, and similar files
2. **File paths required** — Every finding must reference specific file paths
3. **Be prescriptive** — "Use camelCase for functions" not "some functions use camelCase"
4. **Break down large contexts** — Run multiple `brv curate` commands rather than one massive one
5. **Let ByteRover read files** — Use `-f` flags to let ByteRover read files directly (max 5 per command)
6. **Be specific in queries** — Use precise questions for faster, more relevant results
7. **Verify curations** — After storing critical context, run `brv curate view <logId>` to confirm what was stored (logId is printed by `brv curate` on completion). Run `brv curate view --help` to see all options.
