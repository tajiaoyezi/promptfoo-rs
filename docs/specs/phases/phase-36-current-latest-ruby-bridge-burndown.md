# Phase 36: current-latest-ruby-bridge-burndown

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Burn down the final local current-latest `script-bridge` blockers by adding deterministic Ruby subprocess evidence for `src/ruby/rubyUtils.ts` and `src/ruby/wrapper.ts`. Ruby rows may become P0 native fixture evidence only after a real Ruby runtime is invoked under the existing explicit script authorization sandbox and Rust/shell artifact classification agrees. 依据 PRD §Core Capabilities / §Compatibility Matrix、adapter §Constraints、ADR-005、ADR-009、ADR-011、task 9.1、task 35.1、Phase 35 §9。

## 2. Business Value

Phase 35 leaves 25 current-latest P0 golden blockers: config=7, provider=16, script-bridge=2. The two script-bridge blockers are local Ruby runtime rows, not external provider or account authority. A RubyInstaller 3.4 runtime is available locally at `C:\Ruby34-x64\bin\ruby.exe`; proving the Ruby wrapper contract removes the remaining local script-bridge blockers while preserving config/provider external authority, publication, current-target, and impossible zero-bug claim blockers.

## 3. Scope / Modules

`src/script_bridge/mod.rs`, `src/script_bridge/ruby.rs`, `src/compatibility/inventory.rs`, `scripts/release/current-latest-source-inventory.sh`, `tests/current_latest_ruby_bridge_burndown.rs`, `tests/current_latest_script_prompt_python_bridge_burndown.rs`, `target/release-gates/current-latest-source-inventory.json`, `target/release-gates/current-latest-matrix.json`, `target/release-gates/current-latest-golden-corpus.json`, `target/release-gates/current-latest-quality.json`, `docs/compatibility/matrix.md`, `docs/s2v-adapter.md`, `docs/prds/promptfoo-rs.prd.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 36.1 | current-latest-ruby-bridge-burndown | ../tasks/task-36.1-current-latest-ruby-bridge-burndown.md | Ready | 实现 Ruby bridge deterministic subprocess evidence 并将 2 个 Ruby script-bridge blockers 转为 native fixture evidence |

## 5. Dependencies

Depends on Phase 24 current-latest artifacts, Phase 35 script prompt/Python bridge burndown, task 9.1 script bridge sandbox, ADR-005, ADR-009, and ADR-011. This phase requires a local Ruby runtime (`ruby` on PATH or `C:\Ruby34-x64\bin\ruby.exe`) but does not require real provider credentials, private services, legal/brand confirmation, or publication authority.

## 6. Phase Acceptance Criteria

- [ ] Ruby bridge wrapper executes only with explicit authorization, exchanges deterministic JSON stdin/stdout payloads, captures stderr, enforces timeout/stdin/env boundaries through ScriptBridge, and rejects unauthorized execution.
- [ ] Ruby worker-pool behavior executes authorized Ruby subprocess calls with deterministic result ordering and propagates timeout/I/O/invalid-JSON errors.
- [ ] Current-latest `src/ruby/rubyUtils.ts` and `src/ruby/wrapper.ts` rows have P0 native fixture evidence in both Rust and shell artifacts.
- [ ] Current-latest script-bridge blockers drop to zero, total current-latest blockers drop from 25 to 23, and only config/provider external-authority blockers remain.
- [ ] `perfect_refactor_claim_allowed=false` remains because config/provider external authority, publication, current-target, and impossible zero-bug claim blockers still exist.

## 7. Phase Risks

- Ruby runtime availability is an environment dependency; tests must fail closed if no Ruby executable is available.
- Ruby evidence must stay local and deterministic; no fixture may require network, provider credentials, private SDKs, or account-level state.
- Removing script-bridge blockers does not prove config/provider external authority, publication, current-target readiness, or impossible zero-bug claims.

## 8. Definition of Done

Task 36.1 spec is Done, phase §6 smoke passes with `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`, script-bridge blockers are zero, total blockers are 23, and perfect-refactor claim remains blocked by remaining external authority evidence gaps.

## 9. Phase Completion Notes

- **完成日期**：<TBD-after-impl>
- **Phase smoke**：<TBD-after-impl>
- **Artifact evidence**：<TBD-after-impl>
- **保留边界**：<TBD-after-impl>
