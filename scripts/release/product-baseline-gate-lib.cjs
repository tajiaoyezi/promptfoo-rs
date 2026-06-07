const fs = require('fs');

const RESOLVED_DECISION_STATES = new Set([
  'evidence-provided',
  'waived-with-boundary',
]);

function loadJson(path, fallback = null) {
  try {
    return JSON.parse(fs.readFileSync(path, 'utf8'));
  } catch (_) {
    return fallback;
  }
}

function loadAuthorityDecisions(
  manifestPath = 'docs/compatibility/authority-decisions.json',
) {
  const manifest = loadJson(manifestPath, { rows: [] });
  const byId = new Map();
  for (const row of manifest.rows || []) {
    byId.set(String(row.item_id), row);
  }
  return { manifest, byId };
}

function isResolvedAuthorityDecision(itemId, byId) {
  const row = byId.get(String(itemId));
  return row ? RESOLVED_DECISION_STATES.has(String(row.decision_state)) : false;
}

function loadProductBaselineTarget(
  path = 'compatibility/inventory/current-latest-target.json',
) {
  const target = loadJson(path);
  if (!target?.npm_latest?.package_version) {
    return null;
  }
  return {
    package_version: target.npm_latest.package_version,
    git_head: target.npm_latest.git_head,
    git_ref:
      target.github?.npm_tag_ref
      || `refs/tags/${target.npm_latest.package_version}`,
    git_commit: target.npm_latest.git_head,
    npm_integrity: target.npm_latest.integrity,
    acquisition_command:
      target.npm_latest.source
      || 'npm view promptfoo version gitHead dist.tarball dist.integrity --json',
    default_branch_head: target.github?.default_branch_head || null,
    observed_at: target.observed_at || null,
    adr_reference:
      'docs/decisions/adr-012-product-independence-baseline-freeze.md',
  };
}

function loadPublicationEvidence(
  manifestPath = 'docs/compatibility/publication-evidence.json',
) {
  const manifest = loadJson(manifestPath, { rows: [] });
  const byChannel = new Map();
  for (const row of manifest.rows || []) {
    byChannel.set(String(row.channel), row);
  }
  return { manifest, byChannel };
}

function isV1DeferredPublication(channel, byChannel) {
  const row = byChannel.get(String(channel));
  return row?.v1_deferred === true;
}

function isPublishedChannel(channel, byChannel) {
  const row = byChannel.get(String(channel));
  return row?.publication_state === 'published';
}

function v1PublicationScopeReady(requiredChannels, byChannel) {
  if (!requiredChannels.length) {
    return false;
  }
  for (const channel of requiredChannels) {
    const row = byChannel.get(channel);
    if (!row) {
      return false;
    }
    if (row.publication_state === 'published') {
      continue;
    }
    if (row.v1_deferred === true) {
      continue;
    }
    return false;
  }
  return isPublishedChannel('github-releases', byChannel);
}

module.exports = {
  RESOLVED_DECISION_STATES,
  loadJson,
  loadAuthorityDecisions,
  isResolvedAuthorityDecision,
  loadProductBaselineTarget,
  loadPublicationEvidence,
  isV1DeferredPublication,
  isPublishedChannel,
  v1PublicationScopeReady,
};
