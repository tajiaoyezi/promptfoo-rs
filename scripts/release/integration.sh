#!/usr/bin/env bash
set -euo pipefail

cargo test \
  --test golden_diff_ci_release_gate \
  --test provider_assertion_inventory_parity \
  --test redteam_plugin_strategy_parity \
  --test real_p0_golden_corpus_runner \
  --test real_upstream_smoke_gate \
  --test viewer_node_packaging_release \
  --test performance_security_observability_gates \
  --test security_redaction
