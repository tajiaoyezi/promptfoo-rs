# Phase 41: current-latest-github-head-drift-refresh

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Refresh the current-latest target lock after live upstream observation showed GitHub default branch HEAD moved to `9d7d810c2118c63cb537bf05ea2d34c12bd22066` while npm latest and GitHub latest release remain `promptfoo@0.121.14` / `refs/tags/0.121.14`. 依据 PRD §Current Latest Rebaseline Addendum、ADR-007、ADR-009、ADR-011、task 38.1、task 40.1，以及 2026-06-03 live observations from `npm view`, GitHub latest release API, and `git ls-remote`.

## 2. Business Value

The tracked current-latest lock still records GitHub HEAD `4d22e57f5f9b4c7cdde494f00558d9afde8b4975`. Since the user requires a refactor based on the original project current latest, default-branch drift must be explicit and auditable even when npm latest has not published a new package version.

## 3. Scope / Modules

`compatibility/inventory/current-latest-target.json`, `docs/compatibility/current-latest.lock.md`, `tests/current_latest_github_head_drift_refresh.rs`, `tests/current_latest_target_drift_refresh.rs`, `scripts/release/current-latest-target-lock.sh`, `scripts/release/runtime-smoke.sh`, `target/release-gates/current-latest-*.json`, `docs/compatibility/matrix.md`, `docs/prds/promptfoo-rs.prd.md`, `docs/s2v-adapter.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 41.1 | current-latest-github-head-drift-refresh | ../tasks/task-41.1-current-latest-github-head-drift-refresh.md | Done | 刷新 current-latest target lock 的 GitHub default branch HEAD 到 `9d7d810...` 并重跑 downstream gates |

## 5. Dependencies

Depends on Phase 38 target refresh, Phase 39/40 evaluator runtime burndown, ADR-007, ADR-009, ADR-011, and live upstream observations captured on 2026-06-03. No provider credentials, private service account, legal/brand approval, or publication credentials are required.

## 6. Phase Acceptance Criteria

- [x] `current-latest-target.json` and `current-latest.lock.md` record npm latest `0.121.14`, npm gitHead `7a48c5fce614bee617efbb3b7fc93d404c75b628`, GitHub latest release `refs/tags/0.121.14`, and GitHub default branch HEAD `9d7d810c2118c63cb537bf05ea2d34c12bd22066`.
- [x] Runtime smoke consumes the refreshed default branch target and regenerates current-latest source inventory, matrix, golden corpus, quality, release-candidate, and unblock packet artifacts.
- [x] Downstream gates remain fail-closed with `current_latest_claim_allowed=false` and `perfect_refactor_claim_allowed=false`.
- [x] Remaining blockers stay explicit as external authority, current-target, publication authority, or formal waiver decisions.

## 7. Phase Risks

- GitHub default branch can move again during the task; evidence must record the immutable observed HEAD and avoid floating completion claims.
- New default-branch source can introduce new local blockers; those must be surfaced by runtime smoke and not hidden.
- Npm latest and GitHub release staying on `0.121.14` does not make the repository HEAD safe to ignore.

## 8. Definition of Done

Task 41.1 spec is Done, phase §6 smoke passes with task §9 verification, tracked current-latest lock artifacts record GitHub HEAD `9d7d810c2118c63cb537bf05ea2d34c12bd22066`, downstream gate artifact summaries are reflected in docs, and the repository is clean and pushed.

## 9. Phase Completion Notes

- **完成日期**：2026-06-03
- **Phase smoke**：PASS - task 41.1 full §9 verification passed with install, lint, typecheck, unit-test, integration, e2e, coverage, build, runtime-smoke, plus `s2v_coverage_threshold_guard`. `s2v_preflight_phase "docs/specs/phases/phase-41-current-latest-github-head-drift-refresh.md"` passed after this completion update.
- **Artifact evidence**：runtime smoke regenerated `target/release-gates/current-latest-target.json` with `status=locked-with-drift`, `github.default_branch_head=9d7d810c2118c63cb537bf05ea2d34c12bd22066`, and `current_latest_claim_allowed=false`. Current-latest source inventory and matrix both report `status=ready` and `unclassified_rows=[]`; current-latest golden reports `blocker_count=24`; quality reports `blocker_count=4`; release candidate keeps `publication_ready=credential-blocked`; unblock packet reports `required_user_decision_count=31`.
- **Remaining boundaries**：Phase 41 only refreshes moving upstream target evidence. External provider/config authority, current-target policy, publication credentials/approval, and bug-free/perfect-refactor claim boundaries remain unresolved without external decisions or formal waivers.
