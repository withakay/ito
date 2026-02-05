## Context

Ito now centralizes “core” workflow instructions behind `ito agent instruction <artifact>` and ships thin per-harness wrappers. This makes it possible to inject additional user guidance at the instruction-generation layer rather than duplicating guidance across harness prompt files.

We also want the guidance to be user-owned and safe: created automatically, but never overwritten once the user edits it.

## Goals / Non-Goals

**Goals:**

- Provide a single, project-local place for users to write additional LLM guidance.
- Ensure `ito init` creates the file if missing.
- Ensure `ito update` does not overwrite user edits.
- Ensure `ito agent instruction <artifact>` includes the guidance content when present.
- Keep the feature harness-agnostic (Claude/Codex/OpenCode/Copilot) by operating at the CLI layer.

**Non-Goals:**

- Building a full “policy language” or structured config DSL for guidance.
- Per-user (home directory) guidance layering in this first iteration.
- Harness-specific argument interpolation semantics (e.g. `$ARGUMENTS`, `${input:...}`); this feature is about instruction generation, not prompt engines.

## Decisions

- **Guidance file path**: Add `.ito/user-guidance.md` as the canonical user-editable file.
- **Preservation strategy**: Ship the file with a `<!-- ITO:START --> ... <!-- ITO:END -->` managed header. Users add guidance beneath the managed block. Installers update only the managed block, leaving user content untouched.
- **Injection strategy**: When generating instruction artifacts (proposal/spec/design/tasks/research/review/archive/apply), append a section:
  - `## User Guidance` followed by the contents of `.ito/user-guidance.md`.
  - If the file does not exist or is empty, omit the section.

## Risks / Trade-offs

- **Prompt bloat**: Guidance may be large. We should consider a soft size limit (warn or truncate) to avoid excessively long instruction outputs.
- **Ambiguous guidance**: User guidance may conflict with schema instructions. We should define precedence: schema requirements remain authoritative; user guidance is additive.
