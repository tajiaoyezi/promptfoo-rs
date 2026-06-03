# Task 3.2: cache-resume-retry

> ✅ **Status: Done** — task-3.2 已按 RED→GREEN→§9 verification 完成，详见 §10 Completion Notes。

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 3 — eval-runner-cache
**Dependencies**: Phase 3 AUTO order; see adapter Task 总索引

## 1. Background

PRD 要求 promptfoo-rs 在 promptfoo 0.121.13 baseline 下建立 Rust-native core、兼容矩阵和 golden diff release gate。本 task 负责 cache-resume-store 模块中的 cache-resume-retry 工作。

## 2. Goal

实现 cache key、resume cursor、retry-errors 和 backoff 行为。

## 3. Scope

### In Scope

- cache-resume-store 模块中与 cache-resume-retry 直接相关的源码、测试、fixture 和文档。
- 与本 task AC 对应的 compatibility matrix 或 release gate 记录。
- 具体文件清单：src/cache/mod.rs、src/cache/resume.rs、src/eval/retry.rs、tests/cache_resume_retry.rs、test/fixtures/cache-resume-store/。依据 PRD §Technical Approach 的 `cache-resume-store` 边界与 ADR-003。

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
- docs/specs/phases/phase-3-eval-runner-cache.md
- test/features/cache-resume-store.feature
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-003-streaming-jsonl-sqlite-store.md
- docs/decisions/adr-006-compatibility-first-test-toolchain.md

### 5.2 Imports

- Rust crate / module：`serde`、`serde_json`、`sha2`、`sqlx`、`tokio`、内部模块 `cache`、`eval::retry`。依据 ADR-003 / ADR-006。

### 5.3 函数签名

- `cache_key(input: &CacheKeyInput) -> String`
- `ResumeStore::load(path: &Path) -> Result<ResumeState, StoreError>`
- `retry_with_backoff(policy: RetryPolicy, op: impl Future) -> Result<T, EvalError>`
- 接口覆盖 cache key、partial JSONL/SQLite resume、retry-errors 与 backoff；依据 PRD §User Flow / ADR-003。

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Implementation Phases / §Compatibility Matrix): cache key fixture 覆盖 provider/config/test case 输入
- [x] **AC2** (PRD §Implementation Phases / §Compatibility Matrix): resume 能从 partial JSONL/SQLite 状态继续
- [x] **AC3** (PRD §Implementation Phases / §Compatibility Matrix): retry-errors 和 backoff 失败路径有可复现 tests

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-3.2.1 | TEST-3.2.1 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-3.2.2 | TEST-3.2.2 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-3.2.3 | TEST-3.2.3 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |

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
  - src/cache/mod.rs
  - src/cache/resume.rs
  - src/eval/mod.rs
  - src/eval/retry.rs
  - tests/cache_resume_retry.rs
  - docs/specs/tasks/task-3.2-cache-resume-retry.md
- **commit 列表**：
  - f4ffc0c test(cache-resume-store): add task-3.2 cache resume retry RED tests
  - e1e304d feat(cache-resume-store): implement cache resume retry core
  - 本 docs(spec) 回填提交见 git log：docs(spec): 回填 task-3.2 §10 Completion Notes + Status → Done
- **§9 Verification 结果**：
  - install: passed — `cargo fetch` 通过；viewer/npm install 按 adapter N/A 跳过。
  - typecheck: passed — `cargo check --workspace` 通过。
  - unit-test: passed — `cargo test --workspace` 通过，TEST-3.2.1~TEST-3.2.3 加入后 21 个 integration tests 全部通过。
  - manual: passed — 已核对 AC1/AC2/AC3、SCEN-3.2.1~3.2.3、TEST-3.2.1~3.2.3、compatibility matrix 的 P0 Cache/resume/retry/concurrency/delay 行与本实现/测试一致；Codex 非交互环境无 `/dev/tty`，manual key 以人工审查记录留证。
- **剩余风险 / 未做项**：cache key 目前固定为 promptfoo-rs canonical SHA-256 输入模型，后续 compatibility harness 需用 upstream promptfoo 0.121.13 golden fixture 分类差异；SQLite schema 先覆盖 resume 所需 `results` 表，viewer/output task 可扩展查询 schema。
- **下游 task 影响**：Phase 4 provider registry 可复用 `retry_with_backoff`；Phase 5 output/viewer 可复用 `ResumeStore` 与 JSONL/SQLite 结果读取契约。
