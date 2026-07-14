# Design: Archive And Change Discovery

## Overview

Add archive-aware discovery commands and route existing active-change commands through a common resolver that understands scope.

## Commands

```bash
ito archive list --json
ito archive show <change-id> --json
ito list --archived --json
ito list --all --json
```

## Resolver Model

Introduce an explicit scope enum:

- `Active`
- `Archived`
- `All`

Use it for list, show, archive, validate, status, and agent instruction commands where archive behavior matters.

## Archive Directory Parsing

Archived change directories may include date prefixes. The resolver should extract the canonical change ID from date-prefixed archive directory names and preserve the archive directory path in output.

## Error Messages

If a command searches active changes and finds an archived match, the error should say that the target is archived and suggest `ito archive show <change-id>` or an explicit scope flag.

## Risks

`--all` can make partial IDs more ambiguous. Ambiguous responses must include scope and path so agents can choose the right target.
