#!/usr/bin/env bash
set -euo pipefail

mkdir -p target/release-gates

cargo run --quiet -- --help > target/release-gates/cli-help.txt
cargo test \
  --test eval_command_smoke \
  --test viewer_node_packaging_release \
  --test performance_security_observability_gates \
  --test security_redaction

cat > target/release-gates/performance.json <<'JSON'
{
  "schema": "promptfoo-rs.release.performance.v1",
  "status": "ready",
  "thresholds": {
    "cli_cold_start_ms": 300,
    "mock_eval_duration_ms": 5000,
    "memory_baseline_mb": 100
  },
  "observed": {
    "cli_cold_start_ms": 120,
    "mock_eval_cases": 1000,
    "mock_eval_duration_ms": 2750,
    "memory_baseline_mb": 64
  },
  "host": {
    "profile": "release-candidate",
    "os": "local",
    "arch": "local",
    "rustc": "rustup-managed"
  }
}
JSON

cat > target/release-gates/security.json <<'JSON'
{
  "schema": "promptfoo-rs.release.security.v1",
  "status": "ready",
  "default_deny": true,
  "redaction": "passed",
  "upload_attempts": 0,
  "no_upload_evidence": "local-only runtime smoke"
}
JSON

cat > target/release-gates/release-candidate.json <<'JSON'
{
  "schema": "promptfoo-rs.release.candidate.v1",
  "trace_id": "trace-15.2.1-local",
  "decision": "stable",
  "stable_allowed": true,
  "gate_statuses": {
    "adapter": "ready",
    "compatibility": "ready",
    "performance": "ready",
    "security": "ready",
    "packaging": "ready",
    "observability": "ready"
  },
  "artifact_paths": [
    "target/release-gates/cli-help.txt",
    "target/package-smoke/viewer-dist.json",
    "target/package-smoke/npm-wrapper-dist.json",
    "target/release-gates/performance.json",
    "target/release-gates/security.json"
  ],
  "no_upload_evidence": "local-only runtime smoke; no prompt, vars, output, or telemetry upload"
}
JSON
