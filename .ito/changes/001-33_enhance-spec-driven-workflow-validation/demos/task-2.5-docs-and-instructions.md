# Task 2.5: Docs and Instruction Updates

*2026-04-25T22:25:44Z by Showboat 0.6.1*
<!-- showboat-id: d21cf723-890c-4056-8a94-0a9451eaaa33 -->

Documented the validation rules extension, added artifact-specific guidance for the new optional sections, and refreshed the proposal prompt so rule opt-in stays explicit.

```bash
make docs
```

```output
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
   Generated /Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/target/doc/ito_backend/index.html and 9 other files
rm -rf docs/rustdoc
cp -R target/doc docs/rustdoc
```

```bash
rg -n 'Validation Rules Extension|Change Shape|Contract Refs|task-quality|validation.yaml' docs/schema-customization.md ito-rs/crates/ito-templates/assets/instructions/agent/{artifact,new-proposal}.md.j2 .ito/user-prompts/proposal.md
```

```output
ito-rs/crates/ito-templates/assets/instructions/agent/new-proposal.md.j2:173:When the selected schema is `spec-driven`, use the optional proposal `## Change Shape` block only when it adds signal (for example: public API work, higher risk, stateful behavior, or uncertainty about whether a design doc is needed). If you want proposal/spec/task validation beyond the built-in defaults, export the schema and opt into rules through `.ito/templates/schemas/<name>/validation.yaml`.
docs/schema-customization.md:27:- `validation.yaml` when the built-in schema ships validator configuration
docs/schema-customization.md:36:    validation.yaml
docs/schema-customization.md:68:3. If you want opt-in validation rules, edit `.ito/templates/schemas/<name>/validation.yaml`.
docs/schema-customization.md:72:## Validation Rules Extension
docs/schema-customization.md:103:- `contract_refs`: validate requirement-level `Contract Refs` syntax and related proposal anchors
docs/schema-customization.md:107:Built-in `spec-driven` defaults stay quiet in v1: the shipped schema exports the rule machinery, but it does not enable any of these new rules until you opt in through a project-local `validation.yaml` override.
ito-rs/crates/ito-templates/assets/instructions/agent/artifact.md.j2:46:- Use the optional `## Change Shape` block only when it helps clarify type, risk, statefulness, public contract surface, or whether a design doc is warranted.
ito-rs/crates/ito-templates/assets/instructions/agent/artifact.md.j2:47:- `Public Contract` should name exposed contract families only when a requirement will later anchor them with `Contract Refs`.
ito-rs/crates/ito-templates/assets/instructions/agent/artifact.md.j2:48:- Change Shape is advisory. New validation rules stay opt-in through project-local `validation.yaml` overrides after schema export.
ito-rs/crates/ito-templates/assets/instructions/agent/artifact.md.j2:54:- Add `Contract Refs` when the requirement depends on an external interface instead of copying large contract snippets inline.
ito-rs/crates/ito-templates/assets/instructions/agent/artifact.md.j2:66:- Keep `Files`, `Action`, `Verify`, `Done When`, `Requirements`, `Status`, and `Updated At` concrete so task-quality validation can reason about the work.
.ito/user-prompts/proposal.md:33:### Change Shape
.ito/user-prompts/proposal.md:35:- Use the optional `## Change Shape` block when risk, statefulness, public contracts, or design intent would help reviewers understand the proposal faster.
.ito/user-prompts/proposal.md:36:- Keep it advisory and lightweight; do not invent a Change Shape section when it adds no signal.
.ito/user-prompts/proposal.md:41:- If this repo wants proposal/spec/task rule checks such as `capabilities_consistency`, `scenario_grammar`, `contract_refs`, or `task_quality`, export the schema and enable them in `.ito/templates/schemas/<name>/validation.yaml`.
```
