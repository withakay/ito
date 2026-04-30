## Key points
- The Ito orchestration proposal was **consolidated into existing change `028-02_centralize-instruction-source-of-truth`** instead of creating a separate duplicate change.
- A new **`agent-surface-taxonomy`** was introduced to distinguish **direct entrypoint agents** from **delegated role sub-agents**.
- The consolidation makes **`ito agent instruction orchestrate`** the **authoritative source** for overlapping orchestration and multi-agent skills/prompts.
- The document explicitly scopes the consolidation around **overlapping orchestration and multi-agent instruction content** to reduce duplication.
- The affected files include **`.claude/agents/ito-general.md`**, **`.claude/agents/ito-orchestrator.md`**, **`.claude/skills/ito-orchestrate/`**, and **`.claude/skills/ito-test-with-subagent/`**.
- The decision is framed as a workflow-domain note with a clear **proposal → fold into existing change → taxonomy addition → authoritative source designation** flow.

## Structure / sections summary
- **Metadata block**: title, summary, tags, related, keywords, created/updated timestamps.
- **Reason**: explains the rationale for document consolidation into change 028-02 and the taxonomy effort.
- **Raw Concept**: lists the task, specific changes, impacted files, process flow, timestamp, author, and a regex pattern for the change identifier.
- **Narrative**
  - **Structure**: describes the distinction between entrypoint agents and delegated sub-agents.
  - **Dependencies**: notes reliance on the existing 028-02 change and overlap in orchestration-related content.
  - **Highlights**: emphasizes the authoritative orchestration source and duplication prevention.

## Notable entities, patterns, or decisions
- **Change identifier**: `028-02_centralize-instruction-source-of-truth`
- **Taxonomy name**: `agent-surface-taxonomy`
- **Authoritative source**: `ito agent instruction orchestrate`
- **Named agent surfaces**:
  - Entrypoint agents: `ito-general`, `ito-orchestrator`
  - Delegated sub-agents: `planner`, `researcher`, `worker`, `reviewer`, `test-runner`
- **Pattern used**: `^028-02_centralize-instruction-source-of-truth$`
- **Decision**: avoid duplication by folding the orchestration consolidation into an existing change rather than opening a parallel proposal.