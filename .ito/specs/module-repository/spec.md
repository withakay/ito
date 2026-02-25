# Spec: module-repository

## Purpose

Define the `module-repository` capability and its current-truth behavior. This spec captures requirements and scenarios (for example: ModuleRepository provides centralized module access).

## Requirements

### Requirement: ModuleRepository provides centralized module access

A `ModuleRepository` interface SHALL exist in `ito-domain` that provides methods for loading and querying module data.

`ito-core` SHALL provide a filesystem-backed implementation of this interface for production use.

#### Scenario: Get a module by ID

- **GIVEN** a module with ID "005" and name "dev-tooling" exists
- **WHEN** calling `module_repo.get("005")`
- **THEN** it returns a `Module` object with id, name, and description

#### Scenario: List all modules

- **WHEN** calling `module_repo.list()`
- **THEN** it returns a `Vec<ModuleSummary>` with all modules
- **AND** each summary includes id, name, and change count

#### Scenario: List modules with changes

- **WHEN** calling `module_repo.list_with_changes()`
- **THEN** it returns modules along with their associated changes
