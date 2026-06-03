# Phase 38: current-latest-0.121.14-target-refresh

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Refresh the current-latest target lock after the upstream promptfoo package moved from `0.121.13` to `0.121.14`, and fix the target-lock parser path where npm latest and GitHub latest release point to the same tag. 依据 PRD §Current Latest Rebaseline Addendum、ADR-007、ADR-009、ADR-011，以及 2026-06-03 live observations from `npm view promptfoo ...`, GitHub latest release API, and `git ls-remote`.

## 2. Business Value

The repository currently has complete S2V task history for the earlier current-latest target, but the live upstream target changed. This phase prevents perfect-refactor evidence from silently staying pinned to stale `0.121.13` artifacts while still keeping external-authority, publication, and impossible "no potential bugs" claims blocked.

## 3. Scope / Modules

`scripts/release/current-latest-target-lock.sh`, `src/compatibility/inventory.rs`, `compatibility/inventory/current-latest-target.json`, `docs/compatibility/current-latest.lock.md`, `tests/current_latest_target_drift_refresh.rs`, `docs/compatibility/matrix.md`, `docs/prds/promptfoo-rs.prd.md`, `docs/s2v-adapter.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 38.1 | current-latest-0.121.14-target-refresh | ../tasks/task-38.1-current-latest-0.121.14-target-refresh.md | Ready | 刷新 current-latest target lock 到 promptfoo 0.121.14 并修复 npm tag 与 latest release 同 ref 的解析 |

## 5. Dependencies

Depends on Phase 24 current-latest target lock, Phase 37 unblock packet refresh, ADR-007, ADR-009, ADR-011, and live upstream observations captured on 2026-06-03. No provider credentials, private service account, legal/brand approval, or publication credentials are required.

## 6. Phase Acceptance Criteria

- [ ] `current-latest-target.json` and `current-latest.lock.md` record `promptfoo@0.121.14`, npm gitHead `7a48c5fce614bee617efbb3b7fc93d404c75b628`, and GitHub HEAD `4d22e57f5f9b4c7cdde494f00558d9afde8b4975`.
- [ ] The target-lock parser accepts the valid case where `npm_tag_ref == latest_release_ref == refs/tags/0.121.14` and records both commits instead of failing closed.
- [ ] Runtime smoke and downstream current-latest artifacts consume the refreshed target without setting `perfect_refactor_claim_allowed=true`.
- [ ] Remaining blockers stay explicit: config/provider external authority, current-target consistency, publication authority, and unprovable zero-bug claims still require external evidence or formal waiver.

## 7. Phase Risks

- Current latest can move again during implementation; task evidence must record the observed immutable packet and avoid floating `latest` claims.
- Updating target evidence can invalidate older current-latest blocker counts; packet and quality artifacts must remain blocked unless all downstream evidence refers to the same refreshed target.
- A core GitHub latest release can share the npm tag ref; parser logic must not use mutually exclusive `else if` branches for ref accounting.

## 8. Definition of Done

Task 38.1 spec is Done, phase §6 smoke passes with the task §9 verification plan, tracked current-latest lock artifacts point to `0.121.14`, parser and shell tests cover same-ref release accounting, and the repository is clean and pushed.
