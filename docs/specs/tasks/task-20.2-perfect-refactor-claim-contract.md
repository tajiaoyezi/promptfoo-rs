# Task 20.2: perfect-refactor-claim-contract

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 20 — cross-ledger-perfect-claim-closure
**Dependencies**: task-20.1-source-provider-accounting-reconciliation

## 1. Background

Phase 19 的 `release-candidate.json` 可以报告 local `stable_allowed=true`，但同一 artifact 也显示 `external_authority.status=blocked`、`publication_authority.publication_ready=credential-blocked`、`published=false`。这说明 local release gate 与“完全满足 promptfoo 完美重构”不是同一判断。依据 PRD §Success Metrics、PRD §Risks、ADR-008、ADR-009、task-19.4 §10。

## 2. Goal

新增 machine-readable perfect-refactor claim contract：它聚合 source accounting、current upstream target、publication authority、external authority 和 release candidate evidence，明确给出 `perfect_refactor_claim_allowed=false` 以及阻止 claim 的最小 item-level blockers。

## 3. Scope

### In Scope

- `src/release.rs`
- `src/compatibility/provider_assertion.rs`
- `scripts/release/runtime-smoke.sh`
- `target/release-gates/perfect-refactor-claim.json`
- `docs/compatibility/matrix.md`
- `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`
- `docs/release.md`
- `tests/perfect_refactor_claim_contract.rs`

### Out Of Scope

- 不解除 current-upstream rebaseline blocker。
- 不发布真实 artifact。
- 不把 waived-with-boundary 或 credential-blocked 项解释为 perfect-ready。

## 4. Users / Actors

- Project owner：需要准确回答当前项目是否已经完全满足 promptfoo 重构。
- Release reviewer：需要机器可读 gate 区分 local stable 和 perfect-refactor claim。
- Future publisher：需要最小 blocker 清单决定何时可以公开声称 perfect refactor。

## 5. Behavior Contract

Perfect-refactor claim contract 必须聚合已有 release artifacts，不替代它们。只有 source accounting blockers 为 0、current-upstream gate ready、publication authority ready、external authority ready、release candidate stable 且 published evidence 完整时，`perfect_refactor_claim_allowed` 才能为 true。任一外部凭据、账号、法律/品牌、publication 或 current-upstream blocker 存在时必须为 false，并列出 blocker 来源和所需最小决策。

### 5.1 Required Reading

- docs/specs/tasks/task-18.3-current-upstream-rebaseline-gate.md
- docs/specs/tasks/task-18.4-publication-authority-release-gate.md
- docs/specs/tasks/task-19.4-external-authority-blocker-waiver-gate.md
- docs/specs/tasks/task-20.1-source-provider-accounting-reconciliation.md
- docs/decisions/adr-008-binary-first-multi-channel-release.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/release.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`promptfoo_rs::release`、`promptfoo_rs::compatibility::provider_assertion`、`serde_json`、`std::fs`、`std::path::Path`。
- Tooling commands：adapter §Commands Install / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke。

### 5.3 函数签名

- `build_perfect_refactor_claim_contract(inputs: PerfectRefactorClaimInputs) -> PerfectRefactorClaimContract`
- `validate_perfect_refactor_claim(contract: &PerfectRefactorClaimContract) -> PerfectRefactorClaimDecision`
- `write_perfect_refactor_claim_contract(contract: &PerfectRefactorClaimContract, path: &Path) -> Result<(), ReleaseError>`

## 6. Acceptance Criteria

- [ ] **AC1** (PRD §Success Metrics): `perfect-refactor-claim.json` reports `perfect_refactor_claim_allowed=false` while source/current/publication/external blockers remain.
- [ ] **AC2** (ADR-008): local `stable_allowed=true` never implies `published=true` or perfect-refactor claim readiness without publication authority evidence.
- [ ] **AC3** (ADR-009): claim blockers include source accounting, current-upstream, provider external-authority and publication authority records with source artifact paths.
- [ ] **AC4** (docs/release): user-facing docs and audit state the exact boundary: local stable gate can pass, but promptfoo perfect-refactor completion remains blocked until external/current/publication evidence exists.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-20.2.1 | TEST-20.2.1 | tests/perfect_refactor_claim_contract.rs | install, typecheck, unit-test, integration, build | Not Started |
| AC2 | SCEN-20.2.1 | TEST-20.2.2 | tests/perfect_refactor_claim_contract.rs | install, typecheck, unit-test, coverage, build | Not Started |
| AC3 | SCEN-20.2.1 | TEST-20.2.3 | tests/perfect_refactor_claim_contract.rs | install, typecheck, unit-test, runtime-smoke, build | Not Started |
| AC4 | SCEN-20.2.1 | TEST-20.2.4 | tests/perfect_refactor_claim_contract.rs | install, typecheck, unit-test, e2e, build | Not Started |

## 8. Risks

- Claim contract 若只复述 release-candidate 可能无法阻止误读；必须独立列 blocker sources。
- 如果把 external blockers 当成 local waiver 通过，会违反 task 19.4 的边界。
- 当前 upstream rebaseline 需要明确保留，不能被 frozen baseline 成功掩盖。

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
