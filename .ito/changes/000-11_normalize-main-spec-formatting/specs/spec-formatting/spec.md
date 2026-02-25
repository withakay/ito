## ADDED Requirements

### Requirement: Main specs use a canonical document structure

Main spec files under `.ito/specs/<capability>/spec.md` SHALL use a canonical structure so authors can quickly recognize them as "current truth" specifications.

At minimum, main specs MUST contain:

- An H1 title (`# ...`) at the top of the file
- A `## Purpose` section with non-placeholder text
- A `## Requirements` section containing requirement blocks

Main specs MUST NOT contain delta operation section headers:

- `## ADDED Requirements`
- `## MODIFIED Requirements`
- `## REMOVED Requirements`
- `## RENAMED Requirements`

#### Scenario: Author can distinguish truth specs from delta specs

- **GIVEN** a file under `.ito/specs/**/spec.md`
- **WHEN** an author opens the document
- **THEN** they can recognize it as a truth spec because it uses `## Requirements` (not delta operation sections)

#### Scenario: Main spec contains purpose text

- **GIVEN** a file under `.ito/specs/**/spec.md`
- **WHEN** the spec is reviewed
- **THEN** its `## Purpose` section MUST NOT be `TBD`

### Requirement: Normalization preserves requirement semantics

When normalizing main specs to the canonical structure, the process SHALL be semantics-preserving.

Normalization SHALL:

- Preserve all `### Requirement: ...` headings and their associated text
- Preserve all `#### Scenario: ...` headings and their associated steps
- Restrict edits to outer structure (title/purpose/requirements headings) and formatting that does not change meaning

#### Scenario: Requirement and scenario blocks remain intact

- **GIVEN** a main spec that currently uses delta operation headings
- **WHEN** it is normalized
- **THEN** every `### Requirement:` block and every `#### Scenario:` block remains present with the same wording
