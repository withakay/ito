# biome-linting Specification

## Purpose

Replace ESLint with Biome for TypeScript/JavaScript linting while preserving Ito's most important guardrails (notably restricted imports for `@inquirer/*`).

## ADDED Requirements

### Requirement: Linting uses Biome

The project SHALL implement `bun run lint` using Biome and treat any Biome lint violations as failures.

#### Scenario: Developer runs lint

- **WHEN** a developer runs `bun run lint`
- **THEN** the project SHALL lint the codebase using Biome
- **AND** the command SHALL exit non-zero if Biome reports lint violations

### Requirement: Restrict problematic Inquirer imports

The project SHALL prevent static imports from `@inquirer/*` across `src/` (except `src/core/init.ts`) and SHALL surface a clear diagnostic explaining the lazy-import requirement.

#### Scenario: Restricted import is introduced outside the allowed file

- **WHEN** a developer adds an import matching `@inquirer/*` in any file under `src/` except `src/core/init.ts`
- **THEN** `bun run lint` SHALL fail
- **AND** the diagnostic SHALL explain that `@inquirer/*` must be imported lazily (dynamic import) to avoid non-interactive hook hangs

#### Scenario: Allowed file may import Inquirer

- **WHEN** `src/core/init.ts` imports from `@inquirer/*`
- **THEN** `bun run lint` SHALL NOT fail due to the restricted-import rule
