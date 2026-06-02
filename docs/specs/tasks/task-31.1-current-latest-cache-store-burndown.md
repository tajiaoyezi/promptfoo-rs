# Task 31.1: current-latest-cache-store-burndown

**Status**: In Progress
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 31 - current-latest-cache-store-burndown
**Dependencies**: task-3.2-cache-resume-retry, task-5.1-result-store-schema, task-13.2-eval-output-cache-parity, task-24.4-current-latest-exhaustive-quality-gate, task-30.1-current-latest-prompt-processing-burndown

## 1. Background

Phase 30 left tracked-lock phase-smoke artifacts at 52 P0 golden blockers, including 9 `cache-store` blockers. Earlier tasks already proved cache key derivation, resume from partial JSONL/SQLite state, retry/backoff persistence, streaming JSONL append, SQLite query schema, and eval output/cache parity. This task applies only those existing fixtures to current-latest cache/database/storage rows while preserving eval deletion as a P0 blocker and helper-only rows as P1 snapshot evidence. 依据 PRD §Technical Approach / §Compatibility Matrix、ADR-003、ADR-009、ADR-011、task 3.2、task 5.1、task 13.2、Phase 30 §9。

## 2. Goal

Reduce current-latest cache-store P0 golden blockers from 9 generic blockers to 1 explicit blocker while preserving 6 fixture-covered rows as P0 native fixture evidence and 2 helper rows as P1 snapshot evidence.

## 3. Scope

### In Scope

- `src/compatibility/inventory.rs`
- `scripts/release/current-latest-source-inventory.sh`
- `tests/current_latest_cache_store_burndown.rs`
- `target/release-gates/current-latest-source-inventory.json`
- `target/release-gates/current-latest-matrix.json`
- `target/release-gates/current-latest-golden-corpus.json`
- `target/release-gates/current-latest-quality.json`
- `docs/compatibility/matrix.md`
- `docs/s2v-adapter.md`
- `docs/prds/promptfoo-rs.prd.md`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不实现 upstream eval deletion semantics 或 deletion cascade parity。
- 不实现云端/远程 storage provider、外部数据库服务、private service 或真实账号流程。
- 不解决 provider external-authority、script-bridge runtime discovery、eval-runner adaptive/rate-limit、prompt processor、config external、current-target 或 publication blockers。
- 不承诺“无任何潜在 bug”；claim 仍受 ADR-011 的 evidence boundary 约束。

## 4. Users / Actors

- Prompt maintainer: needs current-latest cache/store rows to reuse existing deterministic local persistence fixture evidence when sufficient.
- Release reviewer: needs cache-store blocker reduction to be item-level and not hide eval deletion gaps.
- Data/storage reviewer: needs local filesystem and SQLite evidence separated from cloud, deletion, or lifecycle authority claims.

## 5. Behavior Contract

Current-latest `category=cache-store` rows must be classified by stable id and source path. Fixture-covered rows must use `level=P0`, `implementation_status=native`, `verification_owner=cache-resume-store`, `evidence_kind=fixture`, and `evidence_reference=fixture:<stable-id>`. P1 helper rows must use `level=P1`, `implementation_status=later`, `verification_owner=cache-resume-store`, `evidence_kind=snapshot`, and `evidence_reference=snapshot:<stable-id>`. Unproven eval deletion rows must remain `level=P0`, `implementation_status=blocked`, `verification_owner=cache-resume-store`, `evidence_kind=blocker`, and `evidence_reference=blocker:<stable-id>`. Rust and shell artifact generation must use equivalent cache-store rules.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/phases/phase-24-current-latest-perfect-refactor.md
- docs/specs/phases/phase-30-current-latest-prompt-processing-burndown.md
- docs/specs/tasks/task-3.2-cache-resume-retry.md
- docs/specs/tasks/task-5.1-result-store-schema.md
- docs/specs/tasks/task-13.2-eval-output-cache-parity.md
- docs/specs/tasks/task-24.4-current-latest-exhaustive-quality-gate.md
- docs/specs/tasks/task-30.1-current-latest-prompt-processing-burndown.md
- docs/compatibility/current-latest.lock.md
- docs/compatibility/matrix.md
- docs/decisions/adr-003-streaming-jsonl-sqlite-store.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/cache-resume-store.feature
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `promptfoo_rs::compatibility::inventory::{extract_current_latest_inventory, CurrentLatestTargetLock}`, `serde_json::Value`, `std::path::Path`, `std::process::Command`.
- Tooling commands: `bash scripts/release/current-latest-source-inventory.sh`, `bash scripts/release/current-latest-golden-corpus.sh`, `bash scripts/release/current-latest-quality-gate.sh`, `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `is_current_latest_cache_store_fixture(stable_id: &str, file: &str) -> bool`
- `is_current_latest_cache_store_snapshot(file: &str) -> bool`
- `current_latest_cache_store_blocker_reason(stable_id: &str, file: &str) -> String`
- Shell contract: `isCurrentLatestCacheStoreFixture(id, file)`, `isCurrentLatestCacheStoreSnapshot(file)`, `currentLatestCacheStoreBlockerReason(id, file)`, `metadata(category, id, file)`.

## 6. Acceptance Criteria

- [ ] **AC1** (ADR-003 / task 3.2 / task 5.1 / task 13.2): 6 current-latest cache/database/storage rows have P0 native fixture evidence and do not produce golden release blockers.
- [ ] **AC2** (ADR-009): 2 current-latest database testing/signal helper rows are P1 snapshot evidence and do not weaken P0 persistence semantics.
- [ ] **AC3** (ADR-003 / ADR-011): current-latest eval deletion remains an explicit P0 cache-store blocker.
- [ ] **AC4** (ADR-009): Rust extractor and shell extractor emit equivalent cache-store classification, evidence kind, evidence reference, and owner values.
- [ ] **AC5** (ADR-011 / task 24.4): source/matrix/golden/quality artifacts show cache-store blockers reduced to 1 and total blockers reduced to 44 under the tracked-lock Phase 30 smoke target, while perfect-refactor completion remains false.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-31.1.1 | TEST-31.1.1 | tests/current_latest_cache_store_burndown.rs | install, lint, typecheck, unit-test, integration, build | Ready |
| AC2 | SCEN-31.1.1 | TEST-31.1.2 | tests/current_latest_cache_store_burndown.rs | install, typecheck, unit-test, coverage, build | Ready |
| AC3 | SCEN-31.1.1 | TEST-31.1.3 | tests/current_latest_cache_store_burndown.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Ready |
| AC4 | SCEN-31.1.1 | TEST-31.1.4 | tests/current_latest_cache_store_burndown.rs | install, lint, typecheck, unit-test, coverage, build | Ready |
| AC5 | SCEN-31.1.1 | TEST-31.1.5 | tests/current_latest_cache_store_burndown.rs | install, lint, typecheck, unit-test, integration, e2e, runtime-smoke, build | Ready |

## 8. Risks

- Marking all cache-store rows native would hide eval deletion and helper lifecycle gaps that are not covered by task 3.2/5.1/13.2.
- Downgrading helper rows to P1 must stay traceable as snapshot evidence; it is not a native implementation claim.
- Reducing cache-store blockers does not prove provider external authority, script bridge runtime discovery, eval-runner adaptive/rate-limit, prompt processor parity, publication, or current-target readiness.

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

- **完成日期**：待实施
- **改动文件**：待实施
- **commit 列表**：待实施
- **§9 Verification 结果**：待实施
- **剩余风险 / 未做项**：待实施
- **下游 task 影响**：待实施
