#!/usr/bin/env bash
set -euo pipefail

GATE_DIR="target/release-gates"
ROOT="$GATE_DIR/real-upstream-smoke/latest"
RAW_DIR="$ROOT/raw"
NORMALIZED_DIR="$ROOT/normalized"
DIFF_DIR="$ROOT/diff"
WORK_DIR="$ROOT/work"

rm -rf "$ROOT"
mkdir -p "$RAW_DIR" "$NORMALIZED_DIR" "$DIFF_DIR" "$WORK_DIR"

cargo build --quiet --release --bin promptfoo-rs
BIN="target/release/promptfoo-rs"
if [ -f "${BIN}.exe" ]; then
  BIN="${BIN}.exe"
fi

cat > "$WORK_DIR/promptfooconfig.yaml" <<'YAML'
prompts:
  - "Hello {{name}}"
providers:
  - id: echo
tests:
  - vars:
      name: Ada
    assert:
      - type: contains
        value: Ada
YAML

npm view promptfoo@0.121.13 version gitHead dist.tarball dist.integrity --json > "$ROOT/npm-view.json"

set +e
(cd "$WORK_DIR" && PROMPTFOO_DISABLE_TELEMETRY=1 PROMPTFOO_DISABLE_UPDATE=1 NO_COLOR=1 CI=1 npx --yes promptfoo@0.121.13 eval -c promptfooconfig.yaml --output "../raw/upstream.json" > "../raw/upstream.stdout" 2> "../raw/upstream.stderr")
upstream_exit=$?
"$BIN" eval -c "$WORK_DIR/promptfooconfig.yaml" --output "$RAW_DIR/rs.json" > "$RAW_DIR/rs.stdout" 2> "$RAW_DIR/rs.stderr"
rs_exit=$?
set -e

node - "$ROOT" "$BIN" "$upstream_exit" "$rs_exit" <<'NODE'
const fs = require('fs');
const path = require('path');

const [root, rsBinary, upstreamExit, rsExit] = process.argv.slice(2);
const rawDir = path.join(root, 'raw');
const normalizedDir = path.join(root, 'normalized');
const diffDir = path.join(root, 'diff');
const npmView = JSON.parse(fs.readFileSync(path.join(root, 'npm-view.json'), 'utf8'));

function readJson(relative) {
  return JSON.parse(fs.readFileSync(path.join(root, relative), 'utf8'));
}

const upstream = readJson('raw/upstream.json');
const rs = readJson('raw/rs.json');

const upstreamRows = (((upstream || {}).results || {}).results || []);
const upstreamSummary = {
  engine: 'upstream-promptfoo',
  passed: upstreamRows.filter((row) => row.gradingResult && row.gradingResult.pass === true).length,
  failed: upstreamRows.filter((row) => row.gradingResult && row.gradingResult.pass === false).length,
  total: upstreamRows.length,
};
const rsSummary = {
  engine: 'promptfoo-rs',
  passed: rs.summary && rs.summary.passed,
  failed: rs.summary && rs.summary.failed,
  total: rs.summary && rs.summary.total,
};

fs.writeFileSync(path.join(normalizedDir, 'upstream.json'), JSON.stringify(upstreamSummary, null, 2) + '\n');
fs.writeFileSync(path.join(normalizedDir, 'rs.json'), JSON.stringify(rsSummary, null, 2) + '\n');

const findings = [];
if (Number(upstreamExit) !== 0 || Number(rsExit) !== 0) {
  findings.push({ class: 'bug', path: 'exit_code', upstream: Number(upstreamExit), rs: Number(rsExit) });
}
for (const key of ['passed', 'failed', 'total']) {
  if (upstreamSummary[key] !== rsSummary[key]) {
    findings.push({ class: 'bug', path: `summary.${key}`, upstream: upstreamSummary[key], rs: rsSummary[key] });
  }
}
fs.writeFileSync(path.join(diffDir, 'findings.json'), JSON.stringify(findings, null, 2) + '\n');

const metadata = {
  schema: 'promptfoo-rs.real-upstream-smoke.v1',
  fixture_id: 'real-upstream-smoke-echo',
  baseline: {
    npm: 'promptfoo@0.121.13',
    version: npmView.version,
    gitHead: npmView.gitHead,
    tarball: npmView.dist && npmView.dist.tarball,
    integrity: npmView.dist && npmView.dist.integrity,
  },
  upstream_command: 'PROMPTFOO_DISABLE_TELEMETRY=1 PROMPTFOO_DISABLE_UPDATE=1 NO_COLOR=1 CI=1 npx --yes promptfoo@0.121.13 eval -c promptfooconfig.yaml --output raw/upstream.json',
  rs_binary: rsBinary,
  rs_command: `${rsBinary} eval -c promptfooconfig.yaml --output raw/rs.json`,
  used_test_binary: false,
  upstream_exit_code: Number(upstreamExit),
  rs_exit_code: Number(rsExit),
  status: findings.length === 0 ? 'ready' : 'blocked',
  artifact_paths: [
    'target/release-gates/real-upstream-smoke/latest/metadata.json',
    'target/release-gates/real-upstream-smoke/latest/raw/upstream.json',
    'target/release-gates/real-upstream-smoke/latest/raw/rs.json',
    'target/release-gates/real-upstream-smoke/latest/normalized/upstream.json',
    'target/release-gates/real-upstream-smoke/latest/normalized/rs.json',
    'target/release-gates/real-upstream-smoke/latest/diff/findings.json',
  ],
};
fs.writeFileSync(path.join(root, 'metadata.json'), JSON.stringify(metadata, null, 2) + '\n');
NODE

node -e "const m = JSON.parse(require('fs').readFileSync(process.argv[1], 'utf8')); if (m.used_test_binary || m.status !== 'ready') { console.error(JSON.stringify(m, null, 2)); process.exit(1); }" "$ROOT/metadata.json"
