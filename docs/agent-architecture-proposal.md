# Ito Agent Architecture Proposal

> Tool-agnostic agent system with configurable models, context sizes, and prompts

## Overview

This proposal defines an agent architecture for Ito that:

- Works across **OpenCode**, **Codex CLI**, **Claude Code**, and other AI tools
- Uses **file-based configuration** for models, context sizes, and prompts
- Supports **any AI model** (Claude, GPT, Gemini, Llama, etc.)
- Enables **progressive enhancement** (parallel execution where supported)

## Design Principles

### 1. Tool-Agnostic

All agent definitions are markdown files that any AI tool can load and execute.

### 2. Model-Flexible

Configure which model to use per agent type. Not all models have the same capabilities or context windows.

### 3. Context-Aware

Specify context budgets per agent. A 32k model needs different chunking than a 200k model.

### 4. File-Based State

All coordination happens through files, not tool-specific APIs.

______________________________________________________________________

## Configuration System

### Master Configuration File

**`ito/config.yaml`** (or `config.json`)

```yaml
# Ito Agent Configuration
# Configures models, context sizes, and behavior for different AI tools

version: "1.0"

# Default settings (can be overridden per-agent)
defaults:
  model: "auto"                    # Use tool's default model
  context_budget: 100000           # Default context budget in tokens
  timeout: 300                     # Seconds before timing out
  retry_count: 2                   # Retries on failure

# Tool-specific settings
tools:
  opencode:
    default_model: "claude-sonnet" # Or "gpt-4o", "gemini-pro", etc.
    models:
      fast: "claude-haiku"         # For quick tasks
      balanced: "claude-sonnet"    # Default
      powerful: "claude-opus"      # For complex reasoning
    context_limits:
      claude-haiku: 200000
      claude-sonnet: 200000
      claude-opus: 200000
      gpt-4o: 128000
      gpt-4o-mini: 128000
      gemini-pro: 1000000
      llama-3: 8000

  codex:
    default_model: "gpt-4o"
    models:
      fast: "gpt-4o-mini"
      balanced: "gpt-4o"
      powerful: "o1"
    context_limits:
      gpt-4o: 128000
      gpt-4o-mini: 128000
      o1: 200000
      codex-max: 200000

  claude-code:
    default_model: "sonnet"
    models:
      fast: "haiku"
      balanced: "sonnet"
      powerful: "opus"
    context_limits:
      haiku: 200000
      sonnet: 200000
      opus: 200000

# Agent-specific configurations
agents:
  # Research agents - benefit from web search, need moderate context
  research:
    model_preference: "balanced"
    context_budget: 50000          # Research outputs should be concise
    requires:
      - web_search
      - file_read

  # Execution agents - need full context for code changes
  execution:
    model_preference: "balanced"
    context_budget: "max"          # Use full available context
    requires:
      - file_read
      - file_write
      - bash

  # Review agents - adversarial, benefit from reasoning
  review:
    model_preference: "powerful"   # Better reasoning for security review
    context_budget: 80000
    requires:
      - file_read

  # Planning agents - need reasoning but not huge context
  planning:
    model_preference: "balanced"
    context_budget: 60000
    requires:
      - file_read
      - file_write

# Context management strategies
context_strategy:
  # When context exceeds budget
  overflow_handling: "summarize"   # Options: summarize, truncate, error

  # Files always loaded (if they exist)
  always_include:
    - "ito/planning/STATE.md"
    - "ito/planning/PROJECT.md"

  # Files to prioritize when space is limited
  priority_files:
    - "ito/planning/ROADMAP.md"
    - "ito/research/SUMMARY.md"
```

### Environment-Based Overrides

Support environment variables for CI/CD and different setups:

```bash
# Override model for all agents
export ITO_MODEL="gpt-4o"

# Override context budget
export ITO_CONTEXT_BUDGET=32000

# Force specific tool behavior
export ITO_TOOL="opencode"
```

______________________________________________________________________

## Agent Definitions

Agents are defined as markdown prompt files with YAML frontmatter for configuration.

### Agent File Structure

```
ito/
├── agents/                      # Agent prompt definitions
│   ├── research/
│   │   ├── stack-researcher.md
│   │   ├── feature-researcher.md
│   │   ├── architecture-researcher.md
│   │   ├── pitfall-researcher.md
│   │   └── synthesizer.md
│   ├── planning/
│   │   ├── requirements-extractor.md
│   │   ├── roadmapper.md
│   │   └── task-planner.md
│   ├── execution/
│   │   ├── task-executor.md
│   │   └── verifier.md
│   └── review/
│       ├── security-adversary.md
│       ├── scale-adversary.md
│       ├── edge-case-adversary.md
│       └── review-synthesizer.md
```

### Agent Prompt Format

**`ito/agents/research/stack-researcher.md`**

````markdown
---
# Agent Configuration (YAML frontmatter)
name: stack-researcher
category: research
model_preference: balanced      # fast | balanced | powerful
context_budget: 50000           # tokens (or "max" for full)
timeout: 180                    # seconds

# Required capabilities
requires:
  - web_search
  - file_read

# Input files (loaded into context)
inputs:
  required:
    - "ito/planning/PROJECT.md"
  optional:
    - "ito/planning/STATE.md"
    - "ito/research/SUMMARY.md"

# Output specification
outputs:
  file: "ito/research/investigations/stack-analysis.md"
  append: false
---

# Stack Researcher Agent

## Role
You are a technology researcher evaluating stack choices for a software project.

## Objective
Research and evaluate technology options for the specified domain, providing actionable recommendations.

## Process

1. **Understand Requirements**
   - Read PROJECT.md for project vision and constraints
   - Identify key technical requirements from the domain

2. **Research Current Landscape**
   - Search for current best practices (2024-2025)
   - Evaluate popular libraries and frameworks
   - Check GitHub stars, npm downloads, maintenance status

3. **Evaluate Options**
   - Compare 3-5 viable options
   - Consider: maturity, community, performance, learning curve
   - Note any security concerns or known issues

4. **Make Recommendations**
   - Provide clear primary recommendation with rationale
   - List alternatives for different constraints

## Output Format

Write to `{output_file}`:

```markdown
# Stack Analysis: [Domain]

Generated: [date]
Model: [model used]

## Requirements
- [Key technical requirements derived from project]

## Options Evaluated

| Option | Pros | Cons | Maturity | Recommendation |
|--------|------|------|----------|----------------|
| [Lib1] | ... | ... | High/Med/Low | Primary / Alternative / Avoid |

## Primary Recommendation
**[Choice]**

Rationale: [Why this is the best fit]

## Alternatives
- **[Option 2]**: Use if [specific constraint]
- **[Option 3]**: Use if [different constraint]

## Risks
- [Risk]: [Mitigation]

## References
- [Links to documentation, benchmarks, etc.]
````

## Important Notes

- Focus on practical, production-ready options
- Consider the project's stated constraints
- Be specific about trade-offs, not vague
- Include version numbers where relevant

````

### Agent with Model Override

**`ito/agents/review/security-adversary.md`**

```markdown
---
name: security-adversary
category: review
model_preference: powerful       # Security review benefits from stronger reasoning
context_budget: 80000
timeout: 300

requires:
  - file_read

inputs:
  required:
    - "ito/changes/{change_id}/proposal.md"
    - "ito/changes/{change_id}/specs/**/*.md"
  optional:
    - "ito/specs/**/*.md"  # Existing specs for context

outputs:
  file: "ito/changes/{change_id}/REVIEW.md"
  append: true                   # Multiple reviewers append to same file
  section: "## Security Review"
---

# Security Adversary Agent

## Role
You are a security researcher performing adversarial analysis. Your job is to find vulnerabilities, not validate the design.

## Perspective
Assume attackers are sophisticated and motivated. Think like a malicious actor trying to exploit this system.

## Process

1. **Understand the Attack Surface**
   - Read the proposal and spec changes
   - Identify all user inputs, API endpoints, data flows
   - Map trust boundaries

2. **Systematic Vulnerability Search**

   Check for each category:

   **Authentication & Authorization**
   - [ ] Can auth be bypassed?
   - [ ] Are there privilege escalation paths?
   - [ ] Session management issues?

   **Injection Attacks**
   - [ ] SQL injection points?
   - [ ] XSS vulnerabilities?
   - [ ] Command injection?
   - [ ] Path traversal?

   **Data Security**
   - [ ] Sensitive data exposure?
   - [ ] Insecure data storage?
   - [ ] Insufficient encryption?

   **API Security**
   - [ ] Rate limiting gaps?
   - [ ] CSRF vulnerabilities?
   - [ ] SSRF possibilities?

3. **Rate and Document Findings**
   - HIGH: Exploitable with significant impact
   - MEDIUM: Exploitable with moderate impact or hard to exploit
   - LOW: Minor issues or theoretical

## Output Format

Append to `{output_file}` under `## Security Review`:

```markdown
## Security Review

Reviewed: [date]
Model: [model used]
Reviewer: security-adversary

### Issues Found

#### HIGH: [Issue Title]
- **Location**: [file:line or component]
- **Attack Vector**: [How an attacker would exploit this]
- **Impact**: [What damage could occur]
- **Mitigation**: [Required fix]
- **Status**: [ ] Addressed

#### MEDIUM: [Issue Title]
...

### Recommendations
- [Proactive security improvements not tied to specific issues]

### Areas Not Reviewed
- [Any areas skipped due to context limits or scope]
````

## Important Notes

- Be thorough but practical—focus on real risks
- Don't flag theoretical issues without exploitation path
- Suggest specific mitigations, not vague advice
- If you can't find issues, say so—don't invent them

````

---

## Context Management

### Context Budget Strategies

Different models have different context windows. The system adapts:

```yaml
# In config.yaml
context_strategy:
  # For small context models (8k-32k)
  small_context:
    max_file_size: 5000          # Truncate large files
    max_files: 5                  # Limit loaded files
    summarize_threshold: 3000    # Summarize files over this size

  # For medium context models (64k-128k)
  medium_context:
    max_file_size: 20000
    max_files: 15
    summarize_threshold: 10000

  # For large context models (200k+)
  large_context:
    max_file_size: 50000
    max_files: 50
    summarize_threshold: 30000
````

### Dynamic Context Loading

The system loads context based on available budget:

```
Available Context: [model's limit] - [system prompt] - [buffer]
                 = Usable Context Budget

Loading Priority:
1. Always-include files (STATE.md, PROJECT.md)
2. Required inputs from agent definition
3. Optional inputs (if space remains)
4. Related files (if space remains)
```

### Context Overflow Handling

When content exceeds budget:

1. **Summarize** (default): Generate summary of large files
1. **Truncate**: Keep first N tokens with "\[truncated\]" marker
1. **Error**: Fail and request manual intervention

______________________________________________________________________

## Execution Patterns

### Pattern 1: Sequential Execution (All Tools)

Works with any AI tool, including those without subagent support:

```
┌─────────────────────────────────────────┐
│ Load config.yaml                        │
│ Determine model and context budget      │
└───────────────┬─────────────────────────┘
                │
                ▼
┌─────────────────────────────────────────┐
│ For each agent in workflow:             │
│   1. Load agent prompt file             │
│   2. Load required input files          │
│   3. Execute with configured model      │
│   4. Write output to specified file     │
│   5. Update STATE.md                    │
└─────────────────────────────────────────┘
```

### Pattern 2: Parallel Execution (Tools with Subagent Support)

For Claude Code or tools with parallel capabilities:

```
┌─────────────────────────────────────────┐
│ Orchestrator (minimal context)          │
│ - Loads workflow definition             │
│ - Identifies parallelizable agents      │
└───────────────┬─────────────────────────┘
                │
    ┌───────────┼───────────┐
    │           │           │
    ▼           ▼           ▼
┌───────┐   ┌───────┐   ┌───────┐
│Agent 1│   │Agent 2│   │Agent 3│
│(fresh)│   │(fresh)│   │(fresh)│
└───┬───┘   └───┬───┘   └───┬───┘
    │           │           │
    ▼           ▼           ▼
 output1.md  output2.md  output3.md
    │           │           │
    └───────────┼───────────┘
                │
                ▼
┌─────────────────────────────────────────┐
│ Orchestrator collects outputs           │
│ Proceeds to next wave                   │
└─────────────────────────────────────────┘
```

### Pattern 3: Hybrid (Adaptive)

Detect tool capabilities and adapt:

```yaml
# Workflow definition with parallelization hints
workflow: research
steps:
  - wave: 1
    parallel: true              # Can run in parallel if supported
    agents:
      - stack-researcher
      - feature-researcher
      - pitfall-researcher

  - wave: 2
    parallel: false             # Must be sequential
    agents:
      - synthesizer            # Depends on wave 1 outputs
```

______________________________________________________________________

## Workflow Definitions

Workflows orchestrate multiple agents:

**`ito/workflows/research.yaml`**

```yaml
name: research
description: Complete domain research before proposal

# Model preference for this workflow (overrides agent defaults)
model_preference: balanced

waves:
  - name: investigation
    parallel: true
    agents:
      - stack-researcher
      - feature-researcher
      - architecture-researcher
      - pitfall-researcher

  - name: synthesis
    parallel: false
    agents:
      - synthesizer
    inputs:
      # Explicitly pass outputs from wave 1
      - "ito/research/investigations/*.md"

outputs:
  - "ito/research/SUMMARY.md"

on_complete:
  - update: "ito/planning/STATE.md"
    action: append
    content: |
      ### Research Complete
      - Completed: {date}
      - See: ito/research/SUMMARY.md
```

**`ito/workflows/review.yaml`**

```yaml
name: adversarial-review
description: Red team review of change proposal

parameters:
  change_id:
    required: true
    description: The change ID to review

waves:
  - name: adversarial
    parallel: true
    agents:
      - security-adversary
      - scale-adversary
      - edge-case-adversary

  - name: synthesis
    parallel: false
    agents:
      - review-synthesizer

outputs:
  - "ito/changes/{change_id}/REVIEW.md"

# Gate: block implementation if high severity issues
gate:
  condition: "no HIGH severity issues unaddressed"
  on_fail: "block"
```

______________________________________________________________________

## Tool Integration

### OpenCode Integration

```bash
# Copy agents to OpenCode commands directory
cp -r ito/agents .opencode/commands/ito-agents

# Copy workflows
cp -r ito/workflows .opencode/workflows

# Run a workflow
/ito-agents/research/stack-researcher "authentication"

# Or run full workflow
/workflows/research
```

OpenCode config (`.opencode/config.yaml`):

```yaml
commands:
  directories:
    - .opencode/commands
    - ito/agents          # Direct access to agents

# Model configuration
models:
  default: claude-sonnet
  aliases:
    fast: claude-haiku
    balanced: claude-sonnet
    powerful: claude-opus
```

### Codex CLI Integration

```bash
# Load agent as context
codex --context @ito/agents/research/stack-researcher.md \
      "Research authentication stack options"

# Or use the prompt directly
codex "$(cat ito/agents/research/stack-researcher.md)"
```

### Claude Code Integration

```bash
# Use as slash command
/ito:research stack

# Or reference in conversation
"Follow the agent prompt in ito/agents/research/stack-researcher.md"
```

CLAUDE.md integration:

```markdown
## Custom Commands

### /ito:research [type]
Execute research agent. Types: stack, features, architecture, pitfalls, all

### /ito:review [change-id]
Run adversarial review on change proposal.
```

______________________________________________________________________

## CLI Commands

The `ito` CLI manages configuration and workflows:

```bash
# Configuration
ito config show                    # Display current config
ito config set defaults.model gpt-4o
ito config set agents.research.context_budget 40000

# Agent management
ito agent list                     # List available agents
ito agent show security-adversary  # View agent details
ito agent run stack-researcher     # Run single agent
ito agent run stack-researcher --model gpt-4o  # Override model

# Workflow execution
ito workflow list                  # List workflows
ito workflow run research          # Run research workflow
ito workflow run review --change-id add-auth  # With parameters

# Context analysis
ito context estimate research      # Estimate context usage
ito context check stack-researcher # Verify fits in budget
```

______________________________________________________________________

## Configuration Examples

### Minimal Setup (Defaults)

```yaml
# ito/config.yaml
version: "1.0"
defaults:
  model: "auto"
```

### Small Model Setup (e.g., Llama 8B)

```yaml
version: "1.0"
defaults:
  model: "llama-3-8b"
  context_budget: 6000           # Conservative for 8k model

context_strategy:
  overflow_handling: "summarize"
  small_context:
    max_file_size: 2000
    max_files: 3
```

### Mixed Model Setup

```yaml
version: "1.0"
defaults:
  context_budget: 100000

agents:
  research:
    model_preference: "fast"     # Use cheap model for research
    context_budget: 30000

  review:
    model_preference: "powerful" # Use best model for security
    context_budget: 80000

  execution:
    model_preference: "balanced"
    context_budget: "max"
```

### Enterprise Setup (Multiple Providers)

```yaml
version: "1.0"

tools:
  opencode:
    models:
      fast: "gpt-4o-mini"        # OpenAI for speed
      balanced: "claude-sonnet"   # Anthropic for balance
      powerful: "o1"             # OpenAI for reasoning
    context_limits:
      gpt-4o-mini: 128000
      claude-sonnet: 200000
      o1: 200000

agents:
  research:
    model_preference: "fast"
  review:
    model_preference: "powerful"
  execution:
    model_preference: "balanced"
```

______________________________________________________________________

## Summary

| Feature | Benefit |
|---------|---------|
| **File-based agents** | Works with any AI tool |
| **Configurable models** | Use right model for each task |
| **Context budgets** | Supports 8k to 1M+ context models |
| **YAML frontmatter** | Agent config co-located with prompt |
| **Workflow orchestration** | Coordinate multiple agents |
| **Progressive enhancement** | Parallel when supported, sequential always works |

______________________________________________________________________

## References

- [OpenCode](https://opencode.ai/) - Primary target
- [Codex CLI](https://github.com/openai/codex) - OpenAI's coding agent
- [Claude Code](https://docs.anthropic.com/claude-code) - Anthropic's CLI
- [GSD](https://github.com/glittercowboy/get-shit-done) - Context engineering patterns
