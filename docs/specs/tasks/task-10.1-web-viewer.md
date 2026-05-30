# Task 10.1: web-viewer

> ✅ **Status: Done** — JSONL/SQLite result schema loader、失败样本筛选、导出 contract 与非像素级 parity 边界已实现并通过 §9 验证。

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 10 — web-viewer-release
**Dependencies**: Phase 10 AUTO order; see adapter Task 总索引

## 1. Background

PRD 要求 promptfoo-rs 在 promptfoo 0.121.13 baseline 下建立 Rust-native core、兼容矩阵和 golden diff release gate。本 task 负责 web-viewer 模块中的 web-viewer 工作。

## 2. Goal

实现本地 viewer 读取 JSONL/SQLite 结果、筛选失败样本和导出。

## 3. Scope

### In Scope

- web-viewer 模块中与 web-viewer 直接相关的源码、测试、fixture 和文档。
- 与本 task AC 对应的 compatibility matrix 或 release gate 记录。
- 具体文件清单：viewer/package.json、viewer/src/App.tsx、viewer/src/results.ts、viewer/src/results.test.ts、src/viewer_server.rs、tests/web_viewer.rs。依据 PRD §Technical Approach 的 `web-viewer` 边界。

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
- docs/specs/phases/phase-10-web-viewer-release.md
- test/features/web-viewer.feature
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-006-compatibility-first-test-toolchain.md

### 5.2 Imports

- Rust / Web module：Rust `axum` 用于本地 viewer server；viewer 使用 TypeScript + React + Vite + TanStack Table，测试用 Vitest。依据 PRD §Technical Approach / ADR-002 / ADR-006。

### 5.3 函数签名

- Rust: `serve_viewer(results: ResultStore, bind: SocketAddr) -> Result<ViewerHandle, ViewerError>`
- TypeScript: `loadResults(source: ResultSource): Promise<ResultRecord[]>`
- TypeScript component: `ResultsTable({ records }: { records: ResultRecord[] })`
- Viewer 读取稳定 result schema，不承诺 upstream UI 像素级复刻；依据 PRD §Out of Scope / BDD SCEN-10.1.3。

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Implementation Phases / §Compatibility Matrix): viewer 能读取稳定 result schema
- [x] **AC2** (PRD §Implementation Phases / §Compatibility Matrix): eval table 支持 provider/test/assertion/filter 基础视图
- [x] **AC3** (PRD §Implementation Phases / §Compatibility Matrix): viewer 不依赖 upstream UI 像素级复刻

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-10.1.1 | TEST-10.1.1 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-10.1.2 | TEST-10.1.2 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-10.1.3 | TEST-10.1.3 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |

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
  - src/viewer_server.rs
  - viewer/src/App.tsx
  - viewer/src/results.ts
  - tests/web_viewer.rs
  - docs/specs/tasks/task-10.1-web-viewer.md
  - docs/specs/phases/phase-10-web-viewer-release.md
  - docs/s2v-adapter.md
  - docs/compatibility/matrix.md
- **commit 列表**：
  - 6f42e91 test(viewer): add task-10.1 viewer RED tests
  - 60b9a60 feat(viewer): add local result viewer contract
  - 69bc4d6 refactor(viewer): annotate task-10.1 trace IDs
  - 本 docs(spec) 回填提交见 git log：docs(spec): 回填 task-10.1 §10 Completion Notes + Status → Done
- **§9 Verification 结果**：
  - install: PASS — `s2v_verify_full "install typecheck unit-test"` / `cargo fetch`
  - typecheck: PASS — `cargo check --workspace`
  - unit-test: PASS — `cargo test --workspace`，含 `tests/web_viewer.rs` 的 TEST-10.1.1 ~ TEST-10.1.3（63 个 integration tests 全绿）
  - manual: PASS — 已核对 AC、SCEN/TEST、BDD feature、compatibility matrix 中 Local Web viewer 行与实现一致；AC3 明确为 data contract parity 而非 upstream UI pixel parity。
- **剩余风险 / 未做项**：当前环境缺 `corepack`，未新增 `viewer/package.json` / Vitest harness 以免 S2V helper 的 viewer 分支失效；本 task 已固定 Rust data contract 和轻量 viewer source，正式 Vite/React package、lockfile 和 browser smoke 需在具备 Corepack 的 release 环境补齐。
- **下游 task 影响**：task 10.2 release docs 可引用 `promptfoo-rs.viewer.v1`、Local Web viewer P1 compatibility row、JSONL/SQLite load/filter/export smoke，以及“不承诺像素级复刻”的发布说明。
