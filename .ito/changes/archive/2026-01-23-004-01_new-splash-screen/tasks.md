# Tasks for: 004-01_new-splash-screen

## Execution Notes

- **Tool**: Any
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking

______________________________________________________________________

## Wave 1: Implementation

### Task 1.1: Create Splash Screen Module

- **Files**: `src/core/ui/splash.ts`
- **Dependencies**: None
- **Action**:
  - Create directory `src/core/ui` if it doesn't exist.
  - Create `src/core/ui/splash.ts`.
  - Add a constant string containing the new stylized ASCII art for "ITO".
  - Ensure the art fits within 80 columns.
  - Export a function `getSplash()` that returns the art string.
- **Verify**: `cat src/core/ui/splash.ts`
- **Done When**: File exists and contains the ASCII art constant.
- **Status**: \[x\] complete

### Task 1.2: Integrate Splash Screen

- **Files**: `src/core/init.ts` (or relevant entry point)
- **Dependencies**: Task 1.1
- **Action**:
  - Locate the existing startup banner logic (likely in `src/core/init.ts`).
  - Replace the old "ITO" text generation/animation with a call to `getSplash()`.
  - Ensure it prints to stdout on startup.
- **Verify**: Run the CLI manually to check the output.
- **Done When**: The new ASCII art appears on startup instead of the old one.
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 2: Verification

### Task 2.1: Verify Dimensions

- **Files**: `test/core/ui/splash.test.ts` (new)
- **Dependencies**: Task 1.1
- **Action**:
  - Create a unit test that imports `getSplash`.
  - Assert that every line of the returned string is \<= 80 characters.
- **Verify**: `make test`
- **Done When**: Tests pass.
- **Status**: \[x\] complete
