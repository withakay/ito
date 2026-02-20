## MODIFIED Requirements

### Requirement: Sorting

The command SHALL support deterministic ordering for all list results and SHALL sort ID-bearing lists in ascending ID order by default.

#### Scenario: Default sort is ascending by change ID

- **WHEN** `ito list` is executed without `--sort`
- **THEN** it sorts changes in ascending canonical change ID order

#### Scenario: Sorting changes by name

- **GIVEN** multiple changes exist
- **WHEN** `ito list --sort name` is executed
- **THEN** sort them in ascending canonical change ID order

#### Scenario: Sorting changes by recent remains deterministic

- **GIVEN** multiple changes exist
- **WHEN** `ito list --sort recent` is executed
- **THEN** order changes from most recent to least recent
- **AND** when two changes have the same modified timestamp, order those ties by ascending canonical change ID

#### Scenario: Module lists are ascending by module ID

- **WHEN** `ito list --modules` is executed
- **THEN** module entries are sorted in ascending module ID order

#### Scenario: Spec lists are ascending by spec ID

- **WHEN** `ito list --specs` is executed
- **THEN** spec entries are sorted in ascending spec ID order
