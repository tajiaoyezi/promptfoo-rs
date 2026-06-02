# Task 25.1: current-latest-source-taxonomy-burndown

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 25 — current-latest-source-taxonomy-burndown
**Dependencies**: task-24.4-current-latest-exhaustive-quality-gate

## 1. Background

Phase 24 completed current-latest target locking, source inventory extraction, golden corpus expansion, and quality claim gating, but `target/release-gates/current-latest-quality.json` remains `ready-with-blockers` because source inventory and matrix contain 318 `unclassified:*` rows. These rows are not safe to ignore: every current-latest source row must be classified before the project can reason about true P0/P1/P2 parity work. 依据 Phase 24 §9 artifact evidence、PRD §Current Latest Rebaseline Addendum / §Compatibility Matrix、ADR-009、ADR-011。

## 2. Goal

Replace the current-latest extractor's catch-all `unclassified` category with a deterministic taxonomy for known promptfoo source families, so source inventory and matrix have zero unknown rows while preserving real local, external authority, and publication blockers.

## 3. Scope

### In Scope

- `src/compatibility/inventory.rs`
- `scripts/release/current-latest-source-inventory.sh`
- `tests/current_latest_source_taxonomy_burndown.rs`
- `scripts/release/current-latest-golden-corpus.sh`
- `scripts/release/current-latest-quality-gate.sh`
- `scripts/release/runtime-smoke.sh`
- `target/release-gates/current-latest-source-inventory.json`
- `target/release-gates/current-latest-matrix.json`
- `target/release-gates/current-latest-golden-corpus.json`
- `target/release-gates/current-latest-quality.json`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不把 P0 native/bridge fixture blocker 改名为完成。
- 不提供真实 provider credentials、账号、私有服务、法律/品牌授权或 publication credentials。
- 不承诺“无任何潜在 bug”；claim 仍受 task 24.4 质量声明边界约束。

## 4. Users / Actors

- Compatibility maintainer: needs every current-latest source row to have a stable taxonomy and owner.
- Release reviewer: needs the quality gate to distinguish taxonomy cleanup from real parity completion.
- Future implementer: needs remaining blockers split into local implementation, external authority, and publication work.

## 5. Behavior Contract

The current-latest source inventory extractor must classify every TypeScript/JavaScript source row under the locked upstream source tree into a known capability category. The taxonomy must include current-latest families observed in Phase 24 artifacts: eval runtime, cache/store, prompt processing, scheduler, matchers/assertion support, redteam support/providers, script bridge, viewer/server, schema/model/contracts, integrations, cloud/share, telemetry, migration/update support, and generic runtime support. Each row must receive level, implementation_status, verification_owner, evidence_kind, evidence_reference, and blocker_reason according to PRD §Compatibility Matrix and ADR-009. The matrix and golden corpus must preserve remaining P0 blockers as explicit blockers rather than erasing them.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/phases/phase-24-current-latest-perfect-refactor.md
- docs/specs/tasks/task-24.2-current-latest-source-inventory-reextract.md
- docs/specs/tasks/task-24.3-current-latest-full-function-golden-corpus.md
- docs/specs/tasks/task-24.4-current-latest-exhaustive-quality-gate.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `promptfoo_rs::compatibility::inventory::{extract_current_latest_inventory, reconcile_current_latest_matrix, write_current_latest_inventory_artifacts, CurrentLatestTargetLock}`, `serde_json::Value`, `std::collections::BTreeSet`, `std::path::Path`, `std::process::Command`.
- Tooling commands: `bash scripts/release/current-latest-source-inventory.sh`, `bash scripts/release/current-latest-golden-corpus.sh`, `bash scripts/release/current-latest-quality-gate.sh`, `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `current_latest_file_categories(file: &str) -> Vec<&'static str>`
- `current_latest_default_metadata(category: &str, stable_id: &str, file: &str) -> (String, String, String, String, Option<String>)`
- Shell contract: `bash scripts/release/current-latest-source-inventory.sh`

## 6. Acceptance Criteria

- [x] **AC1** (Phase 24 §9): representative source rows from the 318-row unclassified set are classified into non-`unclassified` categories with level, implementation status, owner, evidence kind, and evidence reference.
- [x] **AC2** (ADR-009): classification preserves P0/P1/P2 semantics: eval/config/cache/provider/script runtime rows remain P0 or explicit blockers when fixture evidence is required; viewer/schema/docs/integration/cloud-share long-tail rows become P1/P2 only with reasoned evidence.
- [x] **AC3** (ADR-011): running the current-latest source inventory script against the locked target writes source inventory and matrix artifacts with `unclassified_rows=[]` and no missing evidence rows.
- [x] **AC4** (task 24.4 claim contract): current-latest golden corpus and quality gate no longer report taxonomy `Unclassified` blockers, but `perfect_refactor_claim_allowed` remains false while real local/external/publication blockers remain.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-25.1.1 | TEST-25.1.1 | tests/current_latest_source_taxonomy_burndown.rs | install, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-25.1.1 | TEST-25.1.2 | tests/current_latest_source_taxonomy_burndown.rs | install, lint, typecheck, unit-test, coverage, build | Done |
| AC3 | SCEN-25.1.1 | TEST-25.1.3 | tests/current_latest_source_taxonomy_burndown.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Done |
| AC4 | SCEN-25.1.1 | TEST-25.1.4 | tests/current_latest_source_taxonomy_burndown.rs | install, lint, typecheck, unit-test, runtime-smoke, build | Done |

## 8. Risks

- If a rule is too broad, it can hide a core P0 surface behind P1/P2 evidence. Tests must include P0-sensitive families such as `src/evaluate.ts`, `src/cache.ts`, `src/scheduler/**`, script bridges, and config-adjacent prompt processing.
- Some upstream paths may move in a future current-latest target. This task uses the locked Phase 24 target; future drift must enter a new S2V task.
- Removing `unclassified` rows can reduce blocker counts without completing behavior parity. AC4 requires the perfect-refactor claim to stay false while real blockers remain.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Lint**: adapter §Commands Lint
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **E2E tests**: adapter §Commands E2E tests
- **Coverage**: adapter §Commands Coverage
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：2026-06-02
- **改动文件**：
  - `docs/prds/promptfoo-rs.prd.md`
  - `docs/s2v-adapter.md`
  - `docs/specs/phases/phase-25-current-latest-source-taxonomy-burndown.md`
  - `docs/specs/tasks/task-25.1-current-latest-source-taxonomy-burndown.md`
  - `test/features/perfect-refactor-parity.feature`
  - `tests/current_latest_source_taxonomy_burndown.rs`
  - `src/compatibility/inventory.rs`
  - `scripts/release/current-latest-source-inventory.sh`
- **commit 列表**：
  - `ab061d7` `docs(spec): add phase 25 current latest taxonomy burndown`
  - `513a06e` `docs(spec): task-25.1 enters implementation`
  - `24bb12c` `test(compatibility): add current latest source taxonomy RED tests`
  - `6ab69c2` `feat(compatibility): classify current latest source taxonomy`
  - `1f734a1` `refactor(compatibility): format current latest taxonomy helpers`
  - `8fd0750` `test(compatibility): add current latest tracing taxonomy RED`
  - `0b00aa8` `feat(compatibility): classify current latest tracing taxonomy`
  - 本次 docs 回填提交：`docs(spec): complete task 25.1 current latest taxonomy burndown`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - lint: PASS — helper 执行 `bash scripts/release/lint.sh` 通过。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-25.1.1 ~ TEST-25.1.4 通过。
  - integration: PASS — helper 执行 `bash scripts/release/integration.sh` 通过，包含 current-latest taxonomy tests。
  - e2e: PASS — helper 执行 `bash scripts/release/e2e.sh` 通过。
  - build: PASS — helper 执行 `cargo build --workspace`、viewer/npm `pnpm build` 通过。
  - coverage: PASS — helper 执行 `bash scripts/release/coverage.sh` 通过；`s2v_coverage_threshold_guard` 通过。
  - runtime-smoke: PASS — helper 执行 `bash scripts/release/runtime-smoke.sh` 通过；真实 artifact 复核显示 `current-latest-source-inventory.json` status=`ready`、rows=3916、unclassified_rows=0、rows_missing_evidence=0，`current-latest-matrix.json` status=`ready`、unclassified_rows=0、rows_missing_evidence=0，`current-latest-golden-corpus.json` status=`ready-with-blockers`、blocker_count=161、无 taxonomy `Unclassified` finding，`current-latest-quality.json` status=`ready-with-blockers`、local_stable_allowed=true、local_current_latest_ready=false、perfect_refactor_claim_allowed=false。
- **剩余风险 / 未做项**：本 task 只消除了 source/matrix unknown taxonomy blocker；仍有 161 个 current-latest P0 golden blockers、current-target claim boundary、21 个 external authority blockers，以及 publication `credential-blocked`。这些不能被声明为“完美重构完成”。
- **下游 task 影响**：后续 task/phase 可直接面向 161 个 P0 golden blockers 做 native/bridge fixture burndown，并继续保留 external authority 与 publication authority 决策包。
