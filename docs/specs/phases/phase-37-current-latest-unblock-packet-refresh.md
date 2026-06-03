# Phase 37: current-latest-unblock-packet-refresh

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Refresh the perfect-refactor unblock packet so it reflects the Phase 36 current-latest evidence instead of the older frozen/source-accounting blocker ledger. The packet must list the exact current-latest golden blockers, current-target blocker, and publication decisions required before any perfect-refactor claim can become true. 依据 PRD §Current Latest Rebaseline Addendum / §Compatibility Matrix、ADR-008、ADR-009、ADR-011、task 22.1、task 24.4、task 36.1。

## 2. Business Value

After Phase 36, local script-bridge blockers are gone and `current-latest-golden-corpus.blocker_count=23` with groups `config=7, provider=16`. The existing `perfect-refactor-unblock-packet.json` still reports older Phase 22 counts such as `source_p0_accounting_blocker_count=22` and `external_authority_blocker_count=21`. That stale handoff can mislead the user about what remains. This phase makes the decision packet match the current-latest goal while preserving fail-closed external authority and publication boundaries.

## 3. Scope / Modules

`scripts/release/perfect-refactor-unblock-packet.sh`, `scripts/release/runtime-smoke.sh`, `target/release-gates/perfect-refactor-unblock-packet.json`, `tests/current_latest_unblock_packet.rs`, `docs/compatibility/matrix.md`, `docs/prds/promptfoo-rs.prd.md`, `docs/s2v-adapter.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 37.1 | current-latest-unblock-packet-refresh | ../tasks/task-37.1-current-latest-unblock-packet-refresh.md | Done | 让 perfect-refactor unblock packet 以 current-latest blockers 为权威决策源 |

## 5. Dependencies

Depends on Phase 24 current-latest quality gate, Phase 36 Ruby bridge burndown, task 22.1 unblock packet gate, task 24.4 current-latest quality gate, ADR-008, ADR-009, and ADR-011. This phase requires no real provider credentials, legal/brand approval, or publication credentials.

## 6. Phase Acceptance Criteria

- [x] `perfect-refactor-unblock-packet.json` declares a current-latest target scope and records `current_latest_golden_blocker_count=23` under the tracked Phase 36 evidence.
- [x] Every current-latest golden blocker appears as a non-auto-resolvable decision item sourced from current-latest artifacts, including config and provider external-authority rows.
- [x] Current-target and publication authority decisions remain explicit and fail-closed; local dry-run or fixture evidence cannot mark them ready.
- [x] Runtime smoke exposes the refreshed packet without changing `perfect_refactor_claim_allowed=false`.
- [x] The packet no longer lets older frozen/source-accounting counts obscure the current-latest remaining work.

## 7. Phase Risks

- A refreshed packet may be misread as a waiver; status must remain `blocked` and all decision items must keep `auto_resolvable=false`.
- Current-latest provider/config blockers must not be deduplicated against older frozen external-authority artifacts if that would hide a current-latest row.
- Publication evidence still requires real credentials, legal/brand approval, and external URL/digest evidence.

## 8. Definition of Done

Task 37.1 spec is Done, phase §6 smoke passes with `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`, current-latest packet fields match `current-latest-golden-corpus.json`, and the repository is clean and pushed.

## 9. Phase Completion Notes

- **完成日期**：2026-06-03
- **Phase smoke**：PASS - `s2v_preflight_phase "docs/specs/phases/phase-37-current-latest-unblock-packet-refresh.md"` 通过；`s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"` 完整通过。第一次 phase smoke 在 15 分钟工具超时处留下 `real-upstream-corpus` 子进程；已按 systematic-debugging 查明为 upstream corpus 子进程卡住而非 packet gate 失败，清理本次验证进程树后用更长 timeout 复跑通过。
- **Artifact evidence**：`perfect-refactor-unblock-packet.target_scope=current-latest`、`status=blocked`、`perfect_refactor_claim_allowed=false`、`current_latest_golden_blocker_count=23`、`current_latest_external_authority_blocker_count=23`、`current_latest_required_decision_count=30`，分组为 `config=7, provider=16, current-latest target=1, publication=6`；`current-latest-quality.status=ready-with-blockers`、`local_current_latest_ready=false`、`perfect_refactor_claim_allowed=false`、quality blockers=4；`release-candidate.perfect_refactor_unblock_packet.required_user_decision_count=30`。
- **保留边界**：Phase 37 只刷新 current-latest blocker handoff artifact；config/provider external authority、current-target、publication authority、真实凭据/账号/私有服务、法律/品牌确认、公开发布 URL/digest 和不可证明的“无潜在 bug”声明仍保持阻塞。
