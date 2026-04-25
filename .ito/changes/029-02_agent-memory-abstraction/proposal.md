<!-- ITO:START -->
## Why

Ito needs a first-class but loosely-coupled concept of agent memory so that
agents can store and recall project knowledge across sessions without every
skill or instruction template hard-coding a specific backend.

Change `029-01_add-byterover-integration` wires ByteRover in as a concrete
installation, but everything about the integration (which CLI, which flags,
which skill, where content is written) currently lives in authored prose.
We want the provider to be *swappable*: ByteRover today, QMD or a plain
markdown-to-folder backend tomorrow, without changing instruction templates
or skills that consume memory.

The chosen abstraction is a small, brv-shaped API surface exposed as Ito
agent instruction artifacts (`memory-capture`, `memory-search`,
`memory-query`) that **delegate** to whatever the user has configured for
each operation.

There is **no default provider**. Until the user configures one, Ito's
memory instructions MUST degrade gracefully per operation (emit provider
setup guidance, exit 0, never error).

## What Changes

- **New**: `memory` section in the Ito config with three independent entries
  keyed by operation name — `capture`, `search`, `query`. Each entry is
  optional and picks one of two shapes:
  - `{ "kind": "skill", "skill": "<skill-id>", "options": { … } }` —
    delegate the operation to an installed skill. `options` is an opaque map
    passed through to the skill (e.g. a target folder path for a markdown
    backend).
  - `{ "kind": "command", "command": "<template>" }` — render a shell
    command line by substituting the operation's placeholders (see below).

  Operations can be mixed and matched (e.g. `capture` via a skill that
  writes markdown to a folder, `search` via an inline `rg` command).

- **New**: Three Ito agent instruction artifacts, one per operation, each
  with its own input schema matching the brv CLI surface:

  | Artifact | Inputs | Placeholders in `command` form |
  | --- | --- | --- |
  | `ito agent instruction memory-capture` | `context` (string), `--file` / `--folder` (repeatable) | `{context}`, `{files}`, `{folders}` |
  | `ito agent instruction memory-search`  | `query` (string), `--limit`, `--scope`                 | `{query}`, `{limit}`, `{scope}`    |
  | `ito agent instruction memory-query`   | `query` (string)                                       | `{query}`                          |

- **New**: Placeholder rendering rules for `command` form (so the
  instruction output is executable as-is, not further templated by the
  agent):

  - **Scalar string** (`{context}`, `{query}`, `{scope}`): substituted with
    the caller-supplied value as a single shell-quoted token. If the caller
    did not supply the input, the placeholder expands to an empty string.
  - **Scalar number** (`{limit}`): substituted as its integer literal. If
    the caller did not supply the input and the operation defines a default
    (e.g. `{limit}` default = 10), the default is used; otherwise the
    placeholder expands to an empty string.
  - **List** (`{files}`, `{folders}`): expanded to a repeated-flag list —
    `{files}` with two files `a.md`, `b.md` renders as
    `--file 'a.md' --file 'b.md'`. Empty list renders as empty string.
  - **Unknown placeholder**: preserved literally in the output (no
    validation error).

- **New**: Graceful degradation — each artifact checks *only the operation
  it exposes*. A project that configures `capture` only will have a working
  `memory-capture` artifact; `memory-search` and `memory-query` will print
  provider-setup guidance for their respective operations.

- **New**: Apply instruction template (`apply.md.j2`) appends a
  "Capture memories" reminder when `memory.capture` is configured. The
  reminder points the agent at `ito agent instruction memory-capture` and
  lists what to capture (decisions, gotchas, non-obvious patterns).

- **New**: Finish instruction template (`finish.md.j2`) appends:
  1. The same memory-capture reminder (when `memory.capture` is configured).
  2. A "Refresh archive and specs" reminder that always renders, always
     includes the specs/doc checks, and includes the archive-confirmation
     check only when the existing finish archive prompt is suppressed
     because the change is already archived. The archive step SHALL NOT be
     double-prompted alongside the existing finish archive prompt.

- **Modified**: `agent-instructions` spec gains ADDED requirements for the
  apply/finish reminders described above.

## Capabilities

### New Capabilities

- `agent-memory-abstraction`: Provider-agnostic memory contract. Specifies
  the three operation artifacts, the per-operation config shape (skill |
  command, independent per op), placeholder rendering rules for the
  command form, graceful per-op degradation, and the no-default-provider
  rule.

### Modified Capabilities

- `agent-instructions`: Apply and finish instruction artifacts gain new
  ADDED requirements for the memory-capture reminder and the finish
  wrap-up reminder (archive + specs + docs refresh).

## Impact

- `ito-config`: add a `MemoryConfig` type — `{ capture: Option<Op>, search: Option<Op>, query: Option<Op> }` where
  `Op` is a `#[serde(tag = "kind")]` enum with variants `skill { skill: String, options: Option<serde_json::Value> }` and `command { command: String }`. Default for the whole section is absent.
- `ito-core`: add a `memory` module that loads the operation config,
  applies the placeholder rendering rules for the command form, and
  emits a `RenderedInstruction` for each operation (either a literal
  command line or a "invoke skill X with these structured inputs"
  directive for the skill form).
- `ito-cli`: three new instruction artifacts — `memory-capture`,
  `memory-search`, `memory-query`. Each accepts the inputs listed in the
  table above via flags, invokes the resolver, and prints the result.
- `ito-templates`: update `apply.md.j2` and `finish.md.j2` with the new
  reminder sections (Jinja-guarded on per-op configuration).
- JSON config schema updated to include `memory` with the per-op shape.
- No breaking changes to existing CLI commands, templates, or config
  keys. New config section and new instruction artifacts are additive
  and optional.

## Open Questions

- Should `options` passed to a skill have a named, validated sub-schema
  (e.g. `root_dir`, `tags`) rather than opaque JSON? _This change: opaque
  JSON. Revisit once we have a second provider skill to shape the
  schema._
- Should finish *run* an archive/spec refresh routine automatically or
  stay as a reminder? _This change: reminder only. Automation is a
  potential follow-on once we see how the reminder plays out in practice._
<!-- ITO:END -->
