---
marp: true
theme: default
paginate: true
size: 16:9
title: AI Assisted Engineering Patterns
description: Tips & Tricks — March 2026
---

<!-- _class: lead -->
# AI Assisted Engineering Patterns
## Tips & Tricks — March 2026

Some thoughts on optimising projects and workflows for AI-assisted engineering.

---

# Core themes

- Optimise for **correctness** before convenience
- Prefer **predictability** over novelty
- Turn intent into **tests, rules, and specs**
- Be deliberate about **context management**
- Use **agents, skills, and planning** to scale work safely

---

# Use strong / strict typing

- Prefer strongly typed languages for non-trivial systems
- If dynamic, add type-checking where possible:
  - **TypeScript**
  - **MyPy** for Python
- Best fit in practice:
  - **Statically compiled**
  - **Strong type assertions**

> The more correctness you can push into the compiler, the better.

---

# Use boring / well-known technology

- "Boring" means:
  - battle-tested
  - stable
  - enterprise-ready
  - slow to change
- LLMs are usually much better trained on mature ecosystems
- Result:
  - fewer surprises
  - easier debugging
  - lower implementation friction

> Avoid betting the project on the newest shiny thing.

---

# Define strict code style & linting rules

- Turn linting up **aggressively**
- Remove ambiguity wherever possible
- Be explicit about:
  - null handling
  - magic strings
  - enums / constants
  - naming and formatting rules
- Strong rules help both humans and models

> What feels strict to humans feels like useful guardrails to an LLM.

---

# Do Red / Green testing

- Start with the problem you want to solve
- Write tests that prove the intended behaviour
- Watch them fail
- Implement until they go green

Why this works especially well with AI:
- AI can draft tests quickly
- You review intent instead of hand-writing everything
- Tests become a **formal expression of specification**

---

# Test integrations end to end

- Unit tests are useful, but **not sufficient**
- Prefer integration tests that prove the software actually works
- Test through real surfaces:
  - browser UI with **Playwright**
  - CLI via **PTY**
  - real services where possible
- Minimise mocking

> AI makes high-effort tests much more practical to produce.

---

# Favour real infrastructure in tests

- Stand up dependencies for test runs:
  - databases
  - key-value stores
  - message queues
  - external service emulators
- Use tooling like:
  - **Docker**
  - **Podman**
  - **Aspire**

Goal:
- prove the system works in realistic conditions
- catch problems unit tests will never see

---

# Use git hooks to enforce checks

- Enforce consistency before code lands
- Common checks:
  - linting
  - tests
  - architectural checks
  - formatting
- Typical preference:
  - lighter checks on **commit**
  - full checks on **push**

Tools:
- **Prek**
- **Husky**
- custom shell / PowerShell scripts

---

# Use a Makefile

Keep a consistent entry point for the development lifecycle.

Typical targets:
- `make init`
- `make build`
- `make run`
- `make check`
- `make test`

Benefits:
- predictable commands across projects
- easy for humans to rediscover
- easy for agents to run correctly
- pairs well with `AGENTS.md`, `CLAUDE.md`, etc.

---

# Context is king

- Every model run has a finite **context window**
- Context limits vary by model and provider
- Some providers expose smaller windows than the base API model
- Some quote **input + output** together, which can be misleading

Practical takeaway:
- context is a scarce resource
- use it intentionally
- avoid wasting it on irrelevant detail

---

# What even are tokens?

- Tokens are units of encoded input
- They are not exactly words or characters
- Common letter combinations, words, or symbols may share tokens
- Multimodal models tokenize images differently, but the idea is similar

Useful mental model:
> Tokens are the budget the model spends to read your world.

Reference:
- https://gpt-tokenizer.dev

---

# Strategies for managing context

Give the model the **right** context, not the **most** context.

- Provide specific criteria and constraints up front
- Avoid making the model search for obvious information
- Avoid forcing it to infer what you already know
- Reduce irrelevant tool output and noisy logs
- Keep prompts concise, explicit, and task-focused

When context fills up:
- quality drops
- reasoning degrades
- hallucinations increase

---

# Agent instruction files

Examples:
- `AGENTS.md`
- `CLAUDE.md`
- Copilot instruction files

Good things to include:
- how to run tests
- how to build and check the project
- key locations (`.env`, secrets, important docs)
- brief architecture guidance

Avoid:
- dumping every preference into one huge file
- repeating information the build can enforce automatically

---

# Keep instructions lean

Problems with bloated instruction files:
- thousands of tokens loaded every session
- repeated for sub-agents too
- lots of noise, little signal

Better approach:
- keep the main instruction file minimal
- link to focused supporting documents
- enforce coding standards in the build
- regularly prune and compress instructions

> Repository hygiene now includes prompt hygiene.

---

# Custom agents and sub-agents

Terminology matters:

- **Harness**
  - the program wrapping the LLM
  - provides tools, instructions, and the agentic loop
- **Agent**
  - effectively a specialised prompt/persona
  - may include model/tool configuration
- **Sub-agent**
  - an agent invoked by another agent for focused work

Examples:
- quick-task agent on a fast cheap model
- deep-thinking agent on a larger reasoning model
- specialised agents for research, tests, or commits

---

# Why sub-agents help with context

Sub-agents create a separate working context.

Process:
1. main agent distils the task into a focused prompt
2. sub-agent works inside its own context window
3. sub-agent returns a synthesised result

Benefits:
- isolates noisy outputs
- keeps the main context cleaner
- works well for well-defined tasks

Caveat:
- a poor prompt in means poor output back

---

# Skills

Skills are similar to agents, but **lazy-loaded**.

- A skill exposes metadata up front:
  - name
  - description
  - optional additional metadata
- The full skill content is loaded only when needed
- This makes skills efficient and discoverable

Example:
- a chart skill can advertise that it should be used for charts/graphs/plots
- only then does the harness load the full instructions

---

# Planning

Most harnesses offer a planning mode or planning agent.

Why planning matters:
- turns exploration into a managed context artifact
- extracts just the information needed for implementation
- reduces ambiguity before code is written
- gives you something concrete to review and iterate on

A good plan is often:
- shorter than the raw research
- clearer than an initial prompt
- safer than improvisation

---

# Spec-driven → change-driven development

Writing bespoke plans every time does not scale well.

Frameworks aim to make planning repeatable:
- **Kiro** popularised spec-driven workflows
- Related tools and ideas include:
  - **Ultraspec**
  - **Speckit**
  - **OpenSpec**
  - **Ito**

The broader idea:
- propose a change
- clarify intent
- generate a proposal/specification
- implement against that artifact

---

# Example change specification

```md
# Specification: Login Button Colour Change

## Functional Requirements
The system shall render the login button with a green background colour.

## Optional Requirements
Where a design system is defined,
the system shall use the approved green colour from the design palette.

Where accessibility requirements apply,
the system shall ensure the button colour meets contrast guidelines.

## Constraint Requirements
The system shall ensure that the colour change does not impact the login process.
```

---

# Final takeaways

- Let **types, linting, and tests** constrain the model
- Prefer **stable tools and repeatable workflows**
- Treat **context** as a critical engineering resource
- Use **plans, specs, agents, and skills** intentionally
- Convert vague intent into **enforceable artifacts**

## Better guardrails = better AI-assisted engineering
