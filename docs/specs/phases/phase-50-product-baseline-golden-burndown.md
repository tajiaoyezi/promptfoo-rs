# Phase 50: product-baseline-golden-burndown

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Close the frozen product-baseline golden corpus burndown by consuming v1 authority waivers in `current-latest-golden-corpus.json` so waived external config/provider P0 rows no longer count as active release blockers while the audit trail (`blocker_count=24`) remains visible. 依据 ADR-012、Phase 48 product baseline lock、Phase 49 gate/waiver alignment、`docs/compatibility/authority-decisions.json`。

## 2. Business Value

Phase 49 aligned unblock packet and external-authority gates with v1 waivers, but golden corpus still reported `blocker_count=24` and blocked `perfect_refactor_claim_allowed`. The remaining 24 rows are exclusively external config (7) and longtail provider authority (17) items already waived for v1 local-first scope. This phase makes golden gate outputs honest: active blockers drop to zero without hiding audit evidence or reintroducing upstream drift refresh.

## 3. Scope / Modules

`scripts/release/current-latest-golden-corpus.sh`, `scripts/release/current-latest-quality-gate.sh`, `scripts/release/runtime-smoke.sh`, `scripts/release/perfect-refactor-unblock-packet.sh`, `src/compatibility/harness.rs`, `src/release.rs`, `tests/product_baseline_golden_burndown.rs`, `tests/current_latest_golden_corpus.rs`, `docs/compatibility/matrix.md`, `docs/release.md`, `docs/s2v-adapter.md`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 50.1 | golden-corpus-authority-waiver-alignment | ../tasks/task-50.1-golden-corpus-authority-waiver-alignment.md | Done | Consume authority waivers in golden corpus; expose active vs audit blocker counts |

## 5. Dependencies

Depends on Phase 49 v1 gate alignment, Phase 48 `promptfoo@0.121.15` lock, ADR-012 product baseline freeze, and `docs/compatibility/authority-decisions.json` with all 24 golden blocker item ids resolved.

## 6. Phase Acceptance Criteria

- [x] `current-latest-golden-corpus.json` records `active_blocker_count=0`, `waived_blocker_count=24`, `blocker_count=24` (audit), and `active_blockers=[]` on frozen baseline.
- [x] `current-latest-quality.json` uses golden `active_blocker_count` (not audit `blocker_count`) and clears golden-corpus quality blockers when waivers cover all rows.
- [x] Runtime smoke regenerates gates with golden active blockers at zero; `perfect_refactor_claim_allowed` may become true only when all other declared gates also agree.
- [x] Rust `build_current_latest_golden_corpus` and shell golden corpus script agree on active/audit waiver split.

## 7. Phase Risks

- Over-filtering could hide unresolved local P0 blockers if waiver matching is too broad.
- Setting `perfect_refactor_claim_allowed=true` must not imply live provider/cloud parity or multi-channel publication beyond v1 policy.

## 8. Definition of Done

Task 50.1 is Done, phase §6 smoke passes, golden active blockers are zero on frozen baseline, audit trail preserved, repository clean and merged via PR.

## 9. Phase Completion Notes

- **完成日期**：2026-06-07
- **Phase smoke**：`bash scripts/release/runtime-smoke.sh` ✅
- **Artifact evidence**：`current-latest-golden-corpus.json` `active_blocker_count=0` / `waived_blocker_count=24` / audit `blocker_count=24` / `status=ready`；`current-latest-quality.json` golden-corpus blockers cleared
- **Remaining boundaries**：Audit trail preserved; no live cloud/provider parity claim; corpus scale threshold still independent of waiver burndown.