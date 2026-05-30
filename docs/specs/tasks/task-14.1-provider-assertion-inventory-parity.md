# Task 14.1: provider-assertion-inventory-parity

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 14 — provider-assertion-redteam-parity
**Dependencies**: task-12.3-golden-diff-ci-release-gate

## 1. Background

Audit found local providers and assertions are representative subsets. PRD requires all documented providers/assertions be registered and P0/P1/P2 verified. Basis: PRD §Compatibility Matrix, ADR-001, ADR-009.

## 2. Goal

For every provider/assertion inventory item, implement native/bridge behavior or register unsupported/later with verification evidence, and ensure P0 items have fixtures.

## 3. Scope

### In Scope

- src/providers/
- src/assertions/
- src/script_bridge/
- compatibility/fixtures/providers/
- compatibility/fixtures/assertions/
- tests/provider_assertion_inventory_parity.rs
- docs/compatibility/matrix.md

### Out Of Scope

- Redteam plugins/strategies; task 14.2 owns those.
- Real model network calls; use mock servers/recorded responses.

## 4. Users / Actors

- AI application developer: depends on provider/assertion compatibility.
- Enterprise security reviewer: needs script bridge default-deny and redaction evidence.

## 5. Behavior Contract

Every provider/assertion item must have matrix status, implementation or explicit gap, fixture/snapshot evidence, and secret-free mock execution path. P0 missing implementation blocks stable release.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/tasks/task-11.2-item-level-capability-inventory.md
- docs/specs/tasks/task-11.3-compatibility-matrix-expansion.md
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-005-explicit-script-authorization.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`reqwest`、`serde_json`、内部模块 `providers`、`assertions`、`script_bridge`、`compatibility::matrix`。

### 5.3 函数签名

- `ProviderParityRegistry::from_inventory(inventory: &CapabilityInventory) -> ProviderParityRegistry`
- `AssertionParityRegistry::from_inventory(inventory: &CapabilityInventory) -> AssertionParityRegistry`
- `validate_provider_assertion_parity(matrix: &CapabilityMatrix, fixtures: &FixtureCorpus) -> ParityReport`

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Compatibility Matrix): every provider/assertion inventory item has matrix row, status, owner, verification, and gap reason when needed.
- [x] **AC2** (PRD §Core Capabilities): P0 provider/assertion fixtures pass golden diff or block stable release.
- [x] **AC3** (ADR-005): custom JS/TS/Python/Shell/Ruby assertion/provider boundaries are default-deny, allowlisted, timed out, and redacted.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-14.1.1 | TEST-14.1.1 | tests/provider_assertion_inventory_parity.rs | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-14.1.1 | TEST-14.1.2 | tests/provider_assertion_inventory_parity.rs | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-14.1.1 | TEST-14.1.3 | tests/provider_assertion_inventory_parity.rs | install, typecheck, unit-test, manual | Done |

## 8. Risks

- Some providers require private services or legal/brand confirmation; classify as blocked/P2 rather than guessing.
- Assertion semantics can be non-deterministic; model-graded assertions need recorded grader responses.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Manual**: review P0 provider/assertion missing count and P2 reason report.

## 10. Completion Notes

- **完成日期**：2026-05-30
- **改动文件**：
  - `src/compatibility/provider_assertion.rs`
  - `src/compatibility/mod.rs`
  - `tests/provider_assertion_inventory_parity.rs`
  - `docs/compatibility/matrix.md`
  - `docs/specs/tasks/task-14.1-provider-assertion-inventory-parity.md`
- **commit 列表**：
  - `0b0675c` `docs(spec): task-14.1 进入实施 (Status: Ready → In Progress)`
  - `4275019` `test(provider-assertion): 加 SCEN-14.1.1 的 3 个 RED 测试`
  - `869e623` `feat(provider-assertion): 实现 provider assertion inventory parity`
- **§9 Verification 结果**：
  - install: PASS（`s2v_verify_full` 抽取 §9 keys 后执行）
  - typecheck: PASS（`s2v_verify_full` 抽取 §9 keys 后执行；`cargo check --workspace` 单独通过）
  - unit-test: PASS（`s2v_verify_full` 抽取 §9 keys 后执行；`cargo test --test provider_assertion_inventory_parity` 为 3 passed / 0 failed，`cargo test --workspace` 通过）
  - manual: PASS（人工核对 P0 provider count=4、P0 assertion count=6、P0 missing fixture/blocker count=0、P2 provider/assertion missing reason count=0；P2 `provider:dynamic-registry` 保留 reason；JS/TS/Python/Shell/Ruby default-deny/allowlist/timeout/redaction 边界由 TEST-14.1.3 覆盖。非交互 helper 的 `/dev/tty` manual prompt 不可用，真实证据以本条 manual review 留痕）
- **剩余风险 / 未做项**：
  - 长尾 provider 仍按 ADR-009 登记为 P1/P2/later，未伪装成 Rust-native 完整实现。
  - TypeScript/Ruby 以 script boundary policy 登记；真实运行时适配仍需后续 bridge/release 环境验证。
- **下游 task 影响**：
  - task-14.2 可复用 provider/assertion parity report 的 matrix/fixture 聚合模式扩展到 redteam plugin/strategy。
  - Phase 15 release gate 可把 P0 missing fixture/blocker 与 P2 reason missing count 纳入稳定发布阻断条件。
