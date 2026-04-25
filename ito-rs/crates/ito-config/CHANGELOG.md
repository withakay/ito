# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## [0.1.28] - 2026-04-25

### 🚀 Features

- *(029-02)* Agent memory abstraction (capture/search/query) ([#211](https://github.com/withakay/ito/pull/211))
- *(016-17)* Add ito list-archive command ([#212](https://github.com/withakay/ito/pull/212))
## [0.1.27] - 2026-04-24

### 🚀 Features

- *(worktrees)* Add worktree lifecycle management and initialization (012-05) ([#203](https://github.com/withakay/ito/pull/203))
- *(025-09)* Add worktree sync command with coordination-first archive ([#204](https://github.com/withakay/ito/pull/204))
## [0.1.26] - 2026-04-13

### 🚀 Features

- *(025-08)* Centralize mutable Ito artifacts onto coordination branch worktree
## [0.1.25] - 2026-03-26

### 🚀 Features

- Add configurable logging of invalid CLI commands ([#183](https://github.com/withakay/ito/pull/183))
## [0.1.24] - 2026-03-24
## [0.1.23] - 2026-03-22

### 🚀 Features

- *(001-31)* Add tmux-aware proposal viewer workflow ([#175](https://github.com/withakay/ito/pull/175))
## [0.1.22] - 2026-03-21
## [0.1.21] - 2026-03-13
## [0.1.20] - 2026-03-11
## [0.1.19] - 2026-03-10

### 🚀 Features

- *(025)* Complete repository backends module ([#161](https://github.com/withakay/ito/pull/161))
## [0.1.18] - 2026-03-07
## [0.1.17] - 2026-03-07
## [0.1.16] - 2026-03-07

### 🚀 Features

- *(024-14)* Add serve-api --init and global config fallback for auth tokens ([#154](https://github.com/withakay/ito/pull/154))
## [0.1.15] - 2026-03-06
## [0.1.14] - 2026-03-02

### 🚀 Features

- *(024-10)* Wire multi-tenant backend server config
## [0.1.13] - 2026-03-01

### 🚀 Features

- Dev integration — schema guidance, audit silence, Pi harness, specs bundle ([#114](https://github.com/withakay/ito/pull/114))
- *(024-01)* Add ito-backend crate with REST API for shared project state
- *(024)* Backend shared-state API + CLI backend client ([#118](https://github.com/withakay/ito/pull/118))
## [0.1.11] - 2026-02-26
## [0.1.10] - 2026-02-25
## [0.1.9] - 2026-02-23

## [0.1.6] - 2026-02-17

### 🚀 Features

- *(002-09)* Add interactive ralph mode ([#64](https://github.com/withakay/ito/pull/64))
- *(019-01)* Add ito path helpers for agent output ([#65](https://github.com/withakay/ito/pull/65))

### 🐛 Bug Fixes

- *(019-01)* Address PR review feedback
- *(config)* Restore build by removing stray token

## [0.1.3] - 2026-02-11

### 🚀 Features

- *(coordination)* Sync change workflows with coordination branch ([#29](https://github.com/withakay/ito/pull/29))

### 🐛 Bug Fixes

- *(coordination)* Address PR review security and safety feedback

### 🚜 Refactor

- *(coordination)* Address remaining PR nitpicks

## [0.1.2] - 2026-02-11

### 🚜 Refactor

- *(ito-config)* Validate ito dir overrides and expand docs

### 🧪 Testing

- Raise coverage for validation and template helpers ([#25](https://github.com/withakay/ito/pull/25))

## [0.1.1] - 2026-02-10

### Other

- Version bump for workspace consistency (no crate-specific changes)

## [0.1.0](https://github.com/withakay/ito/releases/tag/ito-config-v0.1.0) - 2026-02-05

### Fixed

- release-plz

### Other

- moar docs
- add CHANGELOG.md templates for all crates
- The big reset
