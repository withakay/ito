## 1. Asset Consolidation

- [x] 1.1 Create new assets structure in ito-templates (assets/skills/, assets/adapters/, assets/commands/)
- [x] 1.2 Move ito-skills/skills/ to assets/skills/
- [x] 1.3 Move ito-skills/adapters/ to assets/adapters/
- [x] 1.4 Create unified command files in assets/commands/ with expanded frontmatter

## 2. Code Updates

- [x] 2.1 Update ito-templates/src/lib.rs to embed skills, adapters, and commands
- [x] 2.2 Update distribution.rs to use embedded assets with AssetType enum
- [x] 2.3 Add command installation to each harness manifest function
- [x] 2.4 Handle GitHub's .prompt.md suffix requirement

## 3. Cleanup

- [x] 3.1 Remove per-harness skill directories from templates
- [x] 3.2 Remove per-harness command/prompt directories from templates
- [x] 3.3 Delete ito-skills/ directory
- [x] 3.4 Consolidate workflow skills (ito, ito-apply, etc.) to shared location

## 4. Standardization

- [x] 4.1 Fix OpenCode command frontmatter (YAML format)
- [x] 4.2 Fix Codex prompt frontmatter (YAML format)
- [x] 4.3 Fix GitHub prompt frontmatter (YAML format)
- [x] 4.4 Standardize all commands with name, description, category, tags

## 5. Documentation

- [x] 5.1 Update ito-rs/crates/ito-templates/AGENTS.md with new structure
- [x] 5.2 Add harness sync guidance to AGENTS.md
- [x] 5.3 Update root AGENTS.md to reference templates documentation

## 6. Verification

- [x] 6.1 Build ito successfully
- [x] 6.2 Run all tests
- [x] 6.3 Test ito init --tools all installs correctly
