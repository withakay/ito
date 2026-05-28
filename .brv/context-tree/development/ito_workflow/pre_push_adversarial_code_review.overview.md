## Key points
- Ito agents should run an **adversarial code review before pushing** a change branch or opening a PR.
- The review acts as a **pre-push quality gate** to catch **low-hanging fruit** early.
- **P0/P1 issues are blocking** and must be fixed before proceeding.
- **Lower-severity findings** may be handled at the agent’s discretion if fixes are **low-risk** and aligned with the change.
- The workflow is intended to **reduce avoidable PR noise** and improve review quality.
- The described flow is: **change ready → adversarial diff review → fix P0/P1 → optionally address low-risk findings → push/open PR**.

## Structure / sections summary
- **Metadata**: title, summary, tags/keywords, importance, recency, maturity, timestamps.
- **Reason**: states the document captures new pre-push review workflow guidance from the Ito general skill.
- **Raw Concept**: outlines the task, change, workflow, and source authoring file.
- **Narrative**:
  - **Structure**: places the guidance in the Ito workflow as a pre-push quality gate.
  - **Highlights**: emphasizes adversarial review before push/PR and the hard stop on severe issues.
  - **Rules**: formalizes when to run review and how to treat findings by severity.
- **Facts**: lists convention-style requirements for pre-push review, purpose, and severity handling.

## Notable entities, patterns, or decisions
- **Entity:** *Ito agents* — the actors responsible for performing the review.
- **Entity:** *.agents/skills/ito-general/SKILL.md* — the source skill file referenced as the authoring basis.
- **Pattern:** **Adversarial diff review** before push/PR, used as a proactive quality check.
- **Decision:** **Severity gate** where **P0/P1 issues block progress** until resolved.
- **Decision:** **Minor findings are optional** to fix, based on risk and alignment with the change.