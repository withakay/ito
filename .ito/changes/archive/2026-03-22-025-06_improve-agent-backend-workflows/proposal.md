## Why

Even with repository-backed persistence, agents will continue to fail if their instructions still bias them toward editing markdown files directly. In remote/API-backed mode, the practical authoring surface has to be the CLI and its repository-aware commands, and the prompts/skills need to make that easy and explicit.

## What Changes

- Update backend-aware agent instructions, skills, and prompt surfaces so agents use CLI/repository-backed operations instead of editing markdown files directly.
- Define the expected behavior when active-work markdown does not exist locally in remote/API-backed mode.
- Identify and improve the CLI surfaces agents need most often so using the proper interface is practical.
- Add clear guidance for when Git projections are for scanning/backup versus when mutations must go through CLI/repository-backed paths.

## Impact

- Affected specs: `backend-agent-instructions`, `agent-instructions`
- Affected code/docs: agent instructions, skill prompts, backend-mode usage guidance, possibly CLI help text for repository-backed authoring flows
- Behavioral change: agent guidance becomes explicitly CLI-first in remote/API-backed mode and discourages direct markdown editing

## Execution Guidance

- Start after the main repository/CLI surfaces are stable enough to document accurately.
- In practice, this should follow `025-01_wire-change-repository-backends`, `025-02_wire-task-repository-backends`, `025-03_wire-module-repository-backends`, and `025-05_mirror-specs-and-archives-to-backend`.
- This change is intentionally later so the guidance does not get ahead of the actual interfaces.
