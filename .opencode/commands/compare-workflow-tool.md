---
name: compare-workflow-tool
description: Analyze an AI coding workflow tool and compare its workflow to Ito.
category: Research
tags: [research, workflow, comparison, ito]
---

<ToolInput>
$ARGUMENTS
</ToolInput>

# Compare Workflow Tool

You are conducting a structured 3-phase analysis of an AI coding workflow tool
and comparing it to Ito. The <ToolInput> above contains the tool name and/or URL
to analyze.

If <ToolInput> is empty, ask the user for a tool name or URL and stop.

## Core Scope

- Focus only on the workflow the tool presents and how that workflow is
  implemented.
- Ignore AI model support, provider support, and which agents the tool supports.
- Prioritize real workflow behavior over marketing language.
- When documentation and implementation disagree, call out the mismatch.

## Output

Write all output to a single markdown file at:
`.local/docs/agents/workflow-comparison-<tool-name>.md`

Derive `<tool-name>` from the tool name, lowercased and hyphenated.

Create `.local/docs/agents/` if it does not exist.

Make the final document self-contained and structured with these major sections:

1. Tool under analysis
2. Phase 1 - Target tool workflow spec
3. Phase 2 - Ito workflow spec
4. Phase 3 - Comparative analysis
5. Adversarial review
6. Final recommendations for Ito

---

## Phase 1: Analyze the Target Tool's Workflow

Goal: Build a detailed spec of the target tool's development workflow.

1. Fetch the tool's repository or documentation from <ToolInput>. If only a
   name is given, find the canonical repository or documentation first.
2. Read the README, docs, examples, config files, prompts, templates, commands,
   and any workflow/process guides.
3. If the repo has code that implements the workflow, inspect that code to
   understand how the workflow actually works in practice.
4. Produce a structured workflow spec covering:

   - Workflow overview
   - Phases or stages in execution order
   - Artifacts created or updated in each phase
   - State transitions, triggers, and gates
   - Automation vs manual steps
   - Feedback loops, review loops, and recovery paths
   - Configuration and extensibility mechanisms
   - Implementation notes: which files, commands, prompts, or code paths enforce
     the workflow

5. Produce a Mermaid diagram representing the workflow.

   - Prefer `stateDiagram-v2` when the workflow behaves like a state machine.
   - Use `flowchart TD` or `flowchart LR` when the workflow is more linear or
     pipeline-oriented.
   - Show phases, transitions, decision points, feedback loops, and notable
     artifacts.
   - Keep the diagram readable; split into sub-diagrams if necessary.

6. End Phase 1 with a short list of the target tool's defining workflow ideas.

---

## Phase 2: Produce an Equivalent Spec for Ito

Goal: Document Ito's workflow using the same structure as Phase 1.

1. Use Ito workflow instructions and source material to understand the intended
   and implemented workflow.
2. Prefer these sources when relevant:

   - `.ito/AGENTS.md`
   - `.opencode/commands/`
   - `.opencode/skills/`
   - `ito agent instruction schemas`
   - `ito agent instruction project-setup`
   - `ito agent instruction proposal --change <existing-change-id>`
   - `ito agent instruction apply --change <existing-change-id>`
   - `ito agent instruction review --change <existing-change-id>`
   - `ito agent instruction archive --change <existing-change-id>`
   - `ito agent instruction finish --change <existing-change-id>`
   - `ito-rs/crates/ito-core/`
   - `ito-rs/crates/ito-cli/`
   - `ito-rs/crates/ito-templates/`

3. If a change ID is needed, pick an existing change from the repo rather than
   inventing one.
4. Produce the same structured workflow spec as in Phase 1, covering:

   - Workflow overview
   - Phases or stages in execution order
   - Artifacts created or updated in each phase
   - State transitions, triggers, and gates
   - Automation vs manual steps
   - Feedback loops, review loops, and recovery paths
   - Configuration and extensibility mechanisms
   - Implementation notes: which files, commands, prompts, or code paths enforce
     the workflow

5. Produce a Mermaid diagram for Ito using the same diagram style used in Phase
   1 unless a different style is clearly easier to read.
6. End Phase 2 with a short list of Ito's defining workflow ideas.

---

## Phase 3: Comparative Analysis

Goal: Compare the two workflow specs and identify improvement opportunities for
Ito.

Produce all of the following:

1. A side-by-side phase mapping table that aligns the target tool's phases to
   Ito's equivalent phases, highlighting gaps and mismatches.
2. A Mermaid comparison diagram.

   - Prefer `flowchart LR` with two parallel lanes when that is readable.
   - Highlight shared phases, tool-specific phases, divergence points, and loop
     behavior.
   - Make gaps obvious at a glance.

3. Strengths of the target tool's workflow.
4. Strengths of Ito's workflow.
5. Specific improvement opportunities for Ito, each with:

   - What the improvement is
   - Why it matters
   - What it would look like in Ito's workflow
   - Estimated complexity: low, medium, or high
   - Likely Ito components to change

6. Anti-patterns to avoid adopting from the target tool.

---

## Adversarial Review

After drafting the initial comparison, run an adversarial review using a
multi-agent workflow.

Use the Task tool to create an explicit adversarial review with at least these
roles in parallel:

1. `Conservative reviewer`
   - Assume Ito should change as little as possible.
   - Challenge every proposed improvement.
   - Look for hidden complexity, workflow bloat, migration cost, and accidental
     regressions.

2. `Reform reviewer`
   - Argue for adopting the strongest workflow ideas from the target tool.
   - Push for clearer phases, stronger automation, better validation, or better
     user guidance where evidence supports it.

3. `Synthesis reviewer`
   - Reconcile the competing arguments.
   - Keep only recommendations that survive scrutiny from both sides.
   - Separate strong recommendations from speculative ideas.

Preferred execution:

- If a multi-agent orchestrator is available, use it to coordinate the review.
- Otherwise, dispatch parallel subagents manually and synthesize their outputs.
- Keep the adversarial review evidence-based and tied to the documented workflow
  behavior of both tools.

The final document must include:

- A short section summarizing the adversarial positions
- Recommendations that were rejected and why
- Recommendations that survived adversarial review and why

---

## Execution Notes

- Run all three phases in one uninterrupted pass.
- Use parallel research where it helps, especially when gathering evidence from
  the target tool and Ito simultaneously.
- Prefer repository sources and implementation files over summaries when they
  are available.
- Call out uncertainty explicitly when evidence is weak.
- All Mermaid diagrams must be valid fenced code blocks using the `mermaid`
  language tag.
- The final markdown should be readable on its own without requiring the reader
  to open the source repositories.
