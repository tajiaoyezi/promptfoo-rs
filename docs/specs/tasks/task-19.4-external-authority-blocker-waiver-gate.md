# Task 19.4: external-authority-blocker-waiver-gate

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 19 — source-accounting-native-burndown
**Dependencies**: task-19.3-provider-request-response-fixture-burndown

## 1. Background

部分 remaining blockers 无法仅靠本地 mock fixture 证明：Codex provider、Agents SDK/tracing、Assistant、Billing、ChatKit、Realtime、Claude Code auth、真实多渠道发布都涉及账号、私有服务、产品协议、法律/品牌或发布授权。S2V 规则要求这些项保留为 explicit blockers/waivers，不能伪造 ready。依据 PRD §Security / §Release constraints、ADR-008、ADR-009、task 18.4 §10。

## 2. Goal

建立 external authority blocker/waiver gate：所有需要真实凭据、账号、私有服务、产品授权、法律/品牌确认或公开发布承诺的 remaining blockers 都有 item-level waiver/blocker 记录、最小所需用户决策和不可伪造的 release gate 输出。

## 3. Scope

### In Scope

- `src/compatibility/provider_assertion.rs`
- `scripts/release/longtail-classification.sh`
- `scripts/release/installability.sh`
- `scripts/release/runtime-smoke.sh`
- `target/release-gates/external-authority-blockers.json`
- `docs/compatibility/matrix.md`
- `docs/release.md`
- `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`
- `tests/external_authority_blocker_waiver_gate.rs`

### Out Of Scope

- 不获取或使用真实密钥、账号或私有服务权限。
- 不发布真实 GitHub/Cargo/npm/Docker/Homebrew artifact。
- 不做法律/品牌批准；只记录需要批准的最小问题。

## 4. Users / Actors

- Release maintainer：需要知道哪些 blocker 只能由凭据/授权解除。
- Enterprise reviewer：需要确认未把 external authority blocker 伪装为 native parity。
- Future publisher：需要最小决策清单。

## 5. Behavior Contract

External authority gate 必须输出每个 external blocker 的 item id、source reference、authority type、required decision、current status、safe local fallback、release impact。没有真实证据时 status 保持 `blocked` 或 `waived-with-boundary`，不得进入 `ready`。

### 5.1 Required Reading

- docs/specs/tasks/task-18.4-publication-authority-release-gate.md
- docs/specs/tasks/task-19.3-provider-request-response-fixture-burndown.md
- docs/decisions/adr-008-binary-first-multi-channel-release.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/release.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`promptfoo_rs::compatibility::provider_assertion`、`promptfoo_rs::release`、`serde_json`、`std::fs`。
- Tooling commands：adapter §Commands Install / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke。

### 5.3 函数签名

- `collect_external_authority_blockers() -> ExternalAuthorityBlockerReport`
- `validate_external_authority_gate(report: &ExternalAuthorityBlockerReport) -> ExternalAuthorityGateDecision`
- `write_external_authority_blockers(report: &ExternalAuthorityBlockerReport, path: &Path) -> Result<(), CompatibilityEvidenceError>`

## 6. Acceptance Criteria

- [x] **AC1** (S2V blocked protocol): every real credential/account/private-service/legal-brand/publication blocker has item-level authority type and required decision.
- [x] **AC2** (PRD §Security): local fallback/mock evidence is clearly separated from live product parity and never sets status `ready`.
- [x] **AC3** (ADR-008): publication authority blockers from task 18.4 are included or linked, with channel-level published=false preserved.
- [x] **AC4** (ADR-009): release candidate and audit outputs keep external-authority blockers visible and do not claim perfect refactor until resolved.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-19.4.1 | TEST-19.4.1 | tests/external_authority_blocker_waiver_gate.rs | install, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-19.4.1 | TEST-19.4.2 | tests/external_authority_blocker_waiver_gate.rs | install, typecheck, unit-test, coverage, build | Done |
| AC3 | SCEN-19.4.1 | TEST-19.4.3 | tests/external_authority_blocker_waiver_gate.rs | install, typecheck, unit-test, runtime-smoke, build | Done |
| AC4 | SCEN-19.4.1 | TEST-19.4.4 | tests/external_authority_blocker_waiver_gate.rs | install, typecheck, unit-test, e2e, build | Done |

## 8. Risks

- External blockers may require user/legal decisions; this task can only record and gate them.
- Waiver language must not imply native/live parity.
- Public release evidence cannot be created without credentials and explicit authorization.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **E2E tests**: adapter §Commands E2E tests
- **Coverage**: adapter §Commands Coverage
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：2026-05-31
- **改动文件**：
  - `tests/external_authority_blocker_waiver_gate.rs`
  - `src/compatibility/provider_assertion.rs`
  - `scripts/release/integration.sh`
  - `scripts/release/runtime-smoke.sh`
  - `docs/compatibility/matrix.md`
  - `docs/release.md`
  - `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`
  - `docs/s2v-adapter.md`
  - `docs/specs/phases/phase-19-source-accounting-native-burndown.md`
  - `docs/specs/tasks/task-19.4-external-authority-blocker-waiver-gate.md`
- **commit 列表**：
  - `b9dc133` `test(authority): add SCEN-19.4.1 external authority gate RED tests`
  - `8a0d2bf` `feat(authority): add external authority blocker gate`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-19.4.1 ~ TEST-19.4.4 通过。
  - integration: PASS — `bash scripts/release/integration.sh` 通过，包含 `external_authority_blocker_waiver_gate`。
  - e2e: PASS — `bash scripts/release/e2e.sh` 通过。
  - build: PASS — helper 执行 `cargo build --workspace`、viewer/npm `pnpm build` 通过。
  - coverage: PASS — `bash scripts/release/coverage.sh` 通过；`s2v_coverage_threshold_guard` 通过。
  - runtime-smoke: PASS — `bash scripts/release/runtime-smoke.sh` 通过；`external-authority-blockers.json` reports `status=blocked`, `blocker_count=21`, `provider_external_blocker_count=15`, `publication_blocker_count=6`, `ready_count=0`，且 `release-candidate.json.external_authority.status=blocked`。
- **剩余风险 / 未做项**：21 个 external authority records 仍不能由本地仓库解除：15 个 provider product/account/credential blockers 和 6 个 public publication channel blockers 需要真实凭据、账号权限、产品授权、法律/品牌或外部 URL/digest evidence。
- **下游 task 影响**：Phase 19 已具备收尾 smoke 条件；后续若要把 external authority gate 从 blocked 推到 ready，必须新增有授权的 task/ADR，并提供真实外部证据，不能复用本地 mock 或 dry-run 产物。
