# Task 17.4: longtail-provider-assertion-redteam-classification

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 17 — deep-upstream-parity-proof
**Dependencies**: task-17.1-frozen-source-inventory-extractor, task-17.3-real-p0-golden-corpus-runner, task-14.1-provider-assertion-inventory-parity, task-14.2-redteam-plugin-strategy-parity

## 1. Background

当前 P0 provider/assertion/redteam core 已有覆盖，但 frozen upstream source surface 包含大量 provider modules、assertion modules、redteam plugin/strategy files。审计指出本地长尾仍是 curated subset，不能证明完整重构。依据 PRD §Compatibility Matrix、ADR-001、ADR-005、ADR-009、docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md。

## 2. Goal

对 task 17.1 提取出的 provider/assertion/redteam 长尾 rows 逐项分类为 native、bridge、unsupported、later 或 blocked；P0 实现或提供 fixture/blocker，P1 给 snapshot，P2/later/unsupported 给 reason 与用户可见错误。

## 3. Scope

### In Scope

- `compatibility/inventory/`
- `compatibility/matrix/`
- `compatibility/fixtures/providers/`
- `compatibility/fixtures/assertions/`
- `compatibility/fixtures/redteam/`
- `src/providers/`
- `src/assertions/`
- `src/redteam/`
- `src/script_bridge/`
- `src/compatibility/provider_assertion.rs`
- `src/redteam/registry.rs`
- `tests/longtail_provider_assertion_redteam_classification.rs`

### Out Of Scope

- 不调用真实 paid provider APIs；使用 mock servers、recorded responses、bridge contract 或 explicit blocker。
- 不把品牌/法律敏感云能力伪装成本地实现；需要法律/品牌确认的项标 blocked/needs-review。
- 不改变 Phase 17.2 的 CLI command/flag parser；本 task 只处理 provider/assertion/redteam capability runtime/classification。

## 4. Users / Actors

- AI application developer：需要知道某个 provider/assertion/redteam item 是否能迁移。
- Security reviewer：需要确认 script bridge 与 unsupported paths 不泄露 secret、不默认执行代码。
- Release maintainer：需要 P0 missing fixture/blocker 和 P2 missing reason 均为 0。

## 5. Behavior Contract

每个 source-extracted provider/assertion/redteam item 必须有 matrix row、classification、reason、owner、verification evidence。Native/bridge rows 必须有可运行 fixture 或 snapshot；unsupported/later/blocked rows 必须有用户可见错误函数，错误中包含 item id、classification、reason、docs/compatibility link，且不得静默跳过。

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/tasks/task-14.1-provider-assertion-inventory-parity.md
- docs/specs/tasks/task-14.2-redteam-plugin-strategy-parity.md
- docs/specs/tasks/task-17.1-frozen-source-inventory-extractor.md
- docs/specs/tasks/task-17.3-real-p0-golden-corpus-runner.md
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-005-explicit-script-authorization.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`serde_json`、`reqwest`、`regex`、内部模块 `providers`、`assertions`、`redteam`、`script_bridge`、`compatibility::matrix`、`compatibility::provider_assertion`。
- Tooling commands：adapter §Commands Install / Typecheck / Unit Test / Integration tests / Coverage / Runtime smoke / Build。

### 5.3 函数签名

- `classify_provider_item(item: &InventoryItem, policy: &ParityPolicy) -> ProviderClassification`
- `classify_assertion_item(item: &InventoryItem, policy: &ParityPolicy) -> AssertionClassification`
- `classify_redteam_item(item: &InventoryItem, policy: &ParityPolicy) -> RedteamClassification`
- `compatibility_gap_error(item_id: &str, class: GapClass, reason: &str) -> CompatibilityError`
- `validate_longtail_classification(matrix: &CapabilityMatrix, fixtures: &FixtureCorpus) -> LongtailParityReport`

## 6. Acceptance Criteria

- [x] **AC1** (ADR-009): all source-extracted provider/assertion/redteam rows have native/bridge/unsupported/later/blocked classification, owner, verification, reason where needed, and no unresolved/missing-reason rows.
- [x] **AC2** (PRD §Compatibility Matrix): P0 provider/assertion/redteam rows have real fixture, snapshot, or release-blocking blocker; P1 rows have snapshot plans; P2/later/unsupported rows have reason and target.
- [x] **AC3** (ADR-005): script-backed JS/TS/Python/Shell/Ruby provider/assertion rows remain default-deny, allowlisted when enabled, timed out, redacted, and covered by bridge fixtures.
- [x] **AC4** (PRD §User Flow / Redteam): invoking an unsupported/later/blocked provider/assertion/redteam item returns a stable user-visible error with item id, classification, reason, no secret leakage, and nonzero exit where applicable.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-17.4.1 | TEST-17.4.1 | tests/longtail_provider_assertion_redteam_classification.rs | install, typecheck, unit-test, integration, coverage, build | Done |
| AC2 | SCEN-17.4.1 | TEST-17.4.2 | tests/longtail_provider_assertion_redteam_classification.rs | install, typecheck, unit-test, integration, coverage, build | Done |
| AC3 | SCEN-17.4.1 | TEST-17.4.3 | tests/longtail_provider_assertion_redteam_classification.rs | install, typecheck, unit-test, integration, runtime-smoke, build | Done |
| AC4 | SCEN-17.4.1 | TEST-17.4.4 | tests/longtail_provider_assertion_redteam_classification.rs | install, typecheck, unit-test, integration, e2e, build | Done |

## 8. Risks

- Some long-tail providers require private services or legal/brand confirmation; classify as blocked with minimal user decision instead of guessing.
- Over-broad `later` classification could hide required P0 behavior; classification policy must derive from PRD P0/P1/P2 rules and source evidence.
- Bridge execution expands security surface; preserve default deny and explicit authorization.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **E2E tests**: adapter §Commands E2E tests
- **Coverage**: adapter §Commands Coverage
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：2026-05-31
- **改动文件**：
  - `tests/longtail_provider_assertion_redteam_classification.rs`
  - `src/compatibility/provider_assertion.rs`
  - `src/compatibility/matrix.rs`
  - `compatibility/inventory/upstream-items.json`
  - `scripts/release/longtail-classification.sh`
  - `scripts/release/integration.sh`
  - `scripts/release/runtime-smoke.sh`
  - `tests/provider_assertion_inventory_parity.rs`
  - `tests/redteam_plugin_strategy_parity.rs`
  - `docs/compatibility/matrix.md`
  - `docs/s2v-adapter.md`
  - `docs/specs/phases/phase-17-deep-upstream-parity-proof.md`
  - `docs/specs/tasks/task-17.4-longtail-provider-assertion-redteam-classification.md`
- **commit 列表**：
  - `353c385` `test(compatibility): add SCEN-17.4.1 longtail classification RED tests`
  - `ce28501` `feat(compatibility): classify source-extracted longtail parity rows`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-17.4.1 ~ TEST-17.4.4 通过。
  - integration: PASS — `bash scripts/release/integration.sh` 通过，包含 `longtail_provider_assertion_redteam_classification`。
  - e2e: PASS — `bash scripts/release/e2e.sh` 通过。
  - build: PASS — helper 执行 `cargo build --workspace`、viewer/npm `pnpm build` 通过。
  - coverage: PASS — `bash scripts/release/coverage.sh` 通过；`s2v_coverage_threshold_guard` 通过。
  - runtime-smoke: PASS — `bash scripts/release/runtime-smoke.sh` 通过；`target/release-gates/longtail-classification.json` status=`ready-with-blockers`，source_extracted_item_count=433，tracked_longtail_item_count=433，missing_tracked_rows=0，unresolved_rows=0，missing_reason_rows=0，p0_release_blocker_count=37。
- **剩余风险 / 未做项**：37 个 source-extracted P0 provider module rows 当前以 explicit release-blocking blocker 表示；这是 AC2 允许的保守分类，不声称这些 per-file provider modules 已有独立 native fixture。后续 task 17.5 必须把该 blocker evidence 纳入 release readiness，不得把需要凭据/更细 fixture 的长尾项伪装成已公开发布可用。
- **下游 task 影响**：task 17.5 可读取 `target/release-gates/longtail-classification.json`、更新 release readiness evidence，并把 longtail ready-with-blockers 与真实发布凭据 blocker 一起呈现。
