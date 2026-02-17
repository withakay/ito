# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
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
