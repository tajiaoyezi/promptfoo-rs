# Phase 42: current-latest-2ca16c-head-refresh

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Refresh the current-latest target lock after 2026-06-04 live upstream observation showed GitHub default branch HEAD moved to `2ca16c59b64e0afca10533de0f817c0d24eba20a` while npm latest and GitHub latest release remain `promptfoo@0.121.14` / `refs/tags/0.121.14`. 依据 PRD §Current Latest Rebaseline Addendum、ADR-007、ADR-009、ADR-011、task 41.1、docs/audits/promptfoo-current-stage-full-verification-audit-2026-06-03.md，以及 2026-06-04 live observations from `npm view`, GitHub latest release API, and `git ls-remote`.

## 2. Business Value

The tracked current-latest lock still records GitHub HEAD `9d7d810c2118c63cb537bf05ea2d34c12bd22066`. Since the user requires a refactor based on the original project current latest, default-branch drift must be refreshed as a new immutable target packet before any downstream current-latest claim can be audited.

## 3. Scope / Modules

`compatibility/inventory/current-latest-target.json`, `docs/compatibility/current-latest.lock.md`, `tests/current_latest_github_head_drift_refresh.rs`, `tests/current_latest_target_drift_refresh.rs`, `scripts/release/current-latest-target-lock.sh`, `scripts/release/runtime-smoke.sh`, `target/release-gates/current-latest-*.json`, `docs/compatibility/matrix.md`, `docs/prds/promptfoo-rs.prd.md`, `docs/s2v-adapter.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 42.1 | current-latest-2ca16c-head-refresh | ../tasks/task-42.1-current-latest-2ca16c-head-refresh.md | Ready | 刷新 current-latest target lock 的 GitHub default branch HEAD 到 `2ca16c...` 并重跑 downstream gates |

## 5. Dependencies

Depends on Phase 41 target refresh, ADR-007, ADR-009, ADR-011, and live upstream observations captured on 2026-06-04. No provider credentials, private service account, legal/brand approval, or publication credentials are required.

## 6. Phase Acceptance Criteria

- [ ] `current-latest-target.json` and `current-latest.lock.md` record npm latest `0.121.14`, npm gitHead `7a48c5fce614bee617efbb3b7fc93d404c75b628`, GitHub latest release `refs/tags/0.121.14`, and GitHub default branch HEAD `2ca16c59b64e0afca10533de0f817c0d24eba20a`.
- [ ] Runtime smoke consumes the refreshed default branch target and regenerates current-latest source inventory, matrix, golden corpus, quality, release-candidate, and unblock packet artifacts.
- [ ] Downstream gates remain fail-closed with `current_latest_claim_allowed=false` and `perfect_refactor_claim_allowed=false` unless all refreshed evidence and external authority gates are ready.
- [ ] Any new source/matrix/golden blocker introduced by the refreshed HEAD is surfaced explicitly and not removed from compatibility evidence.

## 7. Phase Risks

- GitHub default branch can move again during the task; evidence must record the immutable observed HEAD and avoid floating completion claims.
- New default-branch source can introduce new local blockers; those must be surfaced by runtime smoke and not hidden.
- Npm latest and GitHub release staying on `0.121.14` does not make the repository HEAD safe to ignore.

## 8. Definition of Done

Task 42.1 spec is Done, phase §6 smoke passes with task §9 verification, tracked current-latest lock artifacts record GitHub HEAD `2ca16c59b64e0afca10533de0f817c0d24eba20a`, downstream gate artifact summaries are reflected in docs, and the repository is clean and pushed.

## 9. Phase Completion Notes

- **完成日期**：<TBD-after-impl>
- **Phase smoke**：<TBD-after-impl>
- **Artifact evidence**：<TBD-after-impl>
- **Remaining boundaries**：External provider/config authority, current-target policy, publication credentials/approval, and bug-free/perfect-refactor claim boundaries remain unresolved without external decisions or formal waivers.
