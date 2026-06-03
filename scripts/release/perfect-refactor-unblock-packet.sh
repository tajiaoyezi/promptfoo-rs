#!/usr/bin/env bash
set -euo pipefail

GATE_DIR="${GATE_DIR:-target/release-gates}"
mkdir -p "$GATE_DIR"

node <<'NODE'
const fs = require('fs');

const gateDir = process.env.GATE_DIR || 'target/release-gates';
const read = (name) => JSON.parse(fs.readFileSync(`${gateDir}/${name}`, 'utf8'));
const readOptional = (name) => {
  const path = `${gateDir}/${name}`;
  return fs.existsSync(path) ? JSON.parse(fs.readFileSync(path, 'utf8')) : null;
};
const claim = read('perfect-refactor-claim.json');
const source = read('source-inventory-evidence.json');
const external = read('external-authority-blockers.json');
const publication = read('publication-authority.json');
const distribution = read('upstream-distribution-target.json');
const currentLatestGolden = readOptional('current-latest-golden-corpus.json');
const currentLatestMatrix = readOptional('current-latest-matrix.json');
const currentLatestQuality = readOptional('current-latest-quality.json');
const currentLatestTarget = readOptional('current-latest-target.json');

function artifactFor(name) {
  return `target/release-gates/${name}`;
}

function actorFor(authorityType, itemId) {
  if (String(itemId).startsWith('publication:') || authorityType === 'publication-authority') {
    return 'release maintainer';
  }
  switch (authorityType) {
    case 'account':
      return 'account owner';
    case 'credential':
      return 'credential owner';
    case 'private-service':
      return 'service owner';
    case 'legal-brand':
      return 'legal/brand reviewer';
    default:
      return 'product owner';
  }
}

function publicationLabel(itemId) {
  const channel = String(itemId).replace(/^publication:/, '');
  return {
    cargo: 'Cargo',
    docker: 'Docker',
    'github-action': 'GitHub Action',
    'github-releases': 'GitHub Releases',
    homebrew: 'Homebrew',
    'npm-wrapper': 'npm wrapper',
  }[channel] || channel;
}

function normalizeExternalItem(blocker) {
  const itemId = String(blocker.item_id);
  const authorityType = String(blocker.authority_type || 'external-authority');
  const isPublication = itemId.startsWith('publication:') || authorityType === 'publication-authority';
  const label = publicationLabel(itemId);
  return {
    item_id: itemId,
    category: isPublication ? 'publication-authority' : 'provider-authority',
    authority_type: authorityType,
    required_actor: actorFor(authorityType, itemId),
    required_evidence: isPublication
      ? `${label} publication requires credentials, release authority, legal/brand approval, and external URL/digest evidence`
      : String(blocker.required_decision || 'External product/account authority evidence is required'),
    source_artifact: isPublication
      ? artifactFor('publication-authority.json')
      : artifactFor('external-authority-blockers.json'),
    source_reference: blocker.source_reference || null,
    safe_local_fallback: String(blocker.safe_local_fallback || 'Keep local fixture or dry-run evidence only'),
    release_impact: String(blocker.release_impact || 'Blocks perfect-refactor claim until external evidence exists'),
    auto_resolvable: false,
  };
}

function sourceOnlyItem(itemId) {
  return {
    item_id: itemId,
    category: 'source-accounting',
    authority_type: itemId.startsWith('config:') ? 'product-service-authority' : 'source-accounting',
    required_actor: 'user or maintainer',
    required_evidence: `Native/bridge fixture evidence or approved external-authority waiver for ${itemId}`,
    source_artifact: artifactFor('source-inventory-evidence.json'),
    source_reference: itemId,
    safe_local_fallback: 'Keep the item release-blocking; do not claim parity',
    release_impact: `Blocks perfect-refactor source accounting claim until ${itemId} has evidence`,
    auto_resolvable: false,
  };
}

function currentLatestItemId(blocker) {
  return String(blocker.item_id || blocker.capability || blocker.id || '').trim();
}

function currentLatestMatrixKey(row) {
  return String(row.item_id || row.capability || row.id || '').trim();
}

function currentLatestAuthorityType(itemId) {
  const lower = String(itemId).toLowerCase();
  if (lower.startsWith('config:')) {
    return 'product-service-authority';
  }
  if (lower.includes('billing')) {
    return 'account';
  }
  if (lower.includes('claudecodeauth')) {
    return 'credential';
  }
  if (lower.includes('assistant') || lower.includes('realtime')) {
    return 'private-service';
  }
  return 'product-authority';
}

function currentLatestCategory(itemId) {
  if (String(itemId).startsWith('config:')) {
    return 'current-latest-golden';
  }
  if (String(itemId).startsWith('provider:')) {
    return 'current-latest-golden';
  }
  return 'current-latest-golden';
}

function currentLatestGoldenItem(blocker, matrixRow) {
  const itemId = currentLatestItemId(blocker);
  const authorityType = currentLatestAuthorityType(itemId);
  const reason = String(
    matrixRow?.blocker_reason ||
      matrixRow?.reason ||
      blocker.blocker_reason ||
      blocker.reason ||
      blocker.message ||
      'Current-latest P0 golden blocker requires real evidence or formal waiver'
  );
  const sourceReference = String(
    matrixRow?.source_reference || blocker.source_reference || itemId
  );
  return {
    item_id: itemId,
    category: currentLatestCategory(itemId),
    authority_type: authorityType,
    required_actor: actorFor(authorityType, itemId),
    required_evidence: `${reason}; provide current-latest evidence or approved waiver before this row can stop blocking perfect-refactor parity`,
    source_artifact: artifactFor('current-latest-golden-corpus.json'),
    source_reference: sourceReference,
    safe_local_fallback:
      'Keep this current-latest row release-blocking; do not claim live parity from local mock or dry-run evidence',
    release_impact: `Blocks current-latest perfect-refactor claim until ${itemId} is resolved with real evidence or formal waiver`,
    auto_resolvable: false,
  };
}

function currentLatestTargetItem(target) {
  const sourceReference =
    target?.github?.default_branch_head ||
    target?.target_packet_ref ||
    'current-latest target lock';
  return {
    item_id: 'current-latest:target',
    category: 'current-target',
    authority_type: 'target-policy',
    required_actor: 'maintainer',
    required_evidence:
      'same locked current-latest target source inventory, matrix, golden corpus, quality gate, external authority, publication evidence, and release candidate evidence',
    source_artifact: artifactFor('current-latest-target.json'),
    source_reference: sourceReference,
    safe_local_fallback:
      'Keep current-latest perfect-refactor claim blocked and preserve locked-target wording',
    release_impact:
      'Blocks current-latest perfect-refactor claim until all evidence refers to the same locked target packet',
    auto_resolvable: false,
  };
}

function currentLatestTargetDecisionRequired(target, quality) {
  if (!target) {
    return true;
  }
  if (target.current_latest_claim_allowed !== true) {
    return true;
  }
  const blockers = quality?.blockers || [];
  return blockers.some((blocker) => {
    const category = String(blocker.category || blocker.item_id || '');
    return category.includes('current-target') || category.includes('target');
  });
}

function currentLatestReleaseBlockers(golden) {
  return (golden?.release_blockers || golden?.blockers || [])
    .filter((blocker) => currentLatestItemId(blocker).length > 0);
}

function buildCurrentLatestPacket() {
  const matrixRows = new Map();
  for (const row of currentLatestMatrix?.rows || []) {
    const key = currentLatestMatrixKey(row);
    if (key) {
      matrixRows.set(key, row);
    }
  }

  const goldenBlockers = currentLatestReleaseBlockers(currentLatestGolden);
  const decisions = new Map();
  for (const blocker of goldenBlockers) {
    const itemId = currentLatestItemId(blocker);
    decisions.set(itemId, currentLatestGoldenItem(blocker, matrixRows.get(itemId)));
  }

  if (currentLatestTargetDecisionRequired(currentLatestTarget, currentLatestQuality)) {
    decisions.set('current-latest:target', currentLatestTargetItem(currentLatestTarget));
  }

  for (const channel of publication.channels || []) {
    if (channel.published !== true || channel.authority_status !== 'ready' || !channel.published_evidence) {
      const itemId = `publication:${channel.channel}`;
      decisions.set(itemId, normalizeExternalItem({
        item_id: itemId,
        authority_type: 'publication-authority',
        source_reference: `target/release-gates/publication-authority.json#${channel.channel}`,
        safe_local_fallback: 'Keep dry-run installability evidence only',
        release_impact: `${publicationLabel(itemId)} published=false; public availability remains blocked`,
      }));
    }
  }

  const decisionItems = [...decisions.values()].sort((left, right) =>
    String(left.item_id).localeCompare(String(right.item_id))
  );
  const perfectAllowed =
    claim.perfect_refactor_claim_allowed === true &&
    currentLatestQuality?.perfect_refactor_claim_allowed === true &&
    goldenBlockers.length === 0 &&
    decisionItems.length === 0;

  return {
    schema: 'promptfoo-rs.perfect-refactor-unblock-packet.v1',
    target_scope: 'current-latest',
    status: perfectAllowed ? 'ready' : 'blocked',
    perfect_refactor_claim_allowed: perfectAllowed,
    auto_resolvable: perfectAllowed && decisionItems.length === 0,
    blocker_count: goldenBlockers.length,
    source_p0_accounting_blocker_count: (source.remaining_p0_blockers || []).length,
    external_authority_blocker_count: external.blocker_count || (external.blockers || []).length,
    required_user_decision_count: decisionItems.length,
    current_upstream_rebaseline_required:
      distribution.current_repository_perfect_claim_allowed !== true,
    current_latest_golden_blocker_count: goldenBlockers.length,
    current_latest_external_authority_blocker_count: goldenBlockers.length,
    current_latest_required_decision_count: decisionItems.length,
    decision_items: decisionItems,
    source_artifacts: [
      artifactFor('current-latest-target.json'),
      artifactFor('current-latest-source-inventory.json'),
      artifactFor('current-latest-matrix.json'),
      artifactFor('current-latest-golden-corpus.json'),
      artifactFor('current-latest-quality.json'),
      artifactFor('perfect-refactor-claim.json'),
      artifactFor('source-inventory-evidence.json'),
      artifactFor('external-authority-blockers.json'),
      artifactFor('publication-authority.json'),
      artifactFor('upstream-distribution-target.json'),
      artifactFor('release-candidate.json'),
    ],
  };
}

if (currentLatestGolden && currentLatestMatrix) {
  const packet = buildCurrentLatestPacket();
  fs.writeFileSync(`${gateDir}/perfect-refactor-unblock-packet.json`, `${JSON.stringify(packet, null, 2)}\n`);
  process.exit(0);
}

const decisions = new Map();
for (const blocker of external.blockers || []) {
  const item = normalizeExternalItem(blocker);
  decisions.set(item.item_id, item);
}

for (const itemId of source.remaining_p0_blockers || []) {
  if (!decisions.has(itemId)) {
    decisions.set(itemId, sourceOnlyItem(itemId));
  }
}

if (distribution.current_repository_perfect_claim_allowed !== true || claim.current_perfect_claim_allowed !== true) {
  decisions.set('current-upstream:rebaseline', {
    item_id: 'current-upstream:rebaseline',
    category: 'current-upstream',
    authority_type: 'target-policy',
    required_actor: 'maintainer',
    required_evidence: 'same-ref source inventory, matrix, fixtures, golden corpus, and release candidate evidence; otherwise keep frozen-baseline scope',
    source_artifact: artifactFor('upstream-distribution-target.json'),
    source_reference: 'promptfoo repository HEAD / npm core package target',
    safe_local_fallback: 'Keep current repository perfect-refactor claim blocked and preserve frozen-baseline compatibility wording',
    release_impact: 'Blocks current repository perfect-refactor claim until rebaseline evidence is complete',
    auto_resolvable: false,
  });
}

if (![...decisions.keys()].some((itemId) => itemId.startsWith('publication:'))) {
  for (const channel of publication.channels || []) {
    if (channel.published !== true || channel.authority_status !== 'ready' || !channel.published_evidence) {
      const itemId = `publication:${channel.channel}`;
      decisions.set(itemId, normalizeExternalItem({
        item_id: itemId,
        authority_type: 'publication-authority',
        source_reference: `target/release-gates/publication-authority.json#${channel.channel}`,
        safe_local_fallback: 'Keep dry-run installability evidence only',
        release_impact: `${publicationLabel(itemId)} published=false; public availability remains blocked`,
      }));
    }
  }
}

const decisionItems = [...decisions.values()].sort((left, right) =>
  String(left.item_id).localeCompare(String(right.item_id))
);
const perfectAllowed = claim.perfect_refactor_claim_allowed === true && decisionItems.length === 0;
const packet = {
  schema: 'promptfoo-rs.perfect-refactor-unblock-packet.v1',
  status: perfectAllowed ? 'ready' : 'blocked',
  perfect_refactor_claim_allowed: perfectAllowed,
  auto_resolvable: perfectAllowed && decisionItems.length === 0,
  blocker_count: (claim.blockers || []).length,
  source_p0_accounting_blocker_count: (source.remaining_p0_blockers || []).length,
  external_authority_blocker_count: external.blocker_count || (external.blockers || []).length,
  required_user_decision_count: decisionItems.length,
  current_upstream_rebaseline_required: distribution.current_repository_perfect_claim_allowed !== true,
  decision_items: decisionItems,
  source_artifacts: [
    artifactFor('perfect-refactor-claim.json'),
    artifactFor('source-inventory-evidence.json'),
    artifactFor('external-authority-blockers.json'),
    artifactFor('publication-authority.json'),
    artifactFor('upstream-distribution-target.json'),
    artifactFor('release-candidate.json'),
  ],
};

fs.writeFileSync(`${gateDir}/perfect-refactor-unblock-packet.json`, `${JSON.stringify(packet, null, 2)}\n`);
NODE
