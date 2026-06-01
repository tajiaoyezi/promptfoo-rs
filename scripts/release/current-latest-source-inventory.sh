#!/usr/bin/env bash
set -euo pipefail

GATE_DIR="${CURRENT_LATEST_GATE_DIR:-target/release-gates}"
LOCK_FILE="${CURRENT_LATEST_TARGET_LOCK_FILE:-$GATE_DIR/current-latest-target.json}"
OUT="$GATE_DIR/current-latest-source-inventory.json"
MATRIX_OUT="$GATE_DIR/current-latest-matrix.json"
UPSTREAM_REPO="${CURRENT_LATEST_UPSTREAM_REPO:-https://github.com/promptfoo/promptfoo.git}"

mkdir -p "$GATE_DIR"

if [ ! -f "$LOCK_FILE" ] && [ -f "compatibility/inventory/current-latest-target.json" ]; then
  LOCK_FILE="compatibility/inventory/current-latest-target.json"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

source_root="${CURRENT_LATEST_SOURCE_ROOT:-}"
if [ -z "$source_root" ]; then
  head_sha="$(node -e "const r = JSON.parse(require('fs').readFileSync(process.argv[1], 'utf8')); console.log(r.github.default_branch_head)" "$LOCK_FILE")"
  source_root="$tmpdir/upstream"
  source_root_label="git:${UPSTREAM_REPO}#${head_sha}"
  git init --quiet "$source_root"
  git -C "$source_root" remote add origin "$UPSTREAM_REPO"
  git -C "$source_root" fetch --quiet --depth 1 origin "$head_sha"
  git -C "$source_root" checkout --quiet --detach FETCH_HEAD
else
  source_root_label="$source_root"
fi

node - "$LOCK_FILE" "$source_root" "$OUT" "$MATRIX_OUT" "$source_root_label" <<'NODE'
const fs = require('fs');
const path = require('path');

const [lockPath, sourceRoot, inventoryPath, matrixPath, sourceRootLabel] = process.argv.slice(2);
const lock = JSON.parse(fs.readFileSync(lockPath, 'utf8'));
const head = lock.github && lock.github.default_branch_head;
if (!/^[0-9a-fA-F]{40}$/.test(head || '')) {
  throw new Error('current latest lock missing full default_branch_head SHA');
}

function slug(value) {
  return String(value)
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
}

function stableId(category, name) {
  return `${slug(category)}:${slug(name)}`;
}

function normalize(file) {
  return file.replace(/\\/g, '/').replace(/^package\//, '');
}

function withoutExtension(file) {
  return file.replace(/\.[^.]+$/, '');
}

function isTsOrJs(file) {
  return /\.(tsx?|jsx?|mjs|cjs)$/.test(file);
}

function isCommand(file) {
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

function isOutput(file) {
  return file.startsWith('src/') && /(output|report|csv|junit|sarif|yaml|jsonl)/i.test(file);
}

function isConfig(file) {
  return file.startsWith('src/') && /config/i.test(file);
}

function isViewer(file) {
  return file.startsWith('src/app/') || file.startsWith('src/server/') || file.startsWith('src/openapi/');
}

function isNodeApi(file) {
  return (
    (file === 'src/index.ts' ||
      file === 'src/index.js' ||
      file.startsWith('src/node/') ||
      file.startsWith('npm/src/') ||
      file.startsWith('packages/node/')) &&
    isTsOrJs(file)
  );
}

function isExample(file) {
  return file.startsWith('examples/');
}

function isDocs(file) {
  const lower = file.toLowerCase();
  return file.startsWith('docs/') && (lower.endsWith('.md') || lower.endsWith('.mdx'));
}

function isP0Provider(file) {
  return ['src/providers/openai', 'src/providers/http', 'src/providers/ollama', 'src/providers/anthropic'].some(
    (prefix) => file.startsWith(prefix),
  );
}

function isEvalRuntime(file) {
  return (
    isTsOrJs(file) &&
    (['src/evaluate.ts', 'src/evaluator.ts', 'src/evaluatorHelpers.ts', 'src/testCase.ts'].includes(file) ||
      file.startsWith('src/scheduler/') ||
      file.startsWith('src/testCase/') ||
      file.startsWith('src/optimizer/'))
  );
}

function isCacheStore(file) {
  return (
    isTsOrJs(file) &&
    (file === 'src/cache.ts' || file.startsWith('src/database/') || file.startsWith('src/storage/'))
  );
}

function isPromptProcessing(file) {
  return (
    isTsOrJs(file) &&
    (file.startsWith('src/prompts/') ||
      file.startsWith('src/external/prompts/') ||
      file.startsWith('src/optimizer/'))
  );
}

function isAssertionSupport(file) {
  return (
    isTsOrJs(file) &&
    (file.startsWith('src/matchers/') ||
      file.startsWith('src/external/matchers/') ||
      file.startsWith('src/external/assertions/') ||
      ['src/remoteGrading.ts', 'src/remoteScoring.ts', 'src/guardrails.ts'].includes(file))
  );
}

function isRedteamSupport(file) {
  return file.startsWith('src/redteam/') && isTsOrJs(file);
}

function isSchema(file) {
  return (
    isTsOrJs(file) &&
    (file === 'src/contracts.ts' ||
      file.startsWith('src/types/') ||
      file.startsWith('src/contracts/') ||
      file.startsWith('src/models/') ||
      file.startsWith('src/validators/'))
  );
}

function isScriptBridge(file) {
  return isTsOrJs(file) && (file.startsWith('src/python/') || file.startsWith('src/ruby/'));
}

function isImportExport(file) {
  return isTsOrJs(file) && (file.startsWith('src/importers/') || file.startsWith('src/util/exportToFile/'));
}

function isIntegration(file) {
  return (
    isTsOrJs(file) &&
    (file.startsWith('src/integrations/') || ['src/googleSheets.ts', 'src/microsoftSharepoint.ts'].includes(file))
  );
}

function isCloudShare(file) {
  return (
    isTsOrJs(file) &&
    ([
      'src/share.ts',
      'src/feedback.ts',
      'src/onboarding.ts',
      'src/suggestions.ts',
      'src/telemetry.ts',
      'src/telemetryEvents.ts',
      'src/updates.ts',
    ].includes(file) ||
      file.startsWith('src/updates/'))
  );
}

function isBlobStore(file) {
  return file.startsWith('src/blobs/') && isTsOrJs(file);
}

function isRuntimeSupport(file) {
  return (
    isTsOrJs(file) &&
    (file.startsWith('src/util/') ||
      file.startsWith('src/constants/') ||
      file.startsWith('src/__mocks__/') ||
      [
        'src/cliState.ts',
        'src/constants.ts',
        'src/entrypoint.ts',
        'src/envars.ts',
        'src/envOverrides.ts',
        'src/esm.ts',
        'src/logger.ts',
        'src/logger.browser.ts',
        'src/mainUtils.ts',
        'src/migrate.ts',
        'src/table.ts',
        'src/version.ts',
      ].includes(file))
  );
}

function categoriesFor(file) {
  const categories = [];
  if (isCommand(file)) categories.push('command');
  if (isProvider(file)) categories.push('provider');
  if (isAssertion(file)) categories.push('assertion');
  if (isRedteamPlugin(file)) categories.push('redteam-plugin');
  if (isRedteamStrategy(file)) categories.push('redteam-strategy');
  if (isOutput(file)) categories.push('output');
  if (isConfig(file)) categories.push('config');
  if (isViewer(file)) categories.push('viewer');
  if (isNodeApi(file)) categories.push('node-api');
  if (isExample(file)) categories.push('example');
  if (isDocs(file)) categories.push('docs');
  if (categories.length === 0 && isEvalRuntime(file)) categories.push('eval-runner');
  if (categories.length === 0 && isCacheStore(file)) categories.push('cache-store');
  if (categories.length === 0 && isPromptProcessing(file)) categories.push('prompt-processing');
  if (categories.length === 0 && isAssertionSupport(file)) categories.push('assertion-support');
  if (categories.length === 0 && isRedteamSupport(file)) categories.push('redteam-support');
  if (categories.length === 0 && isSchema(file)) categories.push('schema');
  if (categories.length === 0 && isScriptBridge(file)) categories.push('script-bridge');
  if (categories.length === 0 && isImportExport(file)) categories.push('import-export');
  if (categories.length === 0 && isIntegration(file)) categories.push('integration');
  if (categories.length === 0 && isCloudShare(file)) categories.push('cloud-share');
  if (categories.length === 0 && isBlobStore(file)) categories.push('blob-store');
  if (categories.length === 0 && isRuntimeSupport(file)) categories.push('runtime-support');
  if (categories.length === 0 && file.startsWith('src/') && isTsOrJs(file)) {
    categories.push('unclassified');
  }
  return categories;
}

function metadata(category, id, file) {
  if (category === 'command') return ['P1', 'later', 'cli', 'snapshot', `current-latest command requires CLI behavior snapshot or fixture evidence; item: ${id}`];
  if (category === 'flag') return ['P1', 'later', 'cli', 'snapshot', `current-latest flag requires CLI parity snapshot or fixture evidence; item: ${id}`];
  if (category === 'provider' && isP0Provider(file)) return ['P0', 'blocked', 'provider-runtime', 'blocker', `current-latest P0 provider requires native or bridge fixture evidence; item: ${id}`];
  if (category === 'provider') return ['P2', 'later', 'provider-runtime', 'registration', `current-latest long-tail provider is registered until fixture evidence promotes it; item: ${id}`];
  if (category === 'assertion') return ['P1', 'later', 'assertion-engine', 'snapshot', `current-latest assertion requires snapshot evidence; item: ${id}`];
  if (category === 'redteam-plugin' || category === 'redteam-strategy') return ['P1', 'later', 'redteam-engine', 'snapshot', `current-latest redteam surface requires snapshot evidence; item: ${id}`];
  if (category === 'output') return ['P1', 'later', 'reporting', 'snapshot', `current-latest output surface requires output contract snapshot; item: ${id}`];
  if (category === 'config') return ['P0', 'blocked', 'config-loader', 'blocker', `current-latest config surface requires fixture evidence; item: ${id}`];
  if (category === 'eval-runner') return ['P0', 'blocked', 'eval-runner', 'blocker', `current-latest eval runtime requires fixture evidence; item: ${id}`];
  if (category === 'cache-store') return ['P0', 'blocked', 'cache-resume-store', 'blocker', `current-latest cache and result store surface requires fixture evidence; item: ${id}`];
  if (category === 'prompt-processing') return ['P0', 'blocked', 'config-loader', 'blocker', `current-latest prompt processing surface requires fixture evidence; item: ${id}`];
  if (category === 'script-bridge') return ['P0', 'blocked', 'script-bridge', 'blocker', `current-latest script bridge surface requires authorized subprocess fixture evidence; item: ${id}`];
  if (category === 'viewer') return ['P1', 'later', 'web-viewer', 'snapshot', `current-latest viewer surface requires data-contract or browser snapshot; item: ${id}`];
  if (category === 'assertion-support') return ['P1', 'later', 'assertion-engine', 'snapshot', `current-latest assertion support surface requires matcher or grading snapshot evidence; item: ${id}`];
  if (category === 'redteam-support') return ['P1', 'later', 'redteam-engine', 'snapshot', `current-latest redteam support surface requires registry or behavior snapshot evidence; item: ${id}`];
  if (category === 'schema') return ['P1', 'later', 'protocol', 'snapshot', `current-latest schema/model/contract surface requires protocol snapshot evidence; item: ${id}`];
  if (category === 'import-export') return ['P1', 'later', 'output-writers', 'snapshot', `current-latest import/export surface requires conversion snapshot evidence; item: ${id}`];
  if (category === 'blob-store') return ['P1', 'later', 'eval-runner', 'snapshot', `current-latest blob and media storage surface requires data-contract snapshot evidence; item: ${id}`];
  if (category === 'runtime-support') return ['P1', 'later', 'runtime', 'snapshot', `current-latest runtime support surface requires deterministic snapshot evidence; item: ${id}`];
  if (category === 'node-api') return ['P1', 'later', 'node-api-wrapper', 'snapshot', `current-latest Node API surface requires wrapper contract snapshot; item: ${id}`];
  if (category === 'example') return ['P2', 'later', 'compatibility', 'registration', `current-latest example is registered unless promoted into P0/P1 corpus; item: ${id}`];
  if (category === 'docs') return ['P2', 'later', 'compatibility', 'registration', `current-latest documented workflow is registered until mapped to executable evidence; item: ${id}`];
  if (category === 'integration') return ['P2', 'later', 'compatibility', 'registration', `current-latest external integration is registered until promoted with fixture or authority evidence; item: ${id}`];
  if (category === 'cloud-share') return ['P2', 'unsupported', 'compatibility', 'registration', `current-latest cloud/share surface remains local-first unsupported unless legal brand and service authority are provided; item: ${id}`];
  return ['P0', 'blocked', 'compatibility', 'blocker', `current-latest source row is unclassified and must be mapped before any perfect-refactor claim; item: ${id}`];
}

function evidenceReference(category, id) {
  if (['provider', 'config', 'eval-runner', 'cache-store', 'prompt-processing', 'script-bridge', 'unclassified'].includes(category)) return `blocker:${id}`;
  if (['example', 'docs', 'integration', 'cloud-share'].includes(category)) return `registration:${id}`;
  return `snapshot:${id}`;
}

function sourceReference(file, fragment) {
  return `promptfoo@current-latest:${head}:${file}${fragment ? `#${fragment}` : ''}`;
}

function walk(current, out) {
  for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
    if (entry.isDirectory()) {
      if (['.git', 'node_modules', 'target', '.turbo', '.next', 'dist', 'build'].includes(entry.name)) {
        continue;
      }
      walk(path.join(current, entry.name), out);
      continue;
    }
    if (!entry.isFile()) continue;
    out.push(normalize(path.relative(sourceRoot, path.join(current, entry.name))));
  }
}

function flagsFrom(content) {
  return Array.from(new Set(Array.from(content.matchAll(/--([A-Za-z0-9][A-Za-z0-9_-]*)/g)).map((match) => match[1]))).sort();
}

function addRow(rows, category, file, name, fragment) {
  const slugName = slug(name);
  const id = stableId(category, slugName);
  if (rows.has(id)) return;
  const [level, implementationStatus, owner, evidenceKind, reason] = metadata(category, id, file);
  rows.set(id, {
    stable_id: id,
    category,
    name: slugName,
    source_reference: sourceReference(file, fragment),
    source_file: file,
    level,
    implementation_status: implementationStatus,
    verification_owner: owner,
    evidence_kind: evidenceKind,
    evidence_reference: evidenceReference(category, id),
    blocker_reason: reason,
  });
}

const sourceFiles = [];
walk(sourceRoot, sourceFiles);
sourceFiles.sort();

const rows = new Map();
const sourceCounts = {
  command_related_files: 0,
  provider_files: 0,
  assertion_files: 0,
  redteam_plugin_files: 0,
  redteam_strategy_files: 0,
  viewer_app_files: 0,
  example_files: 0,
  output_files: 0,
  config_files: 0,
};

for (const file of sourceFiles) {
  for (const category of categoriesFor(file)) {
    if (category === 'command') sourceCounts.command_related_files += 1;
    if (category === 'provider') sourceCounts.provider_files += 1;
    if (category === 'assertion') sourceCounts.assertion_files += 1;
    if (category === 'redteam-plugin') sourceCounts.redteam_plugin_files += 1;
    if (category === 'redteam-strategy') sourceCounts.redteam_strategy_files += 1;
    if (category === 'viewer') sourceCounts.viewer_app_files += 1;
    if (category === 'example') sourceCounts.example_files += 1;
    if (category === 'output') sourceCounts.output_files += 1;
    if (category === 'config') sourceCounts.config_files += 1;
    addRow(rows, category, file, withoutExtension(file), null);
  }
  let content = '';
  try {
    content = fs.readFileSync(path.join(sourceRoot, file), 'utf8');
  } catch (_) {
    content = '';
  }
  for (const flag of flagsFrom(content)) {
    addRow(rows, 'flag', file, flag, `--${flag}`);
  }
}

const inventoryRows = Array.from(rows.values()).sort((left, right) => left.stable_id.localeCompare(right.stable_id));
const unclassifiedRows = inventoryRows.filter((row) => row.category === 'unclassified').map((row) => row.stable_id);
const rowsMissingEvidence = inventoryRows
  .filter((row) => !row.evidence_kind || !row.evidence_reference)
  .map((row) => row.stable_id);
const categories = Array.from(new Set(inventoryRows.map((row) => row.category))).sort();
const inventoryStatus = unclassifiedRows.length || rowsMissingEvidence.length ? 'ready-with-blockers' : 'ready';
const extractionTimestamp = `unix:${Math.floor(Date.now() / 1000)}`;

const inventory = {
  schema: 'promptfoo-rs.current-latest-source-inventory.v1',
  status: inventoryStatus,
  target: lock,
  extraction_mode: 'current-latest-locked-source-tree',
  source_root: sourceRootLabel,
  extraction_timestamp: extractionTimestamp,
  source_counts: sourceCounts,
  rows: inventoryRows,
  categories,
  unclassified_rows: unclassifiedRows,
  rows_missing_evidence: rowsMissingEvidence,
  perfect_refactor_claim_allowed: false,
};

const matrixRows = inventoryRows.map((row) => ({
  item_id: row.stable_id,
  category: row.category,
  source_reference: row.source_reference,
  level: row.level,
  implementation_status: row.implementation_status,
  verification_owner: row.verification_owner,
  evidence_kind: row.evidence_kind,
  evidence_reference: row.evidence_reference,
  blocker_reason: row.blocker_reason,
}));
const matrix = {
  schema: 'promptfoo-rs.current-latest-matrix.v1',
  status: unclassifiedRows.length || rowsMissingEvidence.length ? 'ready-with-blockers' : 'ready',
  target_ref: head,
  rows: matrixRows,
  unclassified_rows: unclassifiedRows,
  rows_missing_evidence: rowsMissingEvidence,
  perfect_refactor_claim_allowed:
    unclassifiedRows.length === 0 &&
    rowsMissingEvidence.length === 0 &&
    matrixRows.every((row) => row.implementation_status === 'native' && row.evidence_kind !== 'blocker' && !row.blocker_reason),
};

fs.writeFileSync(inventoryPath, `${JSON.stringify(inventory, null, 2)}\n`);
fs.writeFileSync(matrixPath, `${JSON.stringify(matrix, null, 2)}\n`);

if (process.env.CURRENT_LATEST_WRITE_TRACKED === '1') {
  fs.mkdirSync('compatibility/inventory', { recursive: true });
  fs.mkdirSync('compatibility/matrix', { recursive: true });
  fs.writeFileSync('compatibility/inventory/current-latest-source-inventory.json', `${JSON.stringify(inventory, null, 2)}\n`);
  fs.writeFileSync('compatibility/matrix/current-latest-matrix.json', `${JSON.stringify(matrix, null, 2)}\n`);
}
NODE

node -e "JSON.parse(require('fs').readFileSync(process.argv[1], 'utf8')); JSON.parse(require('fs').readFileSync(process.argv[2], 'utf8'))" "$OUT" "$MATRIX_OUT"
