# Task 7.2: redteam-registry-report

> ✅ **Status: Done** — task-7.2 已按 RED→GREEN→§9 verification 完成，详见 §10 Completion Notes。

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 7 — redteam-core
**Dependencies**: Phase 7 AUTO order; see adapter Task 总索引

## 1. Background

PRD 要求 promptfoo-rs 在 promptfoo 0.121.13 baseline 下建立 Rust-native core、兼容矩阵和 golden diff release gate。本 task 负责 redteam-engine 模块中的 redteam-registry-report 工作。

## 2. Goal

实现核心插件/strategy registry、风险评分和 report 输出。

## 3. Scope

### In Scope

- redteam-engine 模块中与 redteam-registry-report 直接相关的源码、测试、fixture 和文档。
- 与本 task AC 对应的 compatibility matrix 或 release gate 记录。
- 具体文件清单：src/redteam/registry.rs、src/redteam/risk.rs、src/redteam/report.rs、tests/redteam_registry_report.rs、compatibility/fixtures/redteam/。依据 PRD §Compatibility Matrix 的 redteam plugins/strategies 行。

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
- docs/specs/phases/phase-7-redteam-core.md
- test/features/redteam-engine.feature
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-006-compatibility-first-test-toolchain.md

### 5.2 Imports

- Rust crate / module：`serde`、`serde_json`、内部模块 `redteam::registry`、`redteam::risk`、`redteam::report`。依据 ADR-006 / ADR-009。

### 5.3 函数签名

- `RedteamRegistry::core_defaults() -> RedteamRegistry`
- `score_risk(findings: &[RedteamFinding]) -> RiskSummary`
- `write_redteam_report(report: &RedteamReport, writer: impl Write) -> Result<(), RedteamError>`
- 注册表必须登记 P0/P1/P2，report 输出进入 compatibility harness；依据 PRD §Compatibility Matrix。

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Implementation Phases / §Compatibility Matrix): 核心 redteam plugin/strategy 矩阵登记 P0/P1/P2
- [x] **AC2** (PRD §Implementation Phases / §Compatibility Matrix): 风险评分字段稳定并可 snapshot
- [x] **AC3** (PRD §Implementation Phases / §Compatibility Matrix): report 输出能进入 compatibility harness

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-7.2.1 | TEST-7.2.1 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-7.2.2 | TEST-7.2.2 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-7.2.3 | TEST-7.2.3 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |

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
  - src/redteam/mod.rs
  - src/redteam/registry.rs
  - src/redteam/risk.rs
  - src/redteam/report.rs
  - tests/redteam_registry_report.rs
  - docs/specs/tasks/task-7.2-redteam-registry-report.md
  - docs/specs/phases/phase-7-redteam-core.md
  - docs/s2v-adapter.md
  - docs/compatibility/matrix.md
- **commit 列表**：
  - f2c6a0d test(redteam): add task-7.2 registry report RED tests
  - 14d1a2d feat(redteam): add registry risk report contract
  - 本 docs(spec) 回填提交见 git log：docs(spec): 回填 task-7.2 §10 Completion Notes + Status → Done
- **§9 Verification 结果**：
  - install: passed — `cargo fetch` 通过；viewer/npm install 按 adapter N/A 跳过。
  - typecheck: passed — `cargo check --workspace` 通过。
  - unit-test: passed — `cargo test --workspace` 通过，TEST-7.2.1~TEST-7.2.3 加入后 48 个 integration tests 全部通过。
  - manual: passed — 已核对 AC1/AC2/AC3、SCEN-7.2.1~7.2.3、TEST-7.2.1~TEST-7.2.3、compatibility matrix 的 Redteam plugins/strategies 行与本实现/测试一致；Codex 非交互环境无 `/dev/tty`，manual key 以人工审查记录留证。
- **剩余风险 / 未做项**：本 task 固定 core defaults registry、risk score snapshot 和 schema-versioned report artifact；长尾 redteam plugin/strategy 行为仍按 matrix P1/P2 后续扩展。
- **下游 task 影响**：Phase 6 compatibility harness 可将 `promptfoo-rs.redteam.report.v1` artifact 纳入 golden diff；Phase 8 scan/audit 可复用 `RiskSummary` 风险字段。
