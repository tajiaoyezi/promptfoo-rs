#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

VERSION="$(node -e "const fs=require('fs'); const toml=fs.readFileSync('Cargo.toml','utf8'); const m=toml.match(/^version\\s*=\\s*\"([^\"]+)\"/m); if(!m) throw new Error('Cargo.toml version missing'); console.log(m[1]);")"
HOST="$(rustc -vV | awk '/^host: / { print $2 }')"
TARGET="${TARGET:-$HOST}"
DIST="target/github-release-dist"
STAGING="$DIST/staging"
RELEASE_DIR="target/${TARGET}/release"
mkdir -p "$STAGING"

if ! rustup target list --installed | grep -Fxq "$TARGET"; then
  rustup target add "$TARGET"
fi

cargo build --workspace --release --target "$TARGET"

copy_bin() {
  local name="$1"
  local src="${RELEASE_DIR}/${name}"
  if [ -f "${src}.exe" ]; then
    src="${src}.exe"
    cp "$src" "$STAGING/${name}.exe"
  elif [ -f "$src" ]; then
    cp "$src" "$STAGING/$name"
  else
    echo "missing release binary: $name" >&2
    exit 1
  fi
}

copy_bin promptfoo-rs
copy_bin promptfoo

ARTIFACT_BASENAME="promptfoo-rs-${VERSION}-${TARGET}"
if [[ "$TARGET" == *"windows"* ]]; then
  ARCHIVE="$DIST/${ARTIFACT_BASENAME}.zip"
  if command -v powershell.exe >/dev/null 2>&1; then
    powershell.exe -NoProfile -Command "Compress-Archive -Path '${STAGING}\\*' -DestinationPath '${ARCHIVE}' -Force"
  elif command -v zip >/dev/null 2>&1; then
    (cd "$STAGING" && zip -qr "../${ARTIFACT_BASENAME}.zip" .)
  else
    ARCHIVE="$DIST/${ARTIFACT_BASENAME}.tar.gz"
    tar -czf "$ARCHIVE" -C "$STAGING" .
  fi
else
  ARCHIVE="$DIST/${ARTIFACT_BASENAME}.tar.gz"
  tar -czf "$ARCHIVE" -C "$STAGING" .
fi

CHECKSUMS="$DIST/SHA256SUMS"
if command -v sha256sum >/dev/null 2>&1; then
  (cd "$DIST" && sha256sum "$(basename "$ARCHIVE")" > SHA256SUMS)
elif command -v shasum >/dev/null 2>&1; then
  (cd "$DIST" && shasum -a 256 "$(basename "$ARCHIVE")" > SHA256SUMS)
else
  node - "$ARCHIVE" "$CHECKSUMS" <<'NODE'
const crypto = require('crypto');
const fs = require('fs');
const [archive, out] = process.argv.slice(2);
const hash = crypto.createHash('sha256').update(fs.readFileSync(archive)).digest('hex');
fs.writeFileSync(out, `${hash}  ${require('path').basename(archive)}\n`);
NODE
fi

echo "packaged ${ARCHIVE}"
echo "checksums ${CHECKSUMS}"
cat "$CHECKSUMS"
