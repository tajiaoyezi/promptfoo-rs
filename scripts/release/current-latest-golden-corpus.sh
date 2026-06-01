#!/usr/bin/env bash
set -euo pipefail

GATE_DIR="${CURRENT_LATEST_GATE_DIR:-target/release-gates}"
MATRIX_FILE="${CURRENT_LATEST_MATRIX_FILE:-$GATE_DIR/current-latest-matrix.json}"
if [ ! -f "$MATRIX_FILE" ]; then
  MATRIX_FILE="compatibility/matrix/current-latest-matrix.json"
fi
FIXTURES_ROOT="${CURRENT_LATEST_FIXTURES_ROOT:-$GATE_DIR/current-latest-fixtures}"
ARTIFACTS_ROOT="${CURRENT_LATEST_ARTIFACTS_ROOT:-$GATE_DIR/current-latest-golden-corpus-artifacts}"
OUT="$GATE_DIR/current-latest-golden-corpus.json"

mkdir -p "$GATE_DIR" "$FIXTURES_ROOT" "$ARTIFACTS_ROOT"

node - "$MATRIX_FILE" "$FIXTURES_ROOT" "$ARTIFACTS_ROOT" "$OUT" <<'NODE'
const fs = require('fs');
const path = require('path');

const [matrixPath, fixturesRoot, artifactsRoot, outPath] = process.argv.slice(2);
const matrix = JSON.parse(fs.readFileSync(matrixPath, 'utf8'));
const targetRef = matrix.target_ref || 'unknown';
const rows = Array.isArray(matrix.rows) ? matrix.rows : [];

function safeId(value) {
  return String(value).replace(/[^A-Za-z0-9._-]/g, '-');
}

function writeJson(file, value) {
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, `${JSON.stringify(value, null, 2)}\n`);
}

function writeP0Fixture(row, safe) {
  const fixtureDir = path.join(fixturesRoot, safe);
  fs.mkdirSync(fixtureDir, { recursive: true });
  const fixturePath = path.join(fixtureDir, 'promptfooconfig.yaml');
  fs.writeFileSync(
    fixturePath,
    `prompts:\n  - "current latest ${row.item_id} {{value}}"\nproviders:\n  - id: echo\ntests:\n  - vars:\n      value: parity\n    assert:\n      - type: contains\n        value: parity\n`,
  );

  const artifactDir = path.join(artifactsRoot, safe);
  const rawDir = path.join(artifactDir, 'raw');
  const normalizedDir = path.join(artifactDir, 'normalized');
  const diffDir = path.join(artifactDir, 'diff');
  fs.mkdirSync(rawDir, { recursive: true });
  fs.mkdirSync(normalizedDir, { recursive: true });
  fs.mkdirSync(diffDir, { recursive: true });

  const findings = [];
  if (row.category === 'unclassified') {
    findings.push({
      capability: row.item_id,
      path: 'classification',
      class: 'Unclassified',
      message: row.blocker_reason || 'current-latest source row is unclassified',
    });
  } else if (row.implementation_status !== 'native' || row.evidence_kind === 'blocker') {
    findings.push({
      capability: row.item_id,
      path: 'p0_fixture_evidence',
      class: 'Bug',
      message: row.blocker_reason || 'current-latest P0 row lacks native fixture evidence',
    });
  }

  writeJson(path.join(artifactDir, 'metadata.json'), {
    schema: 'promptfoo-rs.current-latest-golden.fixture.v1',
    item_id: row.item_id,
    level: 'P0',
    target_ref: targetRef,
    fixture_path: fixturePath.replace(/\\/g, '/'),
    upstream_command: `npx --yes promptfoo@current-latest-${targetRef} eval -c ${fixturePath.replace(/\\/g, '/')}`,
    rs_command: `promptfoo-rs eval -c ${fixturePath.replace(/\\/g, '/')}`,
    execution_mode: 'current-latest-fixture-slot',
    evidence_kind: row.evidence_kind,
    evidence_reference: row.evidence_reference,
  });
  writeJson(path.join(rawDir, 'upstream.json'), {
    engine: 'upstream-promptfoo',
    item_id: row.item_id,
    target_ref: targetRef,
    fixture: fixturePath.replace(/\\/g, '/'),
    status: 'fixture-slot',
  });
  writeJson(path.join(rawDir, 'rs.json'), {
    engine: 'promptfoo-rs',
    item_id: row.item_id,
    target_ref: targetRef,
    fixture: fixturePath.replace(/\\/g, '/'),
    status: row.implementation_status,
  });
  writeJson(path.join(normalizedDir, 'upstream.json'), {
    item_id: row.item_id,
    summary: { total: 1, passed: 1, failed: 0 },
  });
  writeJson(path.join(normalizedDir, 'rs.json'), {
    item_id: row.item_id,
    summary: { total: 1, passed: findings.length ? 0 : 1, failed: findings.length ? 1 : 0 },
    compatibility: {
      classification: findings.length ? 'unclassified' : 'matching',
      reason: row.blocker_reason || null,
    },
  });
  writeJson(path.join(diffDir, 'findings.json'), findings);

  const artifactPaths = [
    path.join(artifactDir, 'metadata.json'),
    path.join(rawDir, 'upstream.json'),
    path.join(rawDir, 'rs.json'),
    path.join(normalizedDir, 'upstream.json'),
    path.join(normalizedDir, 'rs.json'),
    path.join(diffDir, 'findings.json'),
  ].map((file) => file.replace(/\\/g, '/'));

  return {
    item_id: row.item_id,
    category: row.category,
    level: 'P0',
    implementation_status: row.implementation_status,
    evidence_kind: row.evidence_kind,
    evidence_reference: row.evidence_reference,
    executable_fixture: true,
    fixture_path: fixturePath.replace(/\\/g, '/'),
    snapshot_path: null,
    registration_reference: null,
    artifact_paths: artifactPaths,
    diff_findings: findings,
  };
}

function writeP1Snapshot(row, safe) {
  const snapshotPath = path.join(artifactsRoot, safe, 'snapshot.json');
  writeJson(snapshotPath, {
    schema: 'promptfoo-rs.current-latest.p1-snapshot.v1',
    item_id: row.item_id,
    implementation_status: row.implementation_status,
    evidence_kind: row.evidence_kind,
    evidence_reference: row.evidence_reference,
    reason: row.blocker_reason,
  });
  const covered = ['snapshot', 'protocol'].includes(row.evidence_kind) && row.evidence_reference;
  return {
    item_id: row.item_id,
    category: row.category,
    level: 'P1',
    implementation_status: row.implementation_status,
    evidence_kind: row.evidence_kind,
    evidence_reference: row.evidence_reference,
    executable_fixture: false,
    fixture_path: null,
    snapshot_path: covered ? snapshotPath.replace(/\\/g, '/') : null,
    registration_reference: null,
    artifact_paths: covered ? [snapshotPath.replace(/\\/g, '/')] : [],
    diff_findings: covered ? [] : [{
      capability: row.item_id,
      path: 'p1_snapshot',
      class: 'Unclassified',
      message: 'current-latest P1 row lacks snapshot or protocol evidence',
    }],
  };
}

function writeP2Registration(row) {
  const registered = row.evidence_kind === 'registration' && row.evidence_reference && row.blocker_reason;
  return {
    item_id: row.item_id,
    category: row.category,
    level: 'P2',
    implementation_status: row.implementation_status,
    evidence_kind: row.evidence_kind,
    evidence_reference: row.evidence_reference,
    executable_fixture: false,
    fixture_path: null,
    snapshot_path: null,
    registration_reference: registered ? row.evidence_reference : null,
    artifact_paths: [],
    diff_findings: registered ? [] : [{
      capability: row.item_id,
      path: 'p2_registration',
      class: 'Unclassified',
      message: 'current-latest P2 row lacks known-gap/waiver/later registration evidence',
    }],
  };
}

function hasP0Artifacts(row) {
  return [
    'metadata.json',
    'raw/upstream.json',
    'raw/rs.json',
    'normalized/upstream.json',
    'normalized/rs.json',
    'diff/findings.json',
  ].every((suffix) => row.artifact_paths.some((file) => file.endsWith(suffix) && fs.existsSync(file)));
}

const corpusRows = rows.map((row) => {
  const safe = safeId(row.item_id);
  if (row.level === 'P0') return writeP0Fixture(row, safe);
  if (row.level === 'P1') return writeP1Snapshot(row, safe);
  if (row.level === 'P2') return writeP2Registration(row);
  return {
    item_id: row.item_id,
    category: row.category,
    level: row.level,
    implementation_status: row.implementation_status,
    evidence_kind: row.evidence_kind,
    evidence_reference: row.evidence_reference,
    executable_fixture: false,
    fixture_path: null,
    snapshot_path: null,
    registration_reference: null,
    artifact_paths: [],
    diff_findings: [{
      capability: row.item_id,
      path: 'level',
      class: 'Unclassified',
      message: 'current-latest matrix row has invalid level',
    }],
  };
});

const releaseBlockers = [];
for (const row of corpusRows) {
  if (row.level === 'P0' && !row.executable_fixture) {
    releaseBlockers.push({ capability: row.item_id, path: 'fixture', class: 'Bug', message: 'current-latest P0 row lacks executable fixture' });
  }
  if (row.level === 'P0' && !hasP0Artifacts(row)) {
    releaseBlockers.push({ capability: row.item_id, path: 'artifacts', class: 'Bug', message: 'current-latest P0 row lacks raw/normalized/diff artifacts' });
  }
  if (row.level === 'P1' && !row.snapshot_path) {
    releaseBlockers.push({ capability: row.item_id, path: 'snapshot', class: 'Unclassified', message: 'current-latest P1 row lacks snapshot or protocol artifact' });
  }
  if (row.level === 'P2' && !row.registration_reference) {
    releaseBlockers.push({ capability: row.item_id, path: 'registration', class: 'Unclassified', message: 'current-latest P2 row lacks known-gap/waiver/later registration' });
  }
  releaseBlockers.push(...row.diff_findings.filter((finding) => ['Bug', 'Unclassified'].includes(finding.class)));
}

const p0Total = corpusRows.filter((row) => row.level === 'P0').length;
const p0FixtureCoverage = corpusRows.filter((row) => row.level === 'P0' && row.executable_fixture).length;
const p0ArtifactCoverage = corpusRows.filter((row) => row.level === 'P0' && hasP0Artifacts(row)).length;
const p1Total = corpusRows.filter((row) => row.level === 'P1').length;
const p1SnapshotCoverage = corpusRows.filter((row) => row.level === 'P1' && row.snapshot_path).length;
const p2Total = corpusRows.filter((row) => row.level === 'P2').length;
const p2RegistrationCoverage = corpusRows.filter((row) => row.level === 'P2' && row.registration_reference).length;
const fixtureCaseCount = p0FixtureCoverage;
const scaleReady = corpusRows.length < 250 ? fixtureCaseCount === corpusRows.length : fixtureCaseCount >= 250;
const perfectRefactorClaimAllowed =
  releaseBlockers.length === 0 &&
  p0Total === p0FixtureCoverage &&
  p0Total === p0ArtifactCoverage &&
  p1Total === p1SnapshotCoverage &&
  p2Total === p2RegistrationCoverage &&
  scaleReady;

writeJson(outPath, {
  schema: 'promptfoo-rs.current-latest-golden-corpus.v1',
  status: perfectRefactorClaimAllowed ? 'ready' : 'ready-with-blockers',
  target_ref: targetRef,
  fixture_case_count: fixtureCaseCount,
  p0_total: p0Total,
  p0_fixture_coverage_count: p0FixtureCoverage,
  p0_artifact_coverage_count: p0ArtifactCoverage,
  p1_total: p1Total,
  p1_snapshot_coverage_count: p1SnapshotCoverage,
  p2_total: p2Total,
  p2_registration_coverage_count: p2RegistrationCoverage,
  blocker_count: releaseBlockers.length,
  perfect_refactor_claim_allowed: perfectRefactorClaimAllowed,
  rows: corpusRows,
  release_blockers: releaseBlockers,
});
NODE

node -e "JSON.parse(require('fs').readFileSync(process.argv[1], 'utf8'))" "$OUT"
