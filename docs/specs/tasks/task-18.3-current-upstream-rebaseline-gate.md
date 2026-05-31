# Task 18.3: current-upstream-rebaseline-gate

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 18 — perfect-refactor-blocker-burndown
**Dependencies**: task-18.1-source-inventory-ledger-closure

## 1. Background

当前项目 stable baseline 是 `promptfoo@0.121.13` / `4860e990c7e9a2f8f677173fb92cf9867b34d03f`，但复审发现 GitHub `promptfoo/promptfoo` HEAD 已不同，且 releases 页面有后续 `code-scan-action: 0.1.7`。完美重构声明必须区分 frozen npm baseline 与 current upstream repository。依据 docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md §Scope / §P0 Scope。

## 2. Goal

建立 current-upstream target mode gate：记录 observed HEAD/ref/release、与 frozen baseline 的差异、是否允许 current-perfect claim；没有 current mode 证据时，release candidate 只能声明 frozen-baseline compatibility。

## 3. Scope

### In Scope

- `docs/compatibility/target-policy.md`
- `compatibility/inventory/current-upstream-target.json`
- `scripts/release/current-upstream-policy.sh`
- `scripts/release/runtime-smoke.sh`
- `tests/current_upstream_rebaseline_gate.rs`
- `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`

### Out Of Scope

- 不自动把 moving HEAD 设为 stable release target。
- 不执行历史重写或强推。
- 不承诺公开发布当前 upstream parity，除非 current target mode 全 gate 通过。

## 4. Users / Actors

- Release maintainer：需要防止 frozen evidence 被误称为 current upstream 完成。
- Compatibility reviewer：需要看到当前 HEAD 与 frozen tag 的差异证据。
- Contributor：需要知道新增 upstream 变更是否需要新 phase/task。

## 5. Behavior Contract

Release gate 必须输出 machine-readable current upstream policy。policy 包含 frozen target、observed current HEAD、observed release/tag、observation timestamp、target mode、claim allowed 状态。若 target mode 是 frozen，current-perfect claim 必须为 false；若 target mode 是 current，必须要求 current inventory、fixture corpus、golden diff 和 release gate 全部来自同一 observed ref。

### 5.1 Required Reading

- docs/compatibility/target-policy.md
- docs/compatibility/baseline.lock.md
- docs/specs/tasks/task-11.1-current-upstream-target-policy.md
- docs/specs/tasks/task-17.1-frozen-source-inventory-extractor.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`serde_json`、`std::process::Command`、内部模块 `compatibility::inventory`。
- Tooling commands：`git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.13`、adapter §Commands Install / Typecheck / Unit Test / Integration tests / Runtime smoke / Build。

### 5.3 函数签名

- `CurrentUpstreamObservation::from_ls_remote(output: &str) -> Result<CurrentUpstreamObservation, TargetPolicyError>`
- `evaluate_current_claim_policy(frozen: &FrozenSourceReference, current: &CurrentUpstreamObservation, mode: TargetMode) -> CurrentClaimPolicy`
- `write_current_upstream_policy(policy: &CurrentClaimPolicy, path: &Path) -> Result<(), TargetPolicyError>`

## 6. Acceptance Criteria

- [x] **AC1** (docs/audits §Scope): current upstream HEAD and frozen baseline are both recorded with full SHAs; floating `main/latest` is never accepted as a stable target.
- [x] **AC2** (ADR-007): frozen mode explicitly sets `current_perfect_claim_allowed=false` when current HEAD differs from frozen tag.
- [x] **AC3** (PRD §Compatibility Matrix): current mode requires current source inventory, matrix, fixtures, golden corpus, and release candidate evidence to share the same observed ref.
- [x] **AC4** (ADR-009): audit docs and release candidate summaries display target mode, preventing ambiguous “perfect refactor” claims.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-18.3.1 | TEST-18.3.1 | tests/current_upstream_rebaseline_gate.rs | install, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-18.3.1 | TEST-18.3.2 | tests/current_upstream_rebaseline_gate.rs | install, typecheck, unit-test, integration, build | Done |
| AC3 | SCEN-18.3.1 | TEST-18.3.3 | tests/current_upstream_rebaseline_gate.rs | install, typecheck, unit-test, coverage, build | Done |
| AC4 | SCEN-18.3.1 | TEST-18.3.4 | tests/current_upstream_rebaseline_gate.rs | install, typecheck, unit-test, runtime-smoke, build | Done |

## 8. Risks

- Current upstream moves during execution; scripts must record observed ref rather than use floating names as proof.
- Rebaseline may reveal a large new blocker set; this task gates the claim, not the full implementation.
- GitHub availability can affect observation; network failures must be explicit blockers, not silent pass.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **Coverage**: adapter §Commands Coverage
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：2026-05-31
- **改动文件**：
  - `tests/current_upstream_rebaseline_gate.rs`
  - `src/compatibility/inventory.rs`
  - `scripts/release/current-upstream-policy.sh`
  - `scripts/release/runtime-smoke.sh`
  - `scripts/release/integration.sh`
  - `compatibility/inventory/current-upstream-target.json`
  - `docs/specs/tasks/task-18.3-current-upstream-rebaseline-gate.md`
  - `docs/specs/phases/phase-18-perfect-refactor-blocker-burndown.md`
  - `docs/s2v-adapter.md`
  - `docs/compatibility/target-policy.md`
  - `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`
  - `docs/superpowers/plans/2026-05-31-perfect-refactor-blocker-burndown.md`
- **commit 列表**：
  - `6032925` `test(compatibility): add SCEN-18.3.1 current upstream gate RED tests`
  - `05c0160` `feat(compatibility): add current upstream target-mode gate`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-18.3.1 ~ TEST-18.3.4 通过。
  - integration: PASS — `bash scripts/release/integration.sh` 通过，包含 `current_upstream_rebaseline_gate`。
  - build: PASS — helper 执行 `cargo build --workspace`、viewer/npm `pnpm build` 通过。
  - coverage: PASS — `bash scripts/release/coverage.sh` 通过；`s2v_coverage_threshold_guard` 通过。
  - runtime-smoke: PASS — `bash scripts/release/runtime-smoke.sh` 通过；`target/release-gates/current-upstream-policy.json` status=`ready`，target_mode=`frozen`，current_perfect_claim_allowed=false，current.current_head=`ff8eafd743cf6d63dd85b790ad8a4c73ede5828d`，frozen.git_commit=`4860e990c7e9a2f8f677173fb92cf9867b34d03f`；`target/release-gates/release-candidate.json` 包含 `target_policy` 同步字段。
- **剩余风险 / 未做项**：本 task 只建立 claim gate，不把 moving HEAD 自动设为 stable target。current mode 仍需要 source inventory、matrix、fixtures、golden corpus、release candidate 全部共享同一 observed ref 后才允许 current-perfect claim；当前 frozen mode 下仍只能声明 frozen-baseline compatibility。
- **下游 task 影响**：task 18.4 可在 publication authority gate 中引用 `release-candidate.json.target_policy`，防止把 local installability 或 frozen evidence 当作 current-upstream/publication 完成证据；Phase 18 smoke 需要检查 `current-upstream-policy.json` 与 audit 结论一致。
