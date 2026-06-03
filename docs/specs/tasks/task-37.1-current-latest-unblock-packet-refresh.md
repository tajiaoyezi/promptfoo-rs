# Task 37.1: current-latest-unblock-packet-refresh

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 37 - current-latest-unblock-packet-refresh
**Dependencies**: task-22.1-authority-unblock-packet-gate, task-24.4-current-latest-exhaustive-quality-gate, task-36.1-current-latest-ruby-bridge-burndown

## 1. Background

Phase 36 proves the local Ruby bridge evidence and reduces current-latest golden blockers to 23 (`config=7`, `provider=16`). The existing `perfect-refactor-unblock-packet.json` is still generated from older `source-inventory-evidence.json` and `external-authority-blockers.json` counts (`source_p0_accounting_blocker_count=22`, `external_authority_blocker_count=21`, `required_user_decision_count=29`). It does not make the Phase 36 current-latest blocker list the authoritative decision source. 依据 PRD §Current Latest Rebaseline Addendum / §Compatibility Matrix、ADR-008、ADR-009、ADR-011、task 22.1、task 24.4、task 36.1。

## 2. Goal

Update the unblock packet gate so the packet is current-latest aware: it must enumerate current-latest golden blockers from `current-latest-golden-corpus.json` and `current-latest-matrix.json`, preserve current-target and publication decisions, and keep the perfect-refactor claim blocked until real evidence or approved waivers exist.

## 3. Scope

### In Scope

- `scripts/release/perfect-refactor-unblock-packet.sh`
- `scripts/release/runtime-smoke.sh`
- `target/release-gates/perfect-refactor-unblock-packet.json`
- `tests/current_latest_unblock_packet.rs`
- `docs/compatibility/matrix.md`
- `docs/prds/promptfoo-rs.prd.md`
- `docs/s2v-adapter.md`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不解除 config/provider external-authority blockers。
- 不发布 crate/npm/Docker/Homebrew/GitHub artifacts。
- 不提供真实 provider credentials、账号权限、private service、法律/品牌或产品授权。
- 不把 current-latest golden blockers、publication blockers 或 current-target blockers 标为 waived/ready。

## 4. Users / Actors

- Maintainer: needs an exact current-latest decision packet after local blockers are burned down.
- Future implementation agent: needs to know which remaining items cannot be code-only resolved.
- Release reviewer: needs packet counts and decision items to reconcile with current-latest quality artifacts.

## 5. Behavior Contract

When current-latest artifacts exist, `perfect-refactor-unblock-packet.sh` must treat `current-latest-golden-corpus.json`, `current-latest-matrix.json`, and `current-latest-quality.json` as the authoritative blocker source. The packet must add `target_scope=current-latest`, `current_latest_golden_blocker_count`, `current_latest_external_authority_blocker_count`, and `current_latest_required_decision_count`. Every current-latest golden blocker must appear as a decision item with `source_artifact` pointing to a current-latest artifact, `source_reference` from the matrix row when available, a non-empty `required_actor`, `required_evidence`, `release_impact`, `safe_local_fallback`, and `auto_resolvable=false`. Current-target and publication decisions must remain present. Legacy source/external counts may remain for backward compatibility but must not be the only packet evidence.

### 5.1 Required Reading

- docs/specs/tasks/task-22.1-authority-unblock-packet-gate.md
- docs/specs/tasks/task-24.4-current-latest-exhaustive-quality-gate.md
- docs/specs/tasks/task-36.1-current-latest-ruby-bridge-burndown.md
- docs/specs/phases/phase-36-current-latest-ruby-bridge-burndown.md
- docs/compatibility/matrix.md
- docs/decisions/adr-008-release-channel-and-publication-policy.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust test modules: `std::fs`, `std::path::{Path, PathBuf}`, `std::process::Command`, `serde_json::{json, Value}`.
- Tooling commands: `bash scripts/release/perfect-refactor-unblock-packet.sh`, `bash scripts/release/runtime-smoke.sh`, `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- Shell contract: `GATE_DIR=<dir> bash scripts/release/perfect-refactor-unblock-packet.sh`
- Shell helper behavior: `currentLatestGoldenDecisionItem(blocker, matrixRow) -> decision item`
- JSON contract: `perfect-refactor-unblock-packet.json.target_scope == "current-latest"` when current-latest artifacts exist.

## 6. Acceptance Criteria

- [ ] **AC1** (ADR-011 / task 24.4): packet records `target_scope=current-latest`, `current_latest_golden_blocker_count`, and `current_latest_required_decision_count` derived from current-latest artifacts.
- [ ] **AC2** (ADR-009): every current-latest golden blocker is represented as exactly one non-auto-resolvable decision item, with current-latest source artifact/reference evidence.
- [ ] **AC3** (ADR-008 / task 22.1): current-target and publication authority decisions remain explicit and require real evidence; dry-run or local fixture evidence cannot mark them ready.
- [ ] **AC4** (PRD §Success Metrics): `perfect_refactor_claim_allowed=false` and `status=blocked` remain while current-latest/config/provider/publication/current-target evidence is absent.
- [ ] **AC5** (task 36.1): packet counts reconcile with Phase 36 current-latest evidence: tracked smoke has 23 golden blockers and no script-bridge decisions.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-37.1.1 | TEST-37.1.1 | tests/current_latest_unblock_packet.rs | install, typecheck, unit-test, build | Spec Ready |
| AC2 | SCEN-37.1.1 | TEST-37.1.2 | tests/current_latest_unblock_packet.rs | install, lint, typecheck, unit-test, integration, build | Spec Ready |
| AC3 | SCEN-37.1.1 | TEST-37.1.3 | tests/current_latest_unblock_packet.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Spec Ready |
| AC4 | SCEN-37.1.1 | TEST-37.1.4 | tests/current_latest_unblock_packet.rs | install, lint, typecheck, unit-test, coverage, build | Spec Ready |
| AC5 | SCEN-37.1.1 | TEST-37.1.5 | tests/current_latest_unblock_packet.rs | install, lint, typecheck, unit-test, integration, e2e, coverage, runtime-smoke, build | Spec Ready |

## 8. Risks

- Updating the packet can look like completion if docs omit the fail-closed boundary; AC4 requires status and claim to remain blocked.
- Deduplicating against older external authority artifacts can hide current-latest provider rows; AC2 requires current-latest golden blockers to be the source.
- Runtime smoke must still pass locally; no new network, credential, or publication action is allowed.

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

- **完成日期**：<TBD-after-impl>
- **改动文件**：<TBD-after-impl>
- **commit 列表**：<TBD-after-impl>
- **§9 Verification 结果**：<TBD-after-impl>
- **剩余风险 / 未做项**：<TBD-after-impl>
- **下游 task 影响**：<TBD-after-impl>
