# Task 22.1: authority-unblock-packet-gate

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 22 — perfect-refactor-unblock-packet
**Dependencies**: task-19.4-external-authority-blocker-waiver-gate, task-20.2-perfect-refactor-claim-contract, task-21.1-upstream-distribution-target-gate

## 1. Background

After Phase 21, local frozen-baseline gates can pass while `perfect_refactor_claim_allowed=false`. The remaining blockers are no longer generic implementation gaps: source accounting reports 22 P0 blockers, external authority reports 21 blockers, publication remains credential/legal blocked, and current repository perfect claim is blocked by distribution target drift. The next safe step is not to mark these complete, but to aggregate the exact user/maintainer decisions and evidence required to unblock them. 依据 PRD §Compatibility Matrix / §Release / §Success Metrics、ADR-007、ADR-008、ADR-009、task-19.4 §10、task-20.2 §10、task-21.1 §10。

## 2. Goal

新增 perfect-refactor unblock packet gate：生成 `target/release-gates/perfect-refactor-unblock-packet.json`，把 source/external/current/publication blockers 聚合成最小决策清单；每个 item 都记录 required actor、required evidence、source artifact、release impact 和 `auto_resolvable=false`，并纳入 runtime smoke / release candidate。

## 3. Scope

### In Scope

- `src/release.rs`
- `scripts/release/perfect-refactor-unblock-packet.sh`
- `scripts/release/runtime-smoke.sh`
- `target/release-gates/perfect-refactor-unblock-packet.json`
- `tests/perfect_refactor_unblock_packet.rs`
- `docs/release.md`
- `docs/compatibility/matrix.md`
- `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不提供真实 provider credentials、账号、私有服务权限或发布 token。
- 不公开发布 GitHub Releases、Cargo、npm、Docker、Homebrew 或 GitHub Action artifact。
- 不把 source/external/publication/current blockers 改名为 ready。
- 不把 `perfect_refactor_claim_allowed` 改成 true。

## 4. Users / Actors

- User / maintainer：需要看到解除 perfect-refactor blocker 所需的最小授权、凭据、服务证据和发布证据。
- Release maintainer：需要一个单一 artifact 来判定 perfect-refactor claim 为什么仍被阻塞。
- Future implementation agent：需要知道哪些 blockers 可由代码/fixture 推进，哪些必须等待真实外部决策。

## 5. Behavior Contract

Unblock packet 必须 fail-closed。只要 perfect-refactor claim 仍为 false，packet `status` 必须是 `blocked`，`auto_resolvable` 必须是 false。packet 必须从既有 gate artifacts 派生，不允许通过缺省值隐藏 blocker。provider blockers 同时出现在 source accounting 和 external authority 时，决策项按 item_id 去重；source-only config blockers 仍单独保留。publication blockers 必须要求 credentials、release authority、legal/brand approval 和 external URL/digest evidence。current-upstream blocker 必须要求 same-ref rebaseline evidence 或明确保持 frozen-baseline claim。

### 5.1 Required Reading

- docs/specs/tasks/task-19.4-external-authority-blocker-waiver-gate.md
- docs/specs/tasks/task-20.1-source-provider-accounting-reconciliation.md
- docs/specs/tasks/task-20.2-perfect-refactor-claim-contract.md
- docs/specs/tasks/task-21.1-upstream-distribution-target-gate.md
- docs/compatibility/matrix.md
- docs/compatibility/target-policy.md
- docs/release.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-008-binary-first-multi-channel-release.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`promptfoo_rs::release::{build_perfect_refactor_unblock_packet, validate_perfect_refactor_unblock_packet, write_perfect_refactor_unblock_packet, PerfectRefactorUnblockInputs, PerfectRefactorUnblockItem, PerfectRefactorClaimContract}`、`serde_json`、`std::fs`、`std::path::Path`。
- Tooling commands：`bash scripts/release/perfect-refactor-unblock-packet.sh`、adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke。

### 5.3 函数签名

- `build_perfect_refactor_unblock_packet(inputs: PerfectRefactorUnblockInputs) -> PerfectRefactorUnblockPacket`
- `validate_perfect_refactor_unblock_packet(packet: &PerfectRefactorUnblockPacket) -> PerfectRefactorUnblockValidation`
- `write_perfect_refactor_unblock_packet(packet: &PerfectRefactorUnblockPacket, path: &Path) -> Result<(), ReleaseError>`

## 6. Acceptance Criteria

- [x] **AC1** (task-20.2): packet mirrors `perfect-refactor-claim.json` and keeps `perfect_refactor_claim_allowed=false` while claim blockers remain.
- [x] **AC2** (task-20.1 / task-19.4): packet deduplicates provider source blockers already represented by external authority items, while preserving source-only config blockers.
- [x] **AC3** (ADR-008 / docs/release.md): publication unblock items require credentials, release authority, legal/brand approval, and external URL/digest evidence; dry-run installability cannot mark them ready.
- [x] **AC4** (task-21.1): packet includes current-upstream rebaseline requirement when repository-current perfect claim remains blocked by distribution target drift.
- [x] **AC5** (PRD §Success Metrics): runtime smoke and release candidate include the packet; docs/audit/matrix describe it as a blocker handoff artifact, not completion.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-22.1.1 | TEST-22.1.1 | tests/perfect_refactor_unblock_packet.rs | install, typecheck, unit-test, build | Done |
| AC2 | SCEN-22.1.1 | TEST-22.1.2 | tests/perfect_refactor_unblock_packet.rs | install, typecheck, unit-test, integration, coverage, build | Done |
| AC3 | SCEN-22.1.1 | TEST-22.1.3 | tests/perfect_refactor_unblock_packet.rs | install, typecheck, unit-test, e2e, build | Done |
| AC4 | SCEN-22.1.1 | TEST-22.1.4 | tests/perfect_refactor_unblock_packet.rs | install, typecheck, unit-test, runtime-smoke, build | Done |
| AC5 | SCEN-22.1.1 | TEST-22.1.5 | tests/perfect_refactor_unblock_packet.rs | install, lint, typecheck, unit-test, integration, e2e, coverage, runtime-smoke, build | Done |

## 8. Risks

- 该 packet 可能被误读为 waiver；必须在 schema、status、docs 中明确 `blocked` 与 `auto_resolvable=false`。
- 去重逻辑若按 category 而不是 item_id 处理，可能隐藏 source-only config blockers；测试必须覆盖 provider overlap 和 config-only rows。
- publication evidence 只能由真实外部 URL/digest 和授权证明解除；本 task 不得执行 publish。

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

- **完成日期**：2026-05-31
- **改动文件**：
  - `docs/prds/promptfoo-rs.prd.md`
  - `docs/s2v-adapter.md`
  - `docs/specs/phases/phase-22-perfect-refactor-unblock-packet.md`
  - `docs/specs/tasks/task-22.1-authority-unblock-packet-gate.md`
  - `docs/superpowers/plans/2026-05-31-perfect-refactor-unblock-packet.md`
  - `test/features/perfect-refactor-parity.feature`
  - `tests/perfect_refactor_unblock_packet.rs`
  - `src/release.rs`
  - `scripts/release/perfect-refactor-unblock-packet.sh`
  - `scripts/release/runtime-smoke.sh`
  - `docs/release.md`
  - `docs/compatibility/matrix.md`
  - `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`
- **commit 列表**：
  - `4b69d72 docs(spec): add phase 22 perfect refactor unblock packet`
  - `dd1e9aa test(release): add SCEN-22.1.1 perfect refactor unblock RED tests`
  - `0b0739a feat(release): add perfect refactor unblock packet gate`
- **§9 Verification 结果**：
  - install: PASS — `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"` 于 2026-05-31 执行，adapter Install 通过。
  - lint: PASS — `bash scripts/release/lint.sh` 通过。
  - typecheck: PASS — `cargo check --workspace`、viewer typecheck、npm typecheck 通过。
  - unit-test: PASS — `cargo test --workspace`、viewer tests、npm tests 通过，包含 `tests/perfect_refactor_unblock_packet.rs` 5/5 通过。
  - integration: PASS — adapter Integration tests 通过。
  - e2e: PASS — adapter E2E tests 通过。
  - coverage: PASS — adapter Coverage tests 通过，`s2v_coverage_threshold_guard "$TASK_SPEC"` 通过。
  - build: PASS — adapter Build 通过。
  - runtime-smoke: PASS — adapter Runtime smoke 通过，并生成 `target/release-gates/perfect-refactor-unblock-packet.json`。
- **剩余风险 / 未做项**：该 task 只生成 blocker handoff artifact，不提供真实 provider credentials、账号、私有服务权限、法律/品牌授权、公开发布 URL/digest 或 current-upstream same-ref rebaseline 证据；因此 `perfect_refactor_claim_allowed` 必须继续保持 false。
- **下游 task 影响**：后续若用户提供授权、凭据、外部 URL/digest、正式 waiver 或 current-upstream same-ref 证据，可逐项消费 `perfect-refactor-unblock-packet.json` 中的 decision items；在这些证据缺失时，完美重构目标仍受外部阻塞约束。
