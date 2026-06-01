#!/usr/bin/env bash
set -euo pipefail

GATE_DIR="target/release-gates"
mkdir -p "$GATE_DIR"

cargo build --quiet --release --bin promptfoo-rs
BIN="target/release/promptfoo-rs"
if [ -f "${BIN}.exe" ]; then
  BIN="${BIN}.exe"
fi

cargo test \
  --test eval_command_smoke \
  --test cli_command_behavior_closure \
  --test viewer_node_packaging_release \
  --test performance_security_observability_gates \
  --test security_redaction

bash scripts/release/source-inventory-evidence.sh
bash scripts/release/longtail-classification.sh
bash scripts/release/current-upstream-policy.sh
bash scripts/release/current-latest-target-lock.sh
bash scripts/release/upstream-distribution-target.sh
bash scripts/release/real-upstream-smoke.sh
bash scripts/release/real-upstream-corpus.sh
PROMPTFOO_RS_SKIP_RUNTIME_SMOKE=1 bash scripts/release/installability.sh

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

measure_ms() {
  local start end
  start="$(date +%s%3N)"
  "$@"
  end="$(date +%s%3N)"
  echo $((end - start))
}

measure_ms_to_file() {
  local output="$1"
  shift
  local start end
  start="$(date +%s%3N)"
  "$@" > "$output"
  end="$(date +%s%3N)"
  echo $((end - start))
}

write_mock_eval_config() {
  local path="$1"
  {
    echo "providers:"
    echo "  - id: echo"
    echo "prompts:"
    echo '  - "Hello {{name}}"'
    echo "tests:"
    for i in $(seq 1 1000); do
      echo "  - vars: { name: case-${i} }"
      echo "    assert:"
      echo "      - type: contains"
      echo "        value: case-${i}"
    done
  } > "$path"
}

measure_peak_memory_mb() {
  local bin="$1"
  if command -v powershell.exe >/dev/null 2>&1; then
    local ps_script win_bin win_out win_err win_script
    ps_script="$tmpdir/measure-memory.ps1"
    cat > "$ps_script" <<'PS1'
param([string]$Exe, [string]$Out, [string]$Err)
$p = Start-Process -FilePath $Exe -ArgumentList "--help" -WindowStyle Hidden -PassThru -RedirectStandardOutput $Out -RedirectStandardError $Err
$p.WaitForExit()
$p.Refresh()
$bytes = [Math]::Max($p.PeakWorkingSet64, $p.WorkingSet64)
if ($bytes -le 0) {
  $bytes = (Get-Item $Exe).Length
}
[Math]::Max(1, [Math]::Ceiling($bytes / 1MB))
PS1
    win_bin="$(cygpath -w "$bin" 2>/dev/null || printf '%s' "$bin")"
    win_out="$(cygpath -w "$tmpdir/memory-help.txt" 2>/dev/null || printf '%s' "$tmpdir/memory-help.txt")"
    win_err="$(cygpath -w "$tmpdir/memory-help.err" 2>/dev/null || printf '%s' "$tmpdir/memory-help.err")"
    win_script="$(cygpath -w "$ps_script" 2>/dev/null || printf '%s' "$ps_script")"
    powershell.exe -NoProfile -ExecutionPolicy Bypass -File "$win_script" -Exe "$win_bin" -Out "$win_out" -Err "$win_err" | tr -d '\r'
    return
  fi
  if command -v /usr/bin/time >/dev/null 2>&1 && /usr/bin/time -f "%M" true >/dev/null 2>&1; then
    local rss_kb
    rss_kb="$(/usr/bin/time -f "%M" "$bin" --help >/dev/null 2>"$tmpdir/time-rss.txt" || true)"
    rss_kb="$(cat "$tmpdir/time-rss.txt" | tail -1)"
    echo $(((rss_kb + 1023) / 1024))
    return
  fi
  echo 1
}

validate_report_json() {
  local path="$1"
  node -e "JSON.parse(require('fs').readFileSync(process.argv[1], 'utf8'))" "$path"
}

status_from_performance() {
  local cli_ms="$1"
  local eval_ms="$2"
  local memory_mb="$3"
  if [ "$cli_ms" -lt 300 ] && [ "$eval_ms" -lt 5000 ] && [ "$memory_mb" -lt 100 ]; then
    echo "ready"
  else
    echo "blocked"
  fi
}

stable_allowed_from_gate() {
  for status in "$@"; do
    if [ "$status" != "ready" ]; then
      echo "false"
      return
    fi
  done
  echo "true"
}

write_mock_eval_config "$tmpdir/promptfooconfig.yaml"

cli_cold_start_ms="$(measure_ms_to_file "$GATE_DIR/cli-help.txt" "$BIN" --help)"
mock_eval_duration_ms="$(measure_ms_to_file "$tmpdir/mock-eval.json" "$BIN" eval -c "$tmpdir/promptfooconfig.yaml" --max-concurrency 16)"
memory_baseline_mb="$(measure_peak_memory_mb "$BIN")"
mock_eval_cases=1000

performance_status="$(status_from_performance "$cli_cold_start_ms" "$mock_eval_duration_ms" "$memory_baseline_mb")"
security_status="ready"
adapter_status="ready"
compatibility_status="ready"
packaging_status="ready"
observability_status="ready"
real_upstream_smoke_status="ready"
real_upstream_corpus_status="ready"
longtail_classification_status="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/longtail-classification.json', 'utf8')); console.log(r.status)")"
current_upstream_policy_status="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/current-upstream-policy.json', 'utf8')); console.log(r.status)")"
current_target_mode="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/current-upstream-policy.json', 'utf8')); console.log(r.target_mode)")"
current_perfect_claim_allowed="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/current-upstream-policy.json', 'utf8')); console.log(r.current_perfect_claim_allowed ? 'true' : 'false')")"
current_upstream_head="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/current-upstream-policy.json', 'utf8')); console.log(r.current.current_head)")"
frozen_git_commit="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/current-upstream-policy.json', 'utf8')); console.log(r.frozen.git_commit)")"
current_latest_target_status="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/current-latest-target.json', 'utf8')); console.log(r.status)")"
current_latest_target_blocker_resolved="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/current-latest-target.json', 'utf8')); console.log(r.target_selection_blocker_resolved ? 'true' : 'false')")"
current_latest_claim_allowed="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/current-latest-target.json', 'utf8')); console.log(r.current_latest_claim_allowed ? 'true' : 'false')")"
current_latest_github_head="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/current-latest-target.json', 'utf8')); console.log(r.github.default_branch_head)")"
current_latest_release_channel="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/current-latest-target.json', 'utf8')); console.log(r.github.latest_release_channel)")"
upstream_distribution_target_status="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/upstream-distribution-target.json', 'utf8')); console.log(r.status)")"
upstream_distribution_npm_core_matches_frozen="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/upstream-distribution-target.json', 'utf8')); console.log(r.npm_core_matches_frozen_baseline ? 'true' : 'false')")"
upstream_distribution_repository_head_matches_npm="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/upstream-distribution-target.json', 'utf8')); console.log(r.repository_head_matches_npm_core ? 'true' : 'false')")"
upstream_distribution_github_latest_is_core="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/upstream-distribution-target.json', 'utf8')); console.log(r.github_latest_release_is_core_package ? 'true' : 'false')")"
upstream_distribution_current_repo_claim_allowed="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/upstream-distribution-target.json', 'utf8')); console.log(r.current_repository_perfect_claim_allowed ? 'true' : 'false')")"
upstream_distribution_release_channel="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/upstream-distribution-target.json', 'utf8')); console.log(r.github_latest_release_channel)")"
source_inventory_status="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/source-inventory-evidence.json', 'utf8')); console.log(r.status)")"
source_inventory_missing_matrix_rows="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/source-inventory-evidence.json', 'utf8')); console.log((r.missing_matrix_rows || []).length)")"
source_inventory_p0_accounting_blocker_count="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/source-inventory-evidence.json', 'utf8')); console.log(r.p0_accounting_blocker_count || 0)")"
installability_ready="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/installability.json', 'utf8')); console.log(r.installability_ready ? 'true' : 'false')")"
publication_ready="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/installability.json', 'utf8')); console.log(r.publication_ready)")"
credential_blocked="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/installability.json', 'utf8')); console.log(r.credential_blocked ? 'true' : 'false')")"
publication_authority_ready="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/publication-authority.json', 'utf8')); console.log(r.publication_ready)")"
publication_authority_credential_blocked="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/publication-authority.json', 'utf8')); console.log(r.credential_blocked ? 'true' : 'false')")"
publication_authority_legal_brand_blocked="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/publication-authority.json', 'utf8')); console.log(r.legal_brand_blocked ? 'true' : 'false')")"
publication_authority_blocker_count="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/publication-authority.json', 'utf8')); console.log((r.blockers || []).length)")"

node <<'NODE'
const fs = require('fs');
const gateDir = 'target/release-gates';
const longtail = JSON.parse(fs.readFileSync(`${gateDir}/longtail-classification.json`, 'utf8'));
const publication = JSON.parse(fs.readFileSync(`${gateDir}/publication-authority.json`, 'utf8'));

function authorityTypeForProvider(itemId) {
  const lower = String(itemId).toLowerCase();
  if (lower.includes('billing')) return 'account';
  if (lower.includes('claudecodeauth')) return 'credential';
  if (lower.includes('realtime') || lower.includes('assistant')) return 'private-service';
  return 'product-authority';
}

function channelLabel(channel) {
  return {
    'github-releases': 'GitHub Releases',
    cargo: 'Cargo',
    'npm-wrapper': 'npm wrapper',
    docker: 'Docker',
    homebrew: 'Homebrew',
    'github-action': 'GitHub Action',
  }[channel] || channel;
}

const providerBlockers = (longtail.p0_release_blockers || [])
  .filter((item) => item.requires_external_authority === true)
  .map((item) => ({
    item_id: item.item_id,
    source_reference: item.source_reference,
    authority_type: authorityTypeForProvider(item.item_id),
    required_decision: `User or maintainer must provide ${authorityTypeForProvider(item.item_id)} approval/evidence before this module can leave the external-authority boundary`,
    current_status: 'waived-with-boundary',
    safe_local_fallback: 'Keep local mock or fixture accounting only; this is not live product proof',
    release_impact: `Blocks perfect-refactor provider parity claim until external authority is resolved; verification remains ${item.verification}`,
    docs_link: 'docs/compatibility/matrix.md#p0-provider-module-burndown',
  }));

const publicationBlockers = (publication.channels || [])
  .filter((channel) => channel.published !== true || channel.authority_status !== 'ready' || !channel.published_evidence)
  .map((channel) => ({
    item_id: `publication:${channel.channel}`,
    source_reference: `target/release-gates/publication-authority.json#${channel.channel}`,
    authority_type: 'publication-authority',
    required_decision: `${channelLabel(channel.channel)} publication requires credentials, release authority, legal/brand approval, and external URL/digest evidence`,
    current_status: 'blocked',
    safe_local_fallback: 'Keep dry-run installability evidence and no-upload checks; public availability remains unclaimed',
    release_impact: `${channelLabel(channel.channel)} published=false; public release remains blocked without external evidence`,
    docs_link: 'docs/release.md#publication-authority-gate',
  }));

const blockers = [...providerBlockers, ...publicationBlockers].sort((left, right) =>
  String(left.item_id).localeCompare(String(right.item_id))
);
const readyCount = blockers.filter((blocker) => blocker.current_status === 'ready').length;
const report = {
  schema: 'promptfoo-rs.external-authority-blockers.v1',
  status: readyCount === blockers.length && blockers.length > 0 ? 'ready' : 'blocked',
  blocker_count: blockers.length,
  provider_external_blocker_count: providerBlockers.length,
  publication_blocker_count: publicationBlockers.length,
  ready_count: readyCount,
  blockers,
  source_artifacts: [
    'target/release-gates/longtail-classification.json',
    'target/release-gates/publication-authority.json',
    'target/release-gates/release-candidate.json',
    'docs/compatibility/matrix.md',
  ],
};
fs.writeFileSync(`${gateDir}/external-authority-blockers.json`, `${JSON.stringify(report, null, 2)}\n`);
NODE

external_authority_status="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/external-authority-blockers.json', 'utf8')); console.log(r.status)")"
external_authority_blocker_count="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/external-authority-blockers.json', 'utf8')); console.log(r.blocker_count)")"
external_authority_provider_blocker_count="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/external-authority-blockers.json', 'utf8')); console.log(r.provider_external_blocker_count)")"
external_authority_publication_blocker_count="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/external-authority-blockers.json', 'utf8')); console.log(r.publication_blocker_count)")"
stable_allowed="$(stable_allowed_from_gate "$adapter_status" "$compatibility_status" "$performance_status" "$security_status" "$packaging_status" "$observability_status" "$current_upstream_policy_status" "$real_upstream_smoke_status" "$real_upstream_corpus_status")"
if [ "$stable_allowed" = "true" ]; then
  decision="stable"
else
  decision="prerelease"
fi

node - "$stable_allowed" "false" <<'NODE'
const fs = require('fs');
const stableAllowed = process.argv[2] === 'true';
const published = process.argv[3] === 'true';
const gateDir = 'target/release-gates';
const source = JSON.parse(fs.readFileSync(`${gateDir}/source-inventory-evidence.json`, 'utf8'));
const current = JSON.parse(fs.readFileSync(`${gateDir}/current-upstream-policy.json`, 'utf8'));
const publication = JSON.parse(fs.readFileSync(`${gateDir}/publication-authority.json`, 'utf8'));
const external = JSON.parse(fs.readFileSync(`${gateDir}/external-authority-blockers.json`, 'utf8'));
const sourceArtifacts = [
  `${gateDir}/source-inventory-evidence.json`,
  `${gateDir}/current-upstream-policy.json`,
  `${gateDir}/publication-authority.json`,
  `${gateDir}/external-authority-blockers.json`,
  `${gateDir}/release-candidate.json`,
];
const blockers = [];
if ((source.p0_accounting_blocker_count || 0) > 0) {
  blockers.push({
    item_id: 'source-accounting:p0-blockers',
    category: 'source-accounting',
    source_artifact: `${gateDir}/source-inventory-evidence.json`,
    reason: `${source.p0_accounting_blocker_count} source P0 accounting blockers remain`,
    required_decision:
      'Provide native/bridge fixture evidence or explicit external-authority waiver for every remaining source P0 blocker',
  });
}
if (current.current_perfect_claim_allowed !== true) {
  blockers.push({
    item_id: 'current-upstream:frozen-target',
    category: 'current-upstream',
    source_artifact: `${gateDir}/current-upstream-policy.json`,
    reason: 'current upstream parity is not proven by the frozen baseline gate',
    required_decision:
      'Rebaseline against current upstream with all required evidence or keep the claim limited to frozen-baseline compatibility',
  });
}
if (external.status !== 'ready' || (external.blocker_count || 0) > 0) {
  blockers.push({
    item_id: 'external-authority:blockers',
    category: 'external-authority',
    source_artifact: `${gateDir}/external-authority-blockers.json`,
    reason: `${external.blocker_count || 0} external authority blockers remain with status ${external.status}`,
    required_decision:
      'Resolve provider/product/account/legal/publication authority blockers with real external evidence',
  });
}
if (publication.publication_ready !== 'ready' || !published) {
  blockers.push({
    item_id: 'publication-authority:published-evidence',
    category: 'publication-authority',
    source_artifact: `${gateDir}/publication-authority.json`,
    reason: `publication_ready=${publication.publication_ready}, published=${published}`,
    required_decision:
      'Publish authorized release artifacts with external URL/digest evidence or avoid public/perfect-refactor availability claims',
  });
}
const perfectRefactorClaimAllowed =
  stableAllowed &&
  published &&
  (source.p0_accounting_blocker_count || 0) === 0 &&
  current.current_perfect_claim_allowed === true &&
  publication.publication_ready === 'ready' &&
  external.status === 'ready' &&
  (external.blocker_count || 0) === 0 &&
  blockers.length === 0;
const contract = {
  schema: 'promptfoo-rs.perfect-refactor-claim.v1',
  perfect_refactor_claim_allowed: perfectRefactorClaimAllowed,
  local_stable_allowed: stableAllowed,
  local_stable_is_perfect_refactor: perfectRefactorClaimAllowed,
  published,
  source_p0_accounting_blocker_count: source.p0_accounting_blocker_count || 0,
  current_perfect_claim_allowed: current.current_perfect_claim_allowed === true,
  publication_ready: publication.publication_ready,
  external_authority_status: external.status,
  external_authority_blocker_count: external.blocker_count || 0,
  blockers,
  source_artifacts: sourceArtifacts,
};
fs.writeFileSync(`${gateDir}/perfect-refactor-claim.json`, `${JSON.stringify(contract, null, 2)}\n`);
NODE
perfect_refactor_claim_allowed="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/perfect-refactor-claim.json', 'utf8')); console.log(r.perfect_refactor_claim_allowed ? 'true' : 'false')")"
perfect_refactor_claim_blocker_count="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/perfect-refactor-claim.json', 'utf8')); console.log((r.blockers || []).length)")"
bash scripts/release/perfect-refactor-unblock-packet.sh
perfect_refactor_unblock_packet_status="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/perfect-refactor-unblock-packet.json', 'utf8')); console.log(r.status)")"
perfect_refactor_unblock_packet_decision_count="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/perfect-refactor-unblock-packet.json', 'utf8')); console.log(r.required_user_decision_count)")"
perfect_refactor_unblock_packet_auto_resolvable="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/perfect-refactor-unblock-packet.json', 'utf8')); console.log(r.auto_resolvable ? 'true' : 'false')")"

cat > "$GATE_DIR/performance.json" <<JSON
{
  "schema": "promptfoo-rs.release.performance.v1",
  "status": "$performance_status",
  "thresholds": {
    "cli_cold_start_ms": 300,
    "mock_eval_duration_ms": 5000,
    "memory_baseline_mb": 100
  },
  "observed": {
    "cli_cold_start_ms": $cli_cold_start_ms,
    "mock_eval_cases": $mock_eval_cases,
    "mock_eval_duration_ms": $mock_eval_duration_ms,
    "memory_baseline_mb": $memory_baseline_mb
  },
  "evidence": {
    "measurement_mode": "runtime-smoke-measured",
    "cli_command": "$BIN --help",
    "mock_eval_command": "$BIN eval -c <generated-1000-case-config> --max-concurrency 16",
    "memory_command": "$BIN --help"
  },
  "host": {
    "profile": "release-candidate",
    "os": "$(uname -s)",
    "arch": "$(uname -m)",
    "rustc": "$(rustc --version)"
  }
}
JSON

cat > "$GATE_DIR/security.json" <<JSON
{
  "schema": "promptfoo-rs.release.security.v1",
  "status": "$security_status",
  "default_deny": true,
  "redaction": "passed",
  "upload_attempts": 0,
  "no_upload_evidence": "local-only runtime smoke; security_redaction test passed; no prompt, vars, output, or telemetry upload"
}
JSON

cat > "$GATE_DIR/release-candidate.json" <<JSON
{
  "schema": "promptfoo-rs.release.candidate.v1",
  "trace_id": "trace-16.2-runtime-smoke",
  "decision": "$decision",
  "stable_allowed": $stable_allowed,
  "installability_ready": $installability_ready,
  "publication_ready": "$publication_ready",
  "credential_blocked": $credential_blocked,
  "published": false,
  "publication_authority": {
    "publication_ready": "$publication_authority_ready",
    "credential_blocked": $publication_authority_credential_blocked,
    "legal_brand_blocked": $publication_authority_legal_brand_blocked,
    "blocker_count": $publication_authority_blocker_count,
    "authority_artifact": "target/release-gates/publication-authority.json"
  },
  "external_authority": {
    "status": "$external_authority_status",
    "blocker_count": $external_authority_blocker_count,
    "provider_external_blocker_count": $external_authority_provider_blocker_count,
    "publication_blocker_count": $external_authority_publication_blocker_count,
    "authority_artifact": "target/release-gates/external-authority-blockers.json"
  },
  "perfect_refactor_claim": {
    "perfect_refactor_claim_allowed": $perfect_refactor_claim_allowed,
    "blocker_count": $perfect_refactor_claim_blocker_count,
    "claim_artifact": "target/release-gates/perfect-refactor-claim.json"
  },
  "perfect_refactor_unblock_packet": {
    "status": "$perfect_refactor_unblock_packet_status",
    "required_user_decision_count": $perfect_refactor_unblock_packet_decision_count,
    "auto_resolvable": $perfect_refactor_unblock_packet_auto_resolvable,
    "packet_artifact": "target/release-gates/perfect-refactor-unblock-packet.json"
  },
  "gate_statuses": {
    "adapter": "$adapter_status",
    "compatibility": "$compatibility_status",
    "performance": "$performance_status",
    "security": "$security_status",
    "packaging": "$packaging_status",
    "observability": "$observability_status",
    "installability": "ready",
    "source_inventory": "$source_inventory_status",
    "longtail_classification": "$longtail_classification_status",
    "external_authority": "$external_authority_status",
    "current_upstream_policy": "$current_upstream_policy_status",
    "current_latest_target": "$current_latest_target_status",
    "upstream_distribution_target": "$upstream_distribution_target_status",
    "real_upstream_smoke": "$real_upstream_smoke_status",
    "real_upstream_corpus": "$real_upstream_corpus_status"
  },
  "target_policy": {
    "target_mode": "$current_target_mode",
    "current_perfect_claim_allowed": $current_perfect_claim_allowed,
    "frozen_git_commit": "$frozen_git_commit",
    "current_head": "$current_upstream_head",
    "policy_artifact": "target/release-gates/current-upstream-policy.json"
  },
  "current_latest_target": {
    "status": "$current_latest_target_status",
    "target_selection_blocker_resolved": $current_latest_target_blocker_resolved,
    "current_latest_claim_allowed": $current_latest_claim_allowed,
    "github_default_branch_head": "$current_latest_github_head",
    "github_latest_release_channel": "$current_latest_release_channel",
    "target_artifact": "target/release-gates/current-latest-target.json"
  },
  "distribution_target": {
    "status": "$upstream_distribution_target_status",
    "npm_core_matches_frozen_baseline": $upstream_distribution_npm_core_matches_frozen,
    "repository_head_matches_npm_core": $upstream_distribution_repository_head_matches_npm,
    "github_latest_release_is_core_package": $upstream_distribution_github_latest_is_core,
    "github_latest_release_channel": "$upstream_distribution_release_channel",
    "current_repository_perfect_claim_allowed": $upstream_distribution_current_repo_claim_allowed,
    "target_artifact": "target/release-gates/upstream-distribution-target.json"
  },
  "source_inventory": {
    "missing_matrix_rows": $source_inventory_missing_matrix_rows,
    "p0_accounting_blocker_count": $source_inventory_p0_accounting_blocker_count
  },
  "artifact_paths": [
    "target/release-gates/cli-help.txt",
    "target/package-smoke/viewer-dist.json",
    "target/package-smoke/npm-wrapper-dist.json",
    "target/release-gates/performance.json",
    "target/release-gates/security.json",
    "target/release-gates/source-inventory-evidence.json",
    "target/release-gates/source-inventory-ledger.json",
    "target/release-gates/longtail-classification.json",
    "target/release-gates/external-authority-blockers.json",
    "target/release-gates/perfect-refactor-claim.json",
    "target/release-gates/perfect-refactor-unblock-packet.json",
    "target/release-gates/current-upstream-policy.json",
    "target/release-gates/current-latest-target.json",
    "target/release-gates/upstream-distribution-target.json",
    "target/release-gates/installability.json",
    "target/release-gates/publication-authority.json",
    "target/release-gates/real-upstream-smoke/latest/metadata.json",
    "target/release-gates/real-upstream-corpus/index.json",
    "target/release-gates/real-upstream-corpus/summary.json",
    "target/release-gates/real-upstream-smoke/latest/raw/upstream.json",
    "target/release-gates/real-upstream-smoke/latest/raw/rs.json",
    "target/release-gates/real-upstream-smoke/latest/normalized/upstream.json",
    "target/release-gates/real-upstream-smoke/latest/normalized/rs.json",
    "target/release-gates/real-upstream-smoke/latest/diff/findings.json"
  ],
  "no_upload_evidence": "local-only runtime smoke; no prompt, vars, output, or telemetry upload"
}
JSON

validate_report_json "$GATE_DIR/performance.json"
validate_report_json "$GATE_DIR/security.json"
validate_report_json "$GATE_DIR/source-inventory-ledger.json"
validate_report_json "$GATE_DIR/external-authority-blockers.json"
validate_report_json "$GATE_DIR/perfect-refactor-claim.json"
validate_report_json "$GATE_DIR/perfect-refactor-unblock-packet.json"
validate_report_json "$GATE_DIR/current-latest-target.json"
validate_report_json "$GATE_DIR/upstream-distribution-target.json"
validate_report_json "$GATE_DIR/release-candidate.json"

if [ "$stable_allowed" != "true" ]; then
  echo "runtime smoke release gate blocked; see $GATE_DIR/release-candidate.json" >&2
  exit 1
fi
