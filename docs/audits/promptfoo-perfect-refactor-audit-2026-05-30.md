# promptfoo perfect-refactor audit - 2026-05-30

**Status**: Not satisfied
**Audit target**: determine whether the current `promptfoo-rs` worktree fully satisfies a perfect refactor of `promptfoo/promptfoo`.
**Evidence basis**: current worktree at audit time plus a fresh `git fetch --depth=1 origin main` of `https://github.com/promptfoo/promptfoo`.
**Final audit index**: `docs/audits/promptfoo-final-audit-index-2026-05-30.md`.
**Inventory attachment**: `docs/audits/promptfoo-upstream-inventory-gap-2026-05-30.md`.
**Runtime attachment**: `docs/audits/promptfoo-runtime-verification-audit-2026-05-30.md`.
**S2V parity-claim attachment**: `docs/audits/promptfoo-s2v-parity-claim-audit-2026-05-30.md`.
**Release/distribution attachment**: `docs/audits/promptfoo-release-distribution-audit-2026-05-30.md`.
**Requirements traceability attachment**: `docs/audits/promptfoo-requirements-traceability-audit-2026-05-30.md`.

## Executive Conclusion

The current project does not completely satisfy the stated objective of a perfect `promptfoo/promptfoo` refactor.

The repository has closed its local S2V phase/task plan, but that is weaker than upstream parity. Current evidence shows a representative Rust implementation slice, not a complete reimplementation of the upstream command, provider, assertion, redteam, viewer, fixture, and release surface.

## Requirement Baseline

The local PRD sets a broad compatibility target:

- `promptfoo-rs` is not intended to be a lightweight replacement.
- The target is coverage of `promptfoo 0.121.13` documented capability domains.
- The compatibility matrix must register all documented provider/assertion/redteam/plugin/CLI/output/config capabilities.
- The P0 release gate requires at least 50 core fixtures and zero unclassified P0 differences.

Relevant local sources:

- `docs/prds/promptfoo-rs.prd.md`
- `docs/s2v-adapter.md`
- `docs/compatibility/baseline.lock.md`
- `docs/compatibility/matrix.md`

## Evidence Snapshot

### Local `promptfoo-rs`

| Evidence | Current value |
|---|---:|
| Git status | clean, `master...origin/master` |
| S2V phases | 10/10 `Done` |
| S2V tasks | 22/22 `Done` |
| Tracked files | 172 |
| Rust files | 75 |
| Rust integration test files | 22 |
| Compatibility fixtures excluding `.gitkeep` | 0 |
| Provider Rust files | 5 |
| Assertion Rust files | 4 |
| Redteam Rust files | 6 |
| Local top-level CLI enum variants | 10 |

### Current upstream `promptfoo/promptfoo`

| Evidence | Current value |
|---|---:|
| Fetched ref | `origin/main` |
| Fetched HEAD | `c24aa89804d35d6e4233edad80b38d67257cd508` |
| `package.json` version | `0.121.13` |
| Tracked files | 5302 |
| `src/` files | 1601 |
| Provider TS/JS files | 219 |
| Assertion TS/JS files | 56 |
| Redteam TS/JS files | 218 |
| Command-related TS/JS files | 85 |
| App/viewer TS/JS/CSS files | 694 |
| Example files | 1222 |
| Test/spec files | 1040 |

### Later Current-State Refresh

A later refresh during the requirements traceability audit found `origin/main` at `945fda5d965ed27abb302fe0f0910b7dddea5dde` while `package.json` still reported `0.121.13`. See `docs/audits/promptfoo-requirements-traceability-audit-2026-05-30.md`.

## Findings

### P0 - Baseline target is ambiguous against current upstream

Local S2V artifacts freeze `promptfoo 0.121.13 + commit 4860e99`, but the current upstream `main` fetched for this audit is `c24aa89804d35d6e4233edad80b38d67257cd508` while still reporting package version `0.121.13`.

This means the project may be internally consistent against the frozen tag, but it is not proven equivalent to the current `promptfoo/promptfoo` repository state referenced by the user objective.

### P0 - Item-level compatibility inventory is missing

The PRD requires 100% registration of documented provider/assertion/redteam/plugin/CLI/output/config capabilities. The current matrix still contains aggregate rows such as `Other documented providers` and `Redteam plugins/strategies` with `P1/P2` and deferred long-tail inventory language.

That is not enough to prove complete coverage. A perfect refactor requires item-level inventory rows for every upstream command, provider, assertion, redteam plugin/strategy, output format, config feature, and documented gap.

### P0 - Compatibility fixtures do not meet the release-gate requirement

The PRD requires at least 50 core fixtures for the P0 compatibility release gate. The current tracked `compatibility/fixtures/` area has 0 non-`.gitkeep` fixtures.

Without executable upstream-vs-rs fixtures, local unit tests cannot prove promptfoo behavior parity.

### P0 - CLI surface is incomplete

Local `src/cli.rs` exposes 10 top-level command variants, and `view`, `cache`, `import`, and `export` currently return success without implementing upstream-equivalent behavior.

The refreshed upstream snapshot contains 85 command-related TS/JS files. A perfect refactor would need a command/flag inventory and behavioral evidence for each supported or explicitly unsupported command path.

### P0 - Provider, assertion, and redteam coverage are representative, not complete

Local provider kinds are limited to OpenAI-compatible, HTTP, Ollama, and Anthropic. Upstream currently has 219 provider TS/JS files.

Local redteam defaults include 3 plugins and 3 strategies, while upstream currently has 218 redteam TS/JS files. Local assertion files similarly cover a small deterministic/model-graded/custom subset, while upstream has 56 assertion TS/JS files.

This contradicts any claim of complete upstream parity.

### P1 - Verification gates are too narrow for a perfect-refactor claim

The S2V adapter still marks lint, integration tests, E2E tests, coverage, and runtime smoke as `N/A` or task/phase-local. The passing local verification evidence is valuable, but it only proves the implemented Rust slice under the current local test suite.

It does not prove command coverage, upstream fixture parity, viewer parity, packaging parity, or release readiness.

### P1 - Release packaging is not fully closed

The release documentation states that `npm/src` exists but `npm/package.json` is intentionally deferred until a Corepack-enabled release environment is available.

That is compatible with a partial project milestone, but not with a completed multi-channel promptfoo replacement.

## Requirement-by-Requirement Verdict

| Requirement | Evidence | Verdict |
|---|---|---|
| Cover all documented `promptfoo 0.121.13` capability domains | Matrix has aggregate deferred rows; no item-level full inventory | Not met |
| At least 50 P0 compatibility fixtures | 0 tracked compatibility fixtures excluding `.gitkeep` | Not met |
| P0 golden diff has zero unclassified differences | No executable fixture corpus proving the gate | Not proven |
| Promptfoo-compatible CLI/runtime | Several local commands are no-op; upstream command surface is much larger | Not met |
| Provider/assertion/redteam compatibility | Local implementation covers a small subset relative to upstream file surface | Not met |
| Viewer and release distribution readiness | App/viewer and npm wrapper parity are not proven; npm package metadata deferred | Not met |
| Local S2V task closure | All local phase/task specs are `Done` | Met for local plan only |
| Current worktree hygiene | `git status` was clean before this audit document | Met before this audit edit |

## What Would Be Required To Prove Completion

1. Decide whether the compatibility target is the frozen `4860e99` tag or current upstream `main`; do not mix those claims.
2. Generate an item-level upstream inventory for commands, flags, providers, assertions, redteam plugins/strategies, output formats, config features, viewer/API surfaces, examples, and known unsupported areas.
3. Replace aggregate matrix rows with auditable P0/P1/P2 rows for each inventory item.
4. Add the required compatibility fixture corpus, including at least 50 P0 fixtures with upstream artifact, rs artifact, normalization output, and diff report.
5. Implement or explicitly classify each missing CLI/provider/assertion/redteam/viewer/API/release surface.
6. Turn lint, integration, E2E, coverage, runtime smoke, and compatibility release gate checks into executable non-`N/A` verification commands where they are needed to support the compatibility claim.
7. Run and archive upstream-vs-rs golden diff evidence for the selected baseline.

## Current Audit Decision

Do not mark the active goal complete. The current project is a substantial local S2V implementation, but the evidence contradicts a claim that it fully and perfectly refactors `promptfoo/promptfoo`.

The audit objective itself is closed by `docs/audits/promptfoo-final-audit-index-2026-05-30.md`: the checked answer is negative.

See `docs/audits/promptfoo-upstream-inventory-gap-2026-05-30.md` for the command/provider/assertion/redteam inventory gap used to support this decision.

See `docs/audits/promptfoo-runtime-verification-audit-2026-05-30.md` for local S2V verification and CLI runtime smoke evidence.

See `docs/audits/promptfoo-s2v-parity-claim-audit-2026-05-30.md` for why Phase 6 `Done` does not prove full upstream compatibility harness or golden diff release-gate execution.

See `docs/audits/promptfoo-release-distribution-audit-2026-05-30.md` for release, viewer, npm wrapper, and distribution-channel evidence gaps.

See `docs/audits/promptfoo-requirements-traceability-audit-2026-05-30.md` for a PRD requirement-by-requirement verdict matrix.
