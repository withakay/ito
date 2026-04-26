## Key points
- Plain markdown files under `ito-rs/crates/ito-templates/assets` were retrofitted with `<!-- ITO:START -->` and `<!-- ITO:END -->` markers.
- Files that were already pre-marked were intentionally left unchanged.
- Verification confirmed there were no unmarked plain markdown files in `ito-rs/crates/ito-templates/assets/adapters` that required modification.
- The update standardized marker presence across template assets without rewriting compliant files.
- The work is described as a template bundle retrofit / document marker retrofit for markdown assets.

## Structure / sections summary
- **Metadata**: Title, summary, tags, related, keywords, creation and update timestamps.
- **Reason**: States the purpose as document marker retrofit for template asset markdown files.
- **Raw Concept**: Lists the task, changes made, affected files, workflow, timestamp, and author.
- **Narrative**: Provides contextual explanation, dependencies, highlights, and rules governing the retrofit.
- **Facts**: Summarizes the outcomes as explicit project facts.

## Notable entities, patterns, or decisions
- **Entities / paths**
  - `ito-rs/crates/ito-templates/assets`
  - `ito-rs/crates/ito-templates/assets/adapters`
- **Markers used**
  - `<!-- ITO:START -->`
  - `<!-- ITO:END -->`
- **Decision pattern**
  - Only unmarked plain `.md` files were eligible for retrofitting.
  - Already compliant, pre-marked files were preserved as-is.
- **Verification outcome**
  - Adapter sample verification found no unmarked markdown requiring changes.
