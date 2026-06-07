# Task 49.1: product-baseline-policy-gate-alignment

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 49 — product-baseline-v1-gate-alignment
**Dependencies**: ADR-012, task-48.1-current-latest-0.121.15-target-refresh

## 1. Background

ADR-012 freezes the product compatibility baseline at Phase 48 `promptfoo@0.121.15` and retires default upstream drift refresh. Runtime policy and unblock packet still required `current_upstream_rebaseline_required=true` and listed `current-upstream:rebaseline` decisions.

## 2. Goal

Emit product-baseline upstream policy and unblock-packet fields that record ADR-012 freeze without scheduling rebaseline when GitHub HEAD drifts.

## 3. Scope

### In Scope

- `scripts/release/product-baseline-gate-lib.cjs`
- `scripts/release/current-upstream-policy.sh`
- `src/compatibility/inventory.rs` (`TargetMode::ProductBaseline`)
- `src/release.rs` (`product_baseline_frozen` on unblock inputs/packet)
- `tests/product_baseline_v1_gate_alignment.rs`

### Out Of Scope

- Renaming all `current-latest-*` gate namespaces (cosmetic follow-up).
- Setting `perfect_refactor_claim_allowed=true`.

## 4. Users / Actors

- Maintainer: needs gates to stop implying upstream rebaseline backlog.
- Release reviewer: needs stable_claim wording to reference ADR-012 product baseline.

## 5. Behavior Contract

Default upstream policy uses Phase 48 tracked lock (`compatibility/inventory/current-latest-target.json`), sets `product_baseline_frozen=true`, `current_upstream_rebaseline_required=false`, and `stable_claim=product-baseline compatibility (ADR-012)`. Unblock packet must not emit `current-upstream:rebaseline` when product baseline is frozen.

### 5.1 Required Reading

- docs/decisions/adr-012-product-independence-baseline-freeze.md
- compatibility/inventory/current-latest-target.json
- docs/specs/tasks/task-48.1-current-latest-0.121.15-target-refresh.md

### 5.2 Imports

- Rust: `promptfoo_rs::compatibility::inventory::{TargetMode, evaluate_current_claim_policy}`
- Shell: `scripts/release/current-upstream-policy.sh`, `product-baseline-gate-lib.cjs`

### 5.3 函数签名

- `evaluate_current_claim_policy(frozen, current, TargetMode::ProductBaseline) -> CurrentClaimPolicy`
- `loadProductBaselineTarget(path) -> ProductBaselinePacket | null` (gate lib)

## 6. Acceptance Criteria

- [x] **AC1** (ADR-012): default policy `target_mode=product-baseline`, frozen package `0.121.15`, `current_upstream_rebaseline_required=false`.
- [x] **AC2** (ADR-012): HEAD drift vs npm gitHead is observation-only in policy `reason`; no rebaseline decision item when frozen.
- [x] **AC3** (task 48.1): tracked lock `compatibility/inventory/current-latest-target.json` remains authoritative for product baseline fields.
- [x] **AC4** (PRD §Product Independence): `perfect_refactor_claim_allowed` stays false; `local_stable_allowed` unaffected.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-49.1.1 | TEST-49.1.1 | tests/product_baseline_v1_gate_alignment.rs | install, typecheck, unit-test, build | Done |
| AC2 | SCEN-49.1.1 | TEST-49.1.2 | tests/product_baseline_v1_gate_alignment.rs | install, typecheck, unit-test, integration, build | Done |
| AC3 | SCEN-49.1.1 | TEST-49.2.2 | tests/product_baseline_v1_gate_alignment.rs | install, lint, typecheck, unit-test, build | Done |
| AC4 | SCEN-49.1.1 | TEST-49.1.2 | tests/product_baseline_v1_gate_alignment.rs | install, typecheck, unit-test, runtime-smoke, build | Done |

## 8. Risks

- Policy mode regression could hide real rebaseline needs if ADR-012 is superseded without updating gates.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Lint**: adapter §Commands Lint
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：2026-06-07
- **改动文件**：
  - scripts/release/product-baseline-gate-lib.cjs（新增）
  - scripts/release/current-upstream-policy.sh（修改）
  - scripts/release/perfect-refactor-unblock-packet.sh（修改）
  - src/compatibility/inventory.rs（修改）
  - src/release.rs（修改）
  - tests/product_baseline_v1_gate_alignment.rs（新增）
- **commit 列表**：
  - e1de727 feat(compatibility-gates): Phase 49 product-baseline v1 gate alignment
  - b8ded58 fix(compatibility-gates): resolve gate lib path from script directory
  - 600e7d7 fix(compatibility-gates): align authority gate with v1 resolved manifest rows
- **§9 Verification 结果**：
  - install: ✅
  - lint: ✅
  - typecheck: ✅
  - unit-test: workspace passed / 0 failed
  - integration: ✅
  - build: ✅
  - runtime-smoke: ✅
- **剩余风险**：若 ADR-012 被 supersede 须同步更新 gate 默认 `target_mode` 与 policy 脚本。
- **下游 task 影响**：无