#!/usr/bin/env bash
set -euo pipefail

GATE_DIR="target/release-gates"
OUT="$GATE_DIR/upstream-distribution-target.json"
mkdir -p "$GATE_DIR"

npm_tmp="$(mktemp)"
ls_tmp="$(mktemp)"
trap 'rm -f "$npm_tmp" "$ls_tmp"' EXIT

if [ -n "${UPSTREAM_NPM_VIEW_FILE:-}" ]; then
  cp "$UPSTREAM_NPM_VIEW_FILE" "$npm_tmp"
else
  npm view promptfoo version gitHead dist.tarball dist.integrity time.modified --json > "$npm_tmp"
fi

if [ -n "${UPSTREAM_LS_REMOTE_FILE:-}" ]; then
  cp "$UPSTREAM_LS_REMOTE_FILE" "$ls_tmp"
else
  git ls-remote https://github.com/promptfoo/promptfoo.git \
    HEAD refs/tags/0.121.13 refs/tags/code-scan-action-0.1.7 > "$ls_tmp"
fi

node - "$npm_tmp" "$ls_tmp" "$OUT" <<'NODE'
const fs = require('fs');
const [npmPath, lsRemotePath, outputPath] = process.argv.slice(2);
const frozenSha = '4860e990c7e9a2f8f677173fb92cf9867b34d03f';
const frozenVersion = '0.121.13';
const npmIntegrity = 'sha512-DBPSixUophzcD7S7lML6SqVwnVtrhK5A3HsZ03IG9Xrw0t24r5imG7nLj+YMb0vlAjbdFtE7yFG+rsqDpfYp6g==';

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

function parseNpm(json) {
  const value = JSON.parse(json);
  const version = stringAt(value, ['version']);
  const gitHead = stringAt(value, ['gitHead']);
  const tarball = stringAt(value, ['dist', 'tarball'], 'dist.tarball');
  const integrity = stringAt(value, ['dist', 'integrity'], 'dist.integrity');
  const modified = stringAt(value, ['time', 'modified'], 'time.modified');
  if (!version) throw new Error('npm metadata missing version');
  if (!fullSha(gitHead)) throw new Error('npm metadata missing full gitHead');
  if (!tarball || !tarball.startsWith('https://registry.npmjs.org/')) {
    throw new Error('npm metadata missing registry tarball');
  }
  if (!integrity || !integrity.startsWith('sha512-')) {
    throw new Error('npm metadata missing sha512 integrity');
  }
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

function parseLsRemote(output) {
  let currentHead = null;
  let frozenTagCommit = null;
  let observedReleaseRef = null;
  let observedReleaseCommit = null;
  for (const line of output.split(/\r?\n/).map((entry) => entry.trim()).filter(Boolean)) {
    const [sha, ref] = line.split(/\s+/);
    if (!fullSha(sha)) throw new Error(`ls-remote ref ${ref || '<missing>'} did not contain a full sha`);
    if (ref === 'HEAD') currentHead = sha;
    else if (ref === 'refs/tags/0.121.13') frozenTagCommit = sha;
    else if (ref && ref.startsWith('refs/tags/')) {
      observedReleaseRef = ref;
      observedReleaseCommit = sha;
    }
  }
  if (!currentHead) throw new Error('ls-remote output missing HEAD');
  if (!frozenTagCommit) throw new Error('ls-remote output missing refs/tags/0.121.13');
  return {
    current_head: currentHead,
    frozen_tag_ref: 'refs/tags/0.121.13',
    frozen_tag_commit: frozenTagCommit,
    observed_release_ref: observedReleaseRef,
    observed_release_commit: observedReleaseCommit,
    observed_at: `unix:${Math.floor(Date.now() / 1000)}`,
    source: 'git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.13 refs/tags/code-scan-action-0.1.7',
    evidence_refs: {},
  };
}

function releaseChannel(refName) {
  if (!refName) return 'none';
  const tag = refName.replace(/^refs\/tags\//, '');
  if (tag.startsWith('code-scan-action-')) return 'github-action';
  if (/^[0-9]/.test(tag)) return 'core-package';
  return 'other';
}

function reason(npm, github, frozen, npmMatchesFrozen, headMatchesNpm, latestIsCore, channel) {
  if (headMatchesNpm && latestIsCore) {
    return `npm core package ${npm.package_version}, repository HEAD, and GitHub latest core release share ${npm.git_head}`;
  }
  if (npmMatchesFrozen) {
    const parts = [
      `npm core package ${npm.package_version} matches frozen baseline ${frozen.git_commit}, preserving frozen-baseline evidence for the published core package`,
    ];
    if (!headMatchesNpm) {
      parts.push(`repository HEAD ${github.current_head} differs from npm core gitHead ${npm.git_head}`);
    }
    if (!latestIsCore) {
      parts.push(`GitHub latest observed release ${github.observed_release_ref || '<none>'} is classified as ${channel}, not npm core package evidence`);
    }
    return parts.join('; ');
  }
  return `npm core package ${npm.package_version} (${npm.git_head}) differs from frozen baseline ${frozen.git_commit}`;
}

const npm = parseNpm(fs.readFileSync(npmPath, 'utf8'));
const github = parseLsRemote(fs.readFileSync(lsRemotePath, 'utf8'));
const frozen = {
  package_version: frozenVersion,
  git_ref: 'refs/tags/0.121.13',
  git_commit: frozenSha,
  npm_integrity: npmIntegrity,
  acquisition_command: 'git ls-remote https://github.com/promptfoo/promptfoo.git refs/tags/0.121.13',
  source_files: [],
};
const npmMatchesFrozen =
  npm.package_version === frozen.package_version &&
  npm.git_head === frozen.git_commit &&
  npm.integrity === frozen.npm_integrity;
const headMatchesNpm = github.current_head === npm.git_head;
const channel = releaseChannel(github.observed_release_ref);
const latestIsCore = channel === 'core-package' && github.observed_release_commit === npm.git_head;
const currentReady = headMatchesNpm && latestIsCore;
const report = {
  schema: 'promptfoo-rs.upstream-distribution-target.v1',
  status: currentReady ? 'ready' : (npmMatchesFrozen ? 'ready-with-drift' : 'blocked'),
  frozen,
  npm_core: npm,
  github,
  npm_core_matches_frozen_baseline: npmMatchesFrozen,
  repository_head_matches_npm_core: headMatchesNpm,
  github_latest_release_is_core_package: latestIsCore,
  github_latest_release_channel: channel,
  current_repository_perfect_claim_allowed: currentReady,
  reason: reason(npm, github, frozen, npmMatchesFrozen, headMatchesNpm, latestIsCore, channel),
  observed_at: `unix:${Math.floor(Date.now() / 1000)}`,
};
fs.writeFileSync(outputPath, `${JSON.stringify(report, null, 2)}\n`);
NODE
