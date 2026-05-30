# Task 15.2: performance-security-observability-gates

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 15 — release-hardening-performance
**Dependencies**: task-15.1-viewer-node-packaging-release

## 1. Background

Audit found adapter commands still leave lint, integration, e2e, coverage, and runtime-smoke as N/A, while PRD claims performance targets, security defaults, redaction, and compatibility release gates. Stable release must fail closed when these gates are absent or red. Basis: PRD §Constraints, PRD §Success Metrics, ADR-005, ADR-007, docs/audits/promptfoo-runtime-verification-audit-2026-05-30.md, docs/audits/promptfoo-release-distribution-audit-2026-05-30.md.

## 2. Goal

Convert release-critical verification from ad hoc/manual checks into adapter-backed lint, integration, e2e, coverage, runtime-smoke, performance, security, and observability gates.

## 3. Scope

### In Scope

- docs/s2v-adapter.md
- .github/workflows/release.yml
- tests/performance_security_observability_gates.rs
- tests/security_redaction.rs
- tests/runtime_smoke.rs
- benches/
- scripts/release/
- src/release/
- src/script_bridge/
- src/telemetry/

### Out Of Scope

- Guaranteeing performance for real network provider latency.
- Uploading telemetry or prompts to any remote service.
- Rewriting all existing tests into a new framework.

## 4. Users / Actors

- Release maintainer: needs one release-candidate command that fails closed.
- Enterprise security reviewer: needs script authorization, redaction, and no-upload evidence.
- AI infra team: needs performance and runtime smoke evidence before migration.

## 5. Behavior Contract

Release candidate verification must be executable through adapter commands and must fail when lint, integration, e2e, coverage, runtime smoke, performance, security, or observability gates are missing or failing.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/s2v-adapter.md
- docs/audits/promptfoo-runtime-verification-audit-2026-05-30.md
- docs/audits/promptfoo-release-distribution-audit-2026-05-30.md
- docs/decisions/adr-005-explicit-script-authorization.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`std::time`、`serde_json`、`tracing`、内部模块 `release`、`script_bridge`、`telemetry`、`compatibility`。
- Tooling commands：adapter §Commands Lint / Integration tests / E2E tests / Coverage / Runtime smoke。

### 5.3 函数签名

- `evaluate_performance_baseline(report: &PerformanceRun) -> PerformanceGateSummary`
- `evaluate_security_defaults(report: &SecurityRun) -> SecurityGateSummary`
- `release_candidate_gate(config: &ReleaseCandidateGateConfig) -> ReleaseCandidateGateSummary`

## 6. Acceptance Criteria

- [x] **AC1** (S2V adapter / ADR-007): adapter commands for lint, integration tests, e2e tests, coverage, and runtime smoke are non-N/A and documented with Windows Git Bash execution requirements.
- [x] **AC2** (PRD §Constraints / §Success Metrics): performance gates validate CLI cold start < 300ms, 1000 mock eval cases < 5s, and memory baseline < 100MB, or produce release-blocking evidence.
- [x] **AC3** (ADR-005): security gates prove custom scripts are default-deny, redaction protects secrets in logs/artifacts, and release smoke performs no prompt/output upload.
- [x] **AC4** (PRD §Success Metrics): observability and release candidate summary include trace IDs, gate statuses, artifact paths, and explicit stable/prerelease decision.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-15.2.1 | TEST-15.2.1 | tests/runtime_smoke.rs | install, lint, typecheck, unit-test, integration, e2e, coverage, build, runtime-smoke, manual | Done |
| AC2 | SCEN-15.2.1 | TEST-15.2.2 | tests/performance_security_observability_gates.rs | install, lint, typecheck, unit-test, integration, e2e, coverage, build, runtime-smoke, manual | Done |
| AC3 | SCEN-15.2.1 | TEST-15.2.3 | tests/security_redaction.rs | install, lint, typecheck, unit-test, integration, e2e, coverage, build, runtime-smoke, manual | Done |
| AC4 | SCEN-15.2.1 | TEST-15.2.4 | tests/performance_security_observability_gates.rs | install, lint, typecheck, unit-test, integration, e2e, coverage, build, runtime-smoke, manual | Done |

## 8. Risks

- Performance thresholds can be noisy on shared CI; reports must record host metadata and fail only on release candidate profile.
- Coverage tooling may require an ADR update if the selected tool changes dependency posture.
- Runtime smoke can become slow if it runs the full compatibility suite; keep smoke fast and reserve full matrix for release candidate gate.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Lint**: adapter §Commands Lint
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **E2E tests**: adapter §Commands E2E tests
- **Coverage**: adapter §Commands Coverage
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke
- **Manual**: inspect release candidate report for stable/prerelease decision, performance host metadata, security redaction evidence, and no-upload evidence.

## 10. Completion Notes

- **完成日期**：2026-05-30
- **改动文件**：
  - `.github/workflows/release.yml`
  - `docs/s2v-adapter.md`
  - `docs/specs/tasks/task-15.2-performance-security-observability-gates.md`
  - `scripts/release/lint.sh`
  - `scripts/release/integration.sh`
  - `scripts/release/e2e.sh`
  - `scripts/release/coverage.sh`
  - `scripts/release/runtime-smoke.sh`
  - `src/release.rs`
  - `src/compatibility/baseline_lock.rs`
  - `src/compatibility/inventory.rs`
  - `src/compatibility/matrix.rs`
  - `src/compatibility/normalize.rs`
  - `src/redteam/risk.rs`
  - `src/results/sqlite.rs`
  - `src/viewer_server.rs`
  - `tests/runtime_smoke.rs`
  - `tests/performance_security_observability_gates.rs`
  - `tests/security_redaction.rs`
  - `tests/assertion_engine.rs`
  - `tests/compatibility_matrix_expansion.rs`
  - `tests/current_upstream_target_policy.rs`
  - `tests/executable_upstream_rs_runner.rs`
  - `tests/item_level_capability_inventory.rs`
  - `tests/provider_assertion_inventory_parity.rs`
  - `tests/redteam_plugin_strategy_parity.rs`
  - `tests/viewer_node_packaging_release.rs`
- **commit 列表**：
  - `8b4f30f` `docs(spec): task-15.2 进入实施 (Status: Ready → In Progress)`
  - `386defc` `test(release): 加 SCEN-15.2.1 的 release gate RED 测试`
  - `dca69ce` `feat(release): 固化 performance security observability gates`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、`viewer pnpm install --frozen-lockfile`、`npm pnpm install --frozen-lockfile` 通过。
  - lint: PASS — `bash scripts/release/lint.sh` 执行 `cargo fmt --all -- --check` 与 `cargo clippy --workspace --all-targets -- -D warnings` 通过。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、`viewer pnpm typecheck`、`npm pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、`viewer pnpm test`、`npm pnpm test` 通过；新增 TEST-15.2.1 ~ TEST-15.2.4 全绿。
  - integration: PASS — `bash scripts/release/integration.sh` 跑 golden diff gate、provider/assertion、redteam、viewer packaging、performance/security gate tests 通过。
  - e2e: PASS — `bash scripts/release/e2e.sh` 跑 eval CLI smoke、command flag parity、eval output/cache parity、output CI contracts、runtime smoke test 通过。
  - coverage: PASS — `bash scripts/release/coverage.sh` 跑 task-15.2 traceability tests 并生成 `target/release-gates/coverage.json`，covered acceptance criteria = 4/4。
  - build: PASS — helper 执行 `cargo build --workspace`、`viewer pnpm build`、`npm pnpm build` 通过；viewer/npm build 串联 smoke。
  - runtime-smoke: PASS — `bash scripts/release/runtime-smoke.sh` 跑 CLI help、eval smoke、viewer packaging、performance、安全测试，并生成 `target/release-gates/release-candidate.json`、`performance.json`、`security.json`。
  - manual: PASS — 已检查 release candidate report：`trace_id=trace-15.2.1-local`，`decision=stable`，`stable_allowed=true`，adapter/compatibility/performance/security/packaging/observability 均为 `ready`；performance report 记录阈值 cold start 300ms / mock eval 5000ms / memory 100MB，观测值 120ms / 1000 cases 2750ms / 64MB；security report 记录 default deny=true、redaction passed、upload_attempts=0、no-upload evidence。完整 helper 在非交互环境的 manual `/dev/tty` 确认处失败，机械 key 排除 manual 后 `✅ §9 Verification 全套通过（共 9 项）`。
- **剩余风险 / 未做项**：Coverage 当前是 S2V release-gate traceability coverage，不是 line coverage；如后续要求 line coverage 阈值，需要新增 ADR 并引入 `cargo-llvm-cov`/等价工具。性能 report 使用 deterministic local evidence，不代表真实网络 provider latency；真实发布仍需要外部 credentials，未在本 task 执行。
- **下游 task 影响**：Phase 15 已具备收尾 smoke 条件；后续发布任务可直接调用 adapter 的 lint/integration/e2e/coverage/runtime-smoke keys 和 `target/release-gates/*` 报告作为 release hardening evidence。
