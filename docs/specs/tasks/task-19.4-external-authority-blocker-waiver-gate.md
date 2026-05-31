# Task 19.4: external-authority-blocker-waiver-gate

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 19 — source-accounting-native-burndown
**Dependencies**: task-19.3-provider-request-response-fixture-burndown

## 1. Background

部分 remaining blockers 无法仅靠本地 mock fixture 证明：Codex provider、Agents SDK/tracing、Assistant、Billing、ChatKit、Realtime、Claude Code auth、真实多渠道发布都涉及账号、私有服务、产品协议、法律/品牌或发布授权。S2V 规则要求这些项保留为 explicit blockers/waivers，不能伪造 ready。依据 PRD §Security / §Release constraints、ADR-008、ADR-009、task 18.4 §10。

## 2. Goal

建立 external authority blocker/waiver gate：所有需要真实凭据、账号、私有服务、产品授权、法律/品牌确认或公开发布承诺的 remaining blockers 都有 item-level waiver/blocker 记录、最小所需用户决策和不可伪造的 release gate 输出。

## 3. Scope

### In Scope

- `src/compatibility/provider_assertion.rs`
- `scripts/release/longtail-classification.sh`
- `scripts/release/installability.sh`
- `scripts/release/runtime-smoke.sh`
- `target/release-gates/external-authority-blockers.json`
- `docs/compatibility/matrix.md`
- `docs/release.md`
- `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`
- `tests/external_authority_blocker_waiver_gate.rs`

### Out Of Scope

- 不获取或使用真实密钥、账号或私有服务权限。
- 不发布真实 GitHub/Cargo/npm/Docker/Homebrew artifact。
- 不做法律/品牌批准；只记录需要批准的最小问题。

## 4. Users / Actors

- Release maintainer：需要知道哪些 blocker 只能由凭据/授权解除。
- Enterprise reviewer：需要确认未把 external authority blocker 伪装为 native parity。
- Future publisher：需要最小决策清单。

## 5. Behavior Contract

External authority gate 必须输出每个 external blocker 的 item id、source reference、authority type、required decision、current status、safe local fallback、release impact。没有真实证据时 status 保持 `blocked` 或 `waived-with-boundary`，不得进入 `ready`。

### 5.1 Required Reading

- docs/specs/tasks/task-18.4-publication-authority-release-gate.md
- docs/specs/tasks/task-19.3-provider-request-response-fixture-burndown.md
- docs/decisions/adr-008-binary-first-multi-channel-release.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/release.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`promptfoo_rs::compatibility::provider_assertion`、`promptfoo_rs::release`、`serde_json`、`std::fs`。
- Tooling commands：adapter §Commands Install / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke。

### 5.3 函数签名

- `collect_external_authority_blockers() -> ExternalAuthorityBlockerReport`
- `validate_external_authority_gate(report: &ExternalAuthorityBlockerReport) -> ExternalAuthorityGateDecision`
- `write_external_authority_blockers(report: &ExternalAuthorityBlockerReport, path: &Path) -> Result<(), CompatibilityEvidenceError>`

## 6. Acceptance Criteria

- [ ] **AC1** (S2V blocked protocol): every real credential/account/private-service/legal-brand/publication blocker has item-level authority type and required decision.
- [ ] **AC2** (PRD §Security): local fallback/mock evidence is clearly separated from live product parity and never sets status `ready`.
- [ ] **AC3** (ADR-008): publication authority blockers from task 18.4 are included or linked, with channel-level published=false preserved.
- [ ] **AC4** (ADR-009): release candidate and audit outputs keep external-authority blockers visible and do not claim perfect refactor until resolved.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-19.4.1 | TEST-19.4.1 | tests/external_authority_blocker_waiver_gate.rs | install, typecheck, unit-test, integration, build | Not Started |
| AC2 | SCEN-19.4.1 | TEST-19.4.2 | tests/external_authority_blocker_waiver_gate.rs | install, typecheck, unit-test, coverage, build | Not Started |
| AC3 | SCEN-19.4.1 | TEST-19.4.3 | tests/external_authority_blocker_waiver_gate.rs | install, typecheck, unit-test, runtime-smoke, build | Not Started |
| AC4 | SCEN-19.4.1 | TEST-19.4.4 | tests/external_authority_blocker_waiver_gate.rs | install, typecheck, unit-test, e2e, build | Not Started |

## 8. Risks

- External blockers may require user/legal decisions; this task can only record and gate them.
- Waiver language must not imply native/live parity.
- Public release evidence cannot be created without credentials and explicit authorization.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **E2E tests**: adapter §Commands E2E tests
- **Coverage**: adapter §Commands Coverage
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：<TBD-after-impl>
- **改动文件**：<TBD-after-impl>
- **commit 列表**：<TBD-after-impl>
- **§9 Verification 结果**：<TBD-after-impl>
- **剩余风险 / 未做项**：<TBD-after-impl>
- **下游 task 影响**：<TBD-after-impl>
