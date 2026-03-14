#!/usr/bin/env bash
# Ensure zeroclaw binary exists in src-tauri/binaries/ for Tauri sidecar.
# Prefer download from GitHub Release; fallback to local build when ZEROCLAW_BUILD_FROM_SOURCE=1 or no asset.
# Usage: run from repo root.

set -e
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BINARIES_DIR="$REPO_ROOT/src-tauri/binaries"
ZEROCLAW_REPO="https://github.com/zeroclaw-labs/zeroclaw"
RELEASE_TAG="${ZEROCLAW_RELEASE_TAG:-v0.2.0}"

# Target triple
TARGET="${CARGO_BUILD_TARGET:-$(rustc -vV 2>/dev/null | grep 'host:' | sed 's/.*host: *//')}"
if [ -z "$TARGET" ]; then
  TARGET="x86_64-unknown-linux-gnu"
fi

EXE_EXT=""
case "$TARGET" in
  *-windows-*) EXE_EXT=".exe" ;;
esac
OUT_NAME="zeroclaw-${TARGET}${EXE_EXT}"
OUT_PATH="$BINARIES_DIR/$OUT_NAME"

mkdir -p "$BINARIES_DIR"
if [ -f "$OUT_PATH" ]; then
  echo "zeroclaw binary exists: $OUT_PATH"
  exit 0
fi

get_asset_name() {
  case "$1" in
    x86_64-pc-windows-msvc) echo "zeroclaw-x86_64-pc-windows-msvc.zip" ;;
    x86_64-unknown-linux-gnu) echo "zeroclaw-x86_64-unknown-linux-gnu.tar.gz" ;;
    aarch64-unknown-linux-gnu) echo "zeroclaw-aarch64-unknown-linux-gnu.tar.gz" ;;
    x86_64-apple-darwin) echo "zeroclaw-x86_64-apple-darwin.tar.gz" ;;
    aarch64-apple-darwin) echo "zeroclaw-aarch64-apple-darwin.tar.gz" ;;
    *) echo "" ;;
  esac
}

BUILD_FROM_SOURCE="${ZEROCLAW_BUILD_FROM_SOURCE:-0}"
ASSET_NAME="$(get_asset_name "$TARGET")"

if [ "$BUILD_FROM_SOURCE" != "1" ] && [ -n "$ASSET_NAME" ]; then
  URL="$ZEROCLAW_REPO/releases/download/$RELEASE_TAG/$ASSET_NAME"
  echo "Downloading $URL ..."
  TMPDIR="$(mktemp -d)"
  trap 'rm -rf "$TMPDIR"' EXIT
  if command -v curl >/dev/null 2>&1; then
    curl -sSL -o "$TMPDIR/asset" "$URL"
  elif command -v wget >/dev/null 2>&1; then
    wget -q -O "$TMPDIR/asset" "$URL"
  else
    echo "Need curl or wget to download"
    exit 1
  fi
  if [ -s "$TMPDIR/asset" ]; then
    (cd "$TMPDIR" && tar xzf asset 2>/dev/null || unzip -o -q asset 2>/dev/null || true)
    SRC_EXE=""
    for f in "$TMPDIR"/zeroclaw* "$TMPDIR"/zeroclaw; do
      [ -x "$f" ] || [ -f "$f" ] && SRC_EXE="$f" && break
    done
    if [ -n "$SRC_EXE" ]; then
      cp "$SRC_EXE" "$OUT_PATH"
      [ -n "$EXE_EXT" ] || chmod +x "$OUT_PATH"
      echo "Downloaded to $OUT_PATH"
      exit 0
    fi
  fi
fi

# Fallback: build from source
SUBMODULE_PATH="$REPO_ROOT/crates/zeroclaw"
if [ ! -f "$SUBMODULE_PATH/Cargo.toml" ]; then
  echo "Submodule crates/zeroclaw not found. Run: git submodule update --init crates/zeroclaw"
  echo "Or set ZEROCLAW_BUILD_FROM_SOURCE=0 and ensure a Release asset exists for $TARGET"
  exit 1
fi
echo "Building zeroclaw from source in $SUBMODULE_PATH ..."
(cd "$SUBMODULE_PATH" && cargo build --release --bin zeroclaw)
BUILT="$SUBMODULE_PATH/target/release/zeroclaw$EXE_EXT"
if [ -f "$BUILT" ]; then
  cp "$BUILT" "$OUT_PATH"
  [ -n "$EXE_EXT" ] || chmod +x "$OUT_PATH"
  echo "Built and copied to $OUT_PATH"
  exit 0
fi
echo "Failed to produce $OUT_PATH"
exit 1
