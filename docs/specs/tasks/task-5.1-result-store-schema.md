# Task 5.1: result-store-schema

> ✅ **Status: Done** — task-5.1 已按 RED→GREEN→§9 verification 完成，详见 §10 Completion Notes。

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 5 — output-ci
**Dependencies**: Phase 5 AUTO order; see adapter Task 总索引

## 1. Background

PRD 要求 promptfoo-rs 在 promptfoo 0.121.13 baseline 下建立 Rust-native core、兼容矩阵和 golden diff release gate。本 task 负责 output-writers 模块中的 result-store-schema 工作。

## 2. Goal

定义 JSONL/SQLite result schema 与大结果流式写入策略。

## 3. Scope

### In Scope

- output-writers 模块中与 result-store-schema 直接相关的源码、测试、fixture 和文档。
- 与本 task AC 对应的 compatibility matrix 或 release gate 记录。
- 具体文件清单：src/results/schema.rs、src/results/jsonl.rs、src/results/sqlite.rs、tests/result_store_schema.rs、test/fixtures/output-writers/。依据 PRD §Technical Approach 的 `output-writers` 与 ADR-003。

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
- docs/specs/phases/phase-5-output-ci.md
- test/features/output-writers.feature
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-003-streaming-jsonl-sqlite-store.md
- docs/decisions/adr-004-cli-output-schema-protocol.md
- docs/decisions/adr-006-compatibility-first-test-toolchain.md

### 5.2 Imports

- Rust crate / module：`serde`、`serde_json`、`sqlx`、`tokio`、内部模块 `results`。依据 ADR-003 / ADR-004 / ADR-006。

### 5.3 函数签名

- `ResultRecord { eval_id, case_id, provider_id, assertion_results, latency_ms, metadata, error }`
- `JsonlResultWriter::append(record: &ResultRecord) -> Result<(), StoreError>`
- `SqliteResultStore::insert(record: &ResultRecord) -> Result<(), StoreError>`
- 写入接口必须流式 append，避免 10k case 常驻内存；依据 PRD §Boundary cases / ADR-003。

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Implementation Phases / §Compatibility Matrix): JSONL append schema 覆盖 result、error、metadata、latency shape
- [x] **AC2** (PRD §Implementation Phases / §Compatibility Matrix): SQLite/libSQL schema 支持按 eval、case、provider、assertion 查询
- [x] **AC3** (PRD §Implementation Phases / §Compatibility Matrix): 10k case 写入不需要完整结果集常驻内存

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-5.1.1 | TEST-5.1.1 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-5.1.2 | TEST-5.1.2 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-5.1.3 | TEST-5.1.3 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |

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
  - src/results/mod.rs
  - src/results/schema.rs
  - src/results/jsonl.rs
  - src/results/sqlite.rs
  - tests/result_store_schema.rs
  - docs/specs/tasks/task-5.1-result-store-schema.md
  - docs/specs/phases/phase-5-output-ci.md
  - docs/s2v-adapter.md
  - docs/compatibility/matrix.md
- **commit 列表**：
  - 3bd5393 test(output-writers): add task-5.1 result store RED tests
  - 0ca2c6f feat(output-writers): add streaming result store schema
  - 本 docs(spec) 回填提交见 git log：docs(spec): 回填 task-5.1 §10 Completion Notes + Status → Done
- **§9 Verification 结果**：
  - install: passed — `cargo fetch` 通过；viewer/npm install 按 adapter N/A 跳过。
  - typecheck: passed — `cargo check --workspace` 通过。
  - unit-test: passed — `cargo test --workspace` 通过，TEST-5.1.1~TEST-5.1.3 加入后 33 个 integration tests 全部通过。
  - manual: passed — 已核对 AC1/AC2/AC3、SCEN-5.1.1~5.1.3、TEST-5.1.1~TEST-5.1.3、compatibility matrix 的 JSON/JSONL/CSV/YAML output 行与本实现/测试一致；Codex 非交互环境无 `/dev/tty`，manual key 以人工审查记录留证。
- **剩余风险 / 未做项**：本 task 只定义 JSONL streaming writer、SQLite result/assertion schema 与查询 API；CSV/YAML/JUnit/SARIF/HTML 输出格式和 CLI exit code/stdout/stderr 合约留给 task 5.2。
- **下游 task 影响**：task 5.2 可复用 `ResultRecord`、`AssertionResultRecord`、`ResultStatus`、`JsonlResultWriter` 与 `SqliteResultStore` 作为 output formatter 和 CI artifact 的稳定输入模型。
