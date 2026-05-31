# Task 16.3: source-extracted-inventory-real-upstream-smoke

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 16 — parity-proof-hardening
**Dependencies**: task-12.2-executable-upstream-rs-runner, task-12.3-golden-diff-ci-release-gate, task-16.2-measured-release-gate-reports

## 1. Background

复审发现 item-level inventory 当前主要来自人工 seed，真实 upstream runner 的测试仍可用本地 test binary 代替 upstream，导致“完整重构”证据不足以证明 promptfoo 0.121.13 的已文档化能力被完整枚举且至少一个 P0 smoke 使用真实 upstream npm artifact。依据 PRD §Compatibility Matrix / §Compatibility Harness Design、ADR-007、ADR-009、task-11.2 §10、task-12.2 §10。

## 2. Goal

增加 source-extracted inventory evidence 与真实 `promptfoo@0.121.13` smoke gate，使 release gate 能区分真实 upstream artifacts 与本地替身。

## 3. Scope

### In Scope

- `compatibility/inventory/`
- `compatibility/artifacts/`
- `src/compatibility/inventory.rs`
- `src/compatibility/harness.rs`
- `scripts/release/integration.sh`
- `scripts/release/runtime-smoke.sh`
- `tests/real_upstream_smoke_gate.rs`
- `tests/item_level_capability_inventory.rs`

### Out Of Scope

- 不纳入 moving upstream `main`；兼容目标仍是冻结 baseline `promptfoo@0.121.13`。
- 不需要真实 provider API key；smoke fixture 必须使用 mock/local provider。
- 不解决所有长尾 P1/P2 行为，只要求被发现、分类、登记 reason 或 blocker。

## 4. Users / Actors

- Release maintainer：需要确认 stable gate 没有把本地替身当作 upstream。
- Contributor：需要 source-extracted inventory 输出指出新增或遗漏能力。
- 企业迁移 reviewer：需要看到 upstream artifact、rs artifact、normalized diff 的真实来源。

## 5. Behavior Contract

Inventory extraction 必须读取冻结 upstream package/source evidence 并输出 discovered counts、source references 和 extraction timestamp；real upstream smoke 必须调用 `promptfoo@0.121.13` 与当前 `promptfoo-rs` 二进制执行同一 P0 fixture，持久化 metadata，且 metadata 明确记录 upstream command、npm version、baseline commit 和 rs binary path。缺少真实 upstream artifact 时 stable gate fail closed。

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/compatibility/baseline.lock.md
- docs/specs/tasks/task-11.2-item-level-capability-inventory.md
- docs/specs/tasks/task-12.2-executable-upstream-rs-runner.md
- docs/specs/tasks/task-12.3-golden-diff-ci-release-gate.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`std::process::Command`、`serde_json`、`tempfile`、内部模块 `compatibility::inventory`、`compatibility::harness`、`compatibility::release_gate`。
- Tooling commands：adapter §Commands Install / Typecheck / Unit Test / Integration tests / Runtime smoke / Build。

### 5.3 函数签名

- `extract_inventory_from_upstream_source(source: &UpstreamSource) -> Result<ExtractedInventoryEvidence, InventoryError>`
- `RealUpstreamSmokeRunner::run(fixture: &FixtureManifest) -> Result<RealUpstreamSmokeArtifacts, HarnessError>`
- `validate_real_upstream_evidence(artifacts: &RealUpstreamSmokeArtifacts) -> Result<(), GateError>`

## 6. Acceptance Criteria

- [x] **AC1** (ADR-009): source-extracted inventory evidence records discovered command/provider/assertion/redteam/output/config/API counts and source references for frozen upstream.
- [x] **AC2** (ADR-007): at least one P0 fixture is executed by real `promptfoo@0.121.13` and current `promptfoo-rs` binary, with upstream/rs/normalized/diff artifacts persisted.
- [x] **AC3** (PRD §Success Metrics): stable release gate blocks when real upstream smoke artifacts are missing, stale, or produced by a local test binary substitute.
- [x] **AC4** (PRD §Compatibility Matrix): newly discovered inventory items either receive matrix rows with verification evidence or release-blocking missing-row evidence.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-16.3.1 | TEST-16.3.1 | tests/real_upstream_smoke_gate.rs | install, typecheck, unit-test, integration, build, runtime-smoke | Done |
| AC2 | SCEN-16.3.1 | TEST-16.3.2 | tests/real_upstream_smoke_gate.rs | install, typecheck, unit-test, integration, build, runtime-smoke | Done |
| AC3 | SCEN-16.3.1 | TEST-16.3.3 | tests/real_upstream_smoke_gate.rs | install, typecheck, unit-test, integration, build, runtime-smoke | Done |
| AC4 | SCEN-16.3.1 | TEST-16.3.4 | tests/item_level_capability_inventory.rs | install, typecheck, unit-test, integration, build, runtime-smoke | Done |

## 8. Risks

- npm registry 或 upstream package 下载失败会让 release gate 红；这是公共依赖可用性问题，不需要密钥，不能用本地替身伪造通过。
- Upstream 0.121.13 CLI 可能对 mock fixture 格式要求更严格；fixture 必须保持最小、local-only、无真实 API key。
- Source extraction 规则可能需要迭代；新增 item 必须进入 matrix 或 blocker，而不是被过滤掉。

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：2026-05-31
- **改动文件**：
  - `scripts/release/source-inventory-evidence.sh`
  - `scripts/release/real-upstream-smoke.sh`
  - `scripts/release/integration.sh`
  - `scripts/release/runtime-smoke.sh`
  - `tests/real_upstream_smoke_gate.rs`
  - `docs/s2v-adapter.md`
  - `docs/specs/phases/phase-16-parity-proof-hardening.md`
  - `docs/specs/tasks/task-16.3-source-extracted-inventory-real-upstream-smoke.md`
- **commit 列表**：
  - `babe2f8` `docs(spec): task-16.3 进入实施 (Status: Ready → In Progress)`
  - `75766c9` `test(compatibility): 加 SCEN-16.3.1 的 real upstream smoke RED 测试`
  - `ab1c6a7` `feat(compatibility): 加入真实 upstream smoke 与 source inventory evidence`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-16.3.1 ~ TEST-16.3.4 通过。
  - integration: PASS — `bash scripts/release/integration.sh` 纳入 `real_upstream_smoke_gate` contract tests 并通过。
  - build: PASS — helper 执行 `cargo build --workspace`、viewer/npm `pnpm build` 通过。
  - runtime-smoke: PASS — `bash scripts/release/runtime-smoke.sh` 执行 source inventory evidence 与真实 upstream smoke；`source-inventory-evidence.json` status=ready，inventory item count=44，package file count=536；`real-upstream-smoke/latest/metadata.json` status=ready，upstream/rs exit_code=0，diff findings=`[]`。
- **剩余风险 / 未做项**：source evidence 当前使用 npm pack 文件列表加 inventory source references，不是完整 TypeScript AST 级 semantic extractor；若后续要求自动枚举全部 provider/assertion/redteam 插件，需要新增更深的 extractor task。真实 upstream smoke 依赖公网 npm registry，可用性失败会 fail closed，不需要密钥。
- **下游 task 影响**：Phase 16 已满足收尾条件；runtime/release gate 现在包含 source inventory evidence 与真实 upstream smoke artifacts。
