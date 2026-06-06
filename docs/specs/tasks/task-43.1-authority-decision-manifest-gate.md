# Task 43.1: authority-decision-manifest-gate

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 43 - authority-evidence-intake-gates
**Dependencies**: task-37.1-current-latest-unblock-packet-refresh, task-19.4-external-authority-blocker-waiver-gate

## 1. Background

The unblock packet currently reports `status=blocked`, `auto_resolvable=false`, and `required_user_decision_count=31`. Decision items are grouped across current-latest golden provider/config authority, current-target policy, and publication authority. Without a validated intake artifact, future updates can accidentally turn unresolved external authority gaps into local readiness. 依据 PRD §Current Latest Rebaseline Addendum、ADR-009、ADR-011、task 37.1、docs/audits/promptfoo-current-stage-full-verification-audit-2026-06-03.md。

## 2. Goal

Add a machine-readable authority decision manifest and validation gate that requires every non-auto-resolvable current-latest decision to remain unresolved, carry real evidence, or carry a formal waiver with owner, date, scope, expiration, rationale, and release impact.

## 3. Scope

### In Scope

- New `docs/compatibility/authority-decisions.md` or JSON manifest equivalent.
- Validation logic under `scripts/release/` or Rust compatibility gate tests.
- Tests proving unresolved entries remain blocked and waivers cannot silently allow perfect-refactor claims.
- Updates to current-latest quality or unblock packet wiring only if needed for manifest consumption.

### Out Of Scope

- Providing real provider credentials, account access, private service access, or product authority.
- Approving waivers on behalf of the user or maintainer.
- Publishing artifacts or changing `perfect_refactor_claim_allowed` to true.

## 4. Users / Actors

- Maintainer: records real approvals or formal waivers.
- Compatibility reviewer: verifies each external blocker has an auditable decision state.
- Future implementation agent: consumes a stable manifest without storing secrets.

## 5. Behavior Contract

The manifest must enumerate each non-auto-resolvable decision item from `perfect-refactor-unblock-packet.json`. Each item must have one of `unresolved`, `evidence-provided`, or `waived-with-boundary`. Only `evidence-provided` entries with non-secret evidence references and `waived-with-boundary` entries with owner/date/scope/expiration/risk/release impact can stop blocking their specific item. Aggregate perfect-refactor claims remain false unless all gates and all decision items are ready.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/compatibility/matrix.md
- target/release-gates/perfect-refactor-unblock-packet.json
- target/release-gates/external-authority-blockers.json
- docs/specs/tasks/task-19.4-external-authority-blocker-waiver-gate.md
- docs/specs/tasks/task-37.1-current-latest-unblock-packet-refresh.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `serde_json::Value`, `std::fs`, `std::collections::BTreeMap`.
- Shell/tooling commands: adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke; optional `bash scripts/release/authority-decisions.sh`.

### 5.3 函数签名

- `validate_authority_decisions(unblock_packet: &Value, manifest: &Value) -> AuthorityDecisionReport`
- `AuthorityDecisionReport::perfect_refactor_decision_ready(&self) -> bool`
- Shell contract: `bash scripts/release/authority-decisions.sh`

## 6. Acceptance Criteria

- [x] **AC1** (task 37.1): every `decision_items[]` row in `perfect-refactor-unblock-packet.json` has exactly one corresponding authority decision manifest row.
- [x] **AC2** (ADR-011): unresolved or mock-only evidence rows remain release-blocking and keep `perfect_refactor_claim_allowed=false`.
- [x] **AC3** (ADR-009): waiver rows require owner, approval date, scope, expiration/review date, rationale, and release impact before they can be counted.
- [x] **AC4** (PRD §Security): the manifest stores no real secrets and only records redacted references, approval IDs, URLs, digests, or evidence artifact paths.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-43.1.1 | TEST-43.1.1 | tests/authority_decision_manifest_gate.rs | install, lint, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-43.1.1 | TEST-43.1.2 | tests/authority_decision_manifest_gate.rs | install, typecheck, unit-test, runtime-smoke, build | Done |
| AC3 | SCEN-43.1.1 | TEST-43.1.3 | tests/authority_decision_manifest_gate.rs | install, lint, typecheck, unit-test, coverage, build | Done |
| AC4 | SCEN-43.1.1 | TEST-43.1.4 | tests/authority_decision_manifest_gate.rs | install, lint, typecheck, unit-test, integration, e2e, build | Done |

## 8. Risks

- A manifest can create false confidence if unresolved entries are hidden; AC1 and AC2 require exact row matching and fail-closed behavior.
- Waivers without expiration can become permanent unreviewed gaps; AC3 requires review metadata.
- Evidence references can leak secrets if raw tokens are stored; AC4 forbids real secrets in repo.

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

- **完成日期**：2026-06-06
- **改动文件**：
  - `tests/authority_decision_manifest_gate.rs`
  - `src/release.rs`
  - `docs/compatibility/authority-decisions.json`
  - `scripts/release/authority-decisions.sh`
  - `scripts/release/runtime-smoke.sh`
  - `scripts/release/integration.sh`
  - `docs/specs/tasks/task-43.1-authority-decision-manifest-gate.md`
- **commit 列表**：
  - `07bfbe4` `test(authority): add SCEN-43.1.1 authority decision manifest gate tests`
  - `acd632e` `feat(authority): wire authority decisions gate into runtime smoke`
  - `81e8acc` `refactor(authority): format authority decision gate sources`
  - `5908d0d` `refactor(authority): fix clippy collapsible_match in secret scan`
  - `docs commit` `docs(spec): 回填 task-43.1 §10 Completion Notes + Status → Done`
- **§9 Verification 结果**：
  - install: PASS — `cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过
  - lint: PASS — `bash scripts/release/lint.sh`（含 clippy）通过
  - typecheck: PASS — `cargo check --workspace`、viewer/npm typecheck 通过
  - unit-test: PASS — `cargo test --workspace`、viewer/npm test 通过；TEST-43.1.1 ~ TEST-43.1.4 通过
  - integration: PASS — `bash scripts/release/integration.sh` 含 `authority_decision_manifest_gate` 通过
  - e2e: PASS — `bash scripts/release/e2e.sh` 通过
  - coverage: PASS — `bash scripts/release/coverage.sh` 通过
  - build: PASS — `cargo build --release` 通过
  - runtime-smoke: PASS — `bash scripts/release/runtime-smoke.sh` 生成 `authority-decisions-gate.json` 且 `perfect_refactor_decision_ready=false`
- **剩余风险 / 未做项**：32 条 authority decision 仍为 `unresolved`；真实外部授权证据或正式 waiver 需 maintainer 在 Phase 44 前手动填入 manifest。`unclassified:src-evaluator-inmemorystore` 仍在 unblock packet 中等待分类。
- **下游 task 影响**：task 43.2 可复用同一 intake/fail-closed 模式处理 publication evidence；task 44.1 依赖本 manifest schema 应用真实 evidence 或 waiver。
