# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## [0.1.18] - 2026-03-07
## [0.1.17] - 2026-03-07
## [0.1.16] - 2026-03-07
## [0.1.15] - 2026-03-06

### 🐛 Bug Fixes

- Add yaml frontmatter to loop skill template

### 🚜 Refactor

- Rename ito-apply-change-proposal to ito-apply and ito-write-change-proposal to ito-proposal in command metadata
- Update embedded skill assets
- Update instructions.rs and test support code for skill consolidation
- Add renamed ito template assets

### 🧪 Testing

- Make frontmatter assertion clearer
- Move instruction renderer tests into separate module
- Fix init update recreation regression

### ⚙️ Miscellaneous Tasks

- Remove legacy planning files from project template
- Remove legacy skill assets from templates
## [0.1.14] - 2026-03-02

### 🚀 Features

- Add `ito util parse-id` command and update ito-loop skill to use it ([#125](https://github.com/withakay/ito/pull/125))

### 🐛 Bug Fixes

- Address PR review feedback - path traversal, auth redaction, and docs consistency
## [0.1.13] - 2026-03-01

### 🚀 Features

- *(023-08)* Add Pi as a first-class harness ([#110](https://github.com/withakay/ito/pull/110))
- *(019-07)* Ship embedded schema validation.yaml ([#113](https://github.com/withakay/ito/pull/113))
- Dev integration — schema guidance, audit silence, Pi harness, specs bundle ([#114](https://github.com/withakay/ito/pull/114))
- *(002-17)* Add /loop wrapper for ralph ([#115](https://github.com/withakay/ito/pull/115))
## [0.1.11] - 2026-02-26

### 🚀 Features

- *(023-07)* Harness context inference ([#109](https://github.com/withakay/ito/pull/109))
## [0.1.10] - 2026-02-25

### 🚀 Features

- *(templates)* Add interview-first guidance to proposal workflow
- *(templates)* Add github-copilot harness support to bootstrap
- *(ito)* Add pending schema and instruction artifacts
- *(019-03)* Add --upgrade flag to ito init for marker-scoped template refresh ([#102](https://github.com/withakay/ito/pull/102))

### 🐛 Bug Fixes

- *(templates)* Tighten commands, adapters, schemas, remove stale refs
- *(019-02)* Hide internal guidance scaffolding from instruction output ([#93](https://github.com/withakay/ito/pull/93))
- *(pr95)* Address review feedback on bootstrap and templates
- Align templates exports and formatting for push checks
- *(templates)* Harden apply instruction shell snippets

### 💼 Other

- Resolve PR #95 conflicts by merging main into dev

### 🚜 Refactor

- *(templates)* Deduplicate and tighten instruction templates
- *(templates)* Remove generic skills, rename kept skills to ito- prefix
- *(validate)* Sort imports and add format validation module
- *(templates)* Remove generic skills, rename kept skills to ito- prefix

### 📚 Documentation

- *(commands)* Simplify ito command documentation across harnesses

### 🎨 Styling

- Apply cargo fmt import sorting to validate and template test files

### ⚙️ Miscellaneous Tasks

- Add rust-docs to toolchain and enable doc checks in CI
## [0.1.9] - 2026-02-23

## [0.1.8] - 2026-02-20

### 🚀 Features

- *(001-18)* Add peer-review agent instruction workflow ([#88](https://github.com/withakay/ito/pull/88))

### 🐛 Bug Fixes

- *(ci)* Skip ito audit checks when ito CLI is unavailable

### 📚 Documentation

- Migrate docs site to Zensical and publish via Pages ([#87](https://github.com/withakay/ito/pull/87))

## [0.1.7] - 2026-02-18

### 🚀 Features

- *(016-11)* Support description args in ito create module ([#83](https://github.com/withakay/ito/pull/83))

### 🐛 Bug Fixes

- *(review)* Address PR 78 core and adapter feedback

## [0.1.6] - 2026-02-17

### 🚀 Features

- *(019-01)* Resolve absolute paths in worktree instruction templates
- *(019-01)* Normalize all output paths to absolute
- *(002-09)* Add interactive ralph mode ([#64](https://github.com/withakay/ito/pull/64))
- *(019-01)* Add ito path helpers for agent output ([#65](https://github.com/withakay/ito/pull/65))

### 🐛 Bug Fixes

- Address PR #58 review feedback from Gemini and CodeRabbit
- *(019-01)* Address PR review feedback
- *(019-01)* Address CodeRabbit nitpicks
- *(config)* Restore build by removing stray token

### 📚 Documentation

- *(ito-commit)* Add pre-commit safety guidance for agents

## [0.1.3] - 2026-02-11

### 🚀 Features

- *(config)* Generate and version Ito config schema artifact ([#26](https://github.com/withakay/ito/pull/26))

## [0.1.2] - 2026-02-11

### 💼 Other

- Embed and export workflow schemas ([#15](https://github.com/withakay/ito/pull/15))
- *(ito-templates)* Sanitize custom ito dir names

### 🧪 Testing

- Raise coverage for validation and template helpers ([#25](https://github.com/withakay/ito/pull/25))

## [0.1.1] - 2026-02-10

### Other

- Version bump for workspace consistency (no crate-specific changes)

## [0.1.0](https://github.com/withakay/ito/releases/tag/ito-templates-v0.1.0) - 2026-02-05

### Fixed

- release-plz

### Other

- moar docs
- add CHANGELOG.md templates for all crates
- The big reset
