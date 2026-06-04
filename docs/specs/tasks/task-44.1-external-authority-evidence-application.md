# Task 44.1: external-authority-evidence-application

**Status**: Draft
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 44 - public-stable-release-authority-closure
**Dependencies**: task-43.1-authority-decision-manifest-gate, task-42.1-current-latest-2ca16c-head-refresh

## 1. Background

The current unblock packet requires real decisions for product authority, product-service authority, private-service access, account authority, credentials, and current-target policy. These cannot be inferred from PRD/ADR/code and must not be auto-resolved by an agent. 依据 PRD §Current Latest Rebaseline Addendum、ADR-011、task 37.1、task 43.1。

## 2. Goal

Apply real external authority evidence or formal waivers to current-latest provider/config/current-target decision items without storing secrets and without weakening release gates.

## 3. Scope

### In Scope

- Authority decision manifest rows for current-latest provider/config/current-target items.
- Evidence references, approval IDs, public/private documentation links, and waiver metadata.
- Release gate updates that consume validated decisions.

### Out Of Scope

- Guessing or inventing product/service authority.
- Committing API keys, provider tokens, private service credentials, or account secrets.
- Publishing artifacts.
- Claiming bug-free behavior.

## 4. Users / Actors

- Product owner: approves product-authority and product-service-authority rows.
- Credential/account owner: approves account and credential rows.
- Service owner: approves private-service rows.
- Maintainer: decides current-target policy and formal waiver scope.

## 5. Behavior Contract

Each authority row can move from unresolved only when Phase 43's manifest validator accepts real evidence or a formal waiver. If any required evidence is missing, expired, ambiguous, mock-only, or secret-bearing, the row remains blocked. Aggregate perfect-refactor claim remains false until every gate agrees.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/compatibility/matrix.md
- target/release-gates/perfect-refactor-unblock-packet.json
- docs/specs/tasks/task-43.1-authority-decision-manifest-gate.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `serde_json::Value`, `std::fs`.
- Shell/tooling commands: adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke; Phase 43 authority decision validator.

### 5.3 函数签名

- `apply_authority_decisions(manifest: &Value, release_gates: &Value) -> AuthorityApplicationReport`
- `AuthorityApplicationReport::remaining_blockers(&self) -> Vec<String>`

## 6. Acceptance Criteria

- [ ] **AC1** (ADR-011): every product/account/private-service/credential/current-target item has validated evidence or formal waiver metadata.
- [ ] **AC2** (PRD §Security): no committed artifact contains provider secrets, API keys, account tokens, or private credentials.
- [ ] **AC3** (ADR-009): unresolved or invalid evidence keeps its item release-blocking.
- [ ] **AC4** (PRD §Current Latest Rebaseline Addendum): perfect-refactor claim remains false unless all current-latest gates agree on the same target packet and authority status.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-44.1.1 | TEST-44.1.1 | tests/external_authority_evidence_application.rs | install, lint, typecheck, unit-test, integration, build | Not Started |
| AC2 | SCEN-44.1.1 | TEST-44.1.2 | tests/external_authority_evidence_application.rs | install, lint, typecheck, unit-test, e2e, build | Not Started |
| AC3 | SCEN-44.1.1 | TEST-44.1.3 | tests/external_authority_evidence_application.rs | install, typecheck, unit-test, runtime-smoke, build | Not Started |
| AC4 | SCEN-44.1.1 | TEST-44.1.4 | tests/external_authority_evidence_application.rs | install, lint, typecheck, unit-test, coverage, runtime-smoke, build | Not Started |

## 8. Risks

- This task is blocked without real external decisions; do not convert Draft to Ready until evidence exists.
- Waivers can change public release scope and must be visible in release docs.
- Secret leakage is a release-blocking security failure.

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

- **完成日期**：无（Draft，等待真实外部 evidence）
- **改动文件**：无（未实施）
- **commit 列表**：无（未实施）
- **§9 Verification 结果**：
  - install: 未执行（Draft）
  - lint: 未执行（Draft）
  - typecheck: 未执行（Draft）
  - unit-test: 未执行（Draft）
  - integration: 未执行（Draft）
  - e2e: 未执行（Draft）
  - coverage: 未执行（Draft）
  - build: 未执行（Draft）
  - runtime-smoke: 未执行（Draft）
- **剩余风险 / 未做项**：需要真实产品/账号/凭据/私有服务/current-target policy evidence 或正式 waiver。
- **下游 task 影响**：Task 44.2 和最终 public stable release claim 仍等待本 task 的 evidence。
