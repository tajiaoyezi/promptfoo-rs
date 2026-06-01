#!/usr/bin/env bash
set -euo pipefail

GATE_DIR="target/release-gates"
OUT="$GATE_DIR/current-latest-target.json"
mkdir -p "$GATE_DIR"

npm_tmp="$(mktemp)"
release_tmp="$(mktemp)"
ls_tmp="$(mktemp)"
trap 'rm -f "$npm_tmp" "$release_tmp" "$ls_tmp"' EXIT

node_string() {
  node - "$1" "$2" "$3" <<'NODE'
const fs = require('fs');
const [path, key, flatKey] = process.argv.slice(2);
const value = JSON.parse(fs.readFileSync(path, 'utf8'));
function get(node, parts) {
  for (const part of parts) {
    node = node && typeof node === 'object' ? node[part] : undefined;
  }
  return typeof node === 'string' ? node : undefined;
}
process.stdout.write(get(value, key.split('.')) || value[flatKey] || '');
NODE
}

if [ -n "${CURRENT_LATEST_NPM_VIEW_FILE:-}" ]; then
  cp "$CURRENT_LATEST_NPM_VIEW_FILE" "$npm_tmp"
else
  npm view promptfoo version gitHead dist.tarball dist.integrity time.modified --json > "$npm_tmp"
fi

if [ -n "${CURRENT_LATEST_GITHUB_RELEASE_FILE:-}" ]; then
  cp "$CURRENT_LATEST_GITHUB_RELEASE_FILE" "$release_tmp"
else
  node <<'NODE' > "$release_tmp"
const https = require('https');
const request = https.get(
  'https://api.github.com/repos/promptfoo/promptfoo/releases/latest',
  {
    headers: {
      accept: 'application/vnd.github+json',
      'user-agent': 'promptfoo-rs-current-latest-target-lock',
    },
    timeout: 30000,
  },
  (response) => {
    let body = '';
    response.setEncoding('utf8');
    response.on('data', (chunk) => {
      body += chunk;
    });
    response.on('end', () => {
      if (response.statusCode < 200 || response.statusCode >= 300) {
        console.error(`GitHub latest release lookup failed: ${response.statusCode}`);
        process.exit(1);
      }
      process.stdout.write(body);
    });
  },
);
request.on('timeout', () => {
  request.destroy(new Error('GitHub latest release lookup timed out'));
});
request.on('error', (error) => {
  console.error(error.message);
  process.exit(1);
});
NODE
fi

npm_version="$(node_string "$npm_tmp" version version)"
latest_release_tag="$(node - "$release_tmp" <<'NODE'
const fs = require('fs');
const value = JSON.parse(fs.readFileSync(process.argv[2], 'utf8'));
const tag = value.tagName || value.tag_name;
if (!tag || typeof tag !== 'string') {
  throw new Error('GitHub latest release metadata missing tagName/tag_name');
}
if (tag.trim() !== tag || /[\s\0^~:?*\[\\]/.test(tag) || tag.includes('..')) {
  throw new Error(`GitHub latest release tag is not a safe ref name: ${tag}`);
}
process.stdout.write(tag);
NODE
)"
latest_release_ref="refs/tags/$latest_release_tag"
npm_tag_ref="refs/tags/$npm_version"

if [ -n "${CURRENT_LATEST_LS_REMOTE_FILE:-}" ]; then
  cp "$CURRENT_LATEST_LS_REMOTE_FILE" "$ls_tmp"
else
  git ls-remote https://github.com/promptfoo/promptfoo.git \
    HEAD "$npm_tag_ref" "$latest_release_ref" > "$ls_tmp"
fi

node - "$npm_tmp" "$release_tmp" "$ls_tmp" "$OUT" <<'NODE'
const fs = require('fs');
const [npmPath, releasePath, lsRemotePath, outPath] = process.argv.slice(2);

function stringAt(value, path, flatKey) {
  let node = value;
  for (const part of path) {
    node = node && typeof node === 'object' ? node[part] : undefined;
  }
  if (typeof node === 'string') return node;
  if (flatKey && typeof value[flatKey] === 'string') return value[flatKey];
  return undefined;
}

function fullSha(value) {
  return /^[0-9a-fA-F]{40}$/.test(value || '');
}

function rejectFloating(value) {
  if (/^(latest|main|master|head|\*)$/i.test(value || '') || /^refs\/heads\/(main|master)$/i.test(value || '')) {
    throw new Error(`floating current-latest completion proof is not allowed: ${value}`);
  }
}

function parseNpm(json) {
  const value = JSON.parse(json);
  const version = stringAt(value, ['version']);
  const gitHead = stringAt(value, ['gitHead']);
  const tarball = stringAt(value, ['dist', 'tarball'], 'dist.tarball');
  const integrity = stringAt(value, ['dist', 'integrity'], 'dist.integrity');
  const modified = stringAt(value, ['time', 'modified'], 'time.modified');
  if (!version) throw new Error('npm metadata missing version');
  rejectFloating(version);
  rejectFloating(gitHead);
  if (!fullSha(gitHead)) throw new Error('npm metadata missing full gitHead');
  if (!tarball || !tarball.startsWith('https://registry.npmjs.org/')) throw new Error('npm metadata missing registry tarball');
  if (!integrity || !integrity.startsWith('sha512-')) throw new Error('npm metadata missing sha512 integrity');
  if (!modified) throw new Error('npm metadata missing modified timestamp');
  return {
    package_name: 'promptfoo',
    package_version: version,
    git_head: gitHead,
    tarball,
    integrity,
    modified,
    source: 'npm view promptfoo version gitHead dist.tarball dist.integrity time.modified --json',
  };
}

function parseRelease(json) {
  const value = JSON.parse(json);
  const tag = value.tagName || value.tag_name;
  const commit = value.targetCommitish || value.target_commitish;
  if (!tag) throw new Error('GitHub latest release metadata missing tagName/tag_name');
  if (!commit || !fullSha(commit)) throw new Error('GitHub latest release target commit must be a full 40 character SHA');
  return {
    tag,
    ref: `refs/tags/${tag}`,
    commit,
    name: value.name || tag,
    url: value.htmlUrl || value.html_url || `https://github.com/promptfoo/promptfoo/releases/tag/${tag}`,
    published_at: value.publishedAt || value.published_at || 'unknown',
  };
}

function channel(refName) {
  const tag = refName.replace(/^refs\/tags\//, '');
  if (tag.startsWith('code-scan-action-')) return 'github-action';
  if (/^[0-9]/.test(tag)) return 'core-package';
  return 'other';
}

const npm = parseNpm(fs.readFileSync(npmPath, 'utf8'));
const release = parseRelease(fs.readFileSync(releasePath, 'utf8'));
const npmTagRef = `refs/tags/${npm.package_version}`;
let defaultBranchHead = null;
let npmTagCommit = null;
let latestReleaseCommit = null;
for (const line of fs.readFileSync(lsRemotePath, 'utf8').split(/\r?\n/).map((entry) => entry.trim()).filter(Boolean)) {
  const [sha, ref] = line.split(/\s+/);
  if (!fullSha(sha)) throw new Error(`ls-remote ref ${ref || '<missing>'} did not contain a full sha`);
  if (ref === 'HEAD') defaultBranchHead = sha;
  else if (ref === npmTagRef || ref === `${npmTagRef}^{}`) npmTagCommit = sha;
  else if (ref === release.ref || ref === `${release.ref}^{}`) latestReleaseCommit = sha;
}
if (!defaultBranchHead) throw new Error('ls-remote output missing HEAD');
if (!npmTagCommit) throw new Error(`ls-remote output missing ${npmTagRef}`);
if (!latestReleaseCommit) throw new Error(`ls-remote output missing ${release.ref}`);
if (latestReleaseCommit !== release.commit) {
  throw new Error(`GitHub release metadata commit ${release.commit} differs from ls-remote ${latestReleaseCommit}`);
}
const latestReleaseChannel = channel(release.ref);
const latestReleaseIsCore = latestReleaseChannel === 'core-package' && latestReleaseCommit === npm.git_head;
const defaultBranchMatchesNpm = defaultBranchHead === npm.git_head;
const downstreamRequiredEvidence = [
  'current_latest_source_inventory',
  'current_latest_matrix',
  'current_latest_golden_corpus',
  'current_latest_quality_gate',
  'external_authority_or_waivers',
  'publication_authority_or_waivers',
];
const reason = [
  `npm latest package ${npm.package_version} records gitHead ${npm.git_head}`,
  defaultBranchMatchesNpm ? null : `GitHub default branch HEAD ${defaultBranchHead} differs from npm latest gitHead ${npm.git_head}`,
  latestReleaseChannel === 'core-package' ? null : `GitHub latest release ${release.ref} is classified as ${latestReleaseChannel}, not core package release evidence`,
  'downstream source inventory, golden corpus, quality, external authority, and publication evidence are still required',
].filter(Boolean).join('; ');
const report = {
  schema: 'promptfoo-rs.current-latest-target.v1',
  status: defaultBranchMatchesNpm && latestReleaseIsCore ? 'locked' : 'locked-with-drift',
  npm_latest: npm,
  github: {
    default_branch_head: defaultBranchHead,
    npm_tag_ref: npmTagRef,
    npm_tag_commit: npmTagCommit,
    latest_release_ref: release.ref,
    latest_release_commit: latestReleaseCommit,
    latest_release_name: release.name,
    latest_release_url: release.url,
    latest_release_published_at: release.published_at,
    latest_release_channel: latestReleaseChannel,
    latest_release_is_core_package: latestReleaseIsCore,
    source: 'git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/<npm-version> refs/tags/<latest-release>',
    observed_at: `unix:${Math.floor(Date.now() / 1000)}`,
  },
  target_selection_blocker_resolved: true,
  current_latest_claim_allowed: false,
  downstream_required_evidence: downstreamRequiredEvidence,
  reason,
  observed_at: `unix:${Math.floor(Date.now() / 1000)}`,
};
fs.writeFileSync(outPath, `${JSON.stringify(report, null, 2)}\n`);

if (process.env.CURRENT_LATEST_WRITE_TRACKED === '1') {
  fs.mkdirSync('compatibility/inventory', { recursive: true });
  fs.mkdirSync('docs/compatibility', { recursive: true });
  fs.writeFileSync('compatibility/inventory/current-latest-target.json', `${JSON.stringify(report, null, 2)}\n`);
  fs.writeFileSync(
    'docs/compatibility/current-latest.lock.md',
    `# Current Latest Target Lock\n\n- **Schema**: \`${report.schema}\`\n- **Status**: \`${report.status}\`\n- **Observed At**: \`${report.observed_at}\`\n- **npm latest**: \`promptfoo@${npm.package_version}\` / \`${npm.git_head}\`\n- **npm tarball**: \`${npm.tarball}\`\n- **npm integrity**: \`${npm.integrity}\`\n- **GitHub default branch HEAD**: \`${defaultBranchHead}\`\n- **GitHub latest release**: \`${release.ref}\` / \`${latestReleaseCommit}\` / channel \`${latestReleaseChannel}\`\n- **Target selection blocker resolved**: \`true\`\n- **Current latest claim allowed**: \`false\`\n\n## Reason\n\n${reason}\n\n## Downstream Required Evidence\n\n${downstreamRequiredEvidence.map((item) => `- \`${item}\``).join('\n')}\n`,
  );
}
NODE
