## ADDED Requirements

### Requirement: Deterministic ID ordering for selection lists

The `show` command SHALL order ID-bearing selection and disambiguation lists in ascending ID order.

#### Scenario: Interactive selection lists are ID-ordered

- **WHEN** executing `ito show` in interactive mode
- **THEN** change choices are listed in ascending canonical change ID order
- **AND** spec choices are listed in ascending canonical spec ID order

#### Scenario: Ambiguous match lists are ID-ordered

- **WHEN** executing `ito show <item-name>` and multiple change IDs match
- **THEN** ambiguity matches are printed in ascending canonical change ID order

#### Scenario: Suggested matches are deterministic

- **WHEN** executing `ito show <item-name>` and nearest-match suggestions are printed
- **THEN** the output ordering is deterministic
- **AND** ties in relevance are broken by ascending canonical ID order
