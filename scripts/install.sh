#!/bin/sh

set -eu

REPO="withakay/ito"
INSTALL_DIR_DEFAULT="$HOME/.local/bin"

err() {
  printf '%s\n' "$*" 1>&2
}

need() {
  if ! command -v "$1" >/dev/null 2>&1; then
    err "missing required command: $1"
    exit 1
  fi
}

os_target() {
  OS=$(uname -s)
  ARCH=$(uname -m)

  case "$OS" in
    Darwin)
      case "$ARCH" in
        x86_64) echo "x86_64-apple-darwin" ;;
        arm64) echo "aarch64-apple-darwin" ;;
        *) err "unsupported macOS arch: $ARCH"; exit 1 ;;
      esac
      ;;
    Linux)
      case "$ARCH" in
        x86_64) echo "x86_64-unknown-linux-gnu" ;;
        aarch64|arm64) echo "aarch64-unknown-linux-gnu" ;;
        *) err "unsupported Linux arch: $ARCH"; exit 1 ;;
      esac
      ;;
    *)
      err "unsupported platform: $OS (this installer supports macOS and Linux only)"
      exit 1
      ;;
  esac
}

latest_tag() {
  need curl
  curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
    | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' \
    | head -n 1
}

download() {
  URL="$1"
  OUT="$2"
  curl -fsSL "$URL" -o "$OUT"
}

sha256_file() {
  FILE="$1"
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$FILE" | awk '{print $1}'
  else
    sha256sum "$FILE" | awk '{print $1}'
  fi
}

main() {
  need tar
  need sed
  need awk
  need head

  TARGET=$(os_target)

  VERSION=${ITO_VERSION:-""}
  if [ -z "$VERSION" ]; then
    VERSION=$(latest_tag)
  fi
  if [ -z "$VERSION" ]; then
    err "failed to determine latest release tag"
    exit 1
  fi
  case "$VERSION" in
    v*) TAG="$VERSION" ;;
    *) TAG="v$VERSION" ;;
  esac

  INSTALL_DIR=${ITO_INSTALL_DIR:-"$INSTALL_DIR_DEFAULT"}
  ARCHIVE="ito-${TAG}-${TARGET}.tar.gz"
  CHECKSUM="ito-${TAG}-${TARGET}.sha256"
  BASE_URL=${ITO_BASE_URL:-"https://github.com/$REPO/releases/download/$TAG"}

  TMP=$(mktemp -d)
  trap 'rm -rf "$TMP"' EXIT

  download "$BASE_URL/$ARCHIVE" "$TMP/$ARCHIVE"
  download "$BASE_URL/$CHECKSUM" "$TMP/$CHECKSUM"

  EXPECTED=$(awk '{print $1}' "$TMP/$CHECKSUM" | head -n 1)
  ACTUAL=$(sha256_file "$TMP/$ARCHIVE")
  if [ "$EXPECTED" != "$ACTUAL" ]; then
    err "checksum mismatch for $ARCHIVE"
    err "expected: $EXPECTED"
    err "actual:   $ACTUAL"
    exit 1
  fi

  mkdir -p "$TMP/extract"
  tar -C "$TMP/extract" -xzf "$TMP/$ARCHIVE"

  if [ ! -f "$TMP/extract/ito" ]; then
    err "archive did not contain expected binary: ito"
    exit 1
  fi

  mkdir -p "$INSTALL_DIR"
  chmod +x "$TMP/extract/ito"
  cp "$TMP/extract/ito" "$INSTALL_DIR/ito"

  printf '%s\n' "installed ito to $INSTALL_DIR/ito"
  "$INSTALL_DIR/ito" --version || true
}

main "$@"
