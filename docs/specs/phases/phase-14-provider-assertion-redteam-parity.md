# Phase 14: provider-assertion-redteam-parity

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

把 provider、assertion、redteam plugin/strategy 从代表性子集扩展到 item-level P0/P1/P2 覆盖，并为 native/bridge/unsupported/later 决策提供 fixture 证据。依据 PRD §Compatibility Matrix、ADR-001、ADR-005、ADR-009。

## 2. Business Value

provider/assertion/redteam 长尾是 promptfoo 生态兼容的主要风险。本阶段让每个能力都有可审计状态，不再只依赖粗粒度 known gap。

## 3. Scope / Modules

src/providers/、src/assertions/、src/redteam/、src/script_bridge/、compatibility/fixtures/providers/、compatibility/fixtures/assertions/、compatibility/fixtures/redteam/

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 14.1 | provider-assertion-inventory-parity | ../tasks/task-14.1-provider-assertion-inventory-parity.md | Ready | 按 inventory 实现或登记 provider/assertion 全量能力 |
| 14.2 | redteam-plugin-strategy-parity | ../tasks/task-14.2-redteam-plugin-strategy-parity.md | Ready | 按 inventory 实现或登记 redteam plugin/strategy 全量能力 |

## 5. Dependencies

依赖 Phase 11 item-level inventory、Phase 12 fixture runner。

## 6. Phase Acceptance Criteria

- [ ] 每个 upstream provider/assertion/redteam item 都有 matrix row、status、verification、owner 和 P2/later reason。
- [ ] P0 provider/assertion/redteam core fixtures 通过 golden diff 或被 release gate 阻断。
- [ ] custom script bridge default-deny、allowlist、timeout、redaction 覆盖 JS/TS、Python、Shell/Ruby 决策边界。

## 7. Phase Risks

- 部分 provider 需要真实账号或密钥；必须使用 mock server/recorded artifact，并把真实密钥需求列为 blocked 或 P2。
- redteam 插件可能有法律/安全语义；高风险内容必须保留政策引用和 mock evaluator。

## 8. Definition of Done

- Phase 14 smoke gate 输出 provider/assertion/redteam coverage summary，P0 missing count 为 0，P2 reason missing count 为 0。
