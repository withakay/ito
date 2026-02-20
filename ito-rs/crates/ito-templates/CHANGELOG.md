# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
