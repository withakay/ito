## Context

Ito currently displays a basic ASCII art banner or text upon startup. We want to improve the visual polish of the CLI by introducing a new, stylized ASCII art splash screen. This is a purely cosmetic change to enhance user experience and brand identity within the terminal.

## Goals / Non-Goals

**Goals:**

- Replace the existing startup banner with a new, stylized ASCII art design.
- Ensure the art renders correctly on standard 80-column terminals.
- Centralize the splash screen logic for easy updates.

**Non-Goals:**

- Interactive splash screens or animations.
- Configurable splash screen themes (one standard design for now).
- Changes to any other CLI functionality or command logic.

## Decisions

### 1. ASCII Art Asset

- **Choice**: Embed the ASCII art as a constant string in the code.
- **Rationale**: Simple, zero dependencies, and easy to maintain. No need for external asset loading.
- **Alternatives**: Loading from a text file (adds I/O overhead), generating dynamically (unnecessary complexity).

### 2. Location of Logic

- **Choice**: Create a dedicated `splash.ts` (or similar) utility in `src/cli/ui/` or `src/core/ui/`.
- **Rationale**: Keeps the main CLI entry point clean and allows for potential reuse or testing of the banner display logic.

## Risks / Trade-offs

- **Terminal Width**: If a user has a very narrow terminal (\< 80 cols), the art might wrap and look broken.
  - **Mitigation**: Design the art to be safely within 80 columns. The spec explicitly requires this.
- **Unicode Support**: Some complex ASCII/ANSI art might use characters not supported in all fonts.
  - **Mitigation**: Use standard ASCII or widely supported Unicode block characters.

## Migration Plan

- **Deploy**: This is a code-level update. Users get the new splash screen upon updating the Ito CLI package.
- **Rollback**: Revert to the previous version of the package if the art causes severe display issues (unlikely).
