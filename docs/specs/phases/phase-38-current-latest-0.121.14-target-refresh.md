# Phase 38: current-latest-0.121.14-target-refresh

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Refresh the current-latest target lock after the upstream promptfoo package moved from `0.121.13` to `0.121.14`, and fix the target-lock parser path where npm latest and GitHub latest release point to the same tag. 依据 PRD §Current Latest Rebaseline Addendum、ADR-007、ADR-009、ADR-011，以及 2026-06-03 live observations from `npm view promptfoo ...`, GitHub latest release API, and `git ls-remote`.

## 2. Business Value

The repository currently has complete S2V task history for the earlier current-latest target, but the live upstream target changed. This phase prevents perfect-refactor evidence from silently staying pinned to stale `0.121.13` artifacts while still keeping external-authority, publication, and impossible "no potential bugs" claims blocked.

## 3. Scope / Modules

`scripts/release/current-latest-target-lock.sh`, `scripts/release/runtime-smoke.sh`, `src/compatibility/inventory.rs`, `compatibility/inventory/current-latest-target.json`, `docs/compatibility/current-latest.lock.md`, `tests/current_latest_target_drift_refresh.rs`, `docs/compatibility/matrix.md`, `docs/prds/promptfoo-rs.prd.md`, `docs/s2v-adapter.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 38.1 | current-latest-0.121.14-target-refresh | ../tasks/task-38.1-current-latest-0.121.14-target-refresh.md | Done | 刷新 current-latest target lock 到 promptfoo 0.121.14 并修复 npm tag 与 latest release 同 ref 的解析 |

## 5. Dependencies

Depends on Phase 24 current-latest target lock, Phase 37 unblock packet refresh, ADR-007, ADR-009, ADR-011, and live upstream observations captured on 2026-06-03. No provider credentials, private service account, legal/brand approval, or publication credentials are required.

## 6. Phase Acceptance Criteria

- [x] `current-latest-target.json` and `current-latest.lock.md` record `promptfoo@0.121.14`, npm gitHead `7a48c5fce614bee617efbb3b7fc93d404c75b628`, and GitHub HEAD `4d22e57f5f9b4c7cdde494f00558d9afde8b4975`.
- [x] The target-lock parser accepts the valid case where `npm_tag_ref == latest_release_ref == refs/tags/0.121.14` and records both commits instead of failing closed.
- [x] Runtime smoke and downstream current-latest artifacts consume the refreshed target without setting `perfect_refactor_claim_allowed=true`.
- [x] Remaining blockers stay explicit: config/provider external authority, current-target consistency, publication authority, and unprovable zero-bug claims still require external evidence or formal waiver.

## 7. Phase Risks

- Current latest can move again during implementation; task evidence must record the observed immutable packet and avoid floating `latest` claims.
- Updating target evidence can invalidate older current-latest blocker counts; packet and quality artifacts must remain blocked unless all downstream evidence refers to the same refreshed target.
- A core GitHub latest release can share the npm tag ref; parser logic must not use mutually exclusive `else if` branches for ref accounting.

## 8. Definition of Done

Task 38.1 spec is Done, phase §6 smoke passes with the task §9 verification plan, tracked current-latest lock artifacts point to `0.121.14`, parser and shell tests cover same-ref release accounting, and the repository is clean and pushed.

## 9. Phase Completion Notes

- **完成日期**：2026-06-03
- **Phase smoke**：PASS - `s2v_preflight_phase "docs/specs/phases/phase-38-current-latest-0.121.14-target-refresh.md"` passed after this completion update; task 38.1 full §9 verification passed with install, lint, typecheck, unit-test, integration, e2e, coverage, build, runtime-smoke, plus `s2v_coverage_threshold_guard`.
- **Artifact evidence**：runtime smoke regenerated `target/release-gates/current-latest-target.json` with `status=locked-with-drift`, npm `0.121.14`, npm gitHead `7a48c5fce614bee617efbb3b7fc93d404c75b628`, GitHub HEAD `4d22e57f5f9b4c7cdde494f00558d9afde8b4975`, and `current_latest_claim_allowed=false`. `target/release-gates/current-latest-golden-corpus.json` reports `fixture_case_count=94`, `p0_total=94`, `p1_total=2367`, `p2_total=1417`, and `blocker_count=25`. `target/release-gates/current-latest-quality.json` reports `status=ready-with-blockers`, `local_current_latest_ready=false`, and `blocker_count=6`. `target/release-gates/release-candidate.json` keeps `publication_ready=credential-blocked`.
- **Remaining boundaries**：Phase 38 only resolves target drift and same-ref parsing. It does not supply external provider/account evidence, legal/brand authorization, publication credentials, or a provable zero-bug guarantee.
