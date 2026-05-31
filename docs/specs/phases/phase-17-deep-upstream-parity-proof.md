# Phase 17: deep-upstream-parity-proof

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

把 Phase 16 后仍未满足“完美重构”的深层证据缺口转成可执行实现链路：完整 frozen upstream source inventory、CLI/global/eval/redteam parity、50+ 真实 upstream golden corpus、provider/assertion/redteam 长尾分类与实现证据、发布安装证据。依据 PRD §Compatibility Matrix / §Success Metrics、ADR-004、ADR-007、ADR-008、ADR-009、docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md。

## 2. Business Value

Phase 16 已证明本地 gate 可执行且一个真实 upstream smoke 可通过，但企业迁移 reviewer 仍无法据此确认整个 promptfoo 0.121.13 surface 都被枚举、分类、执行或明确阻断。本阶段让“接近可用”推进到“可审计地证明哪些等价、哪些受控不等价”。

## 3. Scope / Modules

`compatibility/inventory/`、`compatibility/matrix/`、`compatibility/fixtures/`、`compatibility/artifacts/`、`src/compatibility/`、`src/cli.rs`、`src/config/`、`src/eval/`、`src/providers/`、`src/assertions/`、`src/redteam/`、`scripts/release/`、`.github/workflows/`、`docs/compatibility/`、`docs/release.md`、`tests/`、`test/features/perfect-refactor-parity.feature`。

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 17.1 | frozen-source-inventory-extractor | ../tasks/task-17.1-frozen-source-inventory-extractor.md | Done | 从 frozen upstream tag/package 源码提取完整 capability inventory 并阻断 silent omissions |
| 17.2 | cli-global-eval-redteam-parity | ../tasks/task-17.2-cli-global-eval-redteam-parity.md | Done | 扩展 top-level/eval/redteam CLI command/flag parity 或明确 unsupported/later 行为 |
| 17.3 | real-p0-golden-corpus-runner | ../tasks/task-17.3-real-p0-golden-corpus-runner.md | Done | 让至少 50 个 P0 fixture 真实执行 upstream 与 promptfoo-rs 并持久化 diff artifacts |
| 17.4 | longtail-provider-assertion-redteam-classification | ../tasks/task-17.4-longtail-provider-assertion-redteam-classification.md | Done | 将 provider/assertion/redteam 长尾 source rows 全量分类、实现或登记用户可见 gap |
| 17.5 | release-installability-publication-readiness | ../tasks/task-17.5-release-installability-publication-readiness.md | Done | 生成多渠道可安装 artifact dry-run 证据并区分需要真实凭据的公开发布步骤 |

## 5. Dependencies

依赖 Phase 16 已完成的 measured runtime smoke、真实 upstream smoke、source evidence seed；依赖 Phase 12 executable harness / release gate；依赖 Phase 13-14 的 CLI/provider/assertion/redteam 基础；依赖 Phase 15 viewer/npm packaging smoke。

## 6. Phase Acceptance Criteria

- [x] source-extracted inventory 覆盖 frozen tag `4860e990c7e9a2f8f677173fb92cf9867b34d03f` 的 command/provider/assertion/redteam/plugin/strategy/output/config/viewer/API/example surface，且 matrix 对每个 item 给出 P0/P1/P2、status、owner、verification 或 blocker。
- [x] local CLI 对 upstream `promptfoo@0.121.13 --help`、`eval --help`、`redteam --help` 中的 user-visible command/flag 全部实现兼容行为或返回非 0 的 explicit unsupported/later/blocked 错误，并同步 item-level matrix。
- [x] release gate 执行至少 50 个 P0 fixture 的真实 upstream-vs-rs golden diff，持久化 raw/normalized/diff/metadata artifacts；P0 bug、unclassified diff、缺 artifact、使用本地替身均阻断 stable。
- [x] provider/assertion/redteam 长尾 rows 无 unresolved 或 missing-reason 项；P0 rows 有 fixture，P1 rows 有 snapshot，P2/later/unsupported rows 有用户可见错误和 reason。
- [x] release readiness 产出 GitHub archive/checksum、cargo package dry-run、npm pack、viewer/npm smoke、Docker/Homebrew/GitHub Action installability evidence；真实外部发布凭据缺失时记录为 release credential blocker，不伪装成已发布。

## 7. Phase Risks

- 完整 source extraction 可能暴露大量新增 P1/P2 rows；这不是失败，失败是新增 rows 没有分类或 verification owner。
- upstream CLI help 与源码 command registry 不完全等价；task 必须同时记录 runtime help snapshot 和 source file inventory。
- 50+ 真实 upstream fixture 会拉长 runtime smoke；可拆 quick smoke/full release gate，但 stable claim 只能引用 full gate。
- GitHub Releases、Homebrew、crates.io、Docker registry、npm publish 需要真实账号/凭据；本阶段只能做 dry-run/installability，真实发布若必须执行需按 BLOCKED 协议。

## 8. Definition of Done

五个 task 全部 Done 后，执行 phase §6 smoke：`s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`；检查 `target/release-gates/` 中 source inventory、50+ corpus diff、release readiness artifacts；确认 adapter / PRD / compatibility matrix / BDD feature 与 task §10 completion notes 一致。

## 9. Phase Completion Notes

- **完成日期**：2026-05-31
- **Phase smoke**：PASS — `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"` 通过，共 9 项。
- **Artifact evidence**：
  - `target/release-gates/source-inventory-evidence.json`: status=`ready-with-blockers`，source_extracted_item_count=2549，missing_matrix_rows=2116，release_blockers=2116；这些 blocker 是 task 17.1/phase output 的显式阻断记录，不是 silent omission。
  - `target/release-gates/longtail-classification.json`: status=`ready-with-blockers`，tracked_longtail_item_count=433，missing_tracked_rows=0，unresolved_rows=0，missing_reason_rows=0，p0_release_blocker_count=37。
  - `target/release-gates/real-upstream-corpus/index.json`: status=`ready`，observed_p0_fixture_count=50，stable_allowed=true，所有 corpus runs 使用真实 `promptfoo@0.121.13` 与当前 release binary。
  - `target/release-gates/real-upstream-smoke/latest/metadata.json`: status=`ready`，upstream_exit_code=0，rs_exit_code=0，used_test_binary=false。
  - `target/release-gates/installability.json`: installability_ready=true，publication_ready=`credential-blocked`，credential_blocked=true，published=false，artifacts/checksums=6。
- **保留边界**：Phase 17 证明了当前 PRD/P0-P1-P2 policy 下的可审计 release gate 与 blocker ledger；它不声称真实外部渠道已经公开发布，也不把 2116 个 source-inventory blocker 或 37 个 P0 provider-module blocker 当作 native parity。
