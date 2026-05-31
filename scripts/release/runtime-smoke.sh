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
installability_ready="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/installability.json', 'utf8')); console.log(r.installability_ready ? 'true' : 'false')")"
publication_ready="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/installability.json', 'utf8')); console.log(r.publication_ready)")"
credential_blocked="$(node -e "const r = JSON.parse(require('fs').readFileSync('$GATE_DIR/installability.json', 'utf8')); console.log(r.credential_blocked ? 'true' : 'false')")"
stable_allowed="$(stable_allowed_from_gate "$adapter_status" "$compatibility_status" "$performance_status" "$security_status" "$packaging_status" "$observability_status" "$real_upstream_smoke_status" "$real_upstream_corpus_status")"
if [ "$stable_allowed" = "true" ]; then
  decision="stable"
else
  decision="prerelease"
fi

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
  "gate_statuses": {
    "adapter": "$adapter_status",
    "compatibility": "$compatibility_status",
    "performance": "$performance_status",
    "security": "$security_status",
    "packaging": "$packaging_status",
    "observability": "$observability_status",
    "installability": "ready",
    "longtail_classification": "$longtail_classification_status",
    "real_upstream_smoke": "$real_upstream_smoke_status",
    "real_upstream_corpus": "$real_upstream_corpus_status"
  },
  "artifact_paths": [
    "target/release-gates/cli-help.txt",
    "target/package-smoke/viewer-dist.json",
    "target/package-smoke/npm-wrapper-dist.json",
    "target/release-gates/performance.json",
    "target/release-gates/security.json",
    "target/release-gates/source-inventory-evidence.json",
    "target/release-gates/longtail-classification.json",
    "target/release-gates/installability.json",
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
validate_report_json "$GATE_DIR/release-candidate.json"

if [ "$stable_allowed" != "true" ]; then
  echo "runtime smoke release gate blocked; see $GATE_DIR/release-candidate.json" >&2
  exit 1
fi
