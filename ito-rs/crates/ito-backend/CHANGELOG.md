# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## [0.1.14] - 2026-03-02

### 🚀 Features

- *(024-10)* Wire multi-tenant backend server config

### 🐛 Bug Fixes

- Address PR review feedback - path traversal, auth redaction, and docs consistency
## [0.1.13] - 2026-03-01

### 🚀 Features

- *(024-01)* Add ito-backend crate with REST API for shared project state
- *(024)* Backend shared-state API + CLI backend client ([#118](https://github.com/withakay/ito/pull/118))

### 🐛 Bug Fixes

- *(024-01)* Address review feedback (ready response, docs, warnings)
- *(024-01)* Align docs + auth hardening + style nits
- *(ito-backend)* Repair auth + CORS parsing after rebase
- *(backend)* Use ito-domain types directly to fix publish verification errors
- Bump workspace version to 0.1.12 to republish ito-domain/ito-config/ito-core with backend features
