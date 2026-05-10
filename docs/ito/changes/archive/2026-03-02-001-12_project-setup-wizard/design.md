## Context

Ito currently provides scaffolding via `ito init`, and workflow content via `ito agent instruction <artifact>`. The missing piece is a first-run, repo-specific setup step that produces a consistent set of dev commands (build/test/lint) and captures tooling preferences (runtime, package manager, version manager). This step should be agent-driven (interactive) without making `ito init` itself interactive.

## Goals / Non-Goals

**Goals:**

- Provide a first-class “project setup” workflow that can be run in any supported harness.
- Keep `ito init` non-interactive but able to nudge users into the setup flow when needed.
- Generate a reasonable `Makefile` (or Windows alternative) with common targets mapped to the project stack.
- Make stack detection best-effort and safe (no destructive edits without explicit user confirmation in the agent workflow).

**Non-Goals:**

- Perfectly detect every language/toolchain.
- Enforce a single task runner across all platforms.
- Overwrite an existing `Makefile` by default.

## Decisions

- Add a new `ito agent instruction project-setup` artifact.
  - Rationale: instruction artifacts are the right place for interactive workflows in agent harnesses.
- Use `.ito/project.md` as the “setup completeness” signal.
  - Proposed marker (template-installed and machine-checkable):
    - `<!-- ITO:PROJECT_SETUP:INCOMPLETE -->`
    - `<!-- ITO:PROJECT_SETUP:COMPLETE -->`
  - Rationale: aligns with request (“check `.ito/project.md`”) and avoids introducing new files.
- Generate dev command scaffolding via the agent workflow.
  - Rationale: the agent can ask questions and tailor outputs; the CLI stays deterministic.
- Windows alternative: generate `scripts/dev.ps1` (or similar) that provides `build/test/lint/help` entrypoints.
  - Rationale: `make` is not universally present on Windows; PowerShell is.

## Risks / Trade-offs

- Overly chatty setup prompts -> Mitigation: a small “core interview” plus optional advanced questions.
- Conflicting task-runner opinions -> Mitigation: prefer additive outputs and avoid overwriting existing files.
- Marker drift in `.ito/project.md` -> Mitigation: treat marker as advisory; init only hints.

## Migration Plan

- New templates and artifact are additive.
- Existing projects can opt-in by running `/ito-project-setup`.
- Projects that already have Makefiles are left untouched unless explicitly requested.

## Open Questions

- Exact scope of stack detection in v1 (Rust/Node/Python/Go?)
- Whether to store chosen preferences in `.ito/config.json` in addition to updating `.ito/project.md`.
