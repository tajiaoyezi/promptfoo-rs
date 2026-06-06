#!/usr/bin/env bash
set -euo pipefail

GATE_DIR="${GATE_DIR:-target/release-gates}"
MANIFEST_PATH="${AUTHORITY_DECISIONS_MANIFEST:-docs/compatibility/authority-decisions.json}"
OUT="$GATE_DIR/authority-decisions-gate.json"

mkdir -p "$GATE_DIR"

node - "$GATE_DIR" "$MANIFEST_PATH" "$OUT" <<'NODE'
const fs = require('fs');

const [gateDir, manifestPath, outPath] = process.argv.slice(2);
const packet = JSON.parse(
  fs.readFileSync(`${gateDir}/perfect-refactor-unblock-packet.json`, 'utf8'),
);
const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));

const requiredIds = (packet.decision_items || []).map((item) => String(item.item_id));
const rows = manifest.rows || [];
const manifestIds = rows.map((row) => String(row.item_id));
const requiredSet = new Set(requiredIds);
const manifestSet = new Set(manifestIds);

const missing = requiredIds.filter((itemId) => !manifestSet.has(itemId));
const extra = manifestIds.filter((itemId) => !requiredSet.has(itemId));
const duplicates = manifestIds.filter(
  (itemId, index) => manifestIds.indexOf(itemId) !== index,
);

const mockNeedles = [
  'mock',
  'dry-run',
  'dry_run',
  'local-only',
  'local only',
  'fixture-only',
  'fixture only',
  'echo',
  'placeholder',
  'sample-only',
];

function isMockOnly(reference) {
  const lower = String(reference || '').toLowerCase();
  return mockNeedles.some((needle) => lower.includes(needle));
}

function containsSecretLikeValue(text) {
  const lower = String(text || '').trim().toLowerCase();
  if (!lower) return false;
  if (lower.startsWith('sk-') || lower.includes('sk-live-')) return true;
  if (lower.includes('bearer ') && !lower.includes('bearer <redacted>')) return true;
  return [
    'api_key=',
    'apikey=',
    'password=',
    'secret=',
    'token-123',
    'private_key',
    '-----begin ',
  ].some((needle) => lower.includes(needle));
}

function secretValuesInValue(value, secrets = []) {
  if (typeof value === 'string') {
    if (containsSecretLikeValue(value)) secrets.push(value);
    return secrets;
  }
  if (Array.isArray(value)) {
    value.forEach((item) => secretValuesInValue(item, secrets));
    return secrets;
  }
  if (value && typeof value === 'object') {
    Object.values(value).forEach((item) => secretValuesInValue(item, secrets));
  }
  return secrets;
}

function waiverMissingFields(waiver) {
  const required = [
    'owner',
    'approval_date',
    'scope',
    'expiration_or_review_date',
    'rationale',
    'release_impact',
  ];
  if (!waiver) return required;
  return required.filter((field) => !String(waiver[field] || '').trim());
}

let unresolvedCount = 0;
let readyRowCount = 0;
const invalidWaiverRows = [];
const mockOnlyEvidenceRows = [];
const secretBearingRows = [];
const blockers = [];

if (missing.length) {
  blockers.push(`${missing.length} unblock-packet decision items are missing manifest rows`);
}
if (extra.length) {
  blockers.push(`${extra.length} manifest rows do not map to unblock-packet decision items`);
}
if (duplicates.length) {
  blockers.push(`${duplicates.length} manifest rows duplicate item_id values`);
}

for (const row of rows) {
  const itemId = String(row.item_id || '');
  if (secretValuesInValue(row).length) {
    secretBearingRows.push(itemId);
    blockers.push(`${itemId}: manifest row contains secret-like values`);
  }

  switch (row.decision_state) {
    case 'unresolved':
      unresolvedCount += 1;
      blockers.push(`${itemId}: authority decision remains unresolved`);
      break;
    case 'evidence-provided': {
      const references = row.evidence_references || [];
      if (!references.length) {
        blockers.push(`${itemId}: evidence-provided requires non-empty evidence_references`);
        break;
      }
      if (
        references.some(
          (reference) =>
            isMockOnly(reference.reference)
            || String(reference.kind || '').toLowerCase().includes('mock'),
        )
      ) {
        mockOnlyEvidenceRows.push(itemId);
        blockers.push(`${itemId}: evidence-provided references are mock-only or dry-run`);
        break;
      }
      readyRowCount += 1;
      break;
    }
    case 'waived-with-boundary': {
      const missingFields = waiverMissingFields(row.waiver);
      if (missingFields.length) {
        invalidWaiverRows.push(itemId);
        blockers.push(`${itemId}: waiver missing required fields: ${missingFields.join(', ')}`);
        break;
      }
      readyRowCount += 1;
      break;
    }
    default:
      blockers.push(`${itemId}: invalid decision_state '${row.decision_state || ''}'`);
  }
}

const structuralReady = !missing.length
  && !extra.length
  && !duplicates.length
  && !invalidWaiverRows.length
  && !mockOnlyEvidenceRows.length
  && !secretBearingRows.length;
const perfectRefactorDecisionReady = structuralReady
  && requiredIds.length > 0
  && readyRowCount === requiredIds.length
  && unresolvedCount === 0;

const report = {
  schema: 'promptfoo-rs.authority-decisions-gate.v1',
  status: perfectRefactorDecisionReady ? 'ready' : 'blocked',
  perfect_refactor_decision_ready: perfectRefactorDecisionReady,
  required_decision_count: requiredIds.length,
  manifest_row_count: manifestIds.length,
  unresolved_count: unresolvedCount,
  ready_row_count: readyRowCount,
  missing_manifest_rows: missing,
  extra_manifest_rows: extra,
  duplicate_manifest_rows: [...new Set(duplicates)],
  invalid_waiver_rows: invalidWaiverRows,
  mock_only_evidence_rows: mockOnlyEvidenceRows,
  secret_bearing_rows: secretBearingRows,
  blockers,
  manifest_artifact: manifestPath,
  packet_artifact: `${gateDir}/perfect-refactor-unblock-packet.json`,
};

fs.writeFileSync(outPath, `${JSON.stringify(report, null, 2)}\n`);
NODE
