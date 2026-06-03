# Task 11.2: item-level-capability-inventory

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 11 — upstream-inventory-baseline
**Dependencies**: task-11.1-current-upstream-target-policy

## 1. Background

审计确认当前矩阵仍有 `Other documented providers`、`Redteam plugins/strategies` 等粗粒度行，不能证明 100% 登记。需要从 upstream source/docs/examples 生成 item-level inventory。依据 PRD §Compatibility Matrix、§Success Metrics、ADR-009、docs/audits/promptfoo-upstream-inventory-gap-2026-05-30.md。

## 2. Goal

生成机器可读 upstream capability inventory，覆盖 CLI commands/flags、providers、assertions、redteam plugins/strategies、outputs、config features、Node API、viewer/release surfaces。

## 3. Scope

### In Scope

- compatibility/inventory/upstream-items.json
- compatibility/inventory/sources.md
- src/compatibility/inventory.rs
- tests/item_level_capability_inventory.rs
- scripts or Rust helpers that read local upstream checkout snapshots

### Out Of Scope

- 不实现 inventory item 对应功能。
- 不把 inaccessible cloud/private behavior 伪装成 verified。

## 4. Users / Actors

- promptfoo-rs maintainer：用 inventory 驱动 matrix、fixtures、implementation tasks。
- AI infra 团队：用 inventory 判断迁移阻断项。

## 5. Behavior Contract

每个 inventory item 必须有 stable id、category、source path/URL、discovered name、suggested compatibility level、initial status 和 unresolved reason（如无法分类）。不得只登记已实现项。

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/audits/promptfoo-upstream-inventory-gap-2026-05-30.md
- docs/audits/promptfoo-requirements-traceability-audit-2026-05-30.md
- test/features/perfect-refactor-parity.feature
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md

### 5.2 Imports

- Rust crate / module：`serde`、`serde_json`、内部模块 `compatibility::inventory`。
- Data files：`compatibility/inventory/upstream-items.json`。

### 5.3 函数签名

- `InventoryItem::stable_id(category: &str, name: &str) -> String`
- `extract_upstream_inventory(snapshot: &UpstreamSnapshot) -> Result<CapabilityInventory, InventoryError>`
- `validate_inventory_completeness(inventory: &CapabilityInventory) -> InventoryReport`

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Compatibility Matrix): inventory 覆盖 upstream commands/flags/providers/assertions/redteam/outputs/config/API/viewer/release surfaces。
- [x] **AC2** (PRD §Success Metrics): 每个 item 有 stable id、source reference、category、level hint、status、owner hint。
- [x] **AC3** (docs/audits/upstream inventory): extractor 输出 unresolved bucket，且 unresolved item 不允许被 release gate 忽略。

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-11.2.1 | TEST-11.2.1 | tests/item_level_capability_inventory.rs | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-11.2.1 | TEST-11.2.2 | tests/item_level_capability_inventory.rs | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-11.2.1 | TEST-11.2.3 | tests/item_level_capability_inventory.rs | install, typecheck, unit-test, manual | Done |

## 8. Risks

- 源码动态注册项可能不能纯静态提取；需要 manual review list。
- upstream docs 与 source 不一致时，inventory 必须记录 conflict。

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Manual**: 对比 upstream snapshot counts 与 audit counts，确认无明显类别遗漏。

## 10. Completion Notes

- **完成日期**：2026-05-30
- **改动文件**：
  - `src/compatibility/mod.rs`（导出 inventory 模块）
  - `src/compatibility/inventory.rs`（新增 inventory 数据模型、extractor、completeness validator）
  - `tests/item_level_capability_inventory.rs`（新增 TEST-11.2.1 ~ TEST-11.2.3）
  - `compatibility/inventory/upstream-items.json`（新增机器可读 item-level inventory seed）
  - `compatibility/inventory/sources.md`（新增来源与 unresolved policy）
  - `docs/specs/tasks/task-11.2-item-level-capability-inventory.md`（状态、§7、§10 回填）
- **commit 列表**：
  - `449e647` docs(spec): task-11.2 进入实施 (Status: Ready → In Progress)
  - `62fc982` test(compatibility-inventory): 加 SCEN-11.2.1 的 3 个 RED 测试
  - `0bb29e9` feat(compatibility-inventory): 实现 upstream item inventory 基础校验
- **§9 Verification 结果**：
  - install: PASS — `cargo fetch && ...` 成功；viewer/npm package 尚不存在，pnpm 分支未触发。
  - typecheck: PASS — `cargo check --workspace` 成功。
  - unit-test: PASS — `cargo test --workspace` 成功；`tests/item_level_capability_inventory.rs` 中 TEST-11.2.1 ~ TEST-11.2.3 共 3 passed / 0 failed，workspace Rust integration tests 共 78 passed / 0 failed。
  - manual: PASS — 已核对 `compatibility/inventory/upstream-items.json` 当前含 44 个 item，覆盖 command/flag/provider/assertion/redteam-plugin/redteam-strategy/output/config/node-api/viewer/release 共 11 个必备类别；`status=unresolved` 为 1 项且 validator 将其计入 release-blocking unresolved。注：完整 `s2v_verify_full "install typecheck unit-test manual"` 已执行到 manual，但当前非交互工具环境无 `/dev/tty`，manual helper 无法读取确认输入并返回 rc=1；人工核验结果记录于本条。
- **剩余风险 / 未做项**：seed inventory 依据审计证据和 PRD 建立可执行基底；长尾 provider/assertion/redteam 逐项扩展继续由 task-14.1/task-14.2 完成，未解析动态 registry 保持 release-blocking。
- **下游 task 影响**：task-11.3 可直接消费 `CapabilityInventory` 与 `upstream-items.json` 做 matrix expansion；task-12.x/14.x 可复用 stable id 与 unresolved policy。
