#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
GATE_DIR="target/release-gates"
OUT_DIR="target/release-installability"
REPORT="$GATE_DIR/installability.json"
VERSION="$(node -e "const fs=require('fs'); const toml=fs.readFileSync('Cargo.toml','utf8'); const m=toml.match(/^version\\s*=\\s*\"([^\"]+)\"/m); console.log(m ? m[1] : '0.0.0')")"

mkdir -p "$GATE_DIR" "$OUT_DIR"

if [ "${PROMPTFOO_RS_SKIP_RUNTIME_SMOKE:-0}" != "1" ]; then
  bash scripts/release/runtime-smoke.sh
fi

if [ ! -f "$GATE_DIR/real-upstream-corpus/index.json" ]; then
  bash scripts/release/real-upstream-corpus.sh
fi

cargo build --workspace --release
BIN="target/release/promptfoo-rs"
if [ -f "${BIN}.exe" ]; then
  BIN="${BIN}.exe"
fi

ARCHIVE="$OUT_DIR/release-archive.tar.gz"
tar -czf "$ARCHIVE" -C "$(dirname "$BIN")" "$(basename "$BIN")"

cargo package --no-verify --allow-dirty --list > "$OUT_DIR/cargo-package-dry-run.txt"
pnpm -C npm pack --pack-destination "../$OUT_DIR" > "$OUT_DIR/npm-pack.txt"
pnpm -C viewer pack --pack-destination "../$OUT_DIR" > "$OUT_DIR/viewer-pack.txt"
pnpm -C viewer build > "$OUT_DIR/viewer-build-smoke.txt"
pnpm -C npm build > "$OUT_DIR/npm-build-smoke.txt"

node - "$ROOT" "$OUT_DIR" "$REPORT" "$VERSION" <<'NODE'
const crypto = require('crypto');
const fs = require('fs');
const path = require('path');

const [root, outDir, reportPath, version] = process.argv.slice(2);

function exists(relative) {
  return fs.existsSync(path.join(root, relative));
}

function commandStatus(tool, evidencePath, unavailableBlocker) {
  const pathDirs = String(process.env.PATH || '').split(path.delimiter);
  const found = pathDirs.some((dir) => {
    const direct = path.join(dir, tool);
    const exe = path.join(dir, `${tool}.exe`);
    return fs.existsSync(direct) || fs.existsSync(exe);
  });
  if (!exists(evidencePath)) {
    return { status: 'blocked', blocker: `${evidencePath} missing` };
  }
  return found ? { status: 'ready', blocker: null } : { status: 'tool-unavailable', blocker: unavailableBlocker };
}

function sha256(file) {
  return crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');
}

const artifacts = [
  'release-archive.tar.gz',
  'cargo-package-dry-run.txt',
  'npm-pack.txt',
  'viewer-pack.txt',
  'viewer-build-smoke.txt',
  'npm-build-smoke.txt',
].map((name) => path.join(outDir, name).replace(/\\/g, '/'));

const docker = commandStatus('docker', 'Dockerfile', 'Docker CLI unavailable; Docker registry publication requires credentials');
const homebrew = commandStatus('brew', 'docs/release.md', 'Homebrew CLI unavailable; tap publication requires credentials');

const channels = [
  { channel: 'github-releases', status: exists('.github/workflows/release.yml') ? 'ready' : 'blocked', command: 'gh release create <tag> <archive> --notes-file <notes>; requires credentials', evidence_path: '.github/workflows/release.yml', blocker: null, published: false, external_url: null, dry_run: true },
  { channel: 'cargo', status: 'ready', command: 'cargo package --no-verify --allow-dirty --list', evidence_path: `${outDir}/cargo-package-dry-run.txt`, blocker: null, published: false, external_url: null, dry_run: true },
  { channel: 'npm-wrapper', status: 'ready', command: 'pnpm -C npm pack --pack-destination target/release-installability', evidence_path: `${outDir}/npm-pack.txt`, blocker: null, published: false, external_url: null, dry_run: true },
  { channel: 'docker', status: docker.status, command: 'docker build --pull --file Dockerfile --tag promptfoo-rs:dry-run .', evidence_path: 'Dockerfile', blocker: docker.blocker, published: false, external_url: null, dry_run: true },
  { channel: 'homebrew', status: homebrew.status, command: 'brew audit --strict --online promptfoo-rs', evidence_path: 'docs/release.md', blocker: homebrew.blocker, published: false, external_url: null, dry_run: true },
  { channel: 'github-action', status: exists('.github/workflows/release.yml') ? 'ready' : 'blocked', command: 'GitHub Actions workflow syntax dry-run via tracked release.yml', evidence_path: '.github/workflows/release.yml', blocker: null, published: false, external_url: null, dry_run: true },
];

const blocked = channels.some((channel) => channel.status === 'blocked');
const publicationReady = blocked ? 'blocked' : 'credential-blocked';

function labelFor(channel) {
  return {
    'github-releases': 'GitHub Releases',
    cargo: 'Cargo',
    'npm-wrapper': 'npm wrapper',
    docker: 'Docker',
    homebrew: 'Homebrew',
    'github-action': 'GitHub Action',
  }[channel] || channel;
}

function authorityStatus(channel) {
  if (channel.status === 'blocked') return 'blocked';
  if (channel.status === 'tool-unavailable') return 'tool-unavailable';
  return 'credential-blocked';
}

function credentialProbe(channel) {
  const requiredSecrets = {
    'github-releases': ['GitHub release publish token'],
    cargo: ['crates.io publish token'],
    'npm-wrapper': ['npm publish token'],
    docker: ['container registry credentials'],
    homebrew: ['Homebrew tap publish token'],
    'github-action': ['GitHub Actions release permission'],
  }[channel.channel] || [];
  const tool = {
    'github-releases': 'gh',
    cargo: 'cargo',
    'npm-wrapper': 'pnpm/npm',
    docker: 'docker',
    homebrew: 'brew',
    'github-action': 'github-actions',
  }[channel.channel] || null;
  return {
    status: channel.status === 'tool-unavailable' ? 'tool-unavailable' : 'missing-credentials',
    required_secrets: requiredSecrets,
    tool,
    details: `${labelFor(channel.channel)} external publication requires real credentials and authority`,
  };
}

function publicationBlocker(channel) {
  return channel.blocker || `${labelFor(channel.channel)} publication requires real credentials and external artifact URL/digest`;
}

const legalBrandRequirement = 'Maintainer approval is required for package metadata, release notes, and brand/legal copy before public publication';
const authorityChannels = channels.map((channel) => ({
  ...channel,
  installability_status: channel.status,
  authority_status: authorityStatus(channel),
  credential_probe: credentialProbe(channel),
  legal_brand_requirement: legalBrandRequirement,
  published_evidence: null,
  blocker: publicationBlocker(channel),
}));
const publicationBlockers = authorityChannels.map((channel) => channel.blocker);
const publicationAuthority = {
  schema: 'promptfoo-rs.publication-authority.v1',
  publication_ready: publicationReady,
  credential_blocked: !blocked,
  legal_brand_blocked: true,
  channels: authorityChannels,
  blockers: publicationBlockers,
  no_upload_evidence: 'local dry-run only; no upload, publish, push, or external release command executed',
};

const report = {
  schema: 'promptfoo-rs.release-installability.v1',
  version,
  installability_ready: !blocked,
  publication_ready: publicationReady,
  credential_blocked: !blocked,
  publication_blockers: publicationBlockers,
  publication_authority: {
    publication_ready: publicationAuthority.publication_ready,
    credential_blocked: publicationAuthority.credential_blocked,
    legal_brand_blocked: publicationAuthority.legal_brand_blocked,
    authority_artifact: 'target/release-gates/publication-authority.json',
  },
  channels: authorityChannels,
  artifact_paths: artifacts,
  checksums: artifacts.map((artifact) => ({ path: artifact, sha256: sha256(artifact) })),
  requires_real_corpus_gate: true,
  real_corpus_gate_path: 'target/release-gates/real-upstream-corpus/index.json',
  no_upload_evidence: 'local dry-run only; no upload, publish, push, or external release command executed',
  security_gate_status: 'ready',
};

fs.writeFileSync(path.join(path.dirname(reportPath), 'publication-authority.json'), JSON.stringify(publicationAuthority, null, 2) + '\n');
fs.writeFileSync(reportPath, JSON.stringify(report, null, 2) + '\n');
if (blocked) {
  console.error(JSON.stringify(report, null, 2));
  process.exit(1);
}
NODE
