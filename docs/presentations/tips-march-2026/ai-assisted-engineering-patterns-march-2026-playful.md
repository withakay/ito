---
marp: true
theme: gaia
paginate: true
size: 16:9
title: AI Assisted Engineering Patterns
---

<!--
_class: lead
_backgroundColor: #0f172a
_color: #f8fafc
-->
# AI Assisted Engineering Patterns
## Tips & Tricks — March 2026

Some thoughts on how I like to optimise my projects and workflows for AI-assisted engineering.

---

<!--
_backgroundColor: #e0f2fe
_color: #0c4a6e
-->
# Use strong/strict typing

- For anything non-trivial that spreads beyond a handful of files, I want strong typing.
- Best case: a statically compiled language with proper type assertions.
- If not, get as close as you can with tools like:
  - **TypeScript**
  - **MyPy**
- Anything the compiler can check for me is one less thing left to guesswork.

---

<!--
_backgroundColor: #ecfccb
_color: #365314
-->
# Use boring/well-known technology

- By “boring” I mean battle-tested, stable, and slow to surprise you.
- It doesn't have to be glamorous. It has to keep working.
- LLMs have usually seen far more of this software in their training.
- They know how it behaves, how it breaks, and how people normally fix it.
- You give yourself a much better chance of shipping cleanly with less friction.
- You probably do not need the newest database that was vibe-coded in Rust last Tuesday.

---

<!--
_backgroundColor: #fae8ff
_color: #701a75
-->
# Define strict rules around code style and linting

- Turn linting up to full and put it on hard mode.
- Be explicit about nulls, magic strings, enums, and naming.
- Lean on the compiler and linter for deterministic correctness.
- If too much is left to the model, it will eventually make things up.
- Locked-down style also makes the code easier for people to read later.
- Strict rules are not busywork here; they are useful guardrails.

---

<!--
_backgroundColor: #fee2e2
_color: #7f1d1d
-->
# Do Red/Green Testing

- Think about the problem first.
- Write the tests that prove the behaviour you want.
- Watch them fail.
- Then build until they go green.
- In theory this always made sense.
- In practice I hardly ever worked this way before AI-assisted coding.

---

<!--
_backgroundColor: #fecaca
_color: #7f1d1d
-->
# Do Red/Green Testing

- Writing tests for code that does not exist used to feel miserable.
- Your IDE would just scream at you the whole time.
- Now the model can draft the tests and I can proofread the intent.
- That changes the cost/benefit calculation completely.
- The tests become a formal expression of what I actually mean.
- They are the specification, written in executable form.

---

<!--
_backgroundColor: #ede9fe
_color: #4c1d95
-->
# Test integrations end to end

- AI-assisted engineering makes weak test suites painfully obvious.
- Unit tests are fine for proving logic.
- They are nowhere near enough to prove the software really works.
- The best move is to embrace integration tests.
- Yes, they are harder to write.
- Yes, they can be flaky and annoying.

---

<!--
_backgroundColor: #ddd6fe
_color: #4c1d95
-->
# Test integrations end to end

- Historically, that effort often did not feel worth it.
- A bit of manual testing was usually quicker.
- That changes when the AI writes most of the ceremony for you.
- Test through the actual surfaces people use:
  - browser UI with **Playwright**
  - CLI flows through a **PTY**
- Once the heavy lifting is cheap, end-to-end coverage becomes much more attractive.

---

<!--
_backgroundColor: #c7d2fe
_color: #312e81
-->
# Test integrations end to end

- Stand up real dependencies where you can.
- I like to avoid mocking as much as possible.
- Test against real databases, key-value stores, and message queues.
- Emulators are useful; toy stand-ins usually are not.
- Use **Docker**, **Podman**, **Aspire**, or whatever fits the stack.
- The closer the test looks to reality, the more confidence it actually gives you.

---

<!--
_backgroundColor: #dcfce7
_color: #14532d
-->
# Use git hooks to enforce checks

- Use hooks to run the checks that keep you honest.
- Tests, linting, architecture rules, formatting — all fair game.
- My preference is lighter checks on commit and heavier checks on push.
- If the AI is going to make commits or raise PRs, do not rely on memory.
- Force the checks.
- **Prek**, **Husky**, or plain scripts will all do the job.

---

<!--
_backgroundColor: #fce7f3
_color: #831843
-->
# Use a Makefile

- I like a Makefile because it gives the project one obvious front door.
- Use something else if you like; the important bit is consistency.
- Make fits the boring technology rule perfectly.
- LLMs understand it and can run it without fuss.
- It also gives me one place to standardise the development lifecycle.

---

<!--
_backgroundColor: #fbcfe8
_color: #831843
-->
# Use a Makefile

```make
make init
make build
make run
make check
make test
```

- Running `make` should print a help summary of the supported targets.
- When I come back to a project later, that gives me an instant refresher.
- It also gives agents a standard set of commands to follow.

---

<!--
_backgroundColor: #cffafe
_color: #155e75
-->
# Context is King

- Managing an LLM's context window matters a lot more than people think.
- Every model has a limit, and providers expose different limits.
- The same model name does not always mean the same usable context.
- Some products also quote input and output together, which muddies the water.
- In practice, context is just another engineering constraint to manage.

---

<!--
_backgroundColor: #a5f3fc
_color: #155e75
-->
# Context is King

- Claude Sonnet 4.6 via Anthropic API: **200k** input tokens, with a **1M** option.
- Opus is moving to **1M** by default.
- GPT-5.4 via the OpenAI API also goes to **1M**.
- Copilot often exposes smaller windows, commonly **128k**, sometimes more.
- However the vendor markets it, the real question is simple:
- how much useful input can I fit in before the model gets sloppy?

---

<!--
_backgroundColor: #fef3c7
_color: #78350f
-->
# What even are tokens?

- A token is just input that has been recognised and encoded as a number.
- It is not exactly a word and not exactly a character.
- Common words, fragments, punctuation, and symbol pairs can all be tokens.
- Image inputs work on the same basic idea, just with different pattern encoding.
- For our purposes, text is the easiest way to think about it.
- If you want to visualise it, try **https://gpt-tokenizer.dev**.

---

<!--
_backgroundColor: #fde68a
_color: #78350f
-->
# Strategies for managing context

- The goal is not to cram everything in.
- The goal is to give the model exactly what it needs to solve the problem well.
- Token usage matters, but getting the answer right first time matters more.
- Be specific about constraints, acceptance criteria, and project facts.
- The less it has to infer, the less opportunity it has to drift.

---

<!--
_backgroundColor: #fcd34d
_color: #78350f
-->
# Strategies for managing context

- If the model has to go hunting for obvious information, you pay twice.
- First in tool use and search noise.
- Then again in all the irrelevant context it drags back with it.
- When context gets close to full, models usually get worse.
- Some harnesses try to summarise and reload context for you.
- That helps, but useful detail can get lost in the squeeze.

---

<!--
_backgroundColor: #f3e8ff
_color: #581c87
-->
# Agent Instructions

- Session instructions are useful, but very easy to overdo.
- I mean files like `AGENTS.md`, `CLAUDE.md`, and Copilot instruction files.
- Good content includes build steps, test commands, key config locations, and a few architecture notes.
- Copilot's file-glob approach is nice because it only loads guidance when it is relevant.
- The always-on instruction file should stay short and high-value.

---

<!--
_backgroundColor: #e9d5ff
_color: #581c87
-->
# Agent Instructions

- What I would not do is dump the whole universe into one instruction file.
- If `AGENTS.md` grows to a thousand lines, every session pays for that.
- Split deeper guidance into separate docs and link to them.
- Better still, enforce coding standards in the build so the model gets feedback where it needs it.
- These files need pruning just like the rest of the repo.
- If they get bloated, ask the model to help compress them.

---

<!--
_backgroundColor: #fff7ed
_color: #9a3412
-->
# Custom Agents and Sub Agents

- The terminology here is a bit of a mess.
- We used to talk about Copilot or Claude Code as “the agent”.
- Now those tools can contain multiple agents and sub-agents.
- The cleaner word for the outer tool is really **harness**.
- It is the wrapper that runs the loop, exposes tools, and carries out edits.
- That helps separate the tool from the prompt inside it.

---

<!--
_backgroundColor: #fed7aa
_color: #9a3412
-->
# Custom Agents and Sub Agents

- In this sense, an agent is mostly a persona plus a prompt.
- Usually it is a markdown file with instructions, metadata, and sometimes a preferred model.
- So you might have a quick-task agent on a small cheap model.
- And a deeper reasoning agent on GPT-5.4 with a larger thinking budget.
- You can also have specialised agents for research, testing, or commits.

---

<!--
_backgroundColor: #fdba74
_color: #9a3412
-->
# Custom Agents and Sub Agents

- Sub-agents are useful because they get their own context window.
- The calling agent can boil the task down to a tight prompt and hand it off.
- The sub-agent does the noisy work elsewhere and returns a summary.
- That is helpful, but it is not magic.
- A bad prompt still gives you a bad result.
- Sometimes the sub-agent just ends up rediscovering context you already had.

---

<!--
_backgroundColor: #dbeafe
_color: #1e3a8a
-->
# Skills

- Skills are very similar to agents, with one important twist: they are discoverable and lazy-loaded.
- The harness only needs the skill name and description up front.
- The full prompt gets loaded when the job actually calls for it.
- That means you can keep rich instructions without paying the token cost every session.
- In practice, skills are great for repeatable jobs like charts, tests, or code generation.

---

<!--
_backgroundColor: #bfdbfe
_color: #1e3a8a
-->
# Planning

- Most harnesses now offer some kind of planning mode.
- That is basically an instruction set telling the model to stop and think before it starts building.
- A good plan is a managed context file.
- The model can do the broad search work once, then distil what actually matters.
- You can iterate on the plan until most of the ambiguity is gone.
- That is usually a lot cheaper than cleaning up after a vague implementation.

---

<!--
_backgroundColor: #f0fdf4
_color: #166534
-->
# Spec Driven Development

- Writing a bespoke plan every single time is fine, until you notice the pattern repeating.
- That is where spec-driven — or what I now think of as change-driven — development gets useful.
- The basic idea is to turn a vague change request into a concrete artifact.
- That artifact gives the model something firmer to implement against.
- Kiro made this feel real for a lot of people in 2025.

---

<!--
_backgroundColor: #bbf7d0
_color: #166534
-->
# Spec Driven Development

- Since then we have seen a wave of tools around the same idea:
  - **Ultraspec**
  - **Speckit**
  - **OpenSpec**
  - **Ito**
- The names vary, but the move is the same.
- Formalise the change before coding it.

---

<!--
_backgroundColor: #86efac
_color: #166534
-->
# Spec Driven Development

- My usual workflow is pretty simple:
  1. propose a change
  2. answer a few clarifying questions
  3. generate a proposal
  4. turn that into a spec
  5. implement against the spec
- The proposal and the spec are just markdown files.

---

<!--
_backgroundColor: #dcfce7
_color: #166534
-->
# Spec Driven Development

```md
# Specification: Login Button Colour Change

## Functional Requirements
The system shall render the login button with a green background colour.

## Optional Requirements
The system shall use the approved green from the design palette.
```

---

<!--
_backgroundColor: #111827
_color: #f9fafb
-->
# Final takeaway

- Better AI-assisted engineering mostly comes from better guardrails.
- Strong typing, strict linting, and real tests all do useful work.
- Stable tooling beats novelty more often than not.
- Context needs managing just as carefully as code.
- Plans, specs, agents, and skills are there to reduce ambiguity.
- The less room you leave for guesswork, the better the results tend to be.
