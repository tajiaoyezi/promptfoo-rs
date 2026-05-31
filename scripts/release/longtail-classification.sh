#!/usr/bin/env bash
set -euo pipefail

GATE_DIR="target/release-gates"
SOURCE_ITEMS="$GATE_DIR/source-extracted-items.json"
OUT="$GATE_DIR/longtail-classification.json"

if [ ! -f "$SOURCE_ITEMS" ]; then
  bash scripts/release/source-inventory-evidence.sh
fi

if [ "${LONGTAIL_SKIP_UNIT_TEST:-0}" != "1" ]; then
  cargo test --test longtail_provider_assertion_redteam_classification
fi

node - "$SOURCE_ITEMS" "compatibility/inventory/upstream-items.json" "$OUT" "compatibility/fixtures/providers" <<'NODE'
const fs = require('fs');
const path = require('path');

const [sourcePath, inventoryPath, outputPath, providerFixtureRoot] = process.argv.slice(2);
const source = JSON.parse(fs.readFileSync(sourcePath, 'utf8'));
const inventory = JSON.parse(fs.readFileSync(inventoryPath, 'utf8'));
const categories = new Set(['provider', 'assertion', 'redteam-plugin', 'redteam-strategy']);
const trackedById = new Map((inventory.items || []).map((item) => [item.stable_id, item]));
const sourceLongtail = (source.items || []).filter((item) => categories.has(item.category));
const providerFixtureIds = loadFixtureIds(providerFixtureRoot);

const counts = {};
const missingTrackedRows = [];
const unresolvedRows = [];
const missingReasonRows = [];
const p0ReleaseBlockers = [];
const providerModuleRows = [];
const providerModuleResolved = [];

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

function isP0ProviderModuleBlocker(item, token) {
  return (
    item.category === 'provider' &&
    item.level_hint === 'P0' &&
    token === 'blocked' &&
    String(item.stable_id || '').startsWith('provider:src-providers-')
  );
}

function providerModuleFixtureIds(itemId) {
  const direct = {
    'provider:src-providers-anthropic-defaults': ['p0-provider-anthropic-message'],
    'provider:src-providers-anthropic-generic': ['p0-provider-anthropic-message'],
    'provider:src-providers-anthropic-messages': ['p0-provider-anthropic-message'],
    'provider:src-providers-anthropic-types': ['p0-provider-anthropic-message'],
    'provider:src-providers-anthropic-util': ['p0-provider-anthropic-message'],
    'provider:src-providers-anthropic-completion': ['p0-provider-anthropic-completion'],
    'provider:src-providers-http': ['p0-provider-http-get', 'p0-provider-http-post'],
    'provider:src-providers-httpmultipart': ['p0-provider-http-multipart'],
    'provider:src-providers-httptransforms': ['p0-provider-http-transform'],
    'provider:src-providers-ollama': ['p0-provider-ollama-chat'],
    'provider:src-providers-openai-chat': ['p0-provider-openai-chat'],
    'provider:src-providers-openai-completion': ['p0-provider-openai-completion'],
    'provider:src-providers-openai-index': ['p0-provider-openai-chat'],
    'provider:src-providers-openai-types': ['p0-provider-openai-chat'],
    'provider:src-providers-openai-defaults': [
      'p0-provider-openai-env',
      'p0-provider-openai-headers',
    ],
    'provider:src-providers-openai-embedding': ['p0-provider-openai-embedding'],
    'provider:src-providers-openai-image': ['p0-provider-openai-image'],
    'provider:src-providers-openai-moderation': ['p0-provider-openai-moderation'],
    'provider:src-providers-openai-responses': ['p0-provider-openai-responses'],
    'provider:src-providers-openai-transcription': ['p0-provider-openai-transcription'],
    'provider:src-providers-openai-util': [
      'p0-provider-openai-chat',
      'p0-provider-openai-env',
      'p0-provider-openai-headers',
    ],
    'provider:src-providers-openai-video': ['p0-provider-openai-video'],
  };
  return direct[itemId] || [];
}

function resolveProviderModule(item) {
  const expected = providerModuleFixtureIds(item.stable_id);
  const fixture_ids = expected.filter((fixtureId) => providerFixtureIds.has(fixtureId));
  if (fixture_ids.length > 0) {
    return {
      item_id: item.stable_id,
      source_reference: item.source_reference,
      kind: 'fixture-covered',
      reason: `${dedicatedRequestResponseFixtureCount(fixture_ids) > 0 ? 'dedicated request/response' : 'aggregate provider'} fixture evidence (${fixture_ids.join(', ')}) covers ${item.stable_id}; source: ${item.source_reference}`,
      verification: `fixture:${fixture_ids.join('+')}`,
      fixture_ids,
      docs: 'docs/compatibility/matrix.md#p0-provider-module-burndown',
      requires_external_authority: false,
    };
  }
  const blocker = explicitProviderModuleBlockerReason(item.stable_id, item.source_reference);
  return {
    item_id: item.stable_id,
    source_reference: item.source_reference,
    kind: blocker.requires_external_authority ? 'external-blocker' : 'blocked',
    reason: blocker.reason,
    verification: `blocker:${item.stable_id}`,
    fixture_ids: [],
    docs: 'docs/compatibility/matrix.md#p0-provider-module-burndown',
    requires_external_authority: blocker.requires_external_authority,
  };
}

function dedicatedRequestResponseFixtureCount(fixtureIds) {
  const dedicated = new Set([
    'p0-provider-anthropic-completion',
    'p0-provider-http-multipart',
    'p0-provider-openai-completion',
    'p0-provider-openai-embedding',
    'p0-provider-openai-image',
    'p0-provider-openai-moderation',
    'p0-provider-openai-responses',
    'p0-provider-openai-transcription',
    'p0-provider-openai-video',
  ]);
  return fixtureIds.filter((fixtureId) => dedicated.has(fixtureId)).length;
}

function explicitProviderModuleBlockerReason(itemId, sourceReference) {
  const lower = String(itemId).toLowerCase();
  let reason =
    'Provider module needs a dedicated request/response fixture before aggregate provider evidence can prove per-module parity';
  let requires_external_authority = false;
  if (lower.includes('claudecodeauth')) {
    reason =
      'Anthropic Claude Code auth requires real local credential flow and product authority before native parity can be claimed';
    requires_external_authority = true;
  } else if (lower.includes('codex')) {
    reason =
      'OpenAI Codex provider modules require external product authority and private SDK/server credential confirmation before native parity can be claimed';
    requires_external_authority = true;
  } else if (lower.includes('billing')) {
    reason =
      'OpenAI billing module requires account-level credentials and billing authority; no local mock may be treated as published parity';
    requires_external_authority = true;
  } else if (lower.includes('chatkit')) {
    reason =
      'OpenAI ChatKit modules require product authority and browser/session fixture confirmation before native parity can be claimed';
    requires_external_authority = true;
  } else if (lower.includes('agents')) {
    reason =
      'OpenAI Agents SDK and tracing modules require dedicated SDK/trace fixtures plus product contract review';
    requires_external_authority = true;
  } else if (lower.includes('realtime')) {
    reason =
      'OpenAI realtime module requires a dedicated streaming protocol fixture and service contract confirmation';
    requires_external_authority = true;
  } else if (lower.includes('assistant')) {
    reason =
      'OpenAI Assistants module requires a stateful API fixture and account-authorized behavior review';
    requires_external_authority = true;
  }
  return {
    reason: `${reason}; source: ${sourceReference}`,
    requires_external_authority,
  };
}

function genericP0Blocker(item) {
  const reason = String(item.unresolved_reason || '').trim() ||
    'P0 long-tail item remains release-blocking until fixture or explicit blocker evidence exists';
  return {
    item_id: item.stable_id,
    source_reference: item.source_reference,
    reason,
    verification: `blocker:${item.stable_id}`,
  };
}

function loadFixtureIds(root) {
  const ids = new Set();
  function walk(current) {
    if (!fs.existsSync(current)) return;
    for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
      const next = path.join(current, entry.name);
      if (entry.isDirectory()) {
        walk(next);
      } else if (entry.name === 'fixture.yaml') {
        const yaml = fs.readFileSync(next, 'utf8');
        const match = yaml.match(/^id:\s*["']?([^"'\r\n]+)["']?\s*$/m);
        if (match) ids.add(match[1].trim());
      }
    }
  }
  walk(root);
  return ids;
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
    if (isP0ProviderModuleBlocker(tracked, token)) {
      providerModuleRows.push(tracked);
      const resolution = resolveProviderModule(tracked);
      if (resolution.kind === 'fixture-covered') {
        providerModuleResolved.push(resolution);
      } else {
        p0ReleaseBlockers.push(resolution);
      }
    } else {
      p0ReleaseBlockers.push(genericP0Blocker(tracked));
    }
  }
}

const blocked =
  missingTrackedRows.length > 0 || unresolvedRows.length > 0 || missingReasonRows.length > 0;
const status = blocked ? 'blocked' : p0ReleaseBlockers.length > 0 ? 'ready-with-blockers' : 'ready';

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
  p0_release_blocker_count: p0ReleaseBlockers.length,
  p0_release_blockers: p0ReleaseBlockers,
  p0_provider_module_burndown: {
    initial_blocker_count: providerModuleRows.length,
    resolved_by_fixture_count: providerModuleResolved.length,
    new_dedicated_request_response_fixture_count: providerModuleResolved.filter((item) =>
      dedicatedRequestResponseFixtureCount(item.fixture_ids || []) > 0
    ).length,
    remaining_blocker_count: p0ReleaseBlockers.filter((item) =>
      String(item.item_id || '').startsWith('provider:src-providers-')
    ).length,
    external_authority_blocker_count: p0ReleaseBlockers.filter((item) =>
      String(item.item_id || '').startsWith('provider:src-providers-') &&
      item.requires_external_authority === true
    ).length,
    generic_blocker_count: p0ReleaseBlockers.filter((item) =>
      String(item.item_id || '').startsWith('provider:src-providers-') &&
      item.requires_external_authority !== true
    ).length,
    resolved_by_fixture: providerModuleResolved,
  },
  evidence: {
    unit_test: 'cargo test --test longtail_provider_assertion_redteam_classification',
    provider_module_burndown_test: 'cargo test --test p0_provider_module_fixture_burndown',
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
