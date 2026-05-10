# Tasks for: 019-02_internal-guidance-comments

## Execution Notes

- **Mode**: Retrospective capture after implementation
- **Tracking**: Tasks reflect completed implementation work

## Wave 1 - Guidance loading behavior

### Task 1.1: Strip Ito internal comment blocks during guidance load

- **Files**: `ito-rs/crates/ito-core/src/templates/mod.rs`
- **Status**: [x] complete
- **Done When**: Guidance loading ignores content between `<!-- ITO:INTERNAL:START -->` and `<!-- ITO:INTERNAL:END -->`

### Task 1.2: Add regression coverage for internal block stripping

- **Files**: `ito-rs/crates/ito-core/tests/templates_user_guidance.rs`
- **Status**: [x] complete
- **Done When**: Test verifies internal scaffold content is excluded from resolved guidance

## Wave 2 - Template scaffold updates

### Task 2.1: Wrap default guidance scaffold text in Ito internal comments

- **Files**:
  - `ito-rs/crates/ito-templates/assets/default/project/.ito/user-guidance.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.ito/user-prompts/guidance.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.ito/user-prompts/proposal.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.ito/user-prompts/apply.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.ito/user-prompts/tasks.md`
- **Status**: [x] complete
- **Done When**: Placeholder sections remain visible in files but are marked as Ito internal comments

### Task 2.2: Verify template stubs include internal comment markers

- **Files**: `ito-rs/crates/ito-templates/tests/user_guidance_template.rs`
- **Status**: [x] complete
- **Done When**: Template tests assert internal marker presence in all guidance stub templates

## Verification

- [x] `cargo test -p ito-core --test templates_user_guidance`
- [x] `cargo test -p ito-templates --test user_guidance_template`
