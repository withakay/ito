# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## [0.1.27] - 2026-04-24

### 🚀 Features

- *(worktrees)* Add worktree lifecycle management and initialization (012-05) ([#203](https://github.com/withakay/ito/pull/203))
- *(025-09)* Add worktree sync command with coordination-first archive ([#204](https://github.com/withakay/ito/pull/204))
- *(028-01)* Ito agent instruction orchestrate + orchestrator core ([#205](https://github.com/withakay/ito/pull/205))

### 🎨 Styling

- Cargo fmt normalization
## [0.1.26] - 2026-04-13

### 🚀 Features

- *(001-30)* Add HTML browser viewer backend ([#188](https://github.com/withakay/ito/pull/188))
- *(025-08)* Centralize mutable Ito artifacts onto coordination branch worktree
- *(002-18)* Extend ralph orchestration parity ([#195](https://github.com/withakay/ito/pull/195))

### 🚜 Refactor

- Extract inline tests from oversized modules ([#196](https://github.com/withakay/ito/pull/196))
## [0.1.25] - 2026-03-26
## [0.1.24] - 2026-03-24

### 🚀 Features

- *(001-27)* Add requirement traceability chain ([#181](https://github.com/withakay/ito/pull/181))
## [0.1.23] - 2026-03-22

### 🚀 Features

- *(001-31)* Add tmux-aware proposal viewer workflow ([#175](https://github.com/withakay/ito/pull/175))
## [0.1.22] - 2026-03-21

### 🚀 Features

- *(009-03)* Route audit storage off work branches
## [0.1.21] - 2026-03-13

### 🚀 Features

- *(024-18)* Add direct backend import workflow ([#168](https://github.com/withakay/ito/pull/168))
## [0.1.20] - 2026-03-11
## [0.1.19] - 2026-03-10

### 🚀 Features

- *(024-17)* Add backend status and token tooling ([#159](https://github.com/withakay/ito/pull/159))
- *(025)* Complete repository backends module ([#161](https://github.com/withakay/ito/pull/161))
## [0.1.18] - 2026-03-07
## [0.1.17] - 2026-03-07

### 🚀 Features

- *(024-16)* Align Homebrew install and service bootstrap ([#156](https://github.com/withakay/ito/pull/156))
## [0.1.16] - 2026-03-07

### 🚀 Features

- *(024-14)* Add serve-api --init and global config fallback for auth tokens ([#154](https://github.com/withakay/ito/pull/154))
## [0.1.15] - 2026-03-06

### 🎨 Styling

- Format distribution test assertions

### 🧪 Testing

- Update template rename expectations
## [0.1.14] - 2026-03-02

### 🚀 Features

- *(024-10)* Add YAML front matter parsing/writing utilities
- *(024-10)* Integrate front matter validation into change repository
- *(024-11)* Add grep command ([#122](https://github.com/withakay/ito/pull/122))

### 🐛 Bug Fixes

- Address PR review feedback - path traversal, auth redaction, and docs consistency
- *(ralph)* Continue-module skips processed and validates worktree tasks
- *(ralph)* Harden continue-module filtering and commit checks
## [0.1.13] - 2026-03-01

### 🚀 Features

- *(023-08)* Add Pi as a first-class harness ([#110](https://github.com/withakay/ito/pull/110))
- *(019-07)* Ship embedded schema validation.yaml ([#113](https://github.com/withakay/ito/pull/113))
- Dev integration — schema guidance, audit silence, Pi harness, specs bundle ([#114](https://github.com/withakay/ito/pull/114))
- *(002-17)* Add /loop wrapper for ralph ([#115](https://github.com/withakay/ito/pull/115))
- *(024-01)* Add ito-backend crate with REST API for shared project state
- *(024)* Backend shared-state API + CLI backend client ([#118](https://github.com/withakay/ito/pull/118))
## [0.1.11] - 2026-02-26

### 🚀 Features

- *(023-07)* Harness context inference ([#109](https://github.com/withakay/ito/pull/109))
## [0.1.10] - 2026-02-25

### 🚀 Features

- *(019-04)* Schema-driven validation ([#99](https://github.com/withakay/ito/pull/99))
- *(ito)* Add pending schema and instruction artifacts
- *(001-24)* Versioned format specs ([#100](https://github.com/withakay/ito/pull/100))
- *(ito)* Add pending schema and instruction artifacts
- *(001-25)* Add tracking_file_path and fix missing review module export
- *(001-24)* Versioned format specs ([#100](https://github.com/withakay/ito/pull/100))
- *(001-25)* Honor apply.tracks for task tracking ([#103](https://github.com/withakay/ito/pull/103))
- *(019-03)* Add --upgrade flag to ito init for marker-scoped template refresh ([#102](https://github.com/withakay/ito/pull/102))

### 🐛 Bug Fixes

- *(019-02)* Hide internal guidance scaffolding from instruction output ([#93](https://github.com/withakay/ito/pull/93))
- Align templates exports and formatting for push checks
- Remove duplicate tracking_file_path introduced by rebase

### 🚜 Refactor

- *(templates)* Split guidance and task parsing modules

### 🎨 Styling

- Apply cargo fmt import sorting to validate and template test files
## [0.1.9] - 2026-02-23

## [0.1.8] - 2026-02-20

### 🚀 Features

- *(016-12)* Standardize ascending ID ordering across list surfaces ([#85](https://github.com/withakay/ito/pull/85))
- *(001-18)* Add peer-review agent instruction workflow ([#88](https://github.com/withakay/ito/pull/88))

### 📚 Documentation

- Migrate docs site to Zensical and publish via Pages ([#87](https://github.com/withakay/ito/pull/87))

## [0.1.7] - 2026-02-18

### 🚀 Features

- *(016-11)* Support description args in ito create module ([#83](https://github.com/withakay/ito/pull/83))
- *(016-11)* Implement module description parameter handling in core

### 🐛 Bug Fixes

- Improve tasks handling and move checks to pre-push ([#77](https://github.com/withakay/ito/pull/77))
- *(review)* Address PR 78 core and adapter feedback
- *(tasks)* Repair failing task doctest snippets

### 🚜 Refactor

- Address PR review feedback

### 🎨 Styling

- *(tests)* Apply rustfmt updates

### ⚙️ Miscellaneous Tasks

- Autofix lint and formatting issues
- Autofix lint and formatting issues

## [0.1.6] - 2026-02-17

### 🚀 Features

- *(002-15)* Add retriable exit code handling for harness crashes
- *(019-01)* Normalize all output paths to absolute
- *(002-16)* Ralph worktree awareness ([#60](https://github.com/withakay/ito/pull/60))
- *(002-09)* Add interactive ralph mode ([#64](https://github.com/withakay/ito/pull/64))
- *(019-01)* Add ito path helpers for agent output ([#65](https://github.com/withakay/ito/pull/65))

### 🐛 Bug Fixes

- Address PR #58 review feedback from Gemini and CodeRabbit

### 🚜 Refactor

- *(002-15)* Introduce CliHarness trait and deduplicate CLI harnesses
- *(harness)* Migrate HarnessName from struct to enum

### 🎨 Styling

- Sort imports alphabetically in ito-cli and ito-core

### 🧪 Testing

- *(002-15)* Add tests for retriable harness exit code behavior

## [0.1.4] - 2026-02-13

### 🚀 Features

- *(002-14)* Add Ralph harnesses for Claude Code, Codex, and GitHub Copilot ([#48](https://github.com/withakay/ito/pull/48))

## [0.1.3] - 2026-02-11

### 🚀 Features

- *(config)* Generate and version Ito config schema artifact ([#26](https://github.com/withakay/ito/pull/26))
- *(coordination)* Sync change workflows with coordination branch ([#29](https://github.com/withakay/ito/pull/29))

### 🐛 Bug Fixes

- Resolve merge fallout in workflow/template refactor
- *(coordination)* Address PR review security and safety feedback

### 💼 Other

- Embed and export workflow schemas ([#15](https://github.com/withakay/ito/pull/15))
- *(ito-core)* Validate change-id path segments in task flows
- *(ito-core)* Validate process requests and harden temp output

### 🚜 Refactor

- Make ito workflow a no-op surface ([#17](https://github.com/withakay/ito/pull/17))
- Remove workflow command and migrate core workflow module to templates
- Remove workflow compatibility surface across rust crates
- *(ito-core)* Apply explicit matching style in core helpers
- *(coordination)* Address remaining PR nitpicks

### 📚 Documentation

- *(ito-core)* Clarify validation issue and report semantics

### 🧪 Testing

- *(ito-core)* Add ValidationIssue helper tests
- *(ito-core)* Add ReportBuilder behavior coverage

## [0.1.2] - 2026-02-11

### 🐛 Bug Fixes

- Resolve merge fallout in workflow/template refactor

### 💼 Other

- Embed and export workflow schemas ([#15](https://github.com/withakay/ito/pull/15))
- *(ito-core)* Validate change-id path segments in task flows

### 🚜 Refactor

- Make ito workflow a no-op surface ([#17](https://github.com/withakay/ito/pull/17))
- Remove workflow command and migrate core workflow module to templates
- Remove workflow compatibility surface across rust crates

### 🧪 Testing

- Raise coverage for validation and template helpers ([#25](https://github.com/withakay/ito/pull/25))

## [0.1.1] - 2026-02-10

### 🐛 Bug Fixes

- Address PR #9 review feedback
- *(ci)* Fix doctest and formatting regressions

### ⚡ Performance

- Optimize test execution speed (44% reduction) + archive 015-14 ([#9](https://github.com/withakay/ito/pull/9))

## [0.1.0](https://github.com/withakay/ito/releases/tag/ito-core-v0.1.0) - 2026-02-05

### Fixed

- release-plz

### Other

- moar docs
- add CHANGELOG.md templates for all crates
- The big reset
