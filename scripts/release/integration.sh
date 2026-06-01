#!/usr/bin/env bash
set -euo pipefail

cargo test \
  --test golden_diff_ci_release_gate \
  --test provider_assertion_inventory_parity \
  --test redteam_plugin_strategy_parity \
  --test longtail_provider_assertion_redteam_classification \
  --test release_installability_publication_readiness \
  --test publication_authority_release_gate \
  --test current_upstream_rebaseline_gate \
  --test viewer_config_source_reclassification \
  --test core_config_source_fixture_burndown \
  --test provider_request_response_fixture_burndown \
  --test external_authority_blocker_waiver_gate \
  --test real_p0_golden_corpus_runner \
  --test current_latest_golden_corpus \
  --test current_latest_quality_gate \
  --test real_upstream_smoke_gate \
  --test viewer_node_packaging_release \
  --test performance_security_observability_gates \
  --test security_redaction
