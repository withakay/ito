## Key points
- `synced_at_generation` should be populated **only** when coordination sync returns **Synchronized**.
- **RateLimited** is explicitly **not** a fresh success; it indicates no sync was observed during generation.
- Manifesto **full `--operation`** usage requires a paired **`--change`** argument.
- Embedded operation instructions are scoped to the **resolved change state**.
- In rendered manifesto output, **configured** operation instructions may appear, while **unconfigured** operations render as **null**.
- The document frames these rules as implementation notes for **sync reporting** and **operation instruction visibility**.

## Structure / sections summary
- **Title / metadata**: Identifies the document as “Manifesto Instruction Implementation Notes” with a short summary about sync reporting and null rendering behavior.
- **Reason**: States the purpose—documenting constraints for manifesto instruction rendering and sync status reporting.
- **Raw Concept**:
  - Summarizes the task and the specific implementation changes.
  - Defines the flow from generation to coordination sync and how outcomes should be handled.
- **Narrative**:
  - **Structure**: Explains that the notes govern how manifesto generation represents sync outcomes and operation instruction visibility.
  - **Dependencies**: Notes reliance on coordination sync results and resolved change state for full `--operation`.
  - **Highlights**: Reiterates the main behavioral rules.
- **Facts**:
  - Lists the concrete implementation rules as named facts, covering sync population, RateLimited handling, operation/change coupling, scope, and rendering behavior.

## Notable entities, patterns, or decisions
- **Entities / statuses**:
  - `Synchronized`
  - `RateLimited`
  - `synced_at_generation`
  - `--operation`
  - `--change`
- **Behavioral pattern**:
  - A strict conditional flow: generation → coordination sync → only mark success on `Synchronized`; do not infer success from `RateLimited`.
- **Design decision**:
  - Operation instructions are **scoped to resolved change state**, preventing standalone full `--operation` rendering without `--change`.
- **Rendering decision**:
  - Unconfigured operations should render as **null**, making absence explicit rather than implied.