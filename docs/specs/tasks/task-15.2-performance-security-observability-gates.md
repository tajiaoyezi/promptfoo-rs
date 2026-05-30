# Task 15.2: performance-security-observability-gates

**Status**: Ready
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

- [ ] **AC1** (S2V adapter / ADR-007): adapter commands for lint, integration tests, e2e tests, coverage, and runtime smoke are non-N/A and documented with Windows Git Bash execution requirements.
- [ ] **AC2** (PRD §Constraints / §Success Metrics): performance gates validate CLI cold start < 300ms, 1000 mock eval cases < 5s, and memory baseline < 100MB, or produce release-blocking evidence.
- [ ] **AC3** (ADR-005): security gates prove custom scripts are default-deny, redaction protects secrets in logs/artifacts, and release smoke performs no prompt/output upload.
- [ ] **AC4** (PRD §Success Metrics): observability and release candidate summary include trace IDs, gate statuses, artifact paths, and explicit stable/prerelease decision.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-15.2.1 | TEST-15.2.1 | tests/runtime_smoke.rs | install, lint, typecheck, unit-test, integration, e2e, coverage, build, runtime-smoke, manual | Not Started |
| AC2 | SCEN-15.2.1 | TEST-15.2.2 | tests/performance_security_observability_gates.rs | install, lint, typecheck, unit-test, integration, e2e, coverage, build, runtime-smoke, manual | Not Started |
| AC3 | SCEN-15.2.1 | TEST-15.2.3 | tests/security_redaction.rs | install, lint, typecheck, unit-test, integration, e2e, coverage, build, runtime-smoke, manual | Not Started |
| AC4 | SCEN-15.2.1 | TEST-15.2.4 | tests/performance_security_observability_gates.rs | install, lint, typecheck, unit-test, integration, e2e, coverage, build, runtime-smoke, manual | Not Started |

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

- **完成日期**：<TBD-after-impl>
- **改动文件**：<TBD-after-impl>
- **commit 列表**：<TBD-after-impl>
- **§9 Verification 结果**：<TBD-after-impl>
- **剩余风险 / 未做项**：<TBD-after-impl>
- **下游 task 影响**：<TBD-after-impl>
