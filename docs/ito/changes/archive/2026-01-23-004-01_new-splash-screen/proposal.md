## Why

The current Ito splash screen ASCII art needs a visual refresh to improve the CLI user experience and aesthetics. A new, more modern or stylized ASCII art design will give the tool a more polished look upon startup.

## What Changes

- Replace the existing ASCII art text/logo displayed during Ito initialization.
- Ensure the new art fits within standard terminal widths.
- potentially add color or styling support if applicable (though primary focus is the art itself).

### Proposed Art

```
████████████████████████████
██                        ██
█████                  █████
████████████████████████████
██████                ██████
████████████████████████████
██████                ██████
████████████████████████████
█████                  █████
██                        ██
████████████████████████████
```

## Capabilities

### New Capabilities

- `splash-screen-art`: Defines the new ASCII art design and its rendering logic within the CLI.

### Modified Capabilities

<!-- No existing functional capabilities are changing requirements, just the visual asset/output. -->

## Impact

- **CLI Startup**: The visual appearance of the tool's entry point will change.
- **User Experience**: Improved visual polish.
- **Code**: Updates to the module or file responsible for printing the banner/splash screen.
