# Ito Rs Change Id Overflow Specification

## Purpose

Define the `ito-rs-change-id-overflow` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Change ID parser supports overflow change numbers

The `ito-rs` change ID parser SHALL accept change numbers larger than 99.

Canonicalization rules:

- The module component SHALL be normalized to 3 digits (`1` -> `001`).
- The change number component SHALL be normalized to a base-10 integer string with **minimum** 2-digit padding.
  - Example: `2` -> `02`.
  - If the integer requires more than 2 digits, it SHALL NOT be truncated (e.g. `100` -> `100`).
- The name component SHALL be lowercased.

#### Scenario: Change number greater than 99 is accepted

- **WHEN** parsing change ID `1-100_Bar`
- **THEN** the canonical ID is `001-100_bar`
- **AND** the parsed change number string is `100`

#### Scenario: Excessive padding is normalized for large change numbers

- **WHEN** parsing change ID `1-000100_bar`
- **THEN** the canonical ID is `001-100_bar`

#### Scenario: Existing two digit change numbers remain canonical

- **WHEN** parsing change ID `1-2_bar`
- **THEN** the canonical ID is `001-02_bar`

### Requirement: No hard maximum is enforced at 99

The `ito-rs` parser SHALL NOT enforce a maximum change number of 99.

#### Scenario: Large change number is accepted

- **WHEN** parsing change ID `1-1234_example`
- **THEN** parsing succeeds
- **AND** the canonical ID is `001-1234_example`
