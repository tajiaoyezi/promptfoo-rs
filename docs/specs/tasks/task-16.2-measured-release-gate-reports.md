# Task 16.2: measured-release-gate-reports

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 16 — parity-proof-hardening
**Dependencies**: task-15.2-performance-security-observability-gates, task-16.1-cli-command-behavior-closure

## 1. Background

复审发现 `scripts/release/runtime-smoke.sh` 直接写入固定 `performance.json`、`security.json`、`release-candidate.json`，其中 observed performance、security status 和 `stable_allowed=true` 不是从本次执行测量或 gate summary 派生。task-15.2 已建立 release gate API，但 §10 也登记了 deterministic local evidence 不能代表真实 release measurement。依据 PRD §Success Metrics、ADR-005、ADR-007、task-15.2 §10。

## 2. Goal

将 runtime smoke 报告改为从本次命令执行、host metadata、security checks 和 release gate summary 派生；任何缺失或超阈值都必须阻断 stable。

## 3. Scope

### In Scope

- `scripts/release/runtime-smoke.sh`
- `src/release.rs`
- `tests/measured_release_gate_reports.rs`
- `tests/performance_security_observability_gates.rs`
- `tests/runtime_smoke.rs`
- `target/release-gates/*.json` schema expectations

### Out Of Scope

- 不测量真实网络 provider latency。
- 不引入外部 SaaS telemetry。
- 不承诺 line coverage 阈值；coverage 模式仍由 adapter §Coverage 判读规则控制。

## 4. Users / Actors

- Release maintainer：需要 release candidate 报告反映真实 gate outcome。
- 企业安全 reviewer：需要 no-upload、redaction、default-deny 证据来自检查结果。
- AI infra 团队：需要 performance report 记录实际命令、host 和阈值判定。

## 5. Behavior Contract

runtime smoke 必须执行并记录实际 CLI cold-start、mock eval duration、host metadata、安全默认值检查、redaction/no-upload evidence，并用 `release_candidate_gate` 或等价逻辑派生 `stable_allowed`。报告不得包含无来源固定 observed 值；若测量不可用，报告应为 blocked/prerelease，而不是稳定通过。

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/s2v-adapter.md
- docs/specs/tasks/task-15.2-performance-security-observability-gates.md
- docs/decisions/adr-005-explicit-script-authorization.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`std::process`、`std::time`、`serde_json`、内部模块 `release`、`script_bridge`、`compatibility`。
- Tooling commands：adapter §Commands Install / Typecheck / Unit Test / Runtime smoke / Build。

### 5.3 函数签名

- `measure_cli_cold_start(binary: &Path) -> Result<MeasuredCommand, ReleaseError>`
- `measure_mock_eval(binary: &Path, cases: usize) -> Result<MeasuredCommand, ReleaseError>`
- `write_runtime_smoke_reports(config: RuntimeSmokeConfig) -> Result<ReleaseCandidateGateSummary, ReleaseError>`

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Success Metrics): performance report records measured command evidence for CLI cold start and 1000 mock eval cases, plus threshold comparison.
- [x] **AC2** (ADR-005): security report is derived from default-deny/redaction/no-upload checks and blocks stable when evidence is missing.
- [x] **AC3** (ADR-007): release-candidate report derives `decision` and `stable_allowed` from gate statuses, not from a fixed JSON literal.
- [x] **AC4** (S2V adapter): runtime-smoke script fails closed if required measured artifacts are missing, malformed, or marked blocked.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-16.2.1 | TEST-16.2.1 | tests/measured_release_gate_reports.rs | install, typecheck, unit-test, build, runtime-smoke | Done |
| AC2 | SCEN-16.2.1 | TEST-16.2.2 | tests/measured_release_gate_reports.rs | install, typecheck, unit-test, build, runtime-smoke | Done |
| AC3 | SCEN-16.2.1 | TEST-16.2.3 | tests/measured_release_gate_reports.rs | install, typecheck, unit-test, build, runtime-smoke | Done |
| AC4 | SCEN-16.2.1 | TEST-16.2.4 | tests/runtime_smoke.rs | install, typecheck, unit-test, build, runtime-smoke | Done |

## 8. Risks

- Windows/Linux/macOS 对 peak memory 的可观测能力不同；若无法跨平台稳定测量，必须将 method/status 写入 report 并 fail closed 或登记 ADR。
- Debug build 性能可能不代表 release build；报告必须标明 profile。
- 真实环境抖动可能导致 performance gate 红；这是 release gate 的预期行为，不应通过固定值掩盖。

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：2026-05-31
- **改动文件**：
  - `scripts/release/runtime-smoke.sh`
  - `tests/measured_release_gate_reports.rs`
  - `docs/s2v-adapter.md`
  - `docs/specs/phases/phase-16-parity-proof-hardening.md`
  - `docs/specs/tasks/task-16.2-measured-release-gate-reports.md`
- **commit 列表**：
  - `3e4a6fc` `docs(spec): task-16.2 进入实施 (Status: Ready → In Progress)`
  - `c3e95a9` `test(release): 加 SCEN-16.2.1 的 measured gate RED 测试`
  - `c4bbc15` `feat(release): 生成 measured runtime smoke gate reports`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-16.2.1 ~ TEST-16.2.4 通过。
  - build: PASS — helper 执行 `cargo build --workspace`、viewer/npm `pnpm build` 通过。
  - runtime-smoke: PASS — `bash scripts/release/runtime-smoke.sh` 构建 release binary，运行 CLI/eval/performance/security/package smoke，并生成 measured `target/release-gates/performance.json`、`security.json`、`release-candidate.json`。
- **剩余风险 / 未做项**：memory baseline 在 Windows 通过 PowerShell `PeakWorkingSet64` 采样，Unix fallback 依赖 `/usr/bin/time -f %M`；如 CI runner 缺少可用采样工具，runtime smoke 会记录 fallback evidence，后续可用 ADR 引入跨平台专用测量器。性能数据仍是本地 mock eval，不代表真实网络 provider latency。
- **下游 task 影响**：task-16.3 可把 real upstream smoke artifacts 纳入 runtime/release gate；Phase 16 phase smoke 将复用 measured release reports，而不再依赖固定 JSON literals。
