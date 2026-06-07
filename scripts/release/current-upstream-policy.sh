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
    HEAD refs/tags/0.121.15 refs/tags/0.121.13 > "$tmpfile"
fi

node - "$tmpfile" "$OUT" "${CURRENT_UPSTREAM_TARGET_MODE:-product-baseline}" <<'NODE'
const fs = require('fs');
const {
  loadProductBaselineTarget,
} = require('./product-baseline-gate-lib.cjs');

const [inputPath, outputPath, targetModeArg] = process.argv.slice(2);
const lsRemote = fs.readFileSync(inputPath, 'utf8');
const phase1FrozenSha = '4860e990c7e9a2f8f677173fb92cf9867b34d03f';
const phase1NpmIntegrity =
  'sha512-DBPSixUophzcD7S7lML6SqVwnVtrhK5A3HsZ03IG9Xrw0t24r5imG7nLj+YMb0vlAjbdFtE7yFG+rsqDpfYp6g==';
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

function parseLsRemote(output, frozenTagRef) {
  let currentHead = null;
  let frozenTagCommit = null;
  let observedReleaseRef = null;
  let observedReleaseCommit = null;
  for (const line of output
    .split(/\r?\n/)
    .map((entry) => entry.trim())
    .filter(Boolean)) {
    const [sha, ref] = line.split(/\s+/);
    if (!isFullSha(sha)) {
      throw new Error(`ls-remote ref ${ref || '<missing>'} did not contain a full sha`);
    }
    if (ref === 'HEAD') {
      currentHead = sha;
    } else if (ref === frozenTagRef) {
      frozenTagCommit = sha;
    } else if (ref && ref.startsWith('refs/tags/')) {
      observedReleaseRef = ref;
      observedReleaseCommit = sha;
    }
  }
  if (!currentHead) throw new Error('ls-remote output missing HEAD');
  if (!frozenTagCommit) throw new Error(`ls-remote output missing ${frozenTagRef}`);
  return {
    current_head: currentHead,
    frozen_tag_ref: frozenTagRef,
    frozen_tag_commit: frozenTagCommit,
    observed_release_ref: observedReleaseRef,
    observed_release_commit: observedReleaseCommit,
    observed_at: `unix:${Math.floor(Date.now() / 1000)}`,
    source:
      'git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.15 refs/tags/0.121.13',
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
  if (mode === 'product-baseline') {
    const headDiffers = current.current_head !== frozen.git_commit;
    reason = headDiffers
      ? `product baseline frozen at promptfoo@${frozen.package_version} per ADR-012; GitHub HEAD ${current.current_head} drift is observation-only and does not require rebaseline`
      : `product baseline frozen at promptfoo@${frozen.package_version} per ADR-012`;
  } else if (mode === 'frozen' && current.current_head !== frozen.git_commit) {
    reason = `target mode is frozen; current HEAD ${current.current_head} differs from frozen baseline ${frozen.git_commit}`;
  } else if (mode === 'frozen') {
    reason = 'target mode is frozen; current-perfect claims require current mode evidence';
  } else if (currentEvidenceReady) {
    reason = `all current mode evidence shares observed ref ${current.current_head}`;
  } else {
    reason = `current mode evidence is incomplete or mismatched for observed ref ${current.current_head}`;
  }
  let stableClaim = 'frozen-baseline compatibility';
  if (mode === 'product-baseline') {
    stableClaim = 'product-baseline compatibility (ADR-012)';
  } else if (mode === 'current') {
    stableClaim = currentPerfectClaimAllowed
      ? 'current-upstream perfect refactor'
      : 'current-upstream blocked';
  }
  const status =
    mode === 'product-baseline' || mode === 'frozen' || currentPerfectClaimAllowed
      ? 'ready'
      : 'blocked';
  return {
    schema: 'promptfoo-rs.current-upstream-policy.v1',
    status,
    target_mode: mode,
    stable_claim: stableClaim,
    current_perfect_claim_allowed: currentPerfectClaimAllowed,
    product_baseline_frozen: mode === 'product-baseline',
    current_upstream_rebaseline_required: mode === 'current',
    product_baseline_adr:
      mode === 'product-baseline'
        ? 'docs/decisions/adr-012-product-independence-baseline-freeze.md'
        : null,
    reason,
    frozen,
    current,
    required_current_evidence: requiredCurrentEvidence,
    missing_current_evidence: missing,
    mismatched_current_evidence: mismatched,
  };
}

const modeArg = targetModeArg === 'current' ? 'current' : targetModeArg;
let mode = modeArg;
let frozen;
if (modeArg === 'product-baseline') {
  const productBaseline = loadProductBaselineTarget();
  if (!productBaseline) {
    throw new Error('product-baseline mode requires compatibility/inventory/current-latest-target.json');
  }
  frozen = {
    package_version: productBaseline.package_version,
    git_ref: productBaseline.git_ref,
    git_commit: productBaseline.git_commit,
    npm_integrity: productBaseline.npm_integrity,
    acquisition_command: productBaseline.acquisition_command,
    source_files: ['compatibility/inventory/current-latest-target.json'],
    phase1_historical_baseline: {
      package_version: '0.121.13',
      git_ref: 'refs/tags/0.121.13',
      git_commit: phase1FrozenSha,
      npm_integrity: phase1NpmIntegrity,
    },
  };
  mode = 'product-baseline';
} else if (modeArg === 'frozen') {
  frozen = {
    package_version: '0.121.13',
    git_ref: 'refs/tags/0.121.13',
    git_commit: phase1FrozenSha,
    npm_integrity: phase1NpmIntegrity,
    acquisition_command:
      'git ls-remote https://github.com/promptfoo/promptfoo.git refs/tags/0.121.13',
    source_files: [],
  };
  mode = 'frozen';
} else {
  frozen = {
    package_version: '0.121.13',
    git_ref: 'refs/tags/0.121.13',
    git_commit: phase1FrozenSha,
    npm_integrity: phase1NpmIntegrity,
    acquisition_command:
      'git ls-remote https://github.com/promptfoo/promptfoo.git refs/tags/0.121.13',
    source_files: [],
  };
  mode = 'current';
}

const policy = evaluate(frozen, parseLsRemote(lsRemote, frozen.git_ref), mode);
fs.writeFileSync(outputPath, `${JSON.stringify(policy, null, 2)}\n`);
if (policy.status === 'blocked') {
  console.error(JSON.stringify(policy, null, 2));
  process.exit(1);
}
NODE
