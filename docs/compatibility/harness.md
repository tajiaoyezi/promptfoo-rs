# Executable Compatibility Harness

Task 12.2 promotes the compatibility harness from in-memory artifact construction to executable evidence.

## Artifact Tree

Each run is persisted under `compatibility/artifacts/<run-id>/`:

- `metadata.json`: run id, fixture id, pinned baseline, command specs, artifact schema version.
- `raw/upstream.json`: upstream promptfoo command, isolated env, stdout, stderr, exit code, timeout status.
- `raw/rs.json`: promptfoo-rs command, isolated env, stdout, stderr, exit code, timeout status.
- `normalized/upstream.json`: normalized upstream raw artifact.
- `normalized/rs.json`: normalized promptfoo-rs raw artifact.
- `diff/findings.json`: classified diff findings from normalized artifacts.

## Execution Policy

- Commands are spawned without shell string concatenation.
- `env_clear=true` is enforced before adding deterministic env.
- `PROMPTFOO_DISABLE_UPDATE=true`, `PROMPTFOO_DISABLE_TELEMETRY=true`, `NO_COLOR=1`, and `CI=1` are forced.
- Timeouts are required and capped at 120 seconds.
- Fixtures requiring env names containing `KEY`, `TOKEN`, or `SECRET` are rejected before execution.

Basis: PRD §Compatibility Harness Design and ADR-007.
