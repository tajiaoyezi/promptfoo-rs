# Task 24.4: current-latest-exhaustive-quality-gate

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 24 — current-latest-perfect-refactor
**Dependencies**: task-24.3-current-latest-full-function-golden-corpus

## 1. Background

The user requires heavy testing and no potential bugs. No finite engineering process can prove the absence of all possible latent bugs, but the project can enforce a strict claim contract: no known release-blocking defects under declared gates. 依据用户 2026-06-01 澄清、PRD §Success Metrics / §Compatibility Harness Design、ADR-007、ADR-009、ADR-011。

## 2. Goal

Add current-latest quality gates for broad regression, stress, property-style deterministic checks, golden diff saturation, and claim wording. The gate must fail any current-latest perfect-refactor claim when known blockers, untested P0/P1 rows, failing stress tests, or external authority gaps remain.

## 3. Scope

### In Scope

- `scripts/release/current-latest-quality-gate.sh`
- `scripts/release/runtime-smoke.sh`
- `target/release-gates/current-latest-quality.json`
- `target/release-gates/perfect-refactor-claim.json`
- `src/release.rs`
- `tests/current_latest_quality_gate.rs`
- `tests/` regression/stress/property fixtures
- `docs/release.md`
- `docs/compatibility/matrix.md`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不承诺数学意义上的零 bug。
- 不执行需要真实私有密钥的 live provider tests，除非用户提供 credentials。
- 不把 local-only gates 当成 public publication evidence。

## 4. Users / Actors

- Release maintainer: needs a defensible quality claim.
- CI maintainer: needs deterministic heavy test commands.
- User: needs clarity that the project maximizes bug detection without making impossible claims.

## 5. Behavior Contract

The quality gate must aggregate adapter verification, current-latest golden corpus status, deterministic regression/stress/property test results, release smoke, source inventory coverage, and external/publication authority status. It must write machine-readable evidence. It may allow `local_stable_allowed=true` only when all local gates pass. It may allow `perfect_refactor_claim_allowed=true` only when local gates, current target evidence, source inventory, golden corpus, external authority, and publication evidence are all complete or formally waived. It must reject wording equivalent to "no possible bugs".

### 5.1 Required Reading

- docs/specs/tasks/task-24.1-current-latest-upstream-authority-lock.md
- docs/specs/tasks/task-24.2-current-latest-source-inventory-reextract.md
- docs/specs/tasks/task-24.3-current-latest-full-function-golden-corpus.md
- docs/specs/tasks/task-20.2-perfect-refactor-claim-contract.md
- docs/specs/tasks/task-22.1-authority-unblock-packet-gate.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `serde::{Serialize, Deserialize}`, `serde_json::Value`, `std::fs`, `promptfoo_rs::release::{PerfectRefactorClaimContract, ReleaseBlocker}`.
- Tooling commands: `bash scripts/release/current-latest-quality-gate.sh`, `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `build_current_latest_quality_report(inputs: CurrentLatestQualityInputs) -> CurrentLatestQualityReport`
- `evaluate_current_latest_claim(report: &CurrentLatestQualityReport, claim: &PerfectRefactorClaimContract) -> Result<(), ReleaseBlocker>`
- Shell contract: `bash scripts/release/current-latest-quality-gate.sh`

## 6. Acceptance Criteria

- [x] **AC1** (user 2026-06-01): quality report aggregates adapter verification, golden corpus, source inventory coverage, deterministic regression, stress, property-style checks, runtime smoke, and release blockers.
- [x] **AC2** (ADR-011): claim wording is limited to “no known release-blocking defects under declared gates”; “no potential bugs” / “zero possible bugs” wording fails the gate.
- [x] **AC3** (PRD §Success Metrics): current-latest perfect-refactor claim remains false if any P0/P1 evidence, stress/regression/property test, external authority, publication authority, or current target evidence is missing.
- [x] **AC4** (ADR-007): when all local current-latest gates pass but external/publication authority remains absent, the report allows local readiness but blocks public perfect-refactor completion.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-24.4.1 | TEST-24.4.1 | tests/current_latest_quality_gate.rs | install, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-24.4.1 | TEST-24.4.2 | tests/current_latest_quality_gate.rs | install, lint, typecheck, unit-test, coverage, build | Done |
| AC3 | SCEN-24.4.1 | TEST-24.4.3 | tests/current_latest_quality_gate.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Done |
| AC4 | SCEN-24.4.1 | TEST-24.4.4 | tests/current_latest_quality_gate.rs | install, lint, typecheck, unit-test, runtime-smoke, build | Done |

## 8. Risks

- Heavy tests can be slow; release candidate gates may need a quick mode plus full mode, but full mode is required for the claim.
- Property-style tests cannot cover every possible input; they increase confidence but do not prove absence of all bugs.
- External live-provider and publication evidence remain non-code blockers unless supplied.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Lint**: adapter §Commands Lint
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **E2E tests**: adapter §Commands E2E tests
- **Coverage**: adapter §Commands Coverage
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：2026-06-01
- **改动文件**：
  - `src/release.rs`
  - `tests/current_latest_quality_gate.rs`
  - `scripts/release/current-latest-quality-gate.sh`
  - `scripts/release/runtime-smoke.sh`
  - `scripts/release/integration.sh`
  - `scripts/release/coverage.sh`
  - `docs/release.md`
  - `docs/compatibility/matrix.md`
  - `test/features/perfect-refactor-parity.feature`
  - `docs/specs/tasks/task-24.4-current-latest-exhaustive-quality-gate.md`
  - `docs/specs/phases/phase-24-current-latest-perfect-refactor.md`
  - `docs/s2v-adapter.md`
- **commit 列表**：
  - `5103ced` `docs(spec): task-24.4 enters implementation`
  - `a9c0a0e` `test(release): add current latest quality gate RED tests`
  - `4761804` `feat(release): add current latest quality gate`
  - 本次 docs 回填提交：`docs(spec): complete task 24.4 current latest quality gate`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - lint: PASS — helper 执行 `bash scripts/release/lint.sh` 通过。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-24.4.1 ~ TEST-24.4.4 通过。
  - integration: PASS — helper 执行 `bash scripts/release/integration.sh` 通过，包含 `current_latest_quality_gate`。
  - e2e: PASS — helper 执行 `bash scripts/release/e2e.sh` 通过。
  - build: PASS — helper 执行 `cargo build --workspace`、viewer/npm `pnpm build` 通过。
  - coverage: PASS — helper 执行 `bash scripts/release/coverage.sh` 通过，coverage traceability gate 覆盖 TEST-24.4.1 ~ TEST-24.4.4；`s2v_coverage_threshold_guard` 通过。
  - runtime-smoke: PASS — helper 执行 `bash scripts/release/runtime-smoke.sh` 通过；`current-latest-quality.json` status=`ready-with-blockers`，`local_current_latest_ready=false`，`perfect_refactor_claim_allowed=false`，blocker_count=6；`release-candidate.json` 暴露 `current_latest_quality` gate。
- **剩余风险 / 未做项**：当前 quality gate 明确禁止“no potential bugs”/“zero possible bugs”类不可证明承诺，只允许 “no known release-blocking defects under declared gates”。当前 current-latest 仍有 318 个 unclassified source/matrix rows、432 个 golden corpus blockers、21 个 external authority blockers，以及 publication `credential-blocked`，因此 perfect-refactor claim 必须保持 false。
- **下游 task 影响**：Phase 24 可以进入 phase smoke 收尾；若后续要让 current-latest perfect-refactor claim 变 true，需要新增 task 或正式 waiver 消费 `current-latest-quality.json` 中列出的 source/matrix/golden/current-target/external/publication blockers。
