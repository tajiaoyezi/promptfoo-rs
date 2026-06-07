# Phase 48: current-latest-0.121.15-target-refresh

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Refresh the current-latest target lock after 2026-06-07 live upstream observation showed npm latest and GitHub latest release moved to `promptfoo@0.121.15` / `4805856060d026521794d4e69decb938155580ad` while GitHub default branch HEAD moved to `c54a30668ad8319d76c20ae96e6680ad6c51a2c6`. The tracked lock still records `0.121.14` / `2ca16c59b64e0afca10533de0f817c0d24eba20a`. 依据 PRD §Current Latest Rebaseline Addendum、ADR-007、ADR-009、ADR-011、task 42.1、task 47.1。

## 2. Business Value

Downstream source inventory, matrix, golden corpus, quality, and runtime-smoke evidence must target the same live upstream packet. Without this refresh, compatibility gates audit stale `0.121.14` / `2ca16c59` evidence while upstream has already published `0.121.15` and advanced default-branch HEAD.

## 3. Scope / Modules

`compatibility/inventory/current-latest-target.json`, `docs/compatibility/current-latest.lock.md`, `tests/current_latest_0_121_15_target_refresh.rs`, `tests/current_latest_target_drift_refresh.rs`, `tests/current_latest_github_head_drift_refresh.rs`, `scripts/release/current-latest-target-lock.sh`, `scripts/release/runtime-smoke.sh`, `target/release-gates/current-latest-*.json`, `docs/compatibility/matrix.md`, `docs/prds/promptfoo-rs.prd.md`, `docs/s2v-adapter.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 48.1 | current-latest-0.121.15-target-refresh | ../tasks/task-48.1-current-latest-0.121.15-target-refresh.md | Done | 刷新 current-latest target lock 到 `0.121.15` + GitHub HEAD `c54a3066` 并重跑 downstream gates |

## 5. Dependencies

Depends on Phase 47 evaluator in-memory store fixture burndown, Phase 42 HEAD refresh, ADR-007, ADR-009, and ADR-011. No provider credentials, private service account, legal/brand approval, or publication credentials are required.

## 6. Phase Acceptance Criteria

- [x] `current-latest-target.json` and `current-latest.lock.md` record npm latest `0.121.15`, npm gitHead `4805856060d026521794d4e69decb938155580ad`, GitHub latest release `refs/tags/0.121.15`, and GitHub default branch HEAD `c54a30668ad8319d76c20ae96e6680ad6c51a2c6`.
- [x] Runtime smoke consumes the refreshed target and regenerates current-latest source inventory, matrix, golden corpus, quality, release-candidate, and unblock packet artifacts.
- [x] Downstream gates remain fail-closed with `current_latest_claim_allowed=false` and `perfect_refactor_claim_allowed=false` unless all refreshed evidence and external authority gates are ready.
- [x] Prior local fixture evidence from Phase 40/47 survives re-extraction; no new unclassified rows were introduced.

## 7. Phase Risks

- GitHub default branch and npm latest can move again during the task; evidence must record the immutable observed packet.
- New `0.121.15` source can introduce new unclassified rows or blockers; those must be surfaced by runtime smoke.
- Target refresh does not waive external authority, publication, or perfect-refactor claim boundaries.

## 8. Definition of Done

Task 48.1 spec is Done, phase §6 smoke passes with task §9 verification, tracked current-latest lock artifacts record npm `0.121.15` and GitHub HEAD `c54a30668ad8319d76c20ae96e6680ad6c51a2c6`, downstream gate summaries are reflected in docs, and the repository is clean and pushed via PR.

## 9. Phase Completion Notes

- **完成日期**：2026-06-07
- **Phase smoke**：PASS — task 48.1 full §9 verification；runtime smoke regenerated gates with `github.default_branch_head=c54a30668ad8319d76c20ae96e6680ad6c51a2c6`, source inventory `status=ready`, matrix `status=ready`, golden `blocker_count=24`, `perfect_refactor_claim_allowed=false`
- **Artifact evidence**：tracked lock `promptfoo@0.121.15` / HEAD `c54a3066`; evaluator runtime and in-memory store rows remain native fixture evidence; unblock packet `required_user_decision_count=31`
- **Remaining boundaries**：External provider/config authority, current-target policy, publication credentials/approval, and bug-free/perfect-refactor claim boundaries remain unresolved without external decisions or formal waivers.