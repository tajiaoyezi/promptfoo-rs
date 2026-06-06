#!/usr/bin/env node
/**
 * Backfill docs/compatibility/publication-evidence.json for github-releases
 * after a real tagged GitHub Release is published.
 *
 * Usage:
 *   node scripts/release/backfill-github-release-evidence.mjs \
 *     --artifact-url https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.0/promptfoo-rs-0.1.0-x86_64-unknown-linux-gnu.tar.gz \
 *     --digest sha256:abcdef... \
 *     --timestamp 2026-06-06T12:00:00Z
 */
import fs from 'fs';
import path from 'path';

const root = process.cwd();
const manifestPath = path.join(root, 'docs/compatibility/publication-evidence.json');

function parseArgs(argv) {
  const out = {};
  for (let i = 0; i < argv.length; i += 1) {
    const arg = argv[i];
    if (arg === '--artifact-url') out.artifactUrl = argv[++i];
    else if (arg === '--digest') out.digest = argv[++i];
    else if (arg === '--timestamp') out.timestamp = argv[++i];
    else if (arg === '--release-notes') out.releaseNotes = argv[++i];
  }
  return out;
}

const args = parseArgs(process.argv.slice(2));
if (!args.artifactUrl?.startsWith('https://')) {
  console.error('missing or invalid --artifact-url (must be https://)');
  process.exit(1);
}
if (!args.digest?.trim()) {
  console.error('missing --digest');
  process.exit(1);
}

const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
const rows = manifest.rows ?? [];
const idx = rows.findIndex((row) => row.channel === 'github-releases');
if (idx < 0) {
  console.error('github-releases row not found');
  process.exit(1);
}

rows[idx] = {
  channel: 'github-releases',
  publication_state: 'published',
  authority_owner: 'leafiellune',
  credential_authority_reference: 'approval:v1-github-releases-only-2026-06-06',
  legal_brand_approval_reference: 'approval:legal-brand-independent-reimplementation-2026-06-06',
  artifact_url: args.artifactUrl,
  digest: args.digest,
  release_notes_reference: args.releaseNotes ?? 'docs/release-notes/v0.1.0.md',
  publication_timestamp: args.timestamp ?? new Date().toISOString(),
  no_upload_provenance: 'gh release create with verified external artifact URL and checksum',
  notes:
    'v1 GitHub Releases channel published; other channels remain v1_deferred per docs/compatibility/v1-release-authority-policy.md',
};

const published = rows.filter((row) => row.publication_state === 'published').length;
const blocked = rows.filter((row) => row.publication_state === 'blocked').length;
manifest.rows = rows;
manifest.blocked_channel_count = blocked;
manifest.publication_ready = false;
manifest.status = 'credential-blocked';
manifest.notes =
  'github-releases published; aggregate publication_ready stays false until deferred channels are waived or published per gate policy';

fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`);
console.log(`updated ${manifestPath}`);
console.log(`published_channels=${published} blocked_channels=${blocked}`);
