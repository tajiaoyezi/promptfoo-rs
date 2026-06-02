# Phase 35: current-latest-script-prompt-python-bridge-burndown

**Status**: In Progress
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Burn down locally provable current-latest script-backed prompt processor and Python script bridge blockers by adding deterministic Node/Python subprocess evidence on top of the existing explicit script authorization boundary. JavaScript, Python, and executable prompt processors plus `src/python/*` current-latest rows may become P0 native fixture evidence only after dedicated Rust behavior tests and Rust/shell artifact classification agree. Ruby rows remain blockers because this environment has no Ruby runtime and PRD/ADR do not authorize substituting another runtime. 依据 PRD §Core Capabilities / §Compatibility Matrix、adapter §Constraints、ADR-005、ADR-009、ADR-011、task 9.1、task 32.1、Phase 34 §9。

## 2. Business Value

Phase 34 leaves 33 current-latest P0 golden blockers: config=7, prompt-processing=3, provider=16, script-bridge=7. The prompt-processing blockers and five Python bridge blockers are local subprocess behavior that can be tested with explicit authorization, env allowlist, timeout, stdout/stderr, and JSON payload contracts. Proving these rows reduces local blocker noise while preserving Ruby runtime, external provider/config authority, publication, current-target, and impossible zero-bug claim blockers.

## 3. Scope / Modules

`src/script_bridge/mod.rs`, `src/script_bridge/prompt_processor.rs`, `src/script_bridge/python.rs`, `src/compatibility/inventory.rs`, `scripts/release/current-latest-source-inventory.sh`, `tests/current_latest_script_prompt_python_bridge_burndown.rs`, `target/release-gates/current-latest-source-inventory.json`, `target/release-gates/current-latest-matrix.json`, `target/release-gates/current-latest-golden-corpus.json`, `target/release-gates/current-latest-quality.json`, `docs/compatibility/matrix.md`, `docs/s2v-adapter.md`, `docs/prds/promptfoo-rs.prd.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 35.1 | current-latest-script-prompt-python-bridge-burndown | ../tasks/task-35.1-current-latest-script-prompt-python-bridge-burndown.md | Done | 实现 JS/Python/executable prompt processor 与 Python bridge deterministic subprocess evidence，并保留 Ruby blockers |

## 5. Dependencies

Depends on Phase 24 current-latest artifacts, Phase 32 local prompt processor split, Phase 34 phase smoke, task 9.1 script bridge sandbox, task 4.3 custom assertion contract, ADR-005, ADR-009, and ADR-011. This phase requires local Node/Python runtimes per adapter constraints, but does not require real provider credentials, private services, legal/brand confirmation, Ruby installation, or publication authority.

## 6. Phase Acceptance Criteria

- [ ] Script-backed JavaScript, Python, and executable prompt processors run only with explicit authorization, exchange deterministic JSON stdin/stdout payloads, capture stderr, apply env allowlist, and reject unauthorized execution.
- [ ] Python bridge wrapper and worker behavior execute authorized Python subprocess calls with JSON payloads, stderr capture, timeout/error propagation, and deterministic worker-pool result ordering.
- [ ] Current-latest JavaScript/Python/executable prompt processor rows have P0 native fixture evidence in Rust and shell artifacts, and prompt-processing blockers drop to zero.
- [ ] Current-latest `src/python/*` script-bridge rows have P0 native fixture evidence in Rust and shell artifacts; `src/ruby/*` rows remain explicit P0 blockers with runtime-authority reasons.
- [ ] Total current-latest blockers drop from 33 to 25, and `perfect_refactor_claim_allowed=false` remains because config/provider external authority and Ruby blockers still exist.

## 7. Phase Risks

- Treating Ruby source as proven without Ruby would hide a real runtime gap; Ruby rows must remain blockers until Ruby runtime evidence exists.
- Script execution must stay explicit-authorized; reducing blockers must not weaken default-deny, timeout, env allowlist, stderr capture, or secret redaction boundaries.
- Local Node/Python fixture evidence does not prove external provider/account behavior, publication authority, current-target drift, or the impossible claim of no latent bugs.

## 8. Definition of Done

Task 35.1 spec is Done, phase §6 smoke passes with `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`, prompt-processing blockers are zero, script-bridge blockers are Ruby-only, total blockers are 25, and perfect-refactor claim remains blocked by remaining evidence gaps.

## 9. Phase Completion Notes

- **完成日期**：<TBD-after-impl>
- **Phase smoke**：<TBD-after-impl>
- **Artifact evidence**：<TBD-after-impl>
- **保留边界**：<TBD-after-impl>
