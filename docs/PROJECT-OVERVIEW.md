# Project Overview

`promptfoo-rs` is organized as a Rust core plus optional compatibility surfaces. The repository is intentionally evidence-heavy: source code, compatibility fixtures, S2V specs, release gates, and audit documents all contribute to the final project status.

## Repository layout

| Path | Purpose |
|---|---|
| `src/` | Rust CLI, library modules, eval runner, providers, assertions, output formats, compatibility gates, redteam, scan, viewer server, script bridges. |
| `tests/` | Rust integration tests mapped to S2V TEST ids and compatibility requirements. |
| `compatibility/` | Fixture corpus, inventory files, matrix artifacts, and release-gate source evidence. |
| `viewer/` | Local web viewer package. It consumes local result data; it is not a hosted SaaS viewer. |
| `npm/` | Thin Node wrapper package boundary over the Rust core. |
| `.github/workflows/` | Release verification workflow shape. It verifies and uploads evidence artifacts; it does not by itself prove public publication. |
| `scripts/release/` | Lint, integration, e2e, coverage, runtime smoke, installability, and release-gate scripts. |
| `docs/` | Architecture, release rules, compatibility policy, PRD, ADRs, audits, S2V adapter, phase specs, and task specs. |
| `test/features/` | BDD feature files used for S2V traceability. |

## Runtime architecture

The default path is a single Rust binary:

1. CLI parses promptfoo-compatible commands and flags.
2. Config loader normalizes `promptfooconfig.yaml`, env files, prompt files, providers, tests, and assertions.
3. Eval runtime schedules cases, applies cache/resume/retry behavior, executes providers, runs assertions, and emits result records.
4. Output layer writes CI-facing formats such as JSON, JSONL, JUnit, SARIF, CSV, and HTML.
5. Viewer and npm wrapper consume stable data contracts instead of owning the core eval path.
6. Compatibility and release gates read artifacts and decide whether stable, current-latest, or perfect-refactor claims are allowed.

## Compatibility model

The project uses explicit P0/P1/P2 compatibility levels:

- P0 requires runnable fixture or golden-diff evidence.
- P1 requires snapshot or protocol-contract evidence.
- P2 must be visible as unsupported, later, or bridge-backed with a reason.

The compatibility matrix is not marketing copy. It is the source of truth for what is native, what is bridge-backed, and what remains blocked or out of scope.

## Release model

The release model separates local readiness from public publication:

- `local_stable_allowed=true` means the declared local gates pass.
- `published=false` means public registry/release-channel evidence is not complete.
- `perfect_refactor_claim_allowed=false` means the project must not claim complete replacement parity.

See [release.md](release.md) for the authority gates and allowed claim wording.

## Security model

Security defaults are intentionally conservative:

- No default upload.
- No cloud mutation through compatibility commands.
- No raw secret storage in release evidence.
- Script bridge execution requires explicit authorization.
- Provider/API key data must be redacted in reports and release artifacts.

## S2V workflow

S2V is the project development contract:

1. Specs define behavior and acceptance criteria.
2. BDD scenarios and TEST ids connect requirements to tests.
3. RED tests land before GREEN implementation.
4. Verification runs the task's declared keys.
5. Completion notes record commits, verification results, risks, and downstream impact.

Project entry points:

- [AGENTS.md](../AGENTS.md)
- [s2v-adapter.md](s2v-adapter.md)
- [PRD](prds/promptfoo-rs.prd.md)
- [phase specs](specs/phases/)
- [task specs](specs/tasks/)
- [ADRs](decisions/)
