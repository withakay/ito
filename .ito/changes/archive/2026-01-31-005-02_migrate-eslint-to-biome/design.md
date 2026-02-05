## Context

Ito currently uses ESLint (flat config) with `typescript-eslint` to lint `src/`. The configuration includes a critical guardrail that restricts importing `@inquirer/*` in most of the codebase to avoid non-interactive execution hangs (e.g., git hooks / piped stdin / CI). The repository already standardizes on Bun for running scripts and CI.

This change migrates linting and formatting responsibilities from ESLint to Biome while keeping the same entrypoints (`bun run lint` and CI lint step) and preserving the restricted-import behavior.

## Goals / Non-Goals

**Goals:**

- Replace ESLint with Biome for linting and formatting.
- Preserve the `@inquirer/*` restricted-import guardrail with an actionable message.
- Keep the developer/CI interface stable (`bun run lint` still exists and fails on violations).
- Remove ESLint configuration and dependencies cleanly.

**Non-Goals:**

- Changing TypeScript type-checking (`tsc --noEmit`) behavior.
- Changing test runner behavior.
- Broad code-style rewrites beyond what Biome’s formatter/linter requires.

## Decisions

- **Use Biome’s built-in rule for restricted imports.**

  - Choice: Configure `linter.rules.style.noRestrictedImports` in `biome.json`.
  - Rationale: This maps directly to ESLint’s `no-restricted-imports` (including message support and pattern groups) and avoids keeping ESLint for a single rule.

- **Restrict `@inquirer/*` everywhere except `src/core/init.ts`.**

  - Choice: Enable the restriction globally (for `src/**`) and add a Biome override that disables the rule for `src/core/init.ts`.
  - Rationale: Mirrors the existing ESLint exception, while keeping the safety net in place for the rest of the project.

- **Use `biome check` as the primary lint entrypoint.**

  - Choice: Implement `bun run lint` as `biome check` scoped to the source tree.
  - Rationale: `check` is Biome’s integrated command that covers lint + formatting diagnostics, which aligns with the expectation that `lint` fails on style violations.

- **Add explicit formatting commands.**

  - Choice: Add `bun run format` (write) and a check variant used by CI.
  - Rationale: Makes formatting behavior explicit and easy to run locally, and supports a non-mutating CI check.

## Risks / Trade-offs

- **Rule parity drift** → Mitigation: Keep the first iteration conservative (only migrate the currently enforced rules) and adjust Biome rules incrementally.
- **Biome check may surface new formatting diffs** → Mitigation: Introduce `format` as a dedicated command and run formatting once as part of the migration.
- **Edge-case terminal behavior with Inquirer under Bun** → Mitigation: Preserve the restricted-import guardrail and keep Inquirer imports isolated to interactive code paths.

## Migration Plan

1. Add `@biomejs/biome` and create `biome.json` with baseline settings.
1. Replace `lint` script to run Biome; add `format` and `format:check` scripts.
1. Configure `style/noRestrictedImports` to restrict `@inquirer/*` (with a helpful message) and add an override for `src/core/init.ts`.
1. Remove ESLint configuration (`eslint.config.js`) and uninstall ESLint dependencies.
1. Update docs/CI references if they mention ESLint directly (CI should keep calling `bun run lint`).
1. Validate locally and in CI: `bun run lint`, `bun run format:check`, `bunx tsc --noEmit`.

Rollback strategy: revert `package.json` scripts and restore ESLint dependencies/config.

## Open Questions

- Should `bun run lint` enforce formatting (via `biome check`) or remain lint-only and rely on `format:check`? (The default in this design is to use `biome check` for `lint`.)
- Should formatting be applied repo-wide (including docs/config JSON) or scoped to `src/` initially?
