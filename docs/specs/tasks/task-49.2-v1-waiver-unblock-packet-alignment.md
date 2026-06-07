# Task 49.2: v1-waiver-unblock-packet-alignment

**Status**: In Progress
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 49 — product-baseline-v1-gate-alignment
**Dependencies**: task-49.1-product-baseline-policy-gate-alignment, task-44.1-external-authority-evidence-application, task-44.2-public-publication-evidence-application

## 1. Background

Phase 44 applied v1 formal waivers in `authority-decisions.json` and `publication-evidence.json`, but runtime gates still counted all golden/provider/config rows and deferred publication channels as active blockers and unresolved user decisions.

## 2. Goal

Wire authority and publication manifests into external-authority, publication-evidence, unblock-packet, and quality gates so resolved waivers reduce `required_user_decision_count` and `active_blocker_count` without weakening perfect-refactor fail-closed semantics.

## 3. Scope

### In Scope

- `scripts/release/product-baseline-gate-lib.cjs`
- `scripts/release/perfect-refactor-unblock-packet.sh`
- `scripts/release/publication-evidence.sh`
- `scripts/release/current-latest-quality-gate.sh`
- `scripts/release/runtime-smoke.sh`
- `BLOCKED-task-22.1-perfect-refactor-external-authority.md`
- `docs/release.md`

### Out Of Scope

- New waivers beyond existing v1 policy manifests.
- Publishing additional channels beyond GitHub Releases evidence already recorded.

## 4. Users / Actors

- Maintainer: needs unblock packet to show 0 unresolved authority decisions when manifests are complete.
- Users: must still see golden fixture blockers and no perfect-refactor claim.

## 5. Behavior Contract

Decision items with `decision_state` in `{evidence-provided, waived-with-boundary}` in `authority-decisions.json` are excluded from unblock packet `decision_items`. Publication rows with `v1_deferred=true` or `publication_state=published` do not count as active publication blockers for v1 scope. `external-authority-blockers.json` exposes `active_blocker_count` separate from audit `blocker_count`.

### 5.1 Required Reading

- docs/compatibility/v1-release-authority-policy.md
- docs/compatibility/authority-decisions.json
- docs/compatibility/publication-evidence.json
- docs/specs/tasks/task-43.1-authority-decision-manifest-gate.md

### 5.2 Imports

- Shell gate lib: `isResolvedAuthorityDecision`, `v1PublicationScopeReady`
- Manifests: `docs/compatibility/authority-decisions.json`, `docs/compatibility/publication-evidence.json`

### 5.3 函数签名

- `isResolvedAuthorityDecision(itemId, byId) -> boolean`
- `v1PublicationScopeReady(requiredChannels, byChannel) -> boolean`

## 6. Acceptance Criteria

- [ ] **AC1** (v1 policy): authority manifest rows resolve unblock decisions; `authority-decisions-gate` stays `perfect_refactor_decision_ready=true`.
- [ ] **AC2** (v1 policy): publication gate `v1_scope_ready=true` with GitHub Releases published + five deferred channels.
- [ ] **AC3** (ADR-009): golden `blocker_count` remains visible; `perfect_refactor_claim_allowed=false`.
- [ ] **AC4** (BLOCKED-task-22.1): blocker file updated to frozen-baseline / v1 boundary closure; no rebaseline backlog narrative.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-49.2.1 | TEST-49.2.1 | tests/product_baseline_v1_gate_alignment.rs | install, typecheck, unit-test, build | In Progress |
| AC2 | SCEN-49.2.1 | TEST-49.2.3 | tests/product_baseline_v1_gate_alignment.rs | install, typecheck, unit-test, runtime-smoke, build | In Progress |
| AC3 | SCEN-49.2.1 | TEST-49.1.2 | tests/product_baseline_v1_gate_alignment.rs | install, typecheck, unit-test, coverage, runtime-smoke, build | In Progress |
| AC4 | SCEN-49.2.1 | TEST-49.2.1 | tests/product_baseline_v1_gate_alignment.rs | install, lint, typecheck, unit-test, integration, build | In Progress |

## 8. Risks

- Over-filtering could hide unresolved decisions if manifest rows drift from unblock packet item IDs.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Lint**: adapter §Commands Lint
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **Coverage**: adapter §Commands Coverage
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：`<TBD-after-impl>`
- **改动文件**：`<TBD-after-impl>`
- **commit 列表**：`<TBD-after-impl>`
- **§9 Verification 结果**：`<TBD-after-impl>`
- **剩余风险**：`<TBD-after-impl>`
- **下游 task 影响**：无