# Task 50.1: golden-corpus-authority-waiver-alignment

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 50 — product-baseline-golden-burndown
**Dependencies**: task-49.2-v1-waiver-unblock-packet-alignment, task-48.1-current-latest-0.121.15-target-refresh

## 1. Background

Frozen baseline golden corpus reports `blocker_count=24` (config=7, provider=17). Every row is an external authority P0 blocker. Phase 44/49 applied formal v1 waivers in `docs/compatibility/authority-decisions.json` and aligned external-authority/unblock gates, but `current-latest-golden-corpus.sh` still treats all diff findings as active release blockers.

## 2. Goal

Split golden corpus blockers into audit vs active counts by consuming resolved authority manifest rows, update downstream quality/runtime gates to use `active_blocker_count`, and close frozen-baseline golden burndown without deleting audit evidence.

## 3. Scope

### In Scope

- `scripts/release/current-latest-golden-corpus.sh`
- `scripts/release/current-latest-quality-gate.sh`
- `scripts/release/runtime-smoke.sh`
- `src/compatibility/harness.rs`
- `src/release.rs` (golden active blocker field on quality inputs)
- `tests/product_baseline_golden_burndown.rs`
- `tests/current_latest_golden_corpus.rs` (extend waiver case)
- `docs/compatibility/matrix.md`
- `docs/release.md`
- `docs/s2v-adapter.md`

### Out Of Scope

- New authority waivers beyond existing v1 manifest.
- Upstream drift refresh or npm/HEAD target re-lock.
- Renaming `current-latest-*` gate namespaces (cosmetic follow-up).

## 4. Users / Actors

- Maintainer: needs golden burndown closure on frozen baseline without losing audit trail.
- Release reviewer: needs `active_blocker_count` separate from audit `blocker_count`.

## 5. Behavior Contract

Golden corpus generation loads `docs/compatibility/authority-decisions.json` (via `product-baseline-gate-lib.cjs` in shell; Rust equivalent in harness). For each release blocker whose `capability` matches a manifest `item_id` with `decision_state` in `{evidence-provided, waived-with-boundary}`, classify as waived (not active). Emit `blocker_count` (audit total), `active_blocker_count`, `waived_blocker_count`, `release_blockers` (audit), `active_blockers`, `waived_blockers`. `perfect_refactor_claim_allowed` and corpus `status=ready` use active blockers only. Quality gate and runtime smoke pass `active_blocker_count` to `CURRENT_LATEST_GOLDEN_CORPUS_BLOCKER_COUNT`.

### 5.1 Required Reading

- docs/decisions/adr-012-product-independence-baseline-freeze.md
- docs/compatibility/authority-decisions.json
- docs/compatibility/v1-release-authority-policy.md
- docs/specs/tasks/task-49.2-v1-waiver-unblock-packet-alignment.md
- target/release-gates/current-latest-golden-corpus.json (frozen baseline evidence)

### 5.2 Imports

- Shell: `product-baseline-gate-lib.cjs` (`loadAuthorityDecisions`, `isResolvedAuthorityDecision`)
- Rust: `promptfoo_rs::compatibility::harness::{build_current_latest_golden_corpus, GoldenCorpusReport}`
- Rust: `promptfoo_rs::release::{build_current_latest_quality_report, CurrentLatestQualityInputs}`

### 5.3 函数签名

- `isResolvedAuthorityDecision(itemId, byId) -> boolean` (gate lib; reused by golden corpus script)
- `build_current_latest_golden_corpus(matrix_path, fixtures_root, artifacts_root) -> GoldenCorpusReport` (adds active/waived blocker fields)
- `build_current_latest_quality_report(inputs: CurrentLatestQualityInputs) -> CurrentLatestQualityReport` (uses `golden_corpus_active_blocker_count` when set)

## 6. Acceptance Criteria

- [x] **AC1** (ADR-012): frozen baseline golden corpus `active_blocker_count=0`, `waived_blocker_count=24`, audit `blocker_count=24`.
- [x] **AC2** (v1 policy): waived rows reference manifest `item_id`; unresolved local P0 blockers still count as active.
- [x] **AC3** (ADR-009): quality gate clears golden-corpus blocker when `active_blocker_count=0`; audit count remains in golden artifact.
- [x] **AC4** (release.md): runtime smoke regenerates gates; `perfect_refactor_claim_allowed` follows all gates (golden active zero + authority/publication readiness).

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-50.1.1 | TEST-50.1.1 | tests/product_baseline_golden_burndown.rs | install, typecheck, unit-test, runtime-smoke, build | Done |
| AC2 | SCEN-50.1.1 | TEST-50.1.2 | tests/current_latest_golden_corpus.rs | install, typecheck, unit-test, build | Done |
| AC3 | SCEN-50.1.1 | TEST-50.1.3 | tests/product_baseline_golden_burndown.rs | install, typecheck, unit-test, integration, build | Done |
| AC4 | SCEN-50.1.1 | TEST-50.1.4 | tests/product_baseline_golden_burndown.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Done |

## 8. Risks

- Matching blockers by `capability` must not waive unclassified/local bug findings lacking manifest rows.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Lint**: adapter §Commands Lint
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **E2E tests**: adapter §Commands E2E tests
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：2026-06-07
- **改动文件**：
  - scripts/release/current-latest-golden-corpus.sh（authority waiver split + status=ready when active zero）
  - scripts/release/current-latest-quality-gate.sh（active_blocker_count + audit in reason）
  - scripts/release/runtime-smoke.sh（pass active golden blocker count）
  - scripts/release/perfect-refactor-unblock-packet.sh（prefer active_blockers）
  - src/compatibility/harness.rs（Rust parity for active/waived split）
  - src/release.rs（golden_corpus_active_blocker_count on quality inputs）
  - tests/product_baseline_golden_burndown.rs（新增）
  - tests/current_latest_golden_corpus.rs（TEST-50.1.2）
  - tests/current_latest_quality_gate.rs（新字段）
  - docs/compatibility/matrix.md、docs/release.md、docs/s2v-adapter.md
- **commit 列表**：
  - d74bce2 feat(compatibility-gates): Phase 50 golden corpus authority waiver burndown
- **§9 Verification 结果**：
  - install: ✅（cargo 已编译）
  - lint: skipped（adapter N/A）
  - typecheck: ✅（cargo test 编译通过）
  - unit-test: 9 passed（product_baseline_golden_burndown + current_latest_golden_corpus Phase 50 用例）
  - integration: ✅（runtime-smoke 内含 integration 路径）
  - e2e: ✅（runtime-smoke 通过）
  - build: ✅（cargo build release 在 runtime-smoke 内）
  - runtime-smoke: ✅（exit 0；golden active_blocker_count=0，quality blockers=0）
- **剩余风险 / 未做项**：audit `blocker_count=24` 仍可见；不代表 live cloud/provider parity。Corpus scale gate（P0 fixture ≥250）仍独立控制 golden `perfect_refactor_claim_allowed`。
- **下游 task 影响**：无