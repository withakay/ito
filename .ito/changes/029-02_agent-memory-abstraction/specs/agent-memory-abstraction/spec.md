<!-- ITO:START -->
## ADDED Requirements

### Requirement: Memory config is optional and additive

Ito config SHALL gain an optional top-level `memory` section describing the
active memory provider. When the section is absent, Ito MUST treat memory
as "not configured" and SHALL NOT error.

- **Requirement ID**: `agent-memory-abstraction:optional-config`

#### Scenario: Config without memory section loads successfully

- **WHEN** an Ito config omits the `memory` section entirely
- **THEN** `ito validate` succeeds
- **AND** `memory.configured` exposed to instruction templates evaluates to `false`

#### Scenario: Unknown memory provider value is rejected

- **WHEN** an Ito config declares `memory.provider` with a value that is not `commands` or `skill`
- **THEN** `ito validate --strict` fails with a message that lists the accepted values

### Requirement: Memory provider shape — command-based

When `memory.provider == "commands"`, the config SHALL provide a `store`
and a `search` command template (string). The `store` template SHALL include
`{text}` and the `search` template SHALL include `{query}`. These placeholders
remain literal in emitted instructions so the agent can substitute input at
execution time.

- **Requirement ID**: `agent-memory-abstraction:commands-provider`

#### Scenario: Valid commands provider passes validation

- **WHEN** `memory` is `{ provider: "commands", store: "brv curate \"{text}\"", search: "brv search \"{query}\"" }`
- **THEN** `ito validate --strict` succeeds
- **AND** `ito agent instruction memory-capture` emits the configured command template with the literal `{text}` placeholder preserved for the agent to substitute

#### Scenario: Missing placeholder rejected

- **WHEN** `memory.store` is configured without a `{text}` placeholder
- **THEN** `ito validate --strict` fails with an error naming the missing placeholder and the offending field

#### Scenario: Missing store/search rejected

- **WHEN** `memory.provider == "commands"` and either `store` or `search` is missing
- **THEN** `ito validate --strict` fails with a message identifying the missing field

### Requirement: Memory provider shape — skill-based

When `memory.provider == "skill"`, the config SHALL reference an installed
skill by id. Ito's memory instructions SHALL delegate memory operations to
that skill rather than running a shell command.

- **Requirement ID**: `agent-memory-abstraction:skill-provider`

#### Scenario: Valid skill provider passes validation

- **WHEN** `memory` is `{ provider: "skill", skill: "byterover-explore" }` and a skill with that id is discoverable under `.agents/skills/` (or a harness-specific skills dir)
- **THEN** `ito validate --strict` succeeds
- **AND** `ito agent instruction memory-capture` emits guidance that tells the agent to invoke the named skill

#### Scenario: Unknown skill id rejected

- **WHEN** `memory.provider == "skill"` and the referenced skill id is not discoverable in any known skills directory
- **THEN** `ito validate --strict` fails with a message naming the missing skill id and listing searched paths

### Requirement: `ito agent instruction memory-capture` artifact

The CLI SHALL support `ito agent instruction memory-capture` and emit
instructions that tell an agent how to store a newly observed memory.
The output MUST reflect the configured provider.

- **Requirement ID**: `agent-memory-abstraction:memory-capture-artifact`

#### Scenario: Commands provider — renders store command

- **WHEN** memory is configured with `provider: "commands"` and a valid `store` template
- **THEN** `ito agent instruction memory-capture` prints the rendered command line (with the literal `{text}` placeholder preserved for the agent to substitute)
- **AND** the output includes a one-line description of when to capture a memory (decisions, gotchas, patterns)

#### Scenario: Skill provider — renders skill invocation

- **WHEN** memory is configured with `provider: "skill"` and a valid `skill` id
- **THEN** `ito agent instruction memory-capture` prints guidance that instructs the agent to invoke the named skill with the memory text as input

#### Scenario: Not configured — renders setup guidance

- **WHEN** memory is not configured
- **THEN** `ito agent instruction memory-capture` exits 0
- **AND** the output states that memory is not configured and explains the two provider shapes (commands, skill) with a one-line example for each

### Requirement: `ito agent instruction memory-search` artifact

The CLI SHALL support `ito agent instruction memory-search` and emit
instructions that tell an agent how to search stored memory for relevant
context. The output MUST reflect the configured provider.

- **Requirement ID**: `agent-memory-abstraction:memory-search-artifact`

#### Scenario: Commands provider — renders search command

- **WHEN** memory is configured with `provider: "commands"` and a valid `search` template
- **THEN** `ito agent instruction memory-search` prints the rendered command line (with the literal `{query}` placeholder preserved for the agent to substitute)

#### Scenario: Skill provider — renders skill invocation

- **WHEN** memory is configured with `provider: "skill"` and a valid `skill` id
- **THEN** `ito agent instruction memory-search` prints guidance that instructs the agent to invoke the named skill with the search query as input

#### Scenario: Not configured — renders setup guidance

- **WHEN** memory is not configured
- **THEN** `ito agent instruction memory-search` exits 0
- **AND** the output states that memory is not configured and explains how to configure it

### Requirement: No default provider

Ito SHALL NOT ship a default memory provider. A freshly-initialized Ito
project with no memory config MUST behave identically to one with an
explicit "memory absent" config.

- **Requirement ID**: `agent-memory-abstraction:no-default-provider`

#### Scenario: Fresh init does not configure memory

- **WHEN** `ito init` runs on a new project
- **THEN** the resulting Ito config does not contain a `memory` section
- **AND** `ito agent instruction memory-capture` renders the not-configured setup guidance

### Requirement: Templating placeholders are well-defined

The command-based provider SHALL support exactly two placeholders:
`{text}` for store input and `{query}` for search input. Unknown or
additional placeholders SHALL be treated as literal text.

- **Requirement ID**: `agent-memory-abstraction:placeholder-semantics`

#### Scenario: Unknown placeholder passes through as literal

- **WHEN** a configured `store` template contains `{foo}`
- **THEN** the rendered output preserves `{foo}` literally
- **AND** no validation error is raised for unknown placeholders (they are treated as opaque command text)
<!-- ITO:END -->
