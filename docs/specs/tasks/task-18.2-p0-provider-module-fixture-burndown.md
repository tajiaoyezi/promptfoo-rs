# Task 18.2: p0-provider-module-fixture-burndown

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 18 — perfect-refactor-blocker-burndown
**Dependencies**: task-18.1-source-inventory-ledger-closure, task-17.4-longtail-provider-assertion-redteam-classification

## 1. Background

`target/release-gates/longtail-classification.json` 当前报告 37 个 P0 provider module release blockers。它们已经可见，但还没有 fixture/native/bridge evidence 或明确的不可实施阻断决策。依据 PRD §Provider P0、ADR-001、ADR-005、ADR-009、task 17.4 §10。

## 2. Goal

逐项燃尽 37 个 P0 provider module blockers：能本地 mock 的补 fixture/native request-response snapshot；需要外部账号/私有服务/法律品牌确认的保留为 explicit blocker；不得把 P0 降成 P2/later 来制造绿色。

## 3. Scope

### In Scope

- `compatibility/inventory/upstream-items.json`
- `compatibility/fixtures/providers/`
- `src/providers/`
- `src/compatibility/provider_assertion.rs`
- `scripts/release/longtail-classification.sh`
- `tests/p0_provider_module_fixture_burndown.rs`
- `docs/compatibility/matrix.md`

### Out Of Scope

- 不调用真实 paid provider API。
- 不处理 provider 以外的 assertion/redteam P1/P2 rows。
- 不改变 task 18.1 ledger accounting rules。

## 4. Users / Actors

- AI application developer：需要看到 P0 provider modules 是否可迁移。
- Release maintainer：需要 `p0_release_blocker_count` 下降或每项有不可实施阻断证据。
- Security reviewer：需要确认 provider fixtures 不包含真实密钥。

## 5. Behavior Contract

每个 P0 provider module blocker 必须有独立 item id、fixture/snapshot/blocker、reason 和 user-visible error。mockable provider modules 必须有 recorded request/response fixture；不可 mock 或需要授权的项必须保留 `blocker:<item-id>`，包含最小外部决策问题。

### 5.1 Required Reading

- docs/specs/tasks/task-18.1-source-inventory-ledger-closure.md
- docs/specs/tasks/task-17.4-longtail-provider-assertion-redteam-classification.md
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-005-explicit-script-authorization.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`serde_json`、内部模块 `providers`、`compatibility::fixtures`、`compatibility::provider_assertion`。
- Tooling commands：adapter §Commands Install / Typecheck / Unit Test / Integration tests / Coverage / Runtime smoke / Build。

### 5.3 函数签名

- `provider_module_blocker_rows(matrix: &CapabilityMatrix) -> Vec<CapabilityRow>`
- `resolve_provider_module_fixture(row: &CapabilityRow, fixtures: &FixtureCorpus) -> ProviderModuleResolution`
- `validate_p0_provider_module_burndown(matrix: &CapabilityMatrix, fixtures: &FixtureCorpus) -> ProviderModuleBurndownReport`

## 6. Acceptance Criteria

- [ ] **AC1** (PRD §Provider P0): each current P0 provider module blocker has fixture/native evidence or explicit external blocker with reason.
- [ ] **AC2** (ADR-005): no provider fixture requires real API keys or leaks provider secrets.
- [ ] **AC3** (ADR-009): `longtail-classification.json` reports the updated P0 blocker count and lists every remaining blocker by item id and reason.
- [ ] **AC4** (PRD §Compatibility Matrix): user-visible provider gap errors include item id, class, reason, docs link, and nonzero exit.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-18.2.1 | TEST-18.2.1 | tests/p0_provider_module_fixture_burndown.rs | install, typecheck, unit-test, integration, build | Not Started |
| AC2 | SCEN-18.2.1 | TEST-18.2.2 | tests/p0_provider_module_fixture_burndown.rs | install, typecheck, unit-test, coverage, build | Not Started |
| AC3 | SCEN-18.2.1 | TEST-18.2.3 | tests/p0_provider_module_fixture_burndown.rs | install, typecheck, unit-test, integration, runtime-smoke, build | Not Started |
| AC4 | SCEN-18.2.1 | TEST-18.2.4 | tests/p0_provider_module_fixture_burndown.rs | install, typecheck, unit-test, e2e, build | Not Started |

## 8. Risks

- Some P0 provider files may be helper modules under a supported provider rather than standalone provider IDs; classify by source path and runtime behavior before adding fixtures.
- Some providers need external credentials; preserve blocker instead of faking success.
- Over-aggregating all provider files into one fixture would weaken traceability; keep item ids visible.

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
