#!/usr/bin/env python3

import argparse
import os
import sys
from pathlib import Path


def iter_source_files(root: Path) -> list[Path]:
    files: list[Path] = []
    for dirpath, dirnames, filenames in os.walk(root):
        # Skip common build/cache dirs.
        dirnames[:] = [
            d
            for d in dirnames
            if d not in {".git", "target", "node_modules"} and not d.startswith(".")
        ]

        for name in filenames:
            if not name.endswith(".rs"):
                continue
            files.append(Path(dirpath) / name)

    return files


def count_lines(path: Path) -> int:
    with path.open("r", encoding="utf-8", errors="replace") as f:
        return sum(1 for _ in f)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Check Rust source files against soft and hard line limits."
    )
    parser.add_argument(
        "--soft-limit",
        type=int,
        default=1000,
        help="Soft limit: warn but don't fail (default: 1000)",
    )
    parser.add_argument(
        "--hard-limit",
        type=int,
        default=1200,
        help="Hard limit: fail if exceeded (default: 1200)",
    )
    # Keep --max-lines for backwards compatibility, maps to soft-limit
    parser.add_argument(
        "--max-lines",
        type=int,
        default=None,
        help="Alias for --soft-limit (deprecated)",
    )
    parser.add_argument(
        "--root",
        action="append",
        default=[],
        help="Root directory to scan (repeatable). Defaults to ./ito-rs",
    )

    args = parser.parse_args()

    # Handle backwards compatibility
    soft_limit: int = args.max_lines if args.max_lines is not None else args.soft_limit
    hard_limit: int = args.hard_limit
    roots = [Path(r) for r in (args.root or ["ito-rs"])]

    warnings: list[tuple[int, Path]] = []
    errors: list[tuple[int, Path]] = []

    for root in roots:
        if not root.exists():
            continue
        for path in iter_source_files(root):
            n = count_lines(path)
            if n > hard_limit:
                errors.append((n, path))
            elif n > soft_limit:
                warnings.append((n, path))

    # Print warnings but don't fail
    if warnings:
        warnings.sort(key=lambda x: (-x[0], str(x[1])))
        print(
            f"Warning: {len(warnings)} Rust files over soft limit ({soft_limit} lines):",
            file=sys.stderr,
        )
        for n, path in warnings:
            print(f"  - {path}: {n} (consider splitting)", file=sys.stderr)

    # Fail on hard limit violations
    if not errors:
        return 0

    errors.sort(key=lambda x: (-x[0], str(x[1])))
    print(f"Error: {len(errors)} Rust files over hard limit ({hard_limit} lines):")
    for n, path in errors:
        print(f"  - {path}: {n}")

    return 1


if __name__ == "__main__":
    raise SystemExit(main())
