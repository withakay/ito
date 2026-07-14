# Change: Hide internal guidance placeholders from rendered instructions

## Why

Default user-prompt template files currently include visible placeholder sections (for example, `## Your Apply Guidance` and scaffold text). Because instruction generation injects the post-managed content directly, these placeholders leak into agent instruction output and add noise. We need an Ito-specific comment mechanism so template scaffolding remains editable in files but is excluded from rendered guidance.

## What Changes

- Add support for Ito internal comment blocks in guidance loading using `<!-- ITO:INTERNAL:START -->` and `<!-- ITO:INTERNAL:END -->`
- Strip content inside those internal comment blocks from shared and artifact-scoped guidance before instruction composition
- Update project template stubs in `.ito/user-guidance.md` and `.ito/user-prompts/*.md` to wrap scaffold sections in Ito internal comment blocks
- Add tests for stripping behavior and for template marker presence

## Capabilities

### Modified Capabilities

- `instruction-guidance-injection`: guidance loading now strips Ito internal comment blocks before rendering instructions

## Impact

- **Code**: `ito-rs/crates/ito-core/src/templates/mod.rs` guidance loader logic
- **Templates**: `ito-rs/crates/ito-templates/assets/default/project/.ito/user-guidance.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/user-prompts/guidance.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/user-prompts/proposal.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/user-prompts/apply.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/user-prompts/tasks.md`
- **Tests**: `ito-rs/crates/ito-core/tests/templates_user_guidance.rs`, `ito-rs/crates/ito-templates/tests/user_guidance_template.rs`
