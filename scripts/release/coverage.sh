#!/usr/bin/env bash
set -euo pipefail

mkdir -p target/release-gates

cargo test \
  --test runtime_smoke \
  --test performance_security_observability_gates \
  --test security_redaction

cat > target/release-gates/coverage.json <<'JSON'
{
  "schema": "promptfoo-rs.release.coverage.v1",
  "coverage_mode": "s2v-release-gate-traceability",
  "status": "ready",
  "covered_acceptance_criteria": 4,
  "minimum_covered_acceptance_criteria": 4,
  "covered_tests": [
    "TEST-15.2.1",
    "TEST-15.2.2",
    "TEST-15.2.3",
    "TEST-15.2.4"
  ],
  "note": "Line coverage tooling is not introduced in task-15.2; this gate fails when release-critical S2V traceability tests are absent or red."
}
JSON
