# Phase 49: product-baseline-v1-gate-alignment

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Align release gates, unblock packet, and policy artifacts with ADR-012 product-baseline freeze and maintainer v1 authority policy so formal waivers and published GitHub Releases evidence are consumed at runtime without reintroducing upstream drift refresh or `perfect_refactor_claim_allowed=true` without real burndown. 依据 ADR-012、docs/compatibility/v1-release-authority-policy.md、Phase 44 authority closure、Phase 48 product baseline lock。

## 2. Business Value

After strategic alignment (ADR-012), gates still behaved like a live upstream chase: `current_upstream_rebaseline_required=true`, unblock packet listed 31 unresolved decisions despite resolved authority manifests, and publication gate treated v1-deferred channels as hard blockers. This phase makes gate outputs honest for the independent product line while keeping perfect-refactor claims fail-closed.

## 3. Scope / Modules

`scripts/release/product-baseline-gate-lib.cjs`, `scripts/release/current-upstream-policy.sh`, `scripts/release/perfect-refactor-unblock-packet.sh`, `scripts/release/publication-evidence.sh`, `scripts/release/current-latest-quality-gate.sh`, `scripts/release/runtime-smoke.sh`, `src/compatibility/inventory.rs`, `src/release.rs`, `tests/product_baseline_v1_gate_alignment.rs`, `BLOCKED-task-22.1-perfect-refactor-external-authority.md`, `docs/release.md`, `docs/s2v-adapter.md`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 49.1 | product-baseline-policy-gate-alignment | ../tasks/task-49.1-product-baseline-policy-gate-alignment.md | Done | ADR-012 product-baseline policy + rebaseline flag alignment |
| 49.2 | v1-waiver-unblock-packet-alignment | ../tasks/task-49.2-v1-waiver-unblock-packet-alignment.md | Done | Consume authority/publication waivers in unblock + external gates |

## 5. Dependencies

Depends on Phase 48 product baseline lock, Phase 44 v1 authority policy, ADR-012 strategic alignment merge.

## 6. Phase Acceptance Criteria

- [x] `current-upstream-policy.json` defaults to `target_mode=product-baseline` with `product_baseline_frozen=true` and `current_upstream_rebaseline_required=false`.
- [x] `perfect-refactor-unblock-packet.json` filters decision items resolved via `authority-decisions.json` or v1-deferred publication rows; `required_user_decision_count` reflects only unresolved items.
- [x] `external-authority-blockers.json` reports `active_blocker_count=0` when all manifest rows are waived or evidence-provided; `publication-evidence-gate.json` reports `v1_scope_ready=true`.
- [x] `perfect_refactor_claim_allowed` remains `false`; `local_stable_allowed` remains `true`; BLOCKED-task-22.1 narrative narrowed to frozen-baseline / v1 boundaries.

## 7. Phase Risks

- Waivers must remain visible; gates must not imply live parity or full multi-channel publication.
- Golden corpus fixture blockers remain for burndown audit even when authority decisions are waived.

## 8. Definition of Done

Tasks 49.1 and 49.2 are Done, phase §6 smoke passes, gate artifacts align with ADR-012 + v1 policy, repository clean and merged via PR.

## 9. Phase Completion Notes

- **完成日期**：2026-06-07
- **Phase smoke**：`bash scripts/release/runtime-smoke.sh` ✅；`bash scripts/release/integration.sh` ✅
- **Artifact evidence**：`target/release-gates/current-upstream-policy.json`（product-baseline frozen）；`target/release-gates/perfect-refactor-unblock-packet.json`（required_user_decision_count=0）；`target/release-gates/authority-decisions-gate.json`（perfect_refactor_decision_ready=true）
- **Remaining boundaries**：Golden fixture burndown on frozen baseline; no perfect-refactor claim without full gate agreement.