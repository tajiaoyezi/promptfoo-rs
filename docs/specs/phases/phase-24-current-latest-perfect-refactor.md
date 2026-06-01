# Phase 24: current-latest-perfect-refactor

**Status**: In Progress
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

把用户澄清的目标转成可执行 S2V 链路：以原始 promptfoo 当前最新观测为 immutable target packet，重新抽取完整功能 inventory，扩展 golden corpus 与大规模质量 gate，并只在无已知 release-blocking 缺陷时允许 current-latest perfect-refactor claim。依据用户 2026-06-01 澄清、PRD §Compatibility Matrix / §Compatibility Harness Design、ADR-007、ADR-009、ADR-011。

## 2. Business Value

此前项目已经把 frozen baseline、GitHub HEAD drift、non-core GitHub release、source blockers、external authority 和 publication blockers 拆开，但没有用户授权把目标切到“当前最新原始项目”。本 phase 提供该授权后的工程闭环：目标锁定、完整功能再盘点、测试体量扩张、质量 claim 约束。

## 3. Scope / Modules

`docs/compatibility/current-latest.lock.md`、`compatibility/inventory/current-latest-target.json`、`compatibility/inventory/current-latest-source-inventory.json`、`compatibility/matrix/current-latest-matrix.json`、`compatibility/fixtures/current-latest/`、`compatibility/artifacts/current-latest/`、`src/compatibility/`、`src/release.rs`、`scripts/release/`、`tests/`、`test/features/perfect-refactor-parity.feature`、`docs/audits/`。

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 24.1 | current-latest-upstream-authority-lock | ../tasks/task-24.1-current-latest-upstream-authority-lock.md | Done | 将 npm latest、GitHub HEAD、GitHub latest release 观测写成 immutable current-latest target packet |
| 24.2 | current-latest-source-inventory-reextract | ../tasks/task-24.2-current-latest-source-inventory-reextract.md | Done | 从锁定 target 重新抽取完整功能 inventory，并把 silent omission 变成 blocker |
| 24.3 | current-latest-full-function-golden-corpus | ../tasks/task-24.3-current-latest-full-function-golden-corpus.md | Done | 为 current-latest P0/P1 能力扩展 fixtures、snapshots 和 golden diff corpus |
| 24.4 | current-latest-exhaustive-quality-gate | ../tasks/task-24.4-current-latest-exhaustive-quality-gate.md | Ready | 增加大规模回归、stress、property、release quality gate，并限制 claim 为“无已知 release-blocking 缺陷” |

## 5. Dependencies

依赖 Phase 11-23 的 inventory、matrix、golden diff、current-upstream policy、upstream distribution target、unblock packet 和 dynamic latest release observation。该 phase 解除的是“目标授权不明确”阻塞；外部 provider 真实账号、发布凭据、法律/品牌授权仍必须通过 evidence 或 waiver 解决。

## 6. Phase Acceptance Criteria

- [ ] current-latest target packet 同时记录 npm latest stable package、GitHub default branch HEAD、GitHub latest release channel，并拒绝浮动 `latest/main/master/HEAD` 作为 claim 证据。
- [ ] current-latest source inventory 对 command、flag、provider、assertion、redteam、config、output、viewer、Node API、examples/docs rows 做完整 accounting，未分类项阻断 claim。
- [ ] current-latest fixture/golden corpus 覆盖 100% P0 rows，P1 rows 至少有 snapshot 或 protocol tests，P2 rows 有 known gap/waiver/later reason。
- [ ] quality gate 执行 unit/integration/e2e/coverage/build/runtime-smoke、golden diff、stress、property/regression tests；通过后只允许声明“no known release-blocking defects under gates”，不允许声明“no possible bugs”。

## 7. Phase Risks

- Upstream HEAD may move during implementation; every task must lock observed refs and fail closed when evidence drift is detected.
- Full current-latest functionality can expose more blockers than the frozen baseline; blockers must be added to matrix/gates instead of silently waived.
- Live external providers and publication channels need credentials/legal authorization. Without them, evidence must remain mocked/recorded or explicitly blocked.
- A finite test suite cannot prove absence of all possible bugs; claim wording must remain evidence-based.

## 8. Definition of Done

All task 24.1-24.4 specs are Done, phase §6 smoke passes with `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`, current-latest lock/inventory/matrix/corpus/quality artifacts are consistent, and `perfect_refactor_claim_allowed` is true only if all source/current/publication/external quality evidence is present or formally waived.

## 9. Phase Completion Notes

- **完成日期**：<TBD-after-impl>
- **Phase smoke**：<TBD-after-impl>
- **Artifact evidence**：<TBD-after-impl>
- **保留边界**：<TBD-after-impl>
