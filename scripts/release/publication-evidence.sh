#!/usr/bin/env bash
set -euo pipefail

GATE_DIR="${GATE_DIR:-target/release-gates}"
MANIFEST_PATH="${PUBLICATION_EVIDENCE_MANIFEST:-docs/compatibility/publication-evidence.json}"
OUT="$GATE_DIR/publication-evidence-gate.json"

mkdir -p "$GATE_DIR"

node - "$GATE_DIR" "$MANIFEST_PATH" "$OUT" <<'NODE'
const fs = require('fs');
const { v1PublicationScopeReady } = require('./product-baseline-gate-lib.cjs');

const [gateDir, manifestPath, outPath] = process.argv.slice(2);
const authority = JSON.parse(
  fs.readFileSync(`${gateDir}/publication-authority.json`, 'utf8'),
);
const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));

const requiredChannels = (authority.channels || []).map((channel) => String(channel.channel));
const rows = manifest.rows || [];
const manifestChannels = rows.map((row) => String(row.channel));
const requiredSet = new Set(requiredChannels);
const manifestSet = new Set(manifestChannels);

const missing = requiredChannels.filter((channel) => !manifestSet.has(channel));
const extra = manifestChannels.filter((channel) => !requiredSet.has(channel));
const duplicates = manifestChannels.filter(
  (channel, index) => manifestChannels.indexOf(channel) !== index,
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
    'publish token',
    'crates.io publish token',
    'npm publish token',
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

function publishedMissingFields(row) {
  const required = [
    'authority_owner',
    'credential_authority_reference',
    'legal_brand_approval_reference',
    'artifact_url',
    'digest',
    'release_notes_reference',
    'publication_timestamp',
    'no_upload_provenance',
  ];
  return required.filter((field) => !String(row[field] || '').trim());
}

function rowIsDryRunOnly(row) {
  const artifactUrl = String(row.artifact_url || '');
  const digest = String(row.digest || '');
  const provenance = String(row.no_upload_provenance || '');
  return (
    isMockOnly(artifactUrl)
    || isMockOnly(digest)
    || isMockOnly(provenance)
    || artifactUrl.includes('release-installability')
    || artifactUrl.includes('dry-run')
    || provenance.toLowerCase().includes('dry-run')
    || !artifactUrl.startsWith('https://')
  );
}

let blockedChannelCount = 0;
let publishedChannelCount = 0;
let v1DeferredChannelCount = 0;
const incompletePublishedRows = [];
const dryRunOnlyPublishedRows = [];
const secretBearingRows = [];
const blockers = [];

if (missing.length) {
  blockers.push(`${missing.length} publication channels are missing manifest rows`);
}
if (extra.length) {
  blockers.push(`${extra.length} manifest rows do not map to publication-authority channels`);
}
if (duplicates.length) {
  blockers.push(`${duplicates.length} manifest rows duplicate channel values`);
}

for (const row of rows) {
  const channel = String(row.channel || '');
  if (secretValuesInValue(row).length) {
    secretBearingRows.push(channel);
    blockers.push(`${channel}: manifest row contains secret-like values`);
  }

  switch (row.publication_state) {
    case 'blocked':
      if (row.v1_deferred === true) {
        v1DeferredChannelCount += 1;
        break;
      }
      blockedChannelCount += 1;
      blockers.push(`${channel}: publication evidence remains blocked`);
      break;
    case 'published': {
      const missingFields = publishedMissingFields(row);
      if (missingFields.length) {
        incompletePublishedRows.push(channel);
        blockers.push(`${channel}: published row missing required fields: ${missingFields.join(', ')}`);
        break;
      }
      if (rowIsDryRunOnly(row)) {
        dryRunOnlyPublishedRows.push(channel);
        blockers.push(`${channel}: dry-run installability evidence cannot set published=true`);
        break;
      }
      publishedChannelCount += 1;
      break;
    }
    default:
      blockers.push(`${channel}: invalid publication_state '${row.publication_state || ''}'`);
  }
}

const byChannel = new Map(rows.map((row) => [String(row.channel), row]));
const structuralReady = !missing.length
  && !extra.length
  && !duplicates.length
  && !incompletePublishedRows.length
  && !dryRunOnlyPublishedRows.length
  && !secretBearingRows.length;
const fullPublicationReady = structuralReady
  && requiredChannels.length > 0
  && publishedChannelCount === requiredChannels.length
  && blockedChannelCount === 0;
const v1ScopeReady = structuralReady
  && requiredChannels.length > 0
  && v1PublicationScopeReady(requiredChannels, byChannel);
const publicationReady = fullPublicationReady || v1ScopeReady;

const report = {
  schema: 'promptfoo-rs.publication-evidence-gate.v1',
  status: publicationReady ? 'ready' : 'credential-blocked',
  publication_ready: publicationReady,
  v1_scope_ready: v1ScopeReady,
  full_publication_ready: fullPublicationReady,
  required_channel_count: requiredChannels.length,
  manifest_row_count: manifestChannels.length,
  blocked_channel_count: blockedChannelCount,
  published_channel_count: publishedChannelCount,
  v1_deferred_channel_count: v1DeferredChannelCount,
  missing_manifest_rows: missing,
  extra_manifest_rows: extra,
  duplicate_manifest_rows: [...new Set(duplicates)],
  incomplete_published_rows: incompletePublishedRows,
  dry_run_only_published_rows: dryRunOnlyPublishedRows,
  secret_bearing_rows: secretBearingRows,
  blockers,
  manifest_artifact: manifestPath,
  authority_artifact: `${gateDir}/publication-authority.json`,
};

fs.writeFileSync(outPath, `${JSON.stringify(report, null, 2)}\n`);
NODE
