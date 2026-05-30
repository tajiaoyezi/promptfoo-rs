# Phase 11: upstream-inventory-baseline

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

把“当前 upstream promptfoo 仓库状态”和“冻结 0.121.13 baseline”拆成可审计目标，并生成 item-level 兼容矩阵输入。依据 PRD §Upstream Baseline Freeze Strategy、§Compatibility Matrix、ADR-007、ADR-009，以及 docs/audits/promptfoo-final-audit-index-2026-05-30.md。

## 2. Business Value

该阶段消除“包版本相同但 upstream main 漂移”的歧义，让后续实现只面向明确、可重复的兼容目标，并避免 provider/assertion/redteam/CLI 长尾能力被粗粒度矩阵沉默遗漏。

## 3. Scope / Modules

docs/compatibility/、compatibility/inventory/、src/compatibility/、tests/compatibility_inventory.rs、docs/audits/

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 11.1 | current-upstream-target-policy | ../tasks/task-11.1-current-upstream-target-policy.md | Ready | 明确 frozen baseline 与 moving upstream 的兼容声明边界 |
| 11.2 | item-level-capability-inventory | ../tasks/task-11.2-item-level-capability-inventory.md | Ready | 从 upstream 源码/docs/examples 提取 item-level 能力清单 |
| 11.3 | compatibility-matrix-expansion | ../tasks/task-11.3-compatibility-matrix-expansion.md | Ready | 用 item-level inventory 扩展兼容矩阵并阻断沉默遗漏 |

## 5. Dependencies

依赖 Phase 1 和审计包 docs/audits/* 完成。

## 6. Phase Acceptance Criteria

- [ ] Phase 11 所有 task spec 状态为 Done，adapter Phase/Task 索引与文件一致。
- [ ] frozen baseline 与 moving upstream 的声明边界写入 compatibility docs，且 stable release 只引用一个明确目标。
- [ ] item-level inventory 和 expanded matrix 能覆盖 upstream CLI/provider/assertion/redteam/output/config/API/release surface，并能报告遗漏数。

## 7. Phase Risks

- upstream main 会继续漂移；本 phase 必须把 moving target 作为单独登记项，不得覆盖 frozen baseline lock。
- inventory extractor 可能漏掉动态注册项；必须保留 manual review list 和 unresolved item bucket。

## 8. Definition of Done

- 所有 task §10 Completion Notes 回填。
- Phase 11 smoke gate 运行 inventory completeness report 和 matrix no-silent-omission check。
- 后续 Phase 12 可以直接消费 inventory/matrix artifact 生成 fixtures。
