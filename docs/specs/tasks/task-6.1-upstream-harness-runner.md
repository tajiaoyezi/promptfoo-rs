# Task 6.1: upstream-harness-runner

> ✅ **Status: Done** — task-6.1 已按 RED→GREEN→§9 verification 完成，详见 §10 Completion Notes。

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 6 — compatibility-harness
**Dependencies**: Phase 6 AUTO order; see adapter Task 总索引

## 1. Background

PRD 要求 promptfoo-rs 在 promptfoo 0.121.13 baseline 下建立 Rust-native core、兼容矩阵和 golden diff release gate。本 task 负责 compatibility 模块中的 upstream-harness-runner 工作。

## 2. Goal

执行 upstream promptfoo@0.121.13 与 promptfoo-rs 的同输入 fixture runner。

## 3. Scope

### In Scope

- compatibility 模块中与 upstream-harness-runner 直接相关的源码、测试、fixture 和文档。
- 与本 task AC 对应的 compatibility matrix 或 release gate 记录。
- 具体文件清单：src/compatibility/harness.rs、src/compatibility/normalize.rs、tests/upstream_harness_runner.rs、compatibility/fixtures/、compatibility/artifacts/。依据 PRD §Compatibility Harness Design 与 ADR-007。

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
- docs/specs/phases/phase-6-compatibility-harness.md
- test/features/compatibility.feature
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-006-compatibility-first-test-toolchain.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md

### 5.2 Imports

- Rust crate / module：`serde`、`serde_json`、`tempfile`、`assert_cmd`、内部模块 `compatibility::harness`、`compatibility::normalize`。依据 ADR-006 / ADR-007。

### 5.3 函数签名

- `HarnessRunner::run_fixture(fixture: &FixtureSpec) -> Result<HarnessArtifacts, HarnessError>`
- `normalize_artifact(artifact: &Artifact, rules: &NormalizationRules) -> NormalizedArtifact`
- `reject_floating_baseline(reference: &BaselineReference) -> Result<(), HarnessError>`
- Harness 固定 baseline、生成 upstream/rs artifacts，并 snapshot 归一化规则；依据 PRD §Compatibility Harness Design。

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Implementation Phases / §Compatibility Matrix): harness 固定 baseline artifact 并拒绝 latest
- [x] **AC2** (PRD §Implementation Phases / §Compatibility Matrix): 同一 fixture 能生成 upstream artifact 与 rs artifact
- [x] **AC3** (PRD §Implementation Phases / §Compatibility Matrix): 时间、路径、随机 ID、latency 归一化规则有 snapshot

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-6.1.1 | TEST-6.1.1 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-6.1.2 | TEST-6.1.2 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-6.1.3 | TEST-6.1.3 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |

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
  - src/compatibility/mod.rs
  - src/compatibility/harness.rs
  - src/compatibility/normalize.rs
  - tests/upstream_harness_runner.rs
  - docs/specs/tasks/task-6.1-upstream-harness-runner.md
  - docs/specs/phases/phase-6-compatibility-harness.md
  - docs/s2v-adapter.md
  - docs/compatibility/matrix.md
- **commit 列表**：
  - 1c83e82 test(compatibility): add task-6.1 harness RED tests
  - 961a116 feat(compatibility): add upstream harness runner
  - 本 docs(spec) 回填提交见 git log：docs(spec): 回填 task-6.1 §10 Completion Notes + Status → Done
- **§9 Verification 结果**：
  - install: passed — `cargo fetch` 通过；viewer/npm install 按 adapter N/A 跳过。
  - typecheck: passed — `cargo check --workspace` 通过。
  - unit-test: passed — `cargo test --workspace` 通过，TEST-6.1.1~TEST-6.1.3 加入后 39 个 integration tests 全部通过。
  - manual: passed — 已核对 AC1/AC2/AC3、SCEN-6.1.1~6.1.3、TEST-6.1.1~TEST-6.1.3、compatibility matrix 的 Compatibility harness / golden diff gate 行与本实现/测试一致；Codex 非交互环境无 `/dev/tty`，manual key 以人工审查记录留证。
- **剩余风险 / 未做项**：本 task 固定 harness contract、baseline 拒绝规则、paired artifact shape 和 normalization snapshot；真实 upstream Node 执行、artifact 持久化和 gate 分类由 task 6.2 / 后续 CI 接入继续扩展。
- **下游 task 影响**：task 6.2 可复用 `HarnessArtifacts`、`Artifact`、`NormalizedArtifact` 与 `NormalizationRules` 作为 golden diff 和 release gate 输入。
