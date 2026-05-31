#!/usr/bin/env bash
set -euo pipefail

GATE_DIR="target/release-gates"
OUT="$GATE_DIR/current-upstream-policy.json"
mkdir -p "$GATE_DIR"

tmpfile="$(mktemp)"
trap 'rm -f "$tmpfile"' EXIT

if [ -n "${CURRENT_UPSTREAM_LS_REMOTE_FILE:-}" ]; then
  cp "$CURRENT_UPSTREAM_LS_REMOTE_FILE" "$tmpfile"
elif [ -n "${CURRENT_UPSTREAM_LS_REMOTE_FIXTURE:-}" ]; then
  printf '%s\n' "$CURRENT_UPSTREAM_LS_REMOTE_FIXTURE" > "$tmpfile"
else
  git ls-remote https://github.com/promptfoo/promptfoo.git \
    HEAD refs/tags/0.121.13 refs/tags/code-scan-action-0.1.7 > "$tmpfile"
fi

node - "$tmpfile" "$OUT" "${CURRENT_UPSTREAM_TARGET_MODE:-frozen}" <<'NODE'
const fs = require('fs');
const [inputPath, outputPath, targetMode] = process.argv.slice(2);
const lsRemote = fs.readFileSync(inputPath, 'utf8');
const frozenSha = '4860e990c7e9a2f8f677173fb92cf9867b34d03f';
const npmIntegrity = 'sha512-DBPSixUophzcD7S7lML6SqVwnVtrhK5A3HsZ03IG9Xrw0t24r5imG7nLj+YMb0vlAjbdFtE7yFG+rsqDpfYp6g==';
const requiredCurrentEvidence = [
  'source_inventory',
  'matrix',
  'fixtures',
  'golden_corpus',
  'release_candidate',
];

function isFullSha(value) {
  return /^[0-9a-fA-F]{40}$/.test(value);
}

function parseLsRemote(output) {
  let currentHead = null;
  let frozenTagCommit = null;
  let observedReleaseRef = null;
  let observedReleaseCommit = null;
  for (const line of output.split(/\r?\n/).map((entry) => entry.trim()).filter(Boolean)) {
    const [sha, ref] = line.split(/\s+/);
    if (!isFullSha(sha)) {
      throw new Error(`ls-remote ref ${ref || '<missing>'} did not contain a full sha`);
    }
    if (ref === 'HEAD') {
      currentHead = sha;
    } else if (ref === 'refs/tags/0.121.13') {
      frozenTagCommit = sha;
    } else if (ref && ref.startsWith('refs/tags/')) {
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

function evaluate(frozen, current, mode) {
  const missing = [];
  const mismatched = [];
  if (mode === 'current') {
    for (const key of requiredCurrentEvidence) {
      const ref = current.evidence_refs[key];
      if (!ref) missing.push(key);
      else if (ref !== current.current_head) mismatched.push(key);
    }
  }
  const currentEvidenceReady = missing.length === 0 && mismatched.length === 0;
  const currentPerfectClaimAllowed = mode === 'current' && currentEvidenceReady;
  let reason;
  if (mode === 'frozen' && current.current_head !== frozen.git_commit) {
    reason = `target mode is frozen; current HEAD ${current.current_head} differs from frozen baseline ${frozen.git_commit}`;
  } else if (mode === 'frozen') {
    reason = 'target mode is frozen; current-perfect claims require current mode evidence';
  } else if (currentEvidenceReady) {
    reason = `all current mode evidence shares observed ref ${current.current_head}`;
  } else {
    reason = `current mode evidence is incomplete or mismatched for observed ref ${current.current_head}`;
  }
  let stableClaim = 'frozen-baseline compatibility';
  if (mode === 'current') {
    stableClaim = currentPerfectClaimAllowed
      ? 'current-upstream perfect refactor'
      : 'current-upstream blocked';
  }
  return {
    schema: 'promptfoo-rs.current-upstream-policy.v1',
    status: mode === 'frozen' || currentPerfectClaimAllowed ? 'ready' : 'blocked',
    target_mode: mode,
    stable_claim: stableClaim,
    current_perfect_claim_allowed: currentPerfectClaimAllowed,
    reason,
    frozen,
    current,
    required_current_evidence: requiredCurrentEvidence,
    missing_current_evidence: missing,
    mismatched_current_evidence: mismatched,
  };
}

const mode = targetMode === 'current' ? 'current' : 'frozen';
const frozen = {
  package_version: '0.121.13',
  git_ref: 'refs/tags/0.121.13',
  git_commit: frozenSha,
  npm_integrity: npmIntegrity,
  acquisition_command: 'git ls-remote https://github.com/promptfoo/promptfoo.git refs/tags/0.121.13',
  source_files: [],
};
const policy = evaluate(frozen, parseLsRemote(lsRemote), mode);
fs.writeFileSync(outputPath, JSON.stringify(policy, null, 2) + '\n');
if (policy.status === 'blocked') {
  console.error(JSON.stringify(policy, null, 2));
  process.exit(1);
}
NODE
