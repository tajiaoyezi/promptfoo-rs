#!/usr/bin/env bash
set -euo pipefail

BASELINE_VERSION="0.121.13"
BASELINE_COMMIT="4860e990c7e9a2f8f677173fb92cf9867b34d03f"
BASELINE_REF="refs/tags/${BASELINE_VERSION}"
UPSTREAM_REPO="https://github.com/promptfoo/promptfoo.git"
# frozen npm artifact: promptfoo@0.121.13

GATE_DIR="target/release-gates"
mkdir -p "$GATE_DIR"
OUT="$GATE_DIR/source-inventory-evidence.json"
ITEMS_OUT="$GATE_DIR/source-extracted-items.json"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

npm view "promptfoo@${BASELINE_VERSION}" version gitHead dist.tarball dist.integrity --json > "$tmpdir/npm-view.json"
git clone --quiet --filter=blob:none --no-checkout --depth 1 --branch "$BASELINE_VERSION" "$UPSTREAM_REPO" "$tmpdir/upstream"
git -C "$tmpdir/upstream" rev-parse HEAD > "$tmpdir/git-head.txt"
git -C "$tmpdir/upstream" ls-tree -r --name-only HEAD > "$tmpdir/source-files.txt"

node - "$tmpdir/npm-view.json" "$tmpdir/git-head.txt" "$tmpdir/source-files.txt" "compatibility/matrix/items.json" "compatibility/inventory/upstream-items.json" "$ITEMS_OUT" "$OUT" <<'NODE'
const fs = require('fs');

const [
  npmViewPath,
  gitHeadPath,
  sourceFilesPath,
  matrixPath,
  curatedInventoryPath,
  itemsOutputPath,
  evidenceOutputPath,
] = process.argv.slice(2);

const baselineVersion = '0.121.13';
const baselineCommit = '4860e990c7e9a2f8f677173fb92cf9867b34d03f';
const baselineRef = `refs/tags/${baselineVersion}`;
const npmView = JSON.parse(fs.readFileSync(npmViewPath, 'utf8'));
const gitHead = fs.readFileSync(gitHeadPath, 'utf8').trim();
const sourceFiles = fs
  .readFileSync(sourceFilesPath, 'utf8')
  .split(/\r?\n/)
  .map((file) => file.trim().replace(/^package\//, '').replace(/\\/g, '/'))
  .filter(Boolean);
const curatedInventory = JSON.parse(fs.readFileSync(curatedInventoryPath, 'utf8')).items;
const matrixManifest = JSON.parse(fs.readFileSync(matrixPath, 'utf8'));

const auditBaselineCounts = {
  command_related_files: 85,
  provider_files: 219,
  assertion_files: 56,
  redteam_files: 217,
  redteam_plugin_files: 125,
  redteam_strategy_files: 32,
  viewer_app_files: 701,
  example_files: 1220,
};

const sourceCounts = {
  command_related_files: 0,
  provider_files: 0,
  assertion_files: 0,
  redteam_files: 0,
  redteam_plugin_files: 0,
  redteam_strategy_files: 0,
  viewer_app_files: 0,
  example_files: 0,
  output_files: 0,
  config_files: 0,
};

function slug(value) {
  return value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
}

function stableId(category, name) {
  return `${slug(category)}:${slug(name)}`;
}

function withoutExtension(file) {
  return file.replace(/\.[^.]+$/, '');
}

function isTsOrJs(file) {
  return /\.(tsx?|jsx?|mjs|cjs)$/.test(file);
}

function isCommandRelated(file) {
  return (
    (file === 'src/main.ts' ||
      file.startsWith('src/commands/') ||
      file.startsWith('src/redteam/commands/') ||
      file.startsWith('src/codeScan/')) &&
    isTsOrJs(file)
  );
}

function isProvider(file) {
  return file.startsWith('src/providers/') && isTsOrJs(file);
}

function isAssertion(file) {
  return file.startsWith('src/assertions/') && isTsOrJs(file);
}

function isRedteamPlugin(file) {
  return file.startsWith('src/redteam/plugins/') && isTsOrJs(file);
}

function isRedteamStrategy(file) {
  return file.startsWith('src/redteam/strategies/') && isTsOrJs(file);
}

function isRedteam(file) {
  return file.startsWith('src/redteam/') && isTsOrJs(file);
}

function isViewer(file) {
  return file.startsWith('src/app/');
}

function isExample(file) {
  return file.startsWith('examples/');
}

function isOutput(file) {
  return (
    file.startsWith('src/') &&
    /(output|report|csv|junit|sarif|yaml|jsonl)/i.test(file)
  );
}

function isConfig(file) {
  return file.startsWith('src/') && /config/i.test(file);
}

function isP0Provider(file) {
  return [
    'src/providers/openai',
    'src/providers/http',
    'src/providers/ollama',
    'src/providers/anthropic',
  ].some((prefix) => file.startsWith(prefix));
}

function classify(category, file) {
  if (category === 'provider' && isP0Provider(file)) {
    return { level_hint: 'P0', status: 'discovered', owner_hint: 'provider-runtime', unresolved_reason: null };
  }
  if (category === 'provider') {
    return {
      level_hint: 'P2',
      status: 'unresolved',
      owner_hint: 'provider-runtime',
      unresolved_reason: 'source-extracted long-tail provider requires task-17.4 classification',
    };
  }
  if (category === 'config') {
    return { level_hint: 'P0', status: 'discovered', owner_hint: 'config', unresolved_reason: null };
  }
  if (category === 'command') {
    return { level_hint: 'P1', status: 'discovered', owner_hint: 'cli', unresolved_reason: null };
  }
  if (category === 'assertion') {
    return { level_hint: 'P1', status: 'discovered', owner_hint: 'assertion-engine', unresolved_reason: null };
  }
  if (category === 'redteam-plugin' || category === 'redteam-strategy') {
    return { level_hint: 'P1', status: 'discovered', owner_hint: 'redteam-engine', unresolved_reason: null };
  }
  if (category === 'viewer') {
    return { level_hint: 'P1', status: 'discovered', owner_hint: 'viewer', unresolved_reason: null };
  }
  if (category === 'output') {
    return { level_hint: 'P1', status: 'discovered', owner_hint: 'reporting', unresolved_reason: null };
  }
  return {
    level_hint: 'P2',
    status: 'discovered',
    owner_hint: 'compatibility',
    unresolved_reason: 'source-extracted evidence row requires task-17.4 classification if promoted',
  };
}

const itemsById = new Map();

function addItem(category, file) {
  const name = withoutExtension(file);
  const id = stableId(category, name);
  if (itemsById.has(id)) {
    return;
  }
  const classification = classify(category, file);
  itemsById.set(id, {
    stable_id: id,
    category,
    name,
    source_reference: `promptfoo@${baselineVersion}:${file}`,
    ...classification,
  });
}

for (const file of sourceFiles) {
  if (isCommandRelated(file)) {
    sourceCounts.command_related_files += 1;
    addItem('command', file);
  }
  if (isProvider(file)) {
    sourceCounts.provider_files += 1;
    addItem('provider', file);
  }
  if (isAssertion(file)) {
    sourceCounts.assertion_files += 1;
    addItem('assertion', file);
  }
  if (isRedteam(file)) {
    sourceCounts.redteam_files += 1;
  }
  if (isRedteamPlugin(file)) {
    sourceCounts.redteam_plugin_files += 1;
    addItem('redteam-plugin', file);
  }
  if (isRedteamStrategy(file)) {
    sourceCounts.redteam_strategy_files += 1;
    addItem('redteam-strategy', file);
  }
  if (isViewer(file)) {
    sourceCounts.viewer_app_files += 1;
    addItem('viewer', file);
  }
  if (isExample(file)) {
    sourceCounts.example_files += 1;
    addItem('example', file);
  }
  if (isOutput(file)) {
    sourceCounts.output_files += 1;
    addItem('output', file);
  }
  if (isConfig(file)) {
    sourceCounts.config_files += 1;
    addItem('config', file);
  }
}

const items = Array.from(itemsById.values()).sort((left, right) =>
  left.stable_id.localeCompare(right.stable_id),
);

const matrixRows = new Set();
if (Array.isArray(matrixManifest.rows)) {
  for (const row of matrixManifest.rows) {
    matrixRows.add(row.capability);
  }
} else if (matrixManifest.source_inventory) {
  for (const item of curatedInventory) {
    matrixRows.add(item.stable_id);
  }
}

const itemsMissingMetadata = items
  .filter(
    (item) =>
      !item.stable_id ||
      !item.category ||
      !item.name ||
      !item.source_reference ||
      !['P0', 'P1', 'P2'].includes(item.level_hint) ||
      !item.owner_hint ||
      (item.status === 'unresolved' && !item.unresolved_reason),
  )
  .map((item) => item.stable_id || '<missing-stable-id>');

const missingMatrixRows = items
  .filter((item) => !matrixRows.has(item.stable_id))
  .map((item) => item.stable_id);

const releaseBlockers = [];
for (const itemId of missingMatrixRows) {
  releaseBlockers.push({
    item_id: itemId,
    reason: 'missing matrix row for source-extracted item',
  });
}
for (const itemId of itemsMissingMetadata) {
  releaseBlockers.push({
    item_id: itemId,
    reason: 'source-extracted item missing required metadata',
  });
}
for (const [key, minimum] of Object.entries(auditBaselineCounts)) {
  if ((sourceCounts[key] || 0) < minimum) {
    releaseBlockers.push({
      item_id: `source-count:${key}`,
      reason: `source count below 2026-05-31 audit baseline: observed ${sourceCounts[key] || 0}, expected at least ${minimum}`,
    });
  }
}

const baselineProblems = [];
const npmIntegrity = npmView.dist && npmView.dist.integrity
  ? npmView.dist.integrity
  : npmView['dist.integrity'];
const npmTarball = npmView.dist && npmView.dist.tarball
  ? npmView.dist.tarball
  : npmView['dist.tarball'];
if (npmView.version !== baselineVersion) {
  baselineProblems.push(`npm version mismatch: ${npmView.version}`);
}
if (npmView.gitHead !== baselineCommit) {
  baselineProblems.push(`npm gitHead mismatch: ${npmView.gitHead}`);
}
if (gitHead !== baselineCommit) {
  baselineProblems.push(`git tag commit mismatch: ${gitHead}`);
}
if (!String(npmIntegrity || '').startsWith('sha512-')) {
  baselineProblems.push('npm integrity missing sha512 value');
}

for (const problem of baselineProblems) {
  releaseBlockers.push({ item_id: 'baseline', reason: problem });
}

const status =
  baselineProblems.length > 0 || itemsMissingMetadata.length > 0
    ? 'blocked'
    : releaseBlockers.length > 0
      ? 'ready-with-blockers'
      : 'ready';

const baseline = {
  package_version: baselineVersion,
  git_ref: baselineRef,
  git_commit: baselineCommit,
  npm_integrity: npmIntegrity,
  npm_tarball: npmTarball,
  acquisition_command: `git clone --filter=blob:none --no-checkout --depth 1 --branch ${baselineVersion} https://github.com/promptfoo/promptfoo.git && git ls-tree -r --name-only ${baselineCommit}`,
};
const extractionTimestamp = new Date().toISOString();

fs.writeFileSync(
  itemsOutputPath,
  JSON.stringify(
    {
      schema: 'promptfoo-rs.source-extracted-items.v1',
      baseline,
      extraction_timestamp: extractionTimestamp,
      source_counts: sourceCounts,
      items,
    },
    null,
    2,
  ) + '\n',
);

fs.writeFileSync(
  evidenceOutputPath,
  JSON.stringify(
    {
      schema: 'promptfoo-rs.source-inventory-evidence.v2',
      status,
      baseline,
      extraction_timestamp: extractionTimestamp,
      extraction_mode: 'frozen-git-tag-source-tree-plus-npm-integrity',
      audit_baseline_counts: auditBaselineCounts,
      source_counts: sourceCounts,
      inventory_item_count: curatedInventory.length,
      source_extracted_item_count: items.length,
      items_missing_metadata: itemsMissingMetadata,
      missing_matrix_rows: missingMatrixRows,
      silent_omissions: [],
      release_blockers: releaseBlockers,
    },
    null,
    2,
  ) + '\n',
);
NODE

node -e "const e = JSON.parse(require('fs').readFileSync(process.argv[1], 'utf8')); if (e.status === 'blocked') { console.error(JSON.stringify(e, null, 2)); process.exit(1); } if (e.status === 'ready-with-blockers') { console.error('source inventory evidence recorded release blockers; see ' + process.argv[1]); }" "$OUT"
