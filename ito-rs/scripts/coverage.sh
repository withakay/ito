#!/usr/bin/env bash
set -euo pipefail

# Deterministic local coverage without network/cargo plugins.
# Requires Xcode LLVM tools (llvm-profdata, llvm-cov) available via `xcrun`.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET_DIR="$ROOT_DIR/target"
COV_DIR="$TARGET_DIR/coverage"
PROFRAW_DIR="$COV_DIR/profraw"
PROFILE="$COV_DIR/merged.profdata"

mkdir -p "$PROFRAW_DIR"
rm -f "$PROFRAW_DIR"/*.profraw "$PROFILE"

export LLVM_PROFILE_FILE="$PROFRAW_DIR/%m-%p.profraw"
export RUSTFLAGS="${RUSTFLAGS:-} -C instrument-coverage -C link-dead-code"

(cd "$ROOT_DIR" && cargo test --workspace)

xcrun llvm-profdata merge -sparse "$PROFRAW_DIR"/*.profraw -o "$PROFILE"

# Collect instrumented test executables from Cargo JSON output.
EXES=$(cd "$ROOT_DIR" && cargo test --workspace --no-run --message-format=json | \
  python3 -c 'import json,sys; exes=set();
for line in sys.stdin:
  try: obj=json.loads(line)
  except Exception: continue
  exe=obj.get("executable")
  if exe: exes.add(exe)
print("\n".join(sorted(exes)))')

# Keep output readable and focused on our workspace sources.
ARGS=(
  "report"
  "--summary-only"
  "--instr-profile" "$PROFILE"
  "--ignore-filename-regex" "(^|/)\.cargo/registry/|(^|/)rustc/"
)
while IFS= read -r exe; do
  [ -n "$exe" ] || continue
  ARGS+=("--object" "$exe")
done <<< "$EXES"

xcrun llvm-cov "${ARGS[@]}"
