<!-- ITO:START -->
## Context

`029-02_agent-memory-abstraction` added `memory-capture`, `memory-search`, and `memory-query` instruction artifacts plus configurable command/skill providers. The artifacts work, but discovery is weak: agents have to know the artifact names already, and no default installed skill describes when and how to use them.

## Goals / Non-Goals

- Goals:
  - Install a single `ito-memory` skill with the rest of Ito's shared skills.
  - Keep the skill provider-agnostic and route all operations through `ito agent instruction memory-*`.
  - Make the memory artifacts visible in CLI help and examples.
  - Keep this as a small follow-up that can be iterated later.
- Non-Goals:
  - Add a default memory provider.
  - Add three separate memory skills.
  - Add a user-facing top-level memory command.
  - Change the existing memory config shape.

## Decisions

- Decision: Use one `ito-memory` skill instead of separate operation skills.
  - Why: Agents need one mental model for memory. The operation split already exists at the CLI instruction layer.
- Decision: Make the skill a template asset under `assets/skills/`.
  - Why: Shared skills are already installed across harnesses by `ito init` and `ito update`.
- Decision: Update `ito agent instruction --help` directly.
  - Why: This is the canonical LLM-facing discovery surface for instruction artifacts.

## Risks / Trade-offs

- The skill can only guide agents; it does not enforce capture/search/query usage.
  - Mitigation: Keep apply/finish reminders and make the help surface explicit.
- The skill can mention provider setup, but providers are still optional.
  - Mitigation: Route all concrete work through existing not-configured branches.
<!-- ITO:END -->
