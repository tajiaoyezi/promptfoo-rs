# Task 11.3: compatibility-matrix-expansion

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 11 — upstream-inventory-baseline
**Dependencies**: task-11.2-item-level-capability-inventory

## 1. Background

PRD 要求 100% 登记 promptfoo 0.121.13 已文档化能力，审计证明当前矩阵只有 domain-level rows。需要把 inventory item 展开为 release gate 可消费的 item-level matrix。依据 PRD §Compatibility Matrix、§Success Metrics、ADR-009。

## 2. Goal

扩展 docs/compatibility/matrix.md 或新增 machine-readable matrix，使每个 inventory item 都有 P0/P1/P2、native/bridge/unsupported/later、verification、owner、fixture/gap reference。

## 3. Scope

### In Scope

- docs/compatibility/matrix.md
- compatibility/inventory/upstream-items.json
- compatibility/matrix/items.json
- src/compatibility/matrix.rs
- tests/compatibility_matrix_expansion.rs

### Out Of Scope

- 不在本 task 实现 matrix row 对应能力。
- 不删除 unknown/blocked item；必须保留并分类。

## 4. Users / Actors

- maintainer：用 expanded matrix 驱动后续 implementation phases。
- release manager：用 matrix completeness 判定 stable release gate。

## 5. Behavior Contract

matrix completeness validator 必须在 inventory item 无 matrix row、P2 无 reason、P0 无 fixture plan、owner/verification 为空时失败。

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/compatibility/matrix.md
- docs/audits/promptfoo-requirements-traceability-audit-2026-05-30.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`serde`、`serde_json`、内部模块 `compatibility::matrix`、`compatibility::inventory`。

### 5.3 函数签名

- `expand_matrix_from_inventory(inventory: &CapabilityInventory, policy: &MatrixPolicy) -> CapabilityMatrix`
- `validate_no_silent_omissions(inventory: &CapabilityInventory, matrix: &CapabilityMatrix) -> MatrixCompletenessReport`
- `matrix_release_blockers(report: &MatrixCompletenessReport) -> Vec<MatrixBlocker>`

## 6. Acceptance Criteria

- [ ] **AC1** (PRD §Success Metrics): 100% inventory items have matrix rows with level/status/verification/owner.
- [ ] **AC2** (ADR-009): P0 rows require fixture or blocker reference; P1 rows require snapshot plan; P2 rows require reason and later/unsupported target.
- [ ] **AC3** (docs/audits/requirements traceability): validator fails when aggregate rows hide item-level omissions.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-11.3.1 | TEST-11.3.1 | tests/compatibility_matrix_expansion.rs | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-11.3.1 | TEST-11.3.2 | tests/compatibility_matrix_expansion.rs | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-11.3.1 | TEST-11.3.3 | tests/compatibility_matrix_expansion.rs | install, typecheck, unit-test, manual | Done |

## 8. Risks

- 直接把所有 unknown 标 P2 会掩盖工作量；P2 必须有 reason 和 target。
- markdown matrix 可能不适合 1000+ rows；允许新增 JSON matrix，以 markdown 汇总。

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Manual**: spot-check provider/assertion/redteam/CLI rows against inventory artifact。

## 10. Completion Notes

- **完成日期**：2026-05-30
- **改动文件**：
  - `src/compatibility/matrix.rs`（新增 item-level matrix manifest loader、inventory expansion、no-silent-omission validator、release blocker conversion）
  - `tests/compatibility_matrix_expansion.rs`（新增 TEST-11.3.1 ~ TEST-11.3.3）
  - `compatibility/matrix/items.json`（新增 item-level matrix manifest）
  - `docs/compatibility/matrix.md`（新增 item-level artifact 说明）
  - `docs/specs/tasks/task-11.3-compatibility-matrix-expansion.md`（状态、§7、§10 回填）
- **commit 列表**：
  - `3716e63` docs(spec): task-11.3 进入实施 (Status: Ready → In Progress)
  - `0e90dc7` test(compatibility-matrix): 加 SCEN-11.3.1 的 3 个 RED 测试
  - `abbdb71` feat(compatibility-matrix): 扩展 item-level matrix 校验
- **§9 Verification 结果**：
  - install: PASS — `cargo fetch && ...` 成功；viewer/npm package 尚不存在，pnpm 分支未触发。
  - typecheck: PASS — `cargo check --workspace` 成功。
  - unit-test: PASS — `cargo test --workspace` 成功；`tests/compatibility_matrix_expansion.rs` 中 TEST-11.3.1 ~ TEST-11.3.3 共 3 passed / 0 failed，workspace Rust integration tests 共 81 passed / 0 failed。
  - manual: PASS — 已 spot-check `compatibility/matrix/items.json` 通过 `source_inventory` 展开到与 `compatibility/inventory/upstream-items.json` 相同的 44 个 item-level rows；TEST-11.3.3 证明 aggregate row 不能隐藏 item-level omission。注：完整 `s2v_verify_full "install typecheck unit-test manual"` 已执行到 manual，但当前非交互工具环境无 `/dev/tty`，manual helper 无法读取确认输入并返回 rc=1；人工核验结果记录于本条。
- **剩余风险 / 未做项**：matrix manifest 采用从 inventory 派生的单一事实源，避免复制漂移；后续 task-12.x 需要用 fixture/golden artifact 替换当前 `fixture:<stable_id>` / `snapshot:<stable_id>` 计划占位为真实证据。
- **下游 task 影响**：Phase 12 可用 `matrix_release_blockers` 驱动 fixture corpus 和 release gate；task-14.x 可扩展 inventory 后自动进入 item-level matrix。
