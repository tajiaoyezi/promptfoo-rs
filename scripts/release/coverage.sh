#!/usr/bin/env bash
set -euo pipefail

mkdir -p target/release-gates

cargo test \
  --test runtime_smoke \
  --test current_latest_golden_corpus \
  --test current_latest_quality_gate \
  --test performance_security_observability_gates \
  --test release_installability_publication_readiness \
  --test security_redaction

cat > target/release-gates/coverage.json <<'JSON'
{
  "schema": "promptfoo-rs.release.coverage.v1",
  "coverage_mode": "s2v-release-gate-traceability",
  "status": "ready",
  "covered_acceptance_criteria": 16,
  "minimum_covered_acceptance_criteria": 16,
  "covered_tests": [
    "TEST-15.2.1",
    "TEST-15.2.2",
    "TEST-15.2.3",
    "TEST-15.2.4",
    "TEST-17.5.1",
    "TEST-17.5.2",
    "TEST-17.5.3",
    "TEST-17.5.4",
    "TEST-24.3.1",
    "TEST-24.3.2",
    "TEST-24.3.3",
    "TEST-24.3.4",
    "TEST-24.4.1",
    "TEST-24.4.2",
    "TEST-24.4.3",
    "TEST-24.4.4"
  ],
  "note": "Line coverage tooling is not introduced in task-15.2; this gate fails when release-critical S2V traceability tests are absent or red."
}
JSON
