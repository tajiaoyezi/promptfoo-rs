# Task 12.2: executable-upstream-rs-runner

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 12 — compatibility-fixtures-golden-diff
**Dependencies**: task-12.1-p0-fixture-corpus

## 1. Background

Audit found `HarnessRunner` constructs in-memory artifacts and does not execute upstream promptfoo or promptfoo-rs. Perfect refactor proof requires real execution artifacts. Basis: PRD §Compatibility Harness Design, ADR-007, docs/audits/promptfoo-s2v-parity-claim-audit-2026-05-30.md.

## 2. Goal

Implement an executable runner that invokes pinned upstream promptfoo and current promptfoo-rs for the same fixture, then persists upstream, rs, normalized, and diff artifacts.

## 3. Scope

### In Scope

- src/compatibility/harness.rs
- src/compatibility/executor.rs
- compatibility/artifacts/
- tests/executable_upstream_rs_runner.rs
- docs/compatibility/harness.md

### Out Of Scope

- Does not decide fixture corpus contents; task 12.1 owns corpus.
- Does not publish release artifacts; task 12.3 and Phase 15 own release.

## 4. Users / Actors

- Release manager: needs reproducible artifacts for gate decisions.
- Maintainer: debugs fixture diffs and classification.

## 5. Behavior Contract

For each fixture, runner must create isolated work dirs, set deterministic env, disable upstream update checks, run upstream and rs commands, capture stdout/stderr/exit code/files, normalize artifacts, classify diffs, and persist outputs under compatibility/artifacts/<run-id>/.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/tasks/task-6.1-upstream-harness-runner.md
- docs/specs/tasks/task-12.1-p0-fixture-corpus.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`std::process::Command`、`tempfile`、`serde_json`、内部模块 `compatibility::harness`、`compatibility::normalize`、`compatibility::diff`。

### 5.3 函数签名

- `ExecutableHarnessRunner::run_fixture(fixture: &FixtureManifest) -> Result<PersistedRunArtifacts, HarnessError>`
- `PromptfooCommand::upstream_pinned(baseline: &BaselineReference) -> CommandSpec`
- `PromptfooCommand::current_rs(binary: &Path) -> CommandSpec`
- `persist_run_artifacts(run: &HarnessRun, output_dir: &Path) -> Result<PersistedRunArtifacts, HarnessError>`

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Compatibility Harness Design): runner executes upstream promptfoo and promptfoo-rs commands for the same fixture.
- [x] **AC2** (ADR-007): runner persists upstream/rs/raw/normalized/diff artifacts with run metadata.
- [x] **AC3** (docs/audits/S2V parity): command timeout, env isolation, update-disable, and no-secret behavior are enforced.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-12.2.1 | TEST-12.2.1 | tests/executable_upstream_rs_runner.rs | install, typecheck, unit-test, build, manual | Done |
| AC2 | SCEN-12.2.1 | TEST-12.2.2 | tests/executable_upstream_rs_runner.rs | install, typecheck, unit-test, build, manual | Done |
| AC3 | SCEN-12.2.1 | TEST-12.2.3 | tests/executable_upstream_rs_runner.rs | install, typecheck, unit-test, build, manual | Done |

## 8. Risks

- npx/upstream startup may hang; use pinned package, timeout, and `PROMPTFOO_DISABLE_UPDATE=true`.
- Windows shell quoting differs; command spec must avoid shell string concatenation.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Build**: adapter §Commands Build
- **Manual**: inspect one persisted fixture artifact tree.

## 10. Completion Notes

- **完成日期**：2026-05-30
- **改动文件**：
  - `src/compatibility/executor.rs`
  - `src/compatibility/harness.rs`
  - `src/compatibility/mod.rs`
  - `tests/executable_upstream_rs_runner.rs`
  - `docs/compatibility/harness.md`
  - `docs/specs/tasks/task-12.2-executable-upstream-rs-runner.md`
- **commit 列表**：
  - `7ec06be` `docs(spec): task-12.2 进入实施 (Status: Ready → In Progress)`
  - `85ca640` `test(compatibility-harness): 加 SCEN-12.2.1 的 3 个 RED 测试`
  - `eae648a` `feat(compatibility-harness): 实现 executable upstream-rs runner`
- **§9 Verification 结果**：
  - install: PASS — `s2v_verify_full "install typecheck unit-test build"` 中 `cargo fetch` 通过。
  - typecheck: PASS — `cargo check --workspace` 通过。
  - unit-test: PASS — `cargo test --workspace` 通过，包含 `tests/executable_upstream_rs_runner.rs` 的 TEST-12.2.1~TEST-12.2.3。
  - build: PASS — `cargo build --workspace` 通过。
  - manual: PASS — 已检查 persisted artifact tree：`metadata.json`、`raw/upstream.json`、`raw/rs.json`、`normalized/upstream.json`、`normalized/rs.json`、`diff/findings.json`、`work/{upstream,rs}/fixture.{json,yaml}` 均生成；样例 run `TEST-12.2.2` baseline=`promptfoo@0.121.13`，upstream/rs exit_code=0，`PROMPTFOO_DISABLE_UPDATE=true`。非交互 helper 的 full run 仅因 `/dev/tty` manual 确认失败，机械 keys 已单独全绿。
- **剩余风险 / 未做项**：当前测试用本地 test binary 证明无 shell 执行、artifact 持久化和策略约束；真实 npm `promptfoo@0.121.13` 在 CI/发布机上的安装缓存、网络可用性和长耗时 fixture 仍需 task 12.3/Phase 15 gate 继续覆盖。
- **下游 task 影响**：task 12.3 可复用 `PersistedRunArtifacts`、`diff/findings.json` 与 `docs/compatibility/harness.md` 的 artifact contract 接入 stable release gate。
