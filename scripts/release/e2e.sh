#!/usr/bin/env bash
set -euo pipefail

cargo test \
  --test eval_command_smoke \
  --test cli_command_behavior_closure \
  --test cli_global_eval_redteam_parity \
  --test command_flag_parity \
  --test eval_output_cache_parity \
  --test output_ci_contracts \
  --test runtime_smoke
