# BLOCKED — task-22.1 / perfect-refactor external authority

## Phase 49 结案（2026-06-07）— Option B + C，非 perfect-refactor 完成

Maintainer 决策（ADR-012 + v1 policy）：

- **Option B**：项目目标收窄为 **冻结产品基线**（`promptfoo@0.121.15`）上的 `local_stable_allowed=true`，**不**追求 `perfect_refactor_claim_allowed=true` 或 live upstream HEAD 对齐。
- **Option C**：Phase 44 已在 `authority-decisions.json` / `publication-evidence.json` 登记正式 waiver；Phase 49 让 runtime gate **消费**这些 manifest（`active_blocker_count=0`、`v1_scope_ready=true`、`required_user_decision_count=0`），并设置 `current_upstream_rebaseline_required=false`。

**仍保持 fail-closed**：`perfect_refactor_claim_allowed=false`（golden fixture burndown 仍在 `current-latest-golden-corpus.json`）。本文件保留为历史审计；新工作走 Phase 49 gate alignment，不再规划 upstream rebaseline backlog。

---

## 卡住的 AC（历史记录）

- Goal-level AC: `perfect_refactor_claim_allowed=true` for a complete promptfoo perfect-refactor claim.
- This is not a failure of task 22.1 AC1-AC5. Task 22.1 completed its blocker handoff artifact. The remaining failure is the higher-level project goal: the generated claim contract still correctly reports `perfect_refactor_claim_allowed=false`.

## 已尝试方案

1. Phase 19.2 / 19.3 local fixture burn-down: added native/bridge config and provider request/response evidence for rows that can be proven without real external accounts. Result: generic source/provider blockers were reduced, but explicit product/service/credential blockers remained.
2. Phase 19.4 / 20.2 external authority and claim contract gates: added `external-authority-blockers.json` and `perfect-refactor-claim.json` so local stable evidence cannot be mistaken for a perfect-refactor claim. Result: gates pass locally, but the claim remains false because external authority and publication evidence are absent.
3. Phase 21 / 22 target disambiguation and unblock packet: added `upstream-distribution-target.json` and `perfect-refactor-unblock-packet.json` to separate frozen npm core parity from repository-current drift and to list the minimum remaining decisions. Result: `perfect-refactor-unblock-packet.json` reports `status=blocked`, `auto_resolvable=false`, and `required_user_decision_count=29`.

## 当前假设

- The remaining blockers cannot be resolved by code-only changes without violating PRD §Compatibility Matrix / §Release, ADR-008, ADR-009, and task 19.4/20.2/22.1 boundaries.
- Evidence:
  - `target/release-gates/perfect-refactor-claim.json`: `perfect_refactor_claim_allowed=false`, `local_stable_allowed=true`, `published=false`, `publication_ready=credential-blocked`, `source_p0_accounting_blocker_count=22`, `external_authority_blocker_count=21`.
  - `target/release-gates/perfect-refactor-unblock-packet.json`: `status=blocked`, `auto_resolvable=false`, `required_user_decision_count=29`, `current_upstream_rebaseline_required=true`.
  - `target/release-gates/upstream-distribution-target.json`: `npm_core_matches_frozen_baseline=true`, `repository_head_matches_npm_core=false`, `github_latest_release_is_core_package=false`, `current_repository_perfect_claim_allowed=false`.

## 决策需求（写给“未来的自己” / 求助）

- Option A: Provide real external evidence and authorize a new S2V task to consume it:
  - product/service/account/credential evidence for 15 provider authority blockers;
  - product/service evidence or explicit approved waiver for 7 config source-accounting blockers;
  - current-upstream same-ref rebaseline evidence if the claim must target repository HEAD instead of frozen npm core `promptfoo@0.121.13`;
  - credentials, release authority, legal/brand approval, and external URL/digest evidence for Cargo, Docker, GitHub Action, GitHub Releases, Homebrew, and npm wrapper publication.
- Option B: Keep the project at local frozen-baseline stable readiness and explicitly stop asking for a public/current perfect-refactor claim. This does not satisfy the current goal; it narrows the goal.
- Option C: Approve formal waivers for external provider/product/publication/current-upstream blockers. Waivers may document a boundary, but they still must not be represented as live product parity or public perfect-refactor completion unless the claim contract is updated with real evidence and a new ADR/task.

## 当前测试 / 代码状态

- Latest pushed HEAD before this BLOCKED file: `c208e6b docs(spec): phase 22 smoke passed and status done`.
- All task and phase specs in `docs/specs/tasks`, `docs/specs/phases`, and `docs/s2v-adapter.md` are Done with no remaining `Ready`, `Draft`, `In Progress`, `<TBD-by-user>`, or `<TBD-after-impl>` entries.
- Latest Phase 22 smoke passed: `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`.
- The blocking artifacts are:
  - `target/release-gates/perfect-refactor-claim.json`
  - `target/release-gates/perfect-refactor-unblock-packet.json`
  - `target/release-gates/external-authority-blockers.json`
  - `target/release-gates/source-inventory-evidence.json`
  - `target/release-gates/upstream-distribution-target.json`
  - `target/release-gates/publication-authority.json`

## Resume audit — 2026-06-01

- Resumed blocked audit count: 1 of 3 for the same external evidence blocker after the goal was reactivated.
- Fresh commands:
  - `npm view promptfoo version gitHead dist.tarball dist.integrity time.modified --json`
  - `git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.13 refs/tags/code-scan-action-0.1.7`
  - `bash scripts/release/runtime-smoke.sh`
- Fresh evidence:
  - npm core remains `promptfoo@0.121.13` with `gitHead=4860e990c7e9a2f8f677173fb92cf9867b34d03f` and the frozen sha512 integrity.
  - GitHub repository `HEAD` is now `0b93733d48727be67e34433cb0fb1ad21026863a`, which still differs from the npm core gitHead / frozen baseline.
  - `target/release-gates/upstream-distribution-target.json` remains `status=ready-with-drift`, `npm_core_matches_frozen_baseline=true`, `repository_head_matches_npm_core=false`, `github_latest_release_is_core_package=false`, `current_repository_perfect_claim_allowed=false`.
  - `target/release-gates/perfect-refactor-claim.json` remains `perfect_refactor_claim_allowed=false`, `local_stable_allowed=true`, `published=false`, `publication_ready=credential-blocked`, `source_p0_accounting_blocker_count=22`, `external_authority_blocker_count=21`.
  - `target/release-gates/perfect-refactor-unblock-packet.json` remains `status=blocked`, `auto_resolvable=false`, `required_user_decision_count=29`, `current_upstream_rebaseline_required=true`.
- Decision: no new Ready task is justified from this evidence. The only valid next implementation work requires user-provided external authority evidence, publication credentials/approval, approved waivers, or a same-ref current-upstream rebaseline scope.

## Phase 23 progress note — 2026-06-01

- Task 23.1 is implementable without external credentials because it only improves public release-observation freshness: `upstream-distribution-target.sh` now resolves a dynamic latest release ref from GitHub latest release metadata instead of relying on the stale hard-coded `refs/tags/code-scan-action-0.1.7` query.
- This is not an external-authority blocker resolution. The perfect-refactor claim must remain false until source accounting, provider/product authority, publication authority, and current-upstream same-ref rebaseline evidence are supplied or explicitly waived under a new S2V task/ADR.

## Resume audit 2 — 2026-06-01 after Phase 23

- Resumed blocked audit count: 2 of 3 for the same external evidence / publication / current-upstream blocker after the goal was reactivated.
- Fresh commands:
  - `git status --short --branch`
  - `git rev-parse HEAD origin/master`
  - `npm view promptfoo version gitHead dist.tarball dist.integrity time.modified --json`
  - `git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.13 refs/tags/code-scan-action-0.1.7`
  - GitHub latest release metadata fetch: `https://api.github.com/repos/promptfoo/promptfoo/releases/latest`
  - Artifact inspection for `perfect-refactor-claim.json`, `perfect-refactor-unblock-packet.json`, `external-authority-blockers.json`, `publication-authority.json`, `source-inventory-evidence.json`, `longtail-classification.json`
- Fresh evidence:
  - `master` is clean and synced: `HEAD == origin/master == 19da73e48fcdc27658cf7c9e51a2e9eccceecde5`.
  - npm core remains `promptfoo@0.121.13` with `gitHead=4860e990c7e9a2f8f677173fb92cf9867b34d03f`, tarball `https://registry.npmjs.org/promptfoo/-/promptfoo-0.121.13.tgz`, and unchanged sha512 integrity.
  - GitHub latest release metadata returns `tag_name=code-scan-action-0.1.7`, `target_commitish=1c743afe0e4807882e858c4f322fc064fa5f0770`, `published_at=2026-05-29T03:02:57Z`.
  - GitHub repository `HEAD` remains `0b93733d48727be67e34433cb0fb1ad21026863a`, which still differs from the npm core gitHead / frozen baseline.
  - `target/release-gates/upstream-distribution-target.json` now records the dynamic latest release query in `github.source`, but still reports `status=ready-with-drift`, `github_latest_release_is_core_package=false`, and `current_repository_perfect_claim_allowed=false`.
  - `target/release-gates/perfect-refactor-claim.json` remains `perfect_refactor_claim_allowed=false`, `local_stable_allowed=true`, `published=false`, `publication_ready=credential-blocked`, `source_p0_accounting_blocker_count=22`, `external_authority_blocker_count=21`.
  - `target/release-gates/perfect-refactor-unblock-packet.json` remains `status=blocked`, `auto_resolvable=false`, `required_user_decision_count=29`, `current_upstream_rebaseline_required=true`.
  - `target/release-gates/external-authority-blockers.json` still lists 21 blockers: 15 provider product/account/credential authority blockers plus 6 publication blockers.
  - `target/release-gates/publication-authority.json` remains `publication_ready=credential-blocked`, `credential_blocked=true`, `legal_brand_blocked=true`, with all public channels `published=false` and `published_evidence=null`.
- Decision: Phase 23 exhausted the only newly discovered code-only improvement. No additional Ready task can be inferred without violating PRD §Compatibility Matrix / §Release, ADR-008, ADR-009, task 19.4, task 20.2, task 22.1, and task 23.1 boundaries. The remaining options are still user-provided external authority evidence, publication credentials/legal-brand approval, explicit approved waivers, or a new user-approved current-upstream same-ref rebaseline scope.

## Resume audit 3 — 2026-06-01 blocked threshold reached

- Resumed blocked audit count: 3 of 3 for the same external evidence / publication / current-upstream blocker after the goal was reactivated.
- Fresh commands:
  - `git status --short --branch`
  - `git rev-parse HEAD origin/master`
  - `git grep -n -E "^\*\*Status\*\*: (Draft|Ready|In Progress)|<TBD-by-user>|<TBD-after-impl>|\| Ready \|" -- docs/specs/tasks docs/specs/phases docs/s2v-adapter.md`
  - `npm view promptfoo version gitHead dist.tarball dist.integrity time.modified --json`
  - `git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.13 refs/tags/code-scan-action-0.1.7`
  - GitHub latest release metadata fetch: `https://api.github.com/repos/promptfoo/promptfoo/releases/latest`
  - Artifact inspection for `upstream-distribution-target.json`, `perfect-refactor-claim.json`, `perfect-refactor-unblock-packet.json`, `external-authority-blockers.json`, `publication-authority.json`, `source-inventory-evidence.json`, `longtail-classification.json`
- Fresh evidence:
  - `master` is clean and synced before this audit update: `HEAD == origin/master == e343e2fcbd08ea3fdecabcc063b8c1e47070af9f`.
  - No task/phase/adapter implementation entry remains `Draft`, `Ready`, `In Progress`, `<TBD-by-user>`, `<TBD-after-impl>`, or `| Ready |`.
  - npm core remains `promptfoo@0.121.13` with `gitHead=4860e990c7e9a2f8f677173fb92cf9867b34d03f`, tarball `https://registry.npmjs.org/promptfoo/-/promptfoo-0.121.13.tgz`, and unchanged sha512 integrity.
  - GitHub latest release metadata still returns `tag_name=code-scan-action-0.1.7`, `target_commitish=1c743afe0e4807882e858c4f322fc064fa5f0770`, `published_at=2026-05-29T03:02:57Z`.
  - GitHub repository `HEAD` remains `0b93733d48727be67e34433cb0fb1ad21026863a`, which differs from the npm core gitHead / frozen baseline.
  - `target/release-gates/upstream-distribution-target.json` remains `status=ready-with-drift`, dynamic latest release ref `refs/tags/code-scan-action-0.1.7`, `github_latest_release_is_core_package=false`, and `current_repository_perfect_claim_allowed=false`.
  - `target/release-gates/perfect-refactor-claim.json` remains `perfect_refactor_claim_allowed=false`, `local_stable_allowed=true`, `published=false`, `publication_ready=credential-blocked`, `source_p0_accounting_blocker_count=22`, `external_authority_blocker_count=21`.
  - `target/release-gates/perfect-refactor-unblock-packet.json` remains `status=blocked`, `auto_resolvable=false`, `required_user_decision_count=29`, `current_upstream_rebaseline_required=true`.
  - `target/release-gates/external-authority-blockers.json` remains `status=blocked`, `blocker_count=21`; `publication-authority.json` remains `publication_ready=credential-blocked`, `credential_blocked=true`, `legal_brand_blocked=true`, with 6 unpublished public channels.
  - `source-inventory-evidence.json` remains `status=ready-with-blockers`, `p0_accounting_blocker_count=22`; `longtail-classification.json` remains `status=ready-with-blockers`, `p0_release_blocker_count=15`.
- Blocking conclusion: the same blocker has now repeated for three resumed goal turns. There is no remaining code-only or spec-inference task that can honestly advance `perfect_refactor_claim_allowed=true` without user-supplied external evidence, credentials, legal/brand approval, formal waivers, or an explicit current-upstream same-ref rebaseline scope.

## Post-blocked fresh audit 1 — 2026-06-01

- The goal was resumed after being marked blocked, so the blocked audit count restarts at 1 of 3 for this fresh run.
- Fresh commands / sources:
  - `git status --short --branch`
  - `git rev-parse HEAD origin/master`
  - `git grep -n -E "^\*\*Status\*\*: (Draft|Ready|In Progress)|<TBD-by-user>|<TBD-after-impl>|\| Ready \|" -- docs/specs/tasks docs/specs/phases docs/s2v-adapter.md`
  - `npm view promptfoo version gitHead dist.tarball dist.integrity time.modified --json`
  - `git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.13 refs/tags/code-scan-action-0.1.7`
  - GitHub latest release metadata fetch: `https://api.github.com/repos/promptfoo/promptfoo/releases/latest`
  - Public GitHub Releases page: `https://github.com/promptfoo/promptfoo/releases`
  - Artifact inspection for `upstream-distribution-target.json`, `perfect-refactor-claim.json`, `perfect-refactor-unblock-packet.json`, `external-authority-blockers.json`, `publication-authority.json`, `source-inventory-evidence.json`, `longtail-classification.json`
- Fresh evidence:
  - `master` was clean and synced before this audit update: `HEAD == origin/master == abef6acf01c9dbc42282a72af2b6ffd9871cc6c7`.
  - No task/phase/adapter implementation entry remains `Draft`, `Ready`, `In Progress`, `<TBD-by-user>`, `<TBD-after-impl>`, or `| Ready |`.
  - npm core remains `promptfoo@0.121.13` with `gitHead=4860e990c7e9a2f8f677173fb92cf9867b34d03f`, tarball `https://registry.npmjs.org/promptfoo/-/promptfoo-0.121.13.tgz`, and unchanged sha512 integrity.
  - GitHub latest release metadata and GitHub Releases page still show `code-scan-action-0.1.7` as the latest release, with commit `1c743afe0e4807882e858c4f322fc064fa5f0770`, not a core npm package release.
  - GitHub repository `HEAD` remains `0b93733d48727be67e34433cb0fb1ad21026863a`, which differs from the npm core gitHead / frozen baseline.
  - `target/release-gates/upstream-distribution-target.json` remains `status=ready-with-drift`, dynamic latest release ref `refs/tags/code-scan-action-0.1.7`, `github_latest_release_is_core_package=false`, and `current_repository_perfect_claim_allowed=false`.
  - `target/release-gates/perfect-refactor-claim.json` remains `perfect_refactor_claim_allowed=false`, `local_stable_allowed=true`, `published=false`, `publication_ready=credential-blocked`, `source_p0_accounting_blocker_count=22`, `external_authority_blocker_count=21`.
  - `target/release-gates/perfect-refactor-unblock-packet.json` remains `status=blocked`, `auto_resolvable=false`, `required_user_decision_count=29`, `current_upstream_rebaseline_required=true`.
  - `external-authority-blockers.json`, `publication-authority.json`, `source-inventory-evidence.json`, and `longtail-classification.json` still report the same provider/product/credential, publication, source accounting, and longtail blockers.
- Decision: no new code-only Ready task can be inferred in this fresh resumed run. The minimal unblocking input remains one of: user-provided external authority evidence, publication credentials/legal-brand approval, explicit formal waivers, or a new user-approved current-upstream same-ref rebaseline scope.

## Post-blocked fresh audit 2 — 2026-06-01

- Blocked-after-resume audit count: 2 of 3 for the same external evidence / publication / current-upstream blocker.
- Fresh commands / sources:
  - `git status --short --branch`
  - `git rev-parse HEAD origin/master`
  - `git grep -n -E "^\*\*Status\*\*: (Draft|Ready|In Progress)|<TBD-by-user>|<TBD-after-impl>|\| Ready \|" -- docs/specs/tasks docs/specs/phases docs/s2v-adapter.md`
  - `npm view promptfoo version gitHead dist.tarball dist.integrity time.modified --json`
  - `git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.13 refs/tags/code-scan-action-0.1.7`
  - GitHub latest release metadata fetch: `https://api.github.com/repos/promptfoo/promptfoo/releases/latest`
  - Public GitHub Releases page: `https://github.com/promptfoo/promptfoo/releases`
  - Artifact inspection for `upstream-distribution-target.json`, `perfect-refactor-claim.json`, `perfect-refactor-unblock-packet.json`, `external-authority-blockers.json`, `publication-authority.json`, `source-inventory-evidence.json`, `longtail-classification.json`
- Fresh evidence:
  - `master` was clean and synced before this audit update: `HEAD == origin/master == 3271e6ae809c792f320f7f7dbe01c5c77ac2b979`.
  - No task/phase/adapter implementation entry remains `Draft`, `Ready`, `In Progress`, `<TBD-by-user>`, `<TBD-after-impl>`, or `| Ready |`.
  - npm core remains `promptfoo@0.121.13` with `gitHead=4860e990c7e9a2f8f677173fb92cf9867b34d03f`, tarball `https://registry.npmjs.org/promptfoo/-/promptfoo-0.121.13.tgz`, and unchanged sha512 integrity.
  - GitHub latest release metadata and GitHub Releases page still show `code-scan-action-0.1.7` as the latest release, with commit `1c743afe0e4807882e858c4f322fc064fa5f0770`, not a core npm package release.
  - GitHub repository `HEAD` remains `0b93733d48727be67e34433cb0fb1ad21026863a`, which differs from the npm core gitHead / frozen baseline.
  - `target/release-gates/upstream-distribution-target.json` remains `status=ready-with-drift`, dynamic latest release ref `refs/tags/code-scan-action-0.1.7`, `github_latest_release_is_core_package=false`, and `current_repository_perfect_claim_allowed=false`.
  - `target/release-gates/perfect-refactor-claim.json` remains `perfect_refactor_claim_allowed=false`, `local_stable_allowed=true`, `published=false`, `publication_ready=credential-blocked`, `source_p0_accounting_blocker_count=22`, `external_authority_blocker_count=21`.
  - `target/release-gates/perfect-refactor-unblock-packet.json` remains `status=blocked`, `auto_resolvable=false`, `required_user_decision_count=29`, `current_upstream_rebaseline_required=true`, and `blocker_count=4`.
  - `external-authority-blockers.json`, `publication-authority.json`, `source-inventory-evidence.json`, and `longtail-classification.json` still report the same provider/product/credential, publication, source accounting, and longtail blockers.
- Decision: this is the second consecutive fresh resumed audit with the same blocker after the previous blocked status. No additional S2V Ready task can be created without new user-supplied evidence/scope, so the goal stays active and is not re-marked `blocked` until the resumed blocker threshold reaches 3 of 3.

## Post-blocked fresh audit 3 — 2026-06-01 blocked threshold reached

- Blocked-after-resume audit count: 3 of 3 for the same external evidence / publication / current-upstream blocker.
- Fresh commands / sources:
  - `git status --short --branch`
  - `git rev-parse HEAD origin/master`
  - `git grep -n -E "^\*\*Status\*\*: (Draft|Ready|In Progress)|<TBD-by-user>|<TBD-after-impl>|\| Ready \|" -- docs/specs/tasks docs/specs/phases docs/s2v-adapter.md`
  - `npm view promptfoo version gitHead dist.tarball dist.integrity time.modified --json`
  - `git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.13 refs/tags/code-scan-action-0.1.7`
  - GitHub latest release metadata fetch: `https://api.github.com/repos/promptfoo/promptfoo/releases/latest`
  - Public GitHub Releases page: `https://github.com/promptfoo/promptfoo/releases`
  - Artifact inspection for `upstream-distribution-target.json`, `perfect-refactor-claim.json`, `perfect-refactor-unblock-packet.json`, `external-authority-blockers.json`, `publication-authority.json`, `source-inventory-evidence.json`, `longtail-classification.json`
- Fresh evidence:
  - `master` was clean and synced before this audit update: `HEAD == origin/master == 1eccdfd303f1eac62744c5f406885ac43ba2f6c1`.
  - No task/phase/adapter implementation entry remains `Draft`, `Ready`, `In Progress`, `<TBD-by-user>`, `<TBD-after-impl>`, or `| Ready |`.
  - npm core remains `promptfoo@0.121.13` with `gitHead=4860e990c7e9a2f8f677173fb92cf9867b34d03f`, tarball `https://registry.npmjs.org/promptfoo/-/promptfoo-0.121.13.tgz`, and unchanged sha512 integrity.
  - GitHub latest release metadata and GitHub Releases page still show `code-scan-action-0.1.7` as the latest release, with commit `1c743afe0e4807882e858c4f322fc064fa5f0770`, not a core npm package release.
  - GitHub repository `HEAD` remains `0b93733d48727be67e34433cb0fb1ad21026863a`, which differs from the npm core gitHead / frozen baseline.
  - `target/release-gates/upstream-distribution-target.json` remains `status=ready-with-drift`, dynamic latest release ref `refs/tags/code-scan-action-0.1.7`, `github_latest_release_is_core_package=false`, and `current_repository_perfect_claim_allowed=false`.
  - `target/release-gates/perfect-refactor-claim.json` remains `perfect_refactor_claim_allowed=false`, `local_stable_allowed=true`, `published=false`, `publication_ready=credential-blocked`, `source_p0_accounting_blocker_count=22`, `external_authority_blocker_count=21`.
  - `target/release-gates/perfect-refactor-unblock-packet.json` remains `status=blocked`, `auto_resolvable=false`, `required_user_decision_count=29`, `current_upstream_rebaseline_required=true`, and `blocker_count=4`.
  - `external-authority-blockers.json`, `publication-authority.json`, `source-inventory-evidence.json`, and `longtail-classification.json` still report the same provider/product/credential, publication, source accounting, and longtail blockers.
- Blocking conclusion: the same blocker has now repeated for three fresh resumed goal turns after the previous blocked status. There is no remaining code-only or spec-inference task that can honestly advance `perfect_refactor_claim_allowed=true` without user-supplied external evidence, credentials, legal/brand approval, formal waivers, or an explicit current-upstream same-ref rebaseline scope.
