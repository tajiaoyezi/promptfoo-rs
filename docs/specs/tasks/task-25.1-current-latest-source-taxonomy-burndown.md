# Task 25.1: current-latest-source-taxonomy-burndown

**Status**: In Progress
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

- [ ] **AC1** (Phase 24 §9): representative source rows from the 318-row unclassified set are classified into non-`unclassified` categories with level, implementation status, owner, evidence kind, and evidence reference.
- [ ] **AC2** (ADR-009): classification preserves P0/P1/P2 semantics: eval/config/cache/provider/script runtime rows remain P0 or explicit blockers when fixture evidence is required; viewer/schema/docs/integration/cloud-share long-tail rows become P1/P2 only with reasoned evidence.
- [ ] **AC3** (ADR-011): running the current-latest source inventory script against the locked target writes source inventory and matrix artifacts with `unclassified_rows=[]` and no missing evidence rows.
- [ ] **AC4** (task 24.4 claim contract): current-latest golden corpus and quality gate no longer report taxonomy `Unclassified` blockers, but `perfect_refactor_claim_allowed` remains false while real local/external/publication blockers remain.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-25.1.1 | TEST-25.1.1 | tests/current_latest_source_taxonomy_burndown.rs | install, typecheck, unit-test, integration, build | Ready |
| AC2 | SCEN-25.1.1 | TEST-25.1.2 | tests/current_latest_source_taxonomy_burndown.rs | install, lint, typecheck, unit-test, coverage, build | Ready |
| AC3 | SCEN-25.1.1 | TEST-25.1.3 | tests/current_latest_source_taxonomy_burndown.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Ready |
| AC4 | SCEN-25.1.1 | TEST-25.1.4 | tests/current_latest_source_taxonomy_burndown.rs | install, lint, typecheck, unit-test, runtime-smoke, build | Ready |

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

待实施。
