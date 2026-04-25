<!-- ITO:START -->
## ADDED Requirements

### Requirement: Memory config is optional and per-operation

Ito config SHALL gain an optional top-level `memory` section. When present
it SHALL be an object keyed by operation name, with the keys `capture`,
`search`, and `query`. Each operation entry is independently optional.
When the whole `memory` section is absent, Ito MUST treat memory as "not
configured" and SHALL NOT error.

- **Requirement ID**: `agent-memory-abstraction:optional-per-op-config`

#### Scenario: Config without memory section loads successfully

- **WHEN** an Ito config omits the `memory` section entirely
- **THEN** `ito validate --strict` succeeds
- **AND** `memory.capture.configured`, `memory.search.configured`, and `memory.query.configured` - **AND** `memory.capturen templates) all evaluate to `false`

#### Scenario: Partial memory config is valid

- **WHEN** an Ito config declares `memory.capture` but omits `memory.search` and `memory.query`
- **THEN** `ito validate --strict` succeeds
- **AND** `memory.capture.configured` is `true` while `memory.search.configured` and `memory.query.configured` are `false`

#### Scenario: Unknown operation key rejected

- **WHEN** an- **WHEnfig declares `memory.<unknown>` for a key that is not `capture`, `search`, or `query`
- **THEN** `ito validate --strict` fails with a message listing the accepted operation keys

### Requirement: Per-operation shape — skill or command

For each operation entry present under `memory`, the entry SHALL take one
of two shapes, chosen by a `kind` discriminator:

- `{ "kind": "skill", "skill": "<skill-id>", "options": { … } }` — delegate
  the operation to an installed skill. `options` is optional and is an
  opaque JSON value passed through verbatim to the skill; Ito SHALL NOT
  interpret its contents.
- `{ "kind": "command", "command": "<template>" }` — render a shell
  command line by substituting the operation's placeholders (see the
  placeholder-semantics requirement below).

Operations are configured independently: different operations MAY use
different shapes in the same config.

- **Requirement ID**: `agent-memory-abstraction:per-op-shape`

#### Scenario: Mixed shapes across op#### Scenario: Mixed shapes acrmemory.capture` is `{ kind: "skill", skill: "ito-memory-markdown" }` and `memory.search` is `{ kind: "command", command: "rg "{query}" .ito/memories" }`
- **THEN** `ito validate --strict` succeeds
- **AND** `memory-capture` renders as a skill-delegation directive while `memory-search` renders as a command

#### Scenario: Unknown kind rejected

- **WHEN** an operation entry declares `kind` with a value that is not `skill` or `command`
- **THEN** `ito validate --strict` fails with a message that lists the accepted kinds

#### Scenario: Missing required field rejected

- **WHEN** an operation entry declares `kind: "skill"` without a `skill` id, or `kind: "command"` without a `command` template
- **THEN** `ito validate --strict` fails with a message identifying the missing field

#### Scenario: Skill id must resolve

- **WHEN** `kind: "skill"` references an id that is not discoverable under `.agents/skills/`, `.claude/skills/`, or any other known skills directory
- **THEN** `ito validate --strict` fails with a message naming the missing skill id and the directories it searched

### Requirement: Operation input schemas are fixed per operation

Each operation SHALL accept a fixed set of structured inputs from the
calling agent. These inputs drive placeholder substitution (for `command`
shape) and are passed through to the delegated skill (for `skill` shape).
The schemas are fixed by Ito and are not provider-defined.

- `memory-capture` inputs: `context` (string, optional), `files`
  (list of filesystem paths, optional, may repeat), `folders`
  (list of filesystem paths, optional, may repeat).
- `memory-search` inputs: `query` (string, required), `limit`
  (positive integer, optional, default `10`), `scope` (string,
  optional).
- `memory-query` inputs: `query` (string, required).

- **Requirement ID**: `agent-memory-abstraction:operation-input-schemas`

#### Scenario: CLI accepts the documented flags per operation

- **WHEN** an agent invokes `ito agent instruction memory-capture --context "…" --file a.md --file b.md --folder docs/`
- **THEN** the command exits 0
- **AND** the `context`, `files=[a.md, b.md]`, and `folders=[docs/]` inputs reach the resolver

#### Scenario: Required input missing is rejected

- **WHEN** an agent invokes `ito agent instruction memory-search` without `--query`
- **THEN** the command exits non-zero with a usage error naming the missing required input

### Requirement: Placeholder rendering for `command` shape

When an operation is configured with `kind: "command"`, Ito SHALL render
placeholders in the template at instruction-emission time using the
caller-supplied inputs. The rendered output MUST be a shell command line
that is executable as-is (no further substitution required by the agent).

Rendering rules:

- **Scalar string placeholder** (`{context}`, `{query}`, `{scope}`): the
  value is substituted as a single shell-quoted token. Missing values
  render as an empty string. Scalar-string placeholders are always
  shell-quoted (e.g. `{query}` with value `foo bar` renders as `'foo bar'`).
- **Scalar integer placeholder** (`{limit}`): the value is substituted as
  a decimal integer literal. Missing values render as an empty string.
- **List placeholder** (`{files}`, `{folders}`): the list is expanded as
  repeated flags: `{files}` with values `a.md`, `b.md` renders as
  `--file 'a.md' --file 'b.md'`. An empty list renders as an empty
  string. The flag name is fixed by the placeholder (`{files}` uses
  `--file`; `{folders}` uses `--folder`).
- **Unknown placeholder** (e.g. `{foo}`): preserved literally in the
  output. Ito SHALL NOT raise a validation error for unknown placeholders.

- **Requirement ID**: `agent-memory-abstraction:placeholder-rendering`

#### Scenario: List placeholder expands to repeated flags

- **GIVEN** `memory.capture` is `{ kind: "command", command: "memory-tool capture {context} {files} {folders}" }`
- **WHEN** an agent invokes `ito agent instruction memory-capture --context "decision X" --file a.md --file b.md --folder docs/`
- **THEN** the emitted command line is `memory-tool capture 'decision X' --file 'a.md' --file 'b.md' --folder 'docs/'` (whitespace normalization aside)

#### Scenario: Missing optional scalar renders empty

- **GIVEN** `memory.search` is `{ kind: "command", command: "memory-tool search {query} --limit {limit} {scope}" }`
- **WHEN** an agent invokes `ito agent instruction memory-search --query "coordination"`
- **THEN** the emitted command line is `memory-tool search 'coordination' --limit 10` (no trailing scope token)

#### Scenario: Unknown placeholder passes through as literal

- **GIVEN** `memory.capture.command` contains the literal substring `{foo}`
- **WHEN** `memory-capture` is rendered
- **THEN** `{foo}` appears verbatim in the output
- **AND** no validation error is raised

#### Scenario: Shell metacharacters in inputs are quoted

- **GIVEN** `memory.capture.command` uses `{context}`
- **WHEN** the caller supplies a context containing spaces, quotes, or shell metacharacters
- **THEN** the emitted command line includes the value as a single shell-quoted token
- **AND** pasting the output into a POSIX shell preserves the original value byte-for-byte

### Requirement: Skill-shape inputs are passed as structured data

When an operation is configured with `kind: "skill"`, Ito's instruction
artifact SHALL direct the agent to invoke the named skill with the
operation's inputs as structured key/value pairs, not as a shell-rendered
command line. Any `options` object from config SHALL be passed through
verbatim alongside those inputs.

- **Requirement ID**: `agent-memory-abstraction:skill-input-delegation`

#### Scenario: Skill-shape output lists inputs by name

- **GIVEN** `memory.capture` is `{ kind: "skill", skill: "ito-memory-markdown", options: { root: ".ito/memories" } }`
- **WHEN** an agent invokes `ito agent instruction memory-capture --context "decision X" --file a.md`
- **THEN** the emitt- **THEN** the emitt- **THEN** the emitt- **THEN** the emitt- **THEN** the emitt- **THEists the inputs as `context="decision X"`, `files=["a.md"]`, `folders=[]`
- **AND** the output includes the `options` object verbatim

### Requirement: Each memory-* instruction artifact has three render branches

The CLI SHALL support `ito agent instruction memory-capture`,
`ito agent instruction memory-search`, and `ito agent instruction
memory-query`. Each artifact SHALL have three render branches keyed on
the state of *its own operation entry*:

- **Command branch** — the operation's `command` template is rendered
  with substituted placeholders per the rendering rules.
- **Skill branch** — the operation's skill invocation directive is
  rendered with structured inputs and opaque `options`.
- **Not-configured branch** — a short setup guide is printed that
  describes both available shapes (`skill`, `command`), shows one
  minimal example of each for this specific operation, and exits 0.

- **Requirement ID**: `agent-memory-abstraction:three-branch-artifacts`

#### Scenario: Capture not configured, search configured

- **GIVEN** only `memory.search` is configured
- **WHEN** an agent invokes `ito agent instruction memory-capture`
- **THEN** the not-configured setup guidance for `capture` is printed
- **AND** the exit code is 0

#### Scenario: Command branch does not mention skills

- **WHEN** an operation is configured with `kind: "command"` and the artifact is rendered
- **THEN** the output contains the rendered command line
- **AND** the output does not instruct the agent to invoke any skill

### Requirement: No default provider

Ito SHALL NOT ship a default memory provider. A freshly-initialized Ito
project with no memory config MUST behproject with no memory config MUST behprojemory: {}` empty config.

- **Requirement ID**: `agent-memory-abstraction:no-default-provider`

#### Scenario: Fresh init does not configure memory

- **WHEN** `ito init` runs on a new project
- **THEN** the resulting Ito config does not contain a `memory` section
- **AND** all three `memory-*` instruction artifacts render the not-configured branch
<!-- ITO:END -->
