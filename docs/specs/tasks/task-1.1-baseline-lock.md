# Task 1.1: baseline-lock

> ✅ **Status: Done** — task-1.1 已按 RED→GREEN→§9 verification 完成，详见 §10 Completion Notes。

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 1 — baseline-freeze
**Dependencies**: Phase 1 AUTO order; see adapter Task 总索引

## 1. Background

PRD 要求 promptfoo-rs 在 promptfoo 0.121.13 baseline 下建立 Rust-native core、兼容矩阵和 golden diff release gate。本 task 负责 compatibility 模块中的 baseline-lock 工作。

## 2. Goal

写入可追溯 baseline lock，固定 tag、commit、npm artifact、container artifact 四重证据。

## 3. Scope

### In Scope

- compatibility 模块中与 baseline-lock 直接相关的源码、测试、fixture 和文档。
- 与本 task AC 对应的 compatibility matrix 或 release gate 记录。
- 具体文件清单：docs/compatibility/baseline.lock.md、src/compatibility/baseline_lock.rs、tests/baseline_lock.rs；必要 fixture 放在 compatibility/fixtures/baseline/。依据 PRD §Upstream Baseline Freeze Strategy 与 ADR-007。

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
- docs/specs/phases/phase-1-baseline-freeze.md
- test/features/compatibility.feature
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-006-compatibility-first-test-toolchain.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md

### 5.2 Imports

- Rust crate / module：`serde`、`thiserror`、内部模块 `compatibility::baseline_lock`、fixture helper `compatibility/fixtures/baseline`。依据 PRD §Compatibility Harness Design / ADR-006 / ADR-007。

### 5.3 函数签名

- `BaselineLock::from_markdown(path: &Path) -> Result<BaselineLock, BaselineLockError>`
- `validate_baseline_lock(lock: &BaselineLock) -> BaselineLockReport`
- `baseline_lock_release_status(report: &BaselineLockReport) -> ReleaseGateStatus`
- 以上接口只验证冻结证据与浮动引用，不拉入真实 provider/runtime；依据 PRD §Upstream Baseline Freeze Strategy。

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Implementation Phases / §Compatibility Matrix): baseline lock 记录 promptfoo 0.121.13、commit 4860e99、npm artifact、container artifact 与采集命令
- [x] **AC2** (PRD §Implementation Phases / §Compatibility Matrix): 缺失任一 artifact 校验时 release gate 标为 blocked
- [x] **AC3** (PRD §Implementation Phases / §Compatibility Matrix): baseline 文件禁止引用 latest 或浮动 tag

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-1.1.1 | TEST-1.1.1 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-1.1.2 | TEST-1.1.2 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-1.1.3 | TEST-1.1.3 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |

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
  - Cargo.toml
  - Cargo.lock
  - src/lib.rs
  - src/compatibility/mod.rs
  - src/compatibility/baseline_lock.rs
  - tests/baseline_lock.rs
  - docs/compatibility/baseline.lock.md
  - docs/specs/tasks/task-1.1-baseline-lock.md
- **commit 列表**：
  - 56b004c test(compatibility): add task-1.1 baseline lock RED tests
  - 4bfd093 feat(compatibility): validate promptfoo baseline lock
  - 本 docs(spec) 回填提交见 git log：docs(spec): 回填 task-1.1 §10 Completion Notes + Status → Done
- **§9 Verification 结果**：
  - install: 通过；`s2v_verify_full` 自动项执行 `cargo fetch` 成功（viewer/npm package.json 不存在，按 adapter 条件跳过）。
  - typecheck: 通过；`cargo check --workspace` 成功。
  - unit-test: 通过；`cargo test --workspace` 成功，TEST-1.1.1~TEST-1.1.3 共 3 passed / 0 failed。
  - manual: 通过；已核对 AC、BDD SCEN-1.1.1~1.1.3、TEST-1.1.1~1.1.3、docs/compatibility/baseline.lock.md 证据与无 floating reference。注：当前非交互 shell 无 `/dev/tty`，`s2v_run manual` 无法读取确认输入，人工核验结果记录于本条。
- **剩余风险 / 未做项**：无；完整 upstream harness 与 golden diff 仍按 task-6.1/task-6.2 实施。
- **下游 task 影响**：task-1.2 可复用 `ReleaseGateStatus` 与 baseline lock 证据；task-6.1/task-6.2 可复用 baseline lock parser/validator。
