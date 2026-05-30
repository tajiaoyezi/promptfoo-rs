# promptfoo requirements traceability audit - 2026-05-30

**Status**: Audit attachment
**Parent audit**: `docs/audits/promptfoo-perfect-refactor-audit-2026-05-30.md`
**Purpose**: map PRD requirements to current evidence and decide whether each requirement is proven, contradicted, incomplete, or missing.

## Current-State Snapshot

| Item | Current evidence |
|---|---|
| Local branch before this attachment | `master` |
| Local HEAD before this attachment | `3cd4eb71d1c4abb0cd7fab671cc8943a7c26785c` |
| Local tracked files | 177 |
| Local compatibility fixtures excluding `.gitkeep` | 0 |
| Local Rust integration test files | 22 |
| Local audit files before this attachment | 5 |
| Current upstream `origin/main` HEAD | `945fda5d965ed27abb302fe0f0910b7dddea5dde` |
| Current upstream `package.json` version | `0.121.13` |
| Current upstream tracked files | 5302 |
| Current upstream `src/` files | 1601 |
| Current upstream provider TS/JS files | 219 |
| Current upstream assertion TS/JS files | 56 |
| Current upstream redteam TS/JS files | 218 |
| Current upstream command-related TS/JS files | 85 |

The upstream HEAD differs from earlier audit snapshots (`c24aa89804d3...`) while the package version remains `0.121.13`. This reinforces that the local frozen baseline and the moving `promptfoo/promptfoo` repository state are distinct claims.

## PRD Core Capabilities

| PRD requirement | Current evidence | Verdict |
|---|---|---|
| promptfoo-compatible CLI/runtime covering documented domains and at least `eval`, `view`, `cache`, `redteam`, `mcp`, `code-scans`, `scan-model`, `import/export` with common flags | Local CLI has 10 command variants; `view/cache/import/export` are empty success placeholders; `EvalArgs` only accepts `--config`; upstream command surface is much larger | Not met |
| Compatibility harness runs upstream promptfoo and `promptfoo-rs` on the same fixtures and performs golden diff; P0 failure blocks stable | `HarnessRunner` constructs in-memory artifacts; it does not execute upstream or local CLI; `compatibility/fixtures` has 0 real fixtures; `compatibility/artifacts` is absent | Not met |
| Rust-native eval core handles config parsing, scheduling, provider calls, assertions, cache/resume/retry/rate limit, and streaming results | Local modules and tests exist for these contracts, but CLI/runtime coverage is narrow and no upstream golden diff proves promptfoo behavior parity | Partially met |
| P0 providers and script bridge support OpenAI-compatible, HTTP, Ollama, Anthropic, JS/TS, Python, Shell custom provider/assertion boundaries | Local P0 provider files and script bridge tests exist; long-tail provider/assertion inventory is incomplete and not all script/runtime variants are package-tested | Partially met |
| Local viewer and multi-channel distribution: viewer reads JSONL/SQLite; outputs JSON/JSONL/CSV/YAML/HTML/JUnit/SARIF; releases through GitHub Releases, Homebrew, Cargo, Docker, npm wrapper, GitHub Action | Rust-side viewer contract and output modules exist; viewer/npm package metadata is absent; no published artifacts, Homebrew formula, container digest, or npm/crates publication evidence | Not met |

## PRD Out-of-Scope Guardrails

| Guardrail | Current evidence | Verdict |
|---|---|---|
| Do not silently omit documented provider/assertion/redteam/plugin/CLI capabilities; all must be registered as native/bridge/unsupported/later | Matrix still has aggregate rows such as `Other documented providers` and `Redteam plugins/strategies`; no item-level full inventory | Not met |
| Custom scripts must be explicit opt-in | Script bridge and tests cover default-deny and redaction contracts | Partially met |
| Do not redesign promptfoo config, assertion DSL, output format, or CLI semantics | Local CLI/output surface is too incomplete to prove semantic preservation | Not proven |
| Web UI pixel parity is not a 1.0 gate; stable result schema parity is the boundary | Viewer contract explicitly says data parity, not pixel replication | Met for boundary definition |

## PRD Compatibility Matrix Requirements

| Matrix requirement | Current evidence | Verdict |
|---|---|---|
| CLI command/flag inventory is P0 and fully registered | Local matrix has a domain row; no item-level command/flag matrix; upstream currently exposes a much larger command surface | Not met |
| P0 config, eval runner, cache/resume/retry, outputs, and provider fixtures pass golden diff | No real compatibility fixtures; no upstream execution artifacts | Not met |
| Other documented providers are fully registered with P1/P2 evidence | Aggregate `Other documented providers` row remains; no provider submatrix | Not met |
| Redteam plugins/strategies are fully registered with P0/P1/P2 evidence | Aggregate redteam row remains; local registry has 3 plugins and 3 strategies versus upstream's much larger surface | Not met |
| P2 known gaps are registered and not silently omitted | Some P2 concepts are present, but full item inventory is absent, so silent omissions cannot be ruled out | Not proven |

## PRD Success Metrics

| Success metric | Required evidence | Current evidence | Verdict |
|---|---|---|---|
| P0 compatibility release gate | At least 50 core fixtures where upstream promptfoo and `promptfoo-rs` match or have explained differences; zero unclassified P0 diffs | 0 compatibility fixtures; no persisted upstream/rs/normalized/diff artifacts | Not met |
| Common eval migration | `promptfoo-rs eval -c promptfooconfig.yaml` supports common prompts, vars, tests, providers, assertions, cache, resume, retry, and output behavior | Local `eval` accepts config and prints JSON envelope; no CLI `--output` flags or upstream fixture corpus prove common migration | Not met |
| Compatibility matrix completeness | 100% documented provider/assertion/redteam/plugin/CLI/output/config abilities registered with P0/P1/P2, status, verification, owner | Coarse matrix rows and known aggregate deferrals remain | Not met |
| Provider P0 | Four P0 providers runnable with request/response snapshots | Local provider implementations and tests exist; no golden fixture corpus proves upstream parity | Partially met |
| Output P0/P1 | JSON, JSONL, JUnit XML, CSV usable for CI; SARIF schema snapshot | Output modules and tests exist; CLI/output flag end-to-end compatibility is not proven | Partially met |
| Performance baseline | Cold start <300 ms; 1000 mock evals <5s; memory <100MB | No current measurement artifact in audits or specs | Missing |
| Security defaults | Custom scripts rejected unless enabled; redaction fixtures pass | Local script bridge tests cover default-deny and redaction contracts; no full upstream custom script corpus | Partially met |
| Documentation availability | README, architecture, compatibility matrix, contributing guide, GitHub Action example, release gate docs | Documentation exists; several docs describe future or example-only publishing, not completed release evidence | Partially met |

## Verification Gate Requirements

| Gate | Current adapter state | Impact |
|---|---|---|
| lint | `N/A` | Cannot support broad quality claim |
| integration tests | `N/A` globally | Cannot support full compatibility claim |
| E2E tests | `N/A` globally | Cannot prove CLI/viewer workflows |
| coverage | `N/A` | No coverage threshold evidence |
| runtime smoke | `N/A` globally | No unified runtime proof |
| unit-test | active and green in runtime audit | Proves local test suite only |

## Release And Distribution Requirements

| Requirement | Current evidence | Verdict |
|---|---|---|
| GitHub Releases binary with checksum | Docs describe channel; no artifact evidence | Missing |
| Homebrew tap/formula | Docs describe future formula; no formula artifact | Missing |
| Cargo package | Cargo manifest exists; no publication evidence | Not proven |
| Docker image | Dockerfile exists; no image digest or pushed package evidence | Not proven |
| npm wrapper package | `npm/src` exists; `npm/package.json` absent | Not met |
| GitHub Action | Example workflow exists; it does not publish artifacts or run full compatibility gate | Partially met |

## Final Traceability Verdict

The current project has a coherent local S2V implementation slice and substantial documentation. It does not satisfy the stronger objective of a complete or perfect refactor of `promptfoo/promptfoo`.

The decisive missing proof is not a single test failure; it is the absence of requirement-matched evidence:

- no 50-fixture compatibility corpus
- no real upstream-vs-rs golden diff execution
- no item-level 100% compatibility matrix
- no full CLI/flag behavior parity
- no complete provider/assertion/redteam inventory coverage
- no browser/package/release publication evidence
- no global integration/E2E/coverage/runtime smoke gates

Therefore the active goal remains incomplete.
