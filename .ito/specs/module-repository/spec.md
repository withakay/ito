# Module Repository Specification

## Purpose

Define the `module-repository` capability: centralized access to module metadata and listings.

## Requirements

### Requirement: ModuleRepository provides centralized module access

A `ModuleRepository` struct SHALL exist in `ito-workflow` that provides methods for loading and querying module data.

#### Scenario: Get a module by ID

- **GIVEN** a module with ID "005" and name "dev-tooling" exists
- **WHEN** calling `ModuleRepository::get("005")`
- **THEN** it returns a `Module` object with id, name, and description

#### Scenario: List all modules

- **WHEN** calling `ModuleRepository::list()`
- **THEN** it returns a `Vec<ModuleSummary>` with all modules
- **AND** each summary includes id, name, and change count

#### Scenario: List modules with changes

- **WHEN** calling `ModuleRepository::list_with_changes()`
- **THEN** it returns modules along with their associated changes

### Requirement: Module domain model

A `Module` struct SHALL encapsulate module metadata.

#### Scenario: Module contains metadata

- **GIVEN** a module.yaml with name and description
- **WHEN** the module is loaded via `ModuleRepository::get()`
- **THEN** `module.id` contains the module ID (e.g., "005")
- **AND** `module.name` contains the module name (e.g., "dev-tooling")
- **AND** `module.description` contains the description if present

### Requirement: ModuleSummary provides lightweight listing

A `ModuleSummary` struct SHALL provide essential module info for listings.

#### Scenario: Summary includes change counts

- **WHEN** calling `ModuleRepository::list()`
- **THEN** each `ModuleSummary` includes:
  - `id: String`
  - `name: String`
  - `change_count: u32`
