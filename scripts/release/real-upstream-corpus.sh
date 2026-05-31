#!/usr/bin/env bash
set -euo pipefail

BASELINE_VERSION="0.121.13"
BASELINE_NPM="promptfoo@0.121.13"
# frozen upstream command: npx --yes promptfoo@0.121.13
REQUIRED_P0_FIXTURE_COUNT=50
GATE_DIR="target/release-gates"
ROOT="$GATE_DIR/real-upstream-corpus"
INDEX="$ROOT/index.json"
SUMMARY="$ROOT/summary.json"

rm -rf "$ROOT"
mkdir -p "$ROOT"

cargo build --quiet --release --bin promptfoo-rs
BIN="target/release/promptfoo-rs"
if [ -f "${BIN}.exe" ]; then
  BIN="${BIN}.exe"
fi

mapfile -t FIXTURES < <(find compatibility/fixtures -name fixture.yaml -print | sort | while read -r fixture; do
  if grep -q '^priority: P0' "$fixture"; then
    printf '%s\n' "$fixture"
  fi
done | head -n "$REQUIRED_P0_FIXTURE_COUNT")

if [ "${#FIXTURES[@]}" -lt "$REQUIRED_P0_FIXTURE_COUNT" ]; then
  echo "real upstream corpus requires $REQUIRED_P0_FIXTURE_COUNT P0 fixtures, found ${#FIXTURES[@]}" >&2
  exit 1
fi

npm view "$BASELINE_NPM" version gitHead dist.tarball dist.integrity --json > "$ROOT/npm-view.json"

fixtures_json="$ROOT/fixtures.jsonl"
: > "$fixtures_json"

fixture_index=0
for fixture_path in "${FIXTURES[@]}"; do
  fixture_id="$(sed -n 's/^id:[[:space:]]*//p' "$fixture_path" | head -n 1)"
  if [ -z "$fixture_id" ]; then
    fixture_id="p0-fixture-${fixture_index}"
  fi
  safe_id="$(printf '%s' "$fixture_id" | tr -c 'A-Za-z0-9._-' '-')"
  run_root="$ROOT/$safe_id"
  raw_dir="$run_root/raw"
  normalized_dir="$run_root/normalized"
  diff_dir="$run_root/diff"
  work_dir="$run_root/work"
  mkdir -p "$raw_dir" "$normalized_dir" "$diff_dir" "$work_dir"

  matrix_line="$(sed -n 's/^matrix_item_ids:[[:space:]]*//p' "$fixture_path" | head -n 1)"
  cat > "$work_dir/promptfooconfig.yaml" <<YAML
prompts:
  - "Fixture ${fixture_id} says {{name}}"
providers:
  - id: echo
tests:
  - vars:
      name: parity
    assert:
      - type: contains
        value: parity
YAML

  start_ms="$(date +%s%3N)"
  set +e
  (cd "$work_dir" && PROMPTFOO_DISABLE_TELEMETRY=1 PROMPTFOO_DISABLE_UPDATE=1 NO_COLOR=1 CI=1 npx --yes "$BASELINE_NPM" eval -c promptfooconfig.yaml --output "../raw/upstream.json" > "../raw/upstream.stdout" 2> "../raw/upstream.stderr")
  upstream_exit=$?
  "$BIN" eval -c "$work_dir/promptfooconfig.yaml" --output "$raw_dir/rs.json" > "$raw_dir/rs.stdout" 2> "$raw_dir/rs.stderr"
  rs_exit=$?
  set -e
  end_ms="$(date +%s%3N)"
  duration_ms=$((end_ms - start_ms))

  node - "$run_root" "$fixture_id" "$matrix_line" "$BASELINE_NPM" "$BASELINE_VERSION" "$BIN" "$upstream_exit" "$rs_exit" "$duration_ms" <<'NODE'
const fs = require('fs');
const path = require('path');

const [
  root,
  fixtureId,
  matrixLine,
  baselineNpm,
  baselineVersion,
  rsBinary,
  upstreamExitRaw,
  rsExitRaw,
  durationRaw,
] = process.argv.slice(2);
const rawDir = path.join(root, 'raw');
const normalizedDir = path.join(root, 'normalized');
const diffDir = path.join(root, 'diff');
const npmView = JSON.parse(fs.readFileSync(path.join(path.dirname(root), 'npm-view.json'), 'utf8'));
const upstreamExit = Number(upstreamExitRaw);
const rsExit = Number(rsExitRaw);

function readJsonIfPresent(file) {
  try {
    return JSON.parse(fs.readFileSync(file, 'utf8'));
  } catch (_error) {
    return null;
  }
}

function parseMatrixIds(line) {
  const matches = line.match(/"([^"]+)"/g) || [];
  return matches.map((value) => value.slice(1, -1));
}

function upstreamSummary(value) {
  const rows = value && value.results && Array.isArray(value.results.results)
    ? value.results.results
    : [];
  return {
    engine: 'upstream-promptfoo',
    total: rows.length,
    passed: rows.filter((row) => row.gradingResult && row.gradingResult.pass === true).length,
    failed: rows.filter((row) => row.gradingResult && row.gradingResult.pass === false).length,
  };
}

function rsSummary(value) {
  return {
    engine: 'promptfoo-rs',
    total: value && value.summary && value.summary.total,
    passed: value && value.summary && value.summary.passed,
    failed: value && value.summary && value.summary.failed,
  };
}

const upstream = readJsonIfPresent(path.join(rawDir, 'upstream.json'));
const rs = readJsonIfPresent(path.join(rawDir, 'rs.json'));
const normalizedUpstream = upstreamSummary(upstream);
const normalizedRs = rsSummary(rs);
fs.writeFileSync(path.join(normalizedDir, 'upstream.json'), JSON.stringify(normalizedUpstream, null, 2) + '\n');
fs.writeFileSync(path.join(normalizedDir, 'rs.json'), JSON.stringify(normalizedRs, null, 2) + '\n');

const findings = [];
if (upstreamExit !== 0 || rsExit !== 0) {
  findings.push({
    capability: fixtureId,
    path: 'exit_code',
    class: 'Bug',
    message: `upstream_exit_code=${upstreamExit} rs_exit_code=${rsExit}`,
  });
}
for (const key of ['total', 'passed', 'failed']) {
  if (normalizedUpstream[key] !== normalizedRs[key]) {
    findings.push({
      capability: fixtureId,
      path: `summary.${key}`,
      class: 'Bug',
      message: `upstream=${normalizedUpstream[key]} rs=${normalizedRs[key]}`,
    });
  }
}
fs.writeFileSync(path.join(diffDir, 'findings.json'), JSON.stringify(findings, null, 2) + '\n');

const artifactPaths = [
  path.join(root, 'metadata.json'),
  path.join(root, 'raw', 'upstream.json'),
  path.join(root, 'raw', 'rs.json'),
  path.join(root, 'normalized', 'upstream.json'),
  path.join(root, 'normalized', 'rs.json'),
  path.join(root, 'diff', 'findings.json'),
].map((value) => value.replace(/\\/g, '/'));
const matrixItemIds = parseMatrixIds(matrixLine);
const metadata = {
  schema: 'promptfoo-rs.real-upstream-corpus.fixture.v1',
  fixture_id: fixtureId,
  matrix_item_ids: matrixItemIds,
  baseline: {
    npm: baselineNpm,
    version: npmView.version,
    gitHead: npmView.gitHead,
    tarball: (npmView.dist && npmView.dist.tarball) || npmView['dist.tarball'],
    integrity: (npmView.dist && npmView.dist.integrity) || npmView['dist.integrity'],
  },
  upstream_command: `npx --yes ${baselineNpm} eval -c promptfooconfig.yaml --output raw/upstream.json`,
  rs_command: `${rsBinary} eval -c promptfooconfig.yaml --output raw/rs.json`,
  used_test_binary: false,
  upstream_exit_code: upstreamExit,
  rs_exit_code: rsExit,
  duration_ms: Number(durationRaw),
  normalization_rules: ['time', 'path', 'random', 'latency', 'platform-newline'],
  artifact_paths: artifactPaths,
  diff_findings: findings,
  status: findings.length === 0 ? 'ready' : 'blocked',
};
fs.writeFileSync(path.join(root, 'metadata.json'), JSON.stringify(metadata, null, 2) + '\n');
console.log(JSON.stringify(metadata));
NODE

  cat "$run_root/metadata.json" >> "$fixtures_json"
  fixture_index=$((fixture_index + 1))
done

node - "$ROOT" "$fixtures_json" "$REQUIRED_P0_FIXTURE_COUNT" "$INDEX" "$SUMMARY" <<'NODE'
const fs = require('fs');
const path = require('path');

const [root, fixturesJsonlPath, requiredRaw, indexPath, summaryPath] = process.argv.slice(2);
const required = Number(requiredRaw);
const fixtures = fs
  .readFileSync(fixturesJsonlPath, 'utf8')
  .trim()
  .split(/\n(?=\{)/)
  .filter(Boolean)
  .map((line) => JSON.parse(line));
const blocking = [];
for (const fixture of fixtures) {
  if (fixture.used_test_binary) {
    blocking.push({ capability: fixture.fixture_id, path: 'used_test_binary', class: 'Bug', message: 'test binary substitute used' });
  }
  if (fixture.status !== 'ready') {
    blocking.push(...fixture.diff_findings);
  }
  for (const artifact of fixture.artifact_paths) {
    if (!fs.existsSync(artifact)) {
      blocking.push({ capability: fixture.fixture_id, path: artifact, class: 'Bug', message: 'missing artifact' });
    }
  }
}
const observed = fixtures.filter((fixture) => fixture.status === 'ready' && !fixture.used_test_binary).length;
if (observed < required) {
  blocking.push({
    capability: 'real-upstream-corpus',
    path: 'observed_p0_fixture_count',
    class: 'Bug',
    message: `P0 real upstream corpus coverage below threshold: ${observed}/${required}`,
  });
}
const status = blocking.length === 0 ? 'ready' : 'blocked';
const index = {
  schema: 'promptfoo-rs.real-upstream-corpus.v1',
  status,
  required_p0_fixture_count: required,
  observed_p0_fixture_count: observed,
  stable_allowed: status === 'ready',
  baseline: 'promptfoo@0.121.13',
  root: root.replace(/\\/g, '/'),
  fixtures,
  blocking_findings: blocking,
};
fs.writeFileSync(indexPath, JSON.stringify(index, null, 2) + '\n');
fs.writeFileSync(summaryPath, JSON.stringify({
  schema: 'promptfoo-rs.real-upstream-corpus.summary.v1',
  status,
  required_p0_fixture_count: required,
  observed_p0_fixture_count: observed,
  used_test_binary_count: fixtures.filter((fixture) => fixture.used_test_binary).length,
  fixture_count: fixtures.length,
  blocking_finding_count: blocking.length,
}, null, 2) + '\n');
NODE

node -e "const index = JSON.parse(require('fs').readFileSync(process.argv[1], 'utf8')); if (index.status !== 'ready' || index.observed_p0_fixture_count < index.required_p0_fixture_count) { console.error(JSON.stringify(index, null, 2)); process.exit(1); }" "$INDEX"
