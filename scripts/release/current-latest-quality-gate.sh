#!/usr/bin/env bash
set -euo pipefail

GATE_DIR="${CURRENT_LATEST_GATE_DIR:-target/release-gates}"
OUT="$GATE_DIR/current-latest-quality.json"

mkdir -p "$GATE_DIR"

node - "$GATE_DIR" "$OUT" <<'NODE'
const fs = require('fs');
const path = require('path');

const [gateDir, outPath] = process.argv.slice(2);
const allowedClaim = 'no known release-blocking defects under declared gates';
const forbiddenExamples = [
  'no potential bugs',
  'zero possible bugs',
  'bug-free',
  'no possible bugs',
];

function readJson(file, fallback = {}) {
  try {
    return JSON.parse(fs.readFileSync(file, 'utf8'));
  } catch (_) {
    return fallback;
  }
}

function env(name, fallback) {
  return process.env[name] || fallback;
}

function readiness(status) {
  return String(status || '').toLowerCase() === 'ready';
}

function forbiddenClaim(wording) {
  const lower = String(wording || '').toLowerCase();
  return [
    'no potential bug',
    'no potential bugs',
    'zero possible bug',
    'zero possible bugs',
    'no possible bug',
    'no possible bugs',
    'zero bugs',
    'no bugs',
    'bug-free',
    'bug free',
  ].some((needle) => lower.includes(needle));
}

function findArtifact(sourceArtifacts, needle) {
  return sourceArtifacts.find((artifact) => artifact.endsWith(needle)) || `${gateDir}/${needle}`;
}

function blocker(itemId, category, sourceArtifact, reason, requiredDecision) {
  return {
    item_id: itemId,
    category,
    source_artifact: sourceArtifact,
    reason,
    required_decision: requiredDecision,
  };
}

function pushStatusBlocker(blockers, itemId, category, sourceArtifact, status, requiredDecision) {
  if (!readiness(status)) {
    blockers.push(blocker(
      itemId,
      category,
      sourceArtifact,
      `${category} status is ${status}`,
      requiredDecision,
    ));
  }
}

const target = readJson(`${gateDir}/current-latest-target.json`);
const sourceInventory = readJson(`${gateDir}/current-latest-source-inventory.json`);
const matrix = readJson(`${gateDir}/current-latest-matrix.json`);
const golden = readJson(`${gateDir}/current-latest-golden-corpus.json`);
const external = readJson(`${gateDir}/external-authority-blockers.json`);
const publication = readJson(`${gateDir}/publication-authority.json`);
const authorityGate = readJson(`${gateDir}/authority-decisions-gate.json`);
const publicationEvidenceGate = readJson(`${gateDir}/publication-evidence-gate.json`);
const upstreamPolicy = readJson(`${gateDir}/current-upstream-policy.json`);

const sourceArtifacts = [
  `${gateDir}/current-latest-target.json`,
  `${gateDir}/current-latest-source-inventory.json`,
  `${gateDir}/current-latest-matrix.json`,
  `${gateDir}/current-latest-golden-corpus.json`,
  `${gateDir}/release-candidate.json`,
  `${gateDir}/perfect-refactor-claim.json`,
  `${gateDir}/external-authority-blockers.json`,
  `${gateDir}/publication-authority.json`,
];

const currentTargetStatus = env(
  'CURRENT_LATEST_CURRENT_TARGET_STATUS',
  target.target_selection_blocker_resolved === true ? 'ready' : (target.status || 'blocked'),
);
const currentTargetClaimAllowed =
  env('CURRENT_LATEST_CURRENT_TARGET_CLAIM_ALLOWED', target.current_latest_claim_allowed === true ? 'true' : 'false') === 'true';
const sourceStatus = env('CURRENT_LATEST_SOURCE_INVENTORY_STATUS', sourceInventory.status || 'blocked');
const sourceUnclassified = Number(env(
  'CURRENT_LATEST_SOURCE_INVENTORY_UNCLASSIFIED_COUNT',
  Array.isArray(sourceInventory.unclassified_rows) ? sourceInventory.unclassified_rows.length : 0,
));
const matrixStatus = env('CURRENT_LATEST_MATRIX_STATUS', matrix.status || 'blocked');
const matrixUnclassified = Number(env(
  'CURRENT_LATEST_MATRIX_UNCLASSIFIED_COUNT',
  Array.isArray(matrix.unclassified_rows) ? matrix.unclassified_rows.length : 0,
));
const goldenStatus = env('CURRENT_LATEST_GOLDEN_CORPUS_STATUS', golden.status || 'blocked');
const goldenBlockers = Number(env(
  'CURRENT_LATEST_GOLDEN_CORPUS_BLOCKER_COUNT',
  golden.active_blocker_count ?? golden.blocker_count ?? 0,
));
const goldenAuditBlockers = Number(golden.blocker_count || 0);
const adapterStatus = env('CURRENT_LATEST_ADAPTER_STATUS', 'ready');
const regressionStatus = env('CURRENT_LATEST_REGRESSION_STATUS', 'ready');
const stressStatus = env('CURRENT_LATEST_STRESS_STATUS', 'ready');
const propertyStatus = env('CURRENT_LATEST_PROPERTY_STATUS', 'ready');
const runtimeSmokeStatus = env('CURRENT_LATEST_RUNTIME_SMOKE_STATUS', 'ready');
const externalStatus = env('CURRENT_LATEST_EXTERNAL_AUTHORITY_STATUS', external.status || 'blocked');
const externalBlockers = Number(env(
  'CURRENT_LATEST_EXTERNAL_AUTHORITY_BLOCKER_COUNT',
  external.active_blocker_count ?? external.blocker_count ?? 0,
));
const authorityDecisionsReady = authorityGate?.perfect_refactor_decision_ready === true;
const publicationReady = env('CURRENT_LATEST_PUBLICATION_READY', publication.publication_ready || 'blocked');
const publicationScopeReady = publicationEvidenceGate?.v1_scope_ready === true
  || publicationEvidenceGate?.publication_ready === true;
const productBaselineFrozen = upstreamPolicy?.product_baseline_frozen === true;
const localStableAllowed = env('CURRENT_LATEST_LOCAL_STABLE_ALLOWED', 'true') === 'true';
const requestedClaim = env('CURRENT_LATEST_REQUESTED_CLAIM_WORDING', allowedClaim);

const gateStatuses = {
  adapter: adapterStatus,
  source_inventory: sourceStatus,
  current_latest_matrix: matrixStatus,
  golden_corpus: goldenStatus,
  regression: regressionStatus,
  stress: stressStatus,
  property: propertyStatus,
  runtime_smoke: runtimeSmokeStatus,
  current_target: currentTargetStatus,
  external_authority: externalStatus,
  publication_authority: publicationReady,
};

const blockers = [];
if (!readiness(adapterStatus)) {
  blockers.push(blocker(
    'current-latest:adapter-verification',
    'adapter',
    `${gateDir}/release-candidate.json`,
    `adapter verification status is ${adapterStatus}`,
    'Run and pass all adapter verification commands before making a quality claim',
  ));
}
if (!readiness(sourceStatus) || sourceUnclassified > 0) {
  blockers.push(blocker(
    'current-latest:source-inventory',
    'source-inventory',
    findArtifact(sourceArtifacts, 'current-latest-source-inventory.json'),
    `source inventory status=${sourceStatus} unclassified_rows=${sourceUnclassified}`,
    'Classify every current-latest source row and attach fixture, snapshot, blocker, or waiver evidence',
  ));
}
if (!readiness(matrixStatus) || matrixUnclassified > 0) {
  blockers.push(blocker(
    'current-latest:matrix',
    'current-latest-matrix',
    findArtifact(sourceArtifacts, 'current-latest-matrix.json'),
    `current-latest matrix status=${matrixStatus} unclassified_rows=${matrixUnclassified}`,
    'Reconcile every current-latest matrix row to P0/P1/P2 evidence before claiming completeness',
  ));
}
if (!readiness(goldenStatus) || goldenBlockers > 0) {
  blockers.push(blocker(
    'current-latest:golden-corpus',
    'golden-corpus',
    findArtifact(sourceArtifacts, 'current-latest-golden-corpus.json'),
    `golden corpus status=${goldenStatus} active_blocker_count=${goldenBlockers} audit_blocker_count=${goldenAuditBlockers}`,
    'Resolve P0 golden diff blockers and missing P1/P2 evidence before claiming current-latest parity',
  ));
}
pushStatusBlocker(blockers, 'current-latest:regression', 'regression', outPath, regressionStatus, 'Run deterministic regression tests and fix any failing release-critical regression');
pushStatusBlocker(blockers, 'current-latest:stress', 'stress', outPath, stressStatus, 'Run deterministic stress tests and fix any failing release-critical stress case');
pushStatusBlocker(blockers, 'current-latest:property', 'property', outPath, propertyStatus, 'Run deterministic property-style tests and fix any failing invariant');
pushStatusBlocker(blockers, 'current-latest:runtime-smoke', 'runtime-smoke', `${gateDir}/release-candidate.json`, runtimeSmokeStatus, 'Run runtime smoke and fix any release candidate failure');
if (!productBaselineFrozen && (!readiness(currentTargetStatus) || !currentTargetClaimAllowed)) {
  blockers.push(blocker(
    'current-latest:target',
    'current-target',
    findArtifact(sourceArtifacts, 'current-latest-target.json'),
    `current target status=${currentTargetStatus} claim_allowed=${currentTargetClaimAllowed}`,
    'Provide a locked current-latest target packet shared by source, matrix, corpus, and release evidence',
  ));
}
if (!authorityDecisionsReady && (!readiness(externalStatus) || externalBlockers > 0)) {
  blockers.push(blocker(
    'current-latest:external-authority',
    'external-authority',
    findArtifact(sourceArtifacts, 'external-authority-blockers.json'),
    `external authority status=${externalStatus} blocker_count=${externalBlockers}`,
    'Resolve live provider, account, private service, legal, or brand authority gaps with external evidence or formal waivers',
  ));
}
if (!publicationScopeReady && !readiness(publicationReady)) {
  blockers.push(blocker(
    'current-latest:publication-authority',
    'publication-authority',
    findArtifact(sourceArtifacts, 'publication-authority.json'),
    `publication authority status=${publicationReady}`,
    'Provide authorized publication credentials, legal/brand approval, and external URL/digest evidence',
  ));
}
if (forbiddenClaim(requestedClaim)) {
  blockers.push(blocker(
    'current-latest:claim-wording',
    'claim-wording',
    findArtifact(sourceArtifacts, 'perfect-refactor-claim.json'),
    `forbidden claim wording: ${requestedClaim}`,
    `Use only: ${allowedClaim}`,
  ));
}

const localCurrentLatestReady =
  localStableAllowed &&
  readiness(adapterStatus) &&
  readiness(sourceStatus) &&
  sourceUnclassified === 0 &&
  readiness(matrixStatus) &&
  matrixUnclassified === 0 &&
  readiness(goldenStatus) &&
  goldenBlockers === 0 &&
  readiness(regressionStatus) &&
  readiness(stressStatus) &&
  readiness(propertyStatus) &&
  readiness(runtimeSmokeStatus) &&
  (productBaselineFrozen || (readiness(currentTargetStatus) && currentTargetClaimAllowed)) &&
  !forbiddenClaim(requestedClaim);
const publicAuthorityReady =
  (authorityDecisionsReady || (readiness(externalStatus) && externalBlockers === 0))
  && (publicationScopeReady || readiness(publicationReady));
const perfectRefactorClaimAllowed =
  localCurrentLatestReady &&
  publicAuthorityReady &&
  blockers.length === 0 &&
  requestedClaim === allowedClaim;

const report = {
  schema: 'promptfoo-rs.current-latest-quality.v1',
  status: blockers.length === 0 ? 'ready' : 'ready-with-blockers',
  gate_statuses: gateStatuses,
  local_stable_allowed: localStableAllowed,
  local_current_latest_ready: localCurrentLatestReady,
  perfect_refactor_claim_allowed: perfectRefactorClaimAllowed,
  allowed_claim_wording: allowedClaim,
  forbidden_claim_examples: forbiddenExamples,
  requested_claim_wording: requestedClaim,
  blockers,
  source_artifacts: sourceArtifacts,
  deterministic_tests: {
    regression: regressionStatus,
    stress: stressStatus,
    property: propertyStatus,
    evidence: [
      'cargo test --test current_latest_quality_gate',
      'bash scripts/release/integration.sh',
      'bash scripts/release/coverage.sh',
      'bash scripts/release/runtime-smoke.sh',
    ],
  },
};

fs.mkdirSync(path.dirname(outPath), { recursive: true });
fs.writeFileSync(outPath, `${JSON.stringify(report, null, 2)}\n`);
JSON.parse(fs.readFileSync(outPath, 'utf8'));
NODE
