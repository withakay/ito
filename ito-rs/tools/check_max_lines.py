#!/usr/bin/env python3

from __future__ import annotations

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


def load_baseline(path: Path | None) -> dict[str, int]:
    if path is None or not path.exists():
        return {}

    baseline: dict[str, int] = {}
    for line_number, line in enumerate(
        path.read_text(encoding="utf-8").splitlines(), 1
    ):
        line = line.strip()
        if not line or line.startswith("#"):
            continue

        parts = line.split()
        if len(parts) != 2:
            print(
                f"Invalid max-lines baseline entry at {path}:{line_number}: {line}",
                file=sys.stderr,
            )
            raise SystemExit(2)

        file_path, line_count = parts
        try:
            baseline[file_path] = int(line_count)
        except ValueError:
            print(
                f"Invalid line count in max-lines baseline at {path}:{line_number}: {line_count}",
                file=sys.stderr,
            )
            raise SystemExit(2)

    return baseline


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
    parser.add_argument(
        "--baseline",
        type=Path,
        default=None,
        help="Optional '<path> <line-count>' baseline for existing oversized files.",
    )

    args = parser.parse_args()

    # Handle backwards compatibility
    soft_limit: int = args.max_lines if args.max_lines is not None else args.soft_limit
    hard_limit: int = args.hard_limit
    roots = [Path(r) for r in (args.root or ["ito-rs"])]
    baseline = load_baseline(args.baseline)

    warnings: list[tuple[int, Path]] = []
    errors: list[tuple[int, Path]] = []
    baseline_warnings: list[tuple[int, int, Path]] = []

    for root in roots:
        if not root.exists():
            continue
        for path in iter_source_files(root):
            n = count_lines(path)
            path_key = path.as_posix()
            baseline_limit = baseline.get(path_key)
            if baseline_limit is not None and n <= baseline_limit:
                if n > soft_limit:
                    baseline_warnings.append((n, baseline_limit, path))
                continue

            if n > hard_limit:
                errors.append((n, path))
            elif n > soft_limit:
                warnings.append((n, path))

    if baseline_warnings:
        baseline_warnings.sort(key=lambda x: (-x[0], str(x[2])))
        print(
            f"Warning: {len(baseline_warnings)} Rust files exceed limits but remain within baseline:",
            file=sys.stderr,
        )
        for n, baseline_limit, path in baseline_warnings:
            print(f"  - {path}: {n} (baseline {baseline_limit})", file=sys.stderr)

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
