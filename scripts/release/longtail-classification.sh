#!/usr/bin/env bash
set -euo pipefail

GATE_DIR="target/release-gates"
SOURCE_ITEMS="$GATE_DIR/source-extracted-items.json"
OUT="$GATE_DIR/longtail-classification.json"

if [ ! -f "$SOURCE_ITEMS" ]; then
  bash scripts/release/source-inventory-evidence.sh
fi

cargo test --test longtail_provider_assertion_redteam_classification

node - "$SOURCE_ITEMS" "compatibility/inventory/upstream-items.json" "$OUT" <<'NODE'
const fs = require('fs');

const [sourcePath, inventoryPath, outputPath] = process.argv.slice(2);
const source = JSON.parse(fs.readFileSync(sourcePath, 'utf8'));
const inventory = JSON.parse(fs.readFileSync(inventoryPath, 'utf8'));
const categories = new Set(['provider', 'assertion', 'redteam-plugin', 'redteam-strategy']);
const trackedById = new Map((inventory.items || []).map((item) => [item.stable_id, item]));
const sourceLongtail = (source.items || []).filter((item) => categories.has(item.category));

const counts = {};
const missingTrackedRows = [];
const unresolvedRows = [];
const missingReasonRows = [];
let p0ReleaseBlockers = 0;

function classToken(item) {
  const status = String(item.status || '').toLowerCase();
  if (status === 'blocked') return 'blocked';
  if (status === 'unsupported') return 'unsupported';
  if (status === 'later' || item.level_hint === 'P2') return 'later';
  if (status === 'bridge' || item.owner_hint === 'script-bridge') return 'bridge';
  return 'native';
}

function requiresReason(item, token) {
  return item.level_hint === 'P2' || ['blocked', 'unsupported', 'later'].includes(token);
}

for (const sourceItem of sourceLongtail) {
  counts[sourceItem.category] = (counts[sourceItem.category] || 0) + 1;
  const tracked = trackedById.get(sourceItem.stable_id);
  if (!tracked) {
    missingTrackedRows.push(sourceItem.stable_id);
    continue;
  }
  const token = classToken(tracked);
  const reason = String(tracked.unresolved_reason || '').trim();
  if (String(tracked.status || '').toLowerCase() === 'unresolved') {
    unresolvedRows.push(tracked.stable_id);
  }
  if (requiresReason(tracked, token) && !reason) {
    missingReasonRows.push(tracked.stable_id);
  }
  if (tracked.level_hint === 'P0' && token === 'blocked') {
    p0ReleaseBlockers += 1;
  }
}

const blocked =
  missingTrackedRows.length > 0 || unresolvedRows.length > 0 || missingReasonRows.length > 0;
const status = blocked ? 'blocked' : p0ReleaseBlockers > 0 ? 'ready-with-blockers' : 'ready';

const report = {
  schema: 'promptfoo-rs.longtail-classification.v1',
  status,
  source_inventory: sourcePath,
  tracked_inventory: inventoryPath,
  categories: Array.from(categories).sort(),
  source_extracted_item_count: sourceLongtail.length,
  source_extracted_counts: counts,
  tracked_longtail_item_count: sourceLongtail.length - missingTrackedRows.length,
  missing_tracked_rows: missingTrackedRows,
  unresolved_rows: unresolvedRows,
  missing_reason_rows: missingReasonRows,
  p0_release_blocker_count: p0ReleaseBlockers,
  evidence: {
    unit_test: 'cargo test --test longtail_provider_assertion_redteam_classification',
    docs: 'docs/compatibility/matrix.md',
  },
};

fs.mkdirSync(require('path').dirname(outputPath), { recursive: true });
fs.writeFileSync(outputPath, JSON.stringify(report, null, 2) + '\n');
if (status === 'blocked') {
  console.error(JSON.stringify(report, null, 2));
  process.exit(1);
}
NODE
