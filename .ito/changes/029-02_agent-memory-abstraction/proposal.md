<!-- ITO:START -->
## Why

Agent memory (the ability for AI agents to store and recall project knowledge
across sessions) needs to be a first-class concern in Ito. Change
`029-01_add-byterover-integration` wires in one concrete provider, ByteRover,
but the wiring lives as loose skills and shell instructions rather than as a
supported Ito concept.

We want a loose, provider-agnostic abstraction so that:

- Ito itself knows *whether* a memory provider is configured, and *how* to
  invoke it (which commands/skills to use for **store** and **search** /
  **recall** operations).
- Agents can capture and recall memory without hard-coding any specific
  backend — swapping providers is a config change, not a code change.
- `apply` instructions remind the agent to capture useful memories at the end
  of a work session.
- `finish` instructions remind the agent to capture memories *and* to refresh
  archive / canonical specs as part of wrap-up.

There is **no default provider**. Until a user configures one, Ito's
memory-related instructions MUST degrade gracefully (e.g. print guidance that
memory is not configured, and how to configure it).

## What Changes

- **New**: `memory` configuration section in the Ito config. It defines the
  active provider as a small descriptor — either a pair of command templates
  (for `store` and `search`) or a reference to an installed skill that
  implements memory operations. See the new `agent-memory-abstraction` spec
  for exact shape. The field is **optional**; when absent, memory is
  considered "not configured".
- **New**: `ito agent instruction memory-capture` and
  `ito agent instruction memory-search` instruction artifacts. They render:
  (a) the configured provider's commands/skill invocation, or (b) graceful
  "not configured" guidance telling the agent how to configure memory.
- **New**: Apply instruction template (`apply.md.j2`) appends a "Capture
  memories" reminder section at the end that is rendered only when memory is
  configured. The reminder tells the agent to identify useful decisions /
  gotchas / patterns learned during the session and store them via the
  configured provider.
- **New**: Finish instruction template (`finish.md.j2`) appends the same
  memory-capture reminder **and** a "Refresh archive and specs" reminder
  section that instructs the agent to ensure `ito archive …` has been run,
  the canonical specs under `.ito/specs/` reflect the delivered change, and
  any agent-facing docs have been updated.
- **New**: Graceful-degradation behavior: when no provider is configured,
  `memory-capture` / `memory-search` instructions print provider-setup hints
  rather than erroring.
- **Modified**: `agent-instructions` spec — apply and finish instructions
  gain new ADDED requirements for memory reminders and finish wrap-up.

## Capabilities

### New Capabilities

- `agent-memory-abstraction`: Defines the provider-agnostic memory contract:
  config shape, instruction artifacts (`memory-capture`, `memory-search`),
  resolution rules for command-based vs skill-based providers, and
  graceful-degradation behavior when unconfigured.

### Modified Capabilities

- `agent-instructions`: Apply and finish instruction artifacts gain new
  ADDED requirements:
  - Apply instructions SHALL include a memory-capture reminder when memory
    is configured.
  - Finish instructions SHALL include both a memory-capture reminder and a
    wrap-up reminder to refresh archive and canonical specs.

## Impact

- `ito-config`: add a `MemoryConfig` type under the root config, with one
  of two shapes: `{ provider: "commands", store: <cmd>, search: <cmd> }` or
  `{ provider: "skill", skill: <skill-id> }`. Default is absent.
- `ito-core`: add `memory` module that resolves the configured provider,
  emits command templates or skill invocations for the two new instruction
  artifacts, and preserves the literal `{text}` / `{query}` placeholders for
  the agent to substitute at execution time.
- `ito-cli`: add `ito agent instruction memory-capture` and
  `ito agent instruction memory-search`.
- `ito-templates`: update `apply.md.j2` and `finish.md.j2` to include the
  new reminder sections behind a `memory.configured` Jinja guard. No
  changes to rendered output when memory is absent.
- JSON config schema updated to include `memory` section.
- No breaking changes to existing CLI commands, templates, or config keys.
  New config section is additive and optional.

## Open Questions

- Should the command-based provider support multi-step pipelines (e.g. an
  envelope that preprocesses text before calling `store`), or stay limited
  to a single command with `{text}`/`{query}` substitution? _Default for
  this change: single command, single placeholder. Revisit if real providers
  need more._
- Should `finish` refresh be an *executed* step (Ito runs an archive-update
  routine) or just a *reminder* to the agent? _This change: reminder only.
  Automated refresh is a potential follow-on._
<!-- ITO:END -->
