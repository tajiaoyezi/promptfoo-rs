# Task 8.2: scan-audit-sarif

> ✅ **Status: Done** — scan finding schema、SARIF writer 输入契约与 false-positive known limitation 已实现并通过 §9 验证。

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 8 — mcp-scan-audit
**Dependencies**: Phase 8 AUTO order; see adapter Task 总索引

## 1. Background

PRD 要求 promptfoo-rs 在 promptfoo 0.121.13 baseline 下建立 Rust-native core、兼容矩阵和 golden diff release gate。本 task 负责 scan-engine 模块中的 scan-audit-sarif 工作。

## 2. Goal

实现 code-scans、scan-model、model-audit 与 SARIF 输出契约。

## 3. Scope

### In Scope

- scan-engine 模块中与 scan-audit-sarif 直接相关的源码、测试、fixture 和文档。
- 与本 task AC 对应的 compatibility matrix 或 release gate 记录。
- 具体文件清单：src/scan/mod.rs、src/scan/finding.rs、src/output/sarif.rs、tests/scan_audit_sarif.rs、test/fixtures/scan-engine/。依据 PRD §Technical Approach 的 `scan-engine` 与 `output-writers` 边界。

### Out Of Scope

- 不实现本 task AC 之外的长尾 provider/assertion/plugin。
- 不绕过 PRD 的 P0/P1/P2 兼容等级规则。
- 不修改 unrelated phase/task spec。

## 4. Users / Actors

- **AI 应用开发者**：通过 CLI、配置、输出和本地 viewer 感知兼容性。
- **AI infra / 平台工程团队**：在 CI 中依赖 exit code、JUnit/SARIF、golden diff 和 release gate。
- **安全红队团队**：依赖 redteam/MCP/scan/script bridge 的本地可审计执行边界。
- 本 task 无额外 actor；沿用 adapter §Project 中的 AI 应用开发者、AI infra / 平台工程团队、安全红队团队与开源 maintainer。依据 docs/s2v-adapter.md §Project。

## 5. Behavior Contract

本 task 的外部可观察行为以 §6 AC、对应 BDD feature 和 compatibility fixture 为准。任何与 upstream promptfoo 0.121.13 的差异必须登记为 matching / intentional-difference / unsupported / later / upstream-ambiguous / bug。

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/phases/phase-8-mcp-scan-audit.md
- test/features/scan-engine.feature
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-004-cli-output-schema-protocol.md
- docs/decisions/adr-006-compatibility-first-test-toolchain.md

### 5.2 Imports

- Rust crate / module：`serde`、`serde_json`、内部模块 `scan`、`output::sarif`。依据 ADR-004 / ADR-006。

### 5.3 函数签名

- `run_scan(input: ScanInput) -> Result<Vec<Finding>, ScanError>`
- `Finding { rule_id, level, message, locations, metadata }`
- `write_sarif(findings: &[Finding], writer: impl Write) -> Result<(), OutputError>`
- 误报率只登记 known limitation，不作为 1.0 gate；依据 PRD §Technical Risks R5 与 BDD SCEN-8.2.3。

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Implementation Phases / §Compatibility Matrix): scan 命令输出 finding schema snapshot
- [x] **AC2** (PRD §Implementation Phases / §Compatibility Matrix): SARIF writer 通过 schema fixture
- [x] **AC3** (PRD §Implementation Phases / §Compatibility Matrix): 误报率不作为 1.0 gate 但 known limitation 登记

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-8.2.1 | TEST-8.2.1 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-8.2.2 | TEST-8.2.2 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-8.2.3 | TEST-8.2.3 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |

## 8. Risks

- upstream promptfoo 0.121.13 行为未文档化，fixture 可能覆盖不足。
- Windows/macOS/Linux path、env、shell 行为可能漂移。
- Draft 字段未清零就实施会破坏 S2V Ready Gate。

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Manual**: 审核本 task 的 AC、traceability、compatibility matrix 记录与 BDD scenario 是否一致。

## 10. Completion Notes

- **完成日期**：2026-05-30
- **改动文件**：
  - src/lib.rs
  - src/cli.rs
  - src/output/mod.rs
  - src/output/sarif.rs
  - src/scan/mod.rs
  - src/scan/finding.rs
  - tests/scan_audit_sarif.rs
  - docs/specs/tasks/task-8.2-scan-audit-sarif.md
  - docs/specs/phases/phase-8-mcp-scan-audit.md
  - docs/s2v-adapter.md
  - docs/compatibility/matrix.md
- **commit 列表**：
  - 3ac4cf1 test(scan): add task-8.2 SARIF RED tests
  - c180cac feat(scan): add scan SARIF contract
  - 本 docs(spec) 回填提交见 git log：docs(spec): 回填 task-8.2 §10 Completion Notes + Status → Done
- **§9 Verification 结果**：
  - install: PASS — `s2v_verify_full "install typecheck unit-test"` / `cargo fetch`
  - typecheck: PASS — `cargo check --workspace`
  - unit-test: PASS — `cargo test --workspace`，含 `tests/scan_audit_sarif.rs` 的 TEST-8.2.1 ~ TEST-8.2.3（54 个 integration tests 全绿）
  - manual: PASS — 已核对 AC、SCEN/TEST、BDD feature、compatibility matrix 中 code-scans / scan-model / model-audit 行与实现一致。
- **剩余风险 / 未做项**：当前 scan engine 只固定最小 schema 和 `eval(...)` fixture 行为；完整静态规则集、模型审计深度检查与误报率统计仍为后续扩展，且 false-positive rate 已按 PRD R5 登记为非 1.0 gate。
- **下游 task 影响**：Phase 10 release gate / docs 可引用 `promptfoo-rs.scan.v1`、SARIF properties.metadata 和 `scan.false-positive-rate` known limitation；后续 scan 规则扩展需保持 schema 兼容。
