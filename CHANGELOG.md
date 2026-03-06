# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## [0.1.15] - 2026-03-06

### 🚀 Features

- Add finish instruction and consolidate proposal guidance
- Add finish instruction and consolidate proposal guidance

### 🧪 Testing

- Fix init update recreation regression
- Update template rename expectations

### ⚙️ Miscellaneous Tasks

- *(ito-cli)* Trim doc examples to satisfy size gate
- *(ito-cli)* Apply rustfmt after rebase
## [0.1.14] - 2026-03-02

### 🚀 Features

- *(024-10)* Wire multi-tenant backend server config
- *(024-11)* Add grep command ([#122](https://github.com/withakay/ito/pull/122))
- Add `ito util parse-id` command and update ito-loop skill to use it ([#125](https://github.com/withakay/ito/pull/125))

### 🐛 Bug Fixes

- *(serve)* Eliminate ETXTBSY flakiness in tailscale detection tests ([#132](https://github.com/withakay/ito/pull/132))
## [0.1.13] - 2026-03-01

### 🚀 Features

- *(023-08)* Add Pi as a first-class harness ([#110](https://github.com/withakay/ito/pull/110))
- *(019-07)* Ship embedded schema validation.yaml ([#113](https://github.com/withakay/ito/pull/113))
- Dev integration — schema guidance, audit silence, Pi harness, specs bundle ([#114](https://github.com/withakay/ito/pull/114))
- *(024-01)* Add ito-backend crate with REST API for shared project state
- *(024)* Backend shared-state API + CLI backend client ([#118](https://github.com/withakay/ito/pull/118))

### 🐛 Bug Fixes

- *(024-01)* Address review feedback (ready response, docs, warnings)
- *(024-01)* Align docs + auth hardening + style nits
## [0.1.11] - 2026-02-26

### 🚀 Features

- *(023-07)* Harness context inference ([#109](https://github.com/withakay/ito/pull/109))
## [0.1.10] - 2026-02-25

### 🚀 Features

- *(019-04)* Schema-driven validation ([#99](https://github.com/withakay/ito/pull/99))
- *(templates)* Add github-copilot harness support to bootstrap
- *(019-05)* Schema-aware validate and tasks.md validation
- *(001-25)* Add tracking_file_path and fix missing review module export
- *(001-25)* Honor apply.tracks for task tracking ([#103](https://github.com/withakay/ito/pull/103))
- *(019-03)* Add --upgrade flag to ito init for marker-scoped template refresh ([#102](https://github.com/withakay/ito/pull/102))

### 🐛 Bug Fixes

- *(pr95)* Address review feedback on bootstrap and templates
- Align templates exports and formatting for push checks
- Remove duplicate tracking_file_path introduced by rebase

### 🚜 Refactor

- *(templates)* Remove generic skills, rename kept skills to ito- prefix
## [0.1.9] - 2026-02-23

## [0.1.8] - 2026-02-20

### 🚀 Features

- *(016-12)* Standardize ascending ID ordering across list surfaces ([#85](https://github.com/withakay/ito/pull/85))
- *(001-18)* Add peer-review agent instruction workflow ([#88](https://github.com/withakay/ito/pull/88))

### 🐛 Bug Fixes

- Remove extra argument from compute_review_context call ([#89](https://github.com/withakay/ito/pull/89))

### 📚 Documentation

- Migrate docs site to Zensical and publish via Pages ([#87](https://github.com/withakay/ito/pull/87))

## [0.1.7] - 2026-02-18

### 🚀 Features

- *(016-11)* Support description args in ito create module ([#83](https://github.com/withakay/ito/pull/83))
- *(016-11)* Add --description argument for create module command

### 🐛 Bug Fixes

- Improve tasks handling and move checks to pre-push ([#77](https://github.com/withakay/ito/pull/77))

## [0.1.6] - 2026-02-17

### 🚀 Features

- *(016-10)* Type-safe CLI args via bridge type pattern ([#55](https://github.com/withakay/ito/pull/55))
- *(019-01)* Normalize all output paths to absolute
- *(002-16)* Ralph worktree awareness ([#60](https://github.com/withakay/ito/pull/60))
- *(002-09)* Add interactive ralph mode ([#64](https://github.com/withakay/ito/pull/64))
- *(019-01)* Add ito path helpers for agent output ([#65](https://github.com/withakay/ito/pull/65))

### 🐛 Bug Fixes

- *(019-01)* Address PR review feedback
- *(019-01)* Address CodeRabbit nitpicks
- *(019-01)* Address CodeRabbit nitpicks
- *(config)* Restore build by removing stray token

### 🧪 Testing

- *(003-05)* Add unit and integration tests for ralph and harness modules ([#57](https://github.com/withakay/ito/pull/57))

## [0.1.4] - 2026-02-13

### 🚀 Features

- *(002-14)* Add Ralph harnesses for Claude Code, Codex, and GitHub Copilot ([#48](https://github.com/withakay/ito/pull/48))

## [0.1.3] - 2026-02-11

### 🚀 Features

- *(config)* Generate and version Ito config schema artifact ([#26](https://github.com/withakay/ito/pull/26))
- *(coordination)* Sync change workflows with coordination branch ([#29](https://github.com/withakay/ito/pull/29))

### 🐛 Bug Fixes

- *(coordination)* Address PR review security and safety feedback

### 🚜 Refactor

- *(coordination)* Address remaining PR nitpicks

## [0.1.2] - 2026-02-11

### 🐛 Bug Fixes

- Resolve merge fallout in workflow/template refactor

### 💼 Other

- Embed and export workflow schemas ([#15](https://github.com/withakay/ito/pull/15))

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
