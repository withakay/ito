# Tasks for: 001-18_agent-instruction-peer-review

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential (waves 1→2→3→4)
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 001-18_agent-instruction-peer-review
ito tasks next 001-18_agent-instruction-peer-review
ito tasks start 001-18_agent-instruction-peer-review 1.1
ito tasks complete 001-18_agent-instruction-peer-review 1.1
```

______________________________________________________________________

## Wave 1 - Domain and Core Logic

- **Depends On**: None

### Task 1.1: Define PeerReviewContext struct in ito-core workflow

- **Files**: `ito-rs/crates/ito-core/src/workflow/mod.rs`
- **Dependencies**: None
- **Action**:
  Add a `PeerReviewContext` struct (serializable with serde) containing:
  - `change_name: String`
  - `change_dir: String`
  - `schema_name: String`
  - `module_id: Option<String>`
  - `module_name: Option<String>`
  - `artifacts: Vec<ArtifactInfo>` — each with `id`, `path`, `exists`
  - `validation_issues: Vec<ValidationIssueInfo>` — serializable projection of `ValidationIssue`
  - `validation_passed: bool`
  - `task_summary: Option<TaskSummaryInfo>` — total, by_status counts, wave_count
  - `affected_specs: Vec<AffectedSpecInfo>` — main spec ID and path for each delta with `operation: MODIFIED`
  - `user_guidance: Option<String>`
  - `testing_policy: TestingPolicy`
  - `generated_at: String` — ISO 8601 timestamp

  Define the supporting info structs (`ArtifactInfo`, `ValidationIssueInfo`, `TaskSummaryInfo`, `AffectedSpecInfo`). All must derive `Serialize`.
- **Verify**: `cargo check -p ito-core`
- **Done When**: `PeerReviewContext` and supporting types compile with Serialize
- **Updated At**: 2026-02-07
- **Status**: [ ] pending

### Task 1.2: Implement compute_review_context() function

- **Files**: `ito-rs/crates/ito-core/src/workflow/mod.rs`
- **Dependencies**: Task 1.1
- **Action**:
  Add `pub fn compute_review_context(rt: &ItoRuntime, change_name: &str) -> Result<PeerReviewContext>` that:
  1. Resolves the change directory and schema name
  2. Resolves the module (if the change is under one)
  3. Iterates the schema's artifact list, checks each generated path for existence
  4. Runs `validate_change()` and collects issues into serializable form
  5. If `tasks.md` exists, parses it with `parse_tasks_tracking_file()` and computes summary stats (total, complete, in_progress, pending, shelved, wave_count)
  6. Scans `specs/*/spec.md` delta files for `operation: MODIFIED` headers, collects affected main spec IDs
  7. Loads user guidance and testing policy
  8. Sets `generated_at` to current UTC ISO 8601

  Follow the same pattern as `compute_apply_instructions()` for error handling and runtime usage.
- **Verify**: `cargo test -p ito-core --lib -- workflow`
- **Done When**: Function compiles and returns a populated context for a test change directory
- **Updated At**: 2026-02-07
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2 - Template

- **Depends On**: Wave 1

### Task 2.1: Create review.md.j2 template

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/review.md.j2`
- **Dependencies**: None
- **Action**:
  Create the Jinja2 template with these sections:

  **Header**: Change name, module, schema, generation timestamp.

  **Artifact inventory**: Table of all artifacts with exists/missing status.

  **Structural validation results**: If validation was run, show pass/fail and any issues. If all pass, note "no structural issues found."

  **Review sections** (each conditional on artifact existence):

  1. **Proposal review** (if proposal.md exists):
     - Is the "Why" compelling and specific (not generic motivation)?
     - Is the scope appropriate? Not too broad, not too narrow?
     - Are capabilities correctly listed as NEW or MODIFIED?
     - Are breaking changes identified?
     - Is the impact assessment realistic?

  2. **Specs review** (if specs/ directory exists):
     - Does each requirement use SHALL/MUST language?
     - Does each requirement have at least one scenario?
     - Are scenarios testable and specific?
     - For MODIFIED specs: does the delta include the full updated requirement text?
     - Are error/edge cases covered?
     - Do specs contradict existing main specs?

  3. **Design review** (if design.md exists):
     - Are key decisions justified with rationale?
     - Are alternatives considered and rejected with reasons?
     - Are risks identified with mitigations?
     - Is the approach consistent with existing codebase patterns?

  4. **Tasks review** (if tasks.md exists):
     - Show task summary stats (total, by status, waves)
     - Are tasks properly scoped and small enough to verify?
     - Do wave dependencies make sense?
     - Do tasks cover all requirements from specs?
     - Are verify/done-when criteria specific?

  5. **Cross-cutting concerns**:
     - List affected main specs for reviewer to check for conflicts
     - Note any active changes in the same module (reviewer should check)

  **Output format instructions**: Tell the reviewer to produce findings tagged `[blocking]`, `[suggestion]`, or `[note]` per section, then a verdict: `approve`, `request-changes`, or `defer`.

  **Context files**: List all artifact file paths for the reviewer to read.

  Use `{% if %}` blocks for conditional sections. Use `{{ }}` for variable interpolation. Follow the style of `apply.md.j2`.
- **Verify**: Template syntax check via minijinja (covered by unit test in task 2.2)
- **Done When**: Template file exists with all sections, conditional rendering, and variable references matching PeerReviewContext fields
- **Updated At**: 2026-02-07
- **Status**: [ ] pending

### Task 2.2: Unit test for template rendering

- **Files**: `ito-rs/crates/ito-core/tests/workflow_review.rs` (new file)
- **Dependencies**: Task 2.1
- **Action**:
  Add a test that:
  1. Constructs a `PeerReviewContext` with all fields populated (all artifacts present)
  2. Renders `agent/review.md.j2` via `render_instruction_template()`
  3. Asserts the output contains: change name, all section headers, validation status, artifact table, affected specs list
  4. Constructs a minimal context (only proposal exists, no specs/design/tasks)
  5. Renders again and asserts conditional sections are absent

  Follow the pattern of existing `workflow_templates.rs` tests.
- **Verify**: `cargo test -p ito-core --test workflow_review`
- **Done When**: Both test cases pass — full context and minimal context render correctly
- **Updated At**: 2026-02-07
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3 - CLI Integration

- **Depends On**: Wave 2

### Task 3.1: Wire review handler into instructions.rs

- **Files**: `ito-rs/crates/ito-cli/src/app/instructions.rs`
- **Dependencies**: None
- **Action**:
  In `handle_agent_instruction()`, add a special-case block for `artifact == "review"`:
  1. Require `--change` flag (error if missing: "review instruction requires --change <id>")
  2. Call `compute_review_context(rt, &change_name)`
  3. Render `agent/review.md.j2` with the context
  4. Output as text (default) or JSON (if `--json` flag)

  Place this block before the generic `resolve_instructions()` fallthrough, alongside the existing `apply` handler. Follow the same error handling pattern.
- **Verify**: `cargo build -p ito-cli && ito agent instruction review --change 001-18_agent-instruction-peer-review`
- **Done When**: `ito agent instruction review --change <id>` produces rendered review instructions instead of ArtifactNotFound error
- **Updated At**: 2026-02-07
- **Status**: [ ] pending

### Task 3.2: Integration test for review instruction

- **Files**: `ito-rs/crates/ito-cli/tests/instruction_review.rs` (new file)
- **Dependencies**: Task 3.1
- **Action**:
  Add an integration test that:
  1. Sets up a temp directory with a minimal .ito project structure (module, change, proposal.md)
  2. Runs `ito agent instruction review --change <test-change>`
  3. Asserts exit code 0
  4. Asserts output contains expected sections (proposal review checklist, output format)
  5. Tests error case: `ito agent instruction review` without `--change` returns error
- **Verify**: `cargo test -p ito-cli --test instruction_review`
- **Done When**: Both success and error test cases pass
- **Updated At**: 2026-02-07
- **Status**: [ ] pending

______________________________________________________________________

## Wave 4 - Skills and Templates

- **Depends On**: Wave 3

### Task 4.1: Update ito-review skill to document peer review usage

- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-review/SKILL.md`
- **Dependencies**: None
- **Action**:
  Update the skill to document that `ito agent instruction review --change <id>` now works and is the primary entry point for peer-reviewing proposals. Note:
  - When to use: after proposal/specs/design/tasks are written, before implementation
  - What it does: generates a structured review checklist with embedded validation results
  - How to use: run the command, follow the checklist, produce tagged findings
  - Output format: `[blocking]`/`[suggestion]`/`[note]` findings + verdict

  Ensure the skill continues to work with the existing `ito-review` OpenCode skill (which delegates to this).
- **Verify**: Read the updated skill file and verify it matches the actual command behavior
- **Done When**: Skill accurately documents the working review instruction
- **Updated At**: 2026-02-07
- **Status**: [ ] pending

### Task 4.2: Update bootstrap template to reflect working review instruction

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/bootstrap.md.j2`
- **Dependencies**: None
- **Action**:
  Review the bootstrap template's references to `ito agent instruction review`. Ensure they accurately describe the command now that it works. Add a brief note about when to use it in the workflow (post-proposal, pre-implementation).
- **Verify**: Read the template and verify consistency with the actual command
- **Done When**: Bootstrap template accurately describes the working review instruction
- **Updated At**: 2026-02-07
- **Status**: [ ] pending

______________________________________________________________________

## Wave 5 (Checkpoint)

- **Depends On**: Wave 4

### Task 5.1: End-to-end validation

- **Type**: checkpoint (requires human approval before proceeding)
- **Files**: All files from waves 1-4
- **Dependencies**: All prior tasks
- **Action**:
  1. Run `make check` — all lints and tests pass
  2. Run `make test` — full test suite passes
  3. Run `ito agent instruction review --change 001-18_agent-instruction-peer-review` — produces well-formatted review instructions with all sections
  4. Verify the ito-review skill works end-to-end
  5. Verify `ito agent instruction review` without `--change` gives a clear error
- **Done When**: Human confirms all checks pass and output quality is acceptable
- **Updated At**: 2026-02-07
- **Status**: [ ] pending

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[>] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)

## Wave Guidelines

- Waves group related tasks that can be executed in parallel
- Task dependencies must be complete before starting dependent tasks
- Wave dependencies are declared via `- **Depends On**: ...`
- Task dependencies MUST be within the same wave
- Checkpoint waves require human approval before proceeding
