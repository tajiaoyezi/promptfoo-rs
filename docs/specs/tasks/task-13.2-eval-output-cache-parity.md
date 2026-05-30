# Task 13.2: eval-output-cache-parity

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 13 — cli-output-eval-parity
**Dependencies**: task-13.1-command-flag-parity

## 1. Background

PRD happy path requires `eval -c ... --output results.jsonl --output junit.xml`, cache/resume/retry, and CI-compatible exit code. Existing tests prove local contracts, not upstream diff parity. Basis: PRD §User Flow / §Compatibility Matrix, ADR-003, ADR-004.

## 2. Goal

Make common eval migration workflows pass golden diff for config loading, output files, cache/resume/retry state, stdout/stderr, and exit code.

## 3. Scope

### In Scope

- src/cli.rs
- src/eval/
- src/cache/
- src/output/
- src/results/
- tests/eval_output_cache_parity.rs
- compatibility/fixtures/eval/

### Out Of Scope

- Long-tail providers/assertions beyond fixtures chosen for P0 eval migration.
- Real network model calls; use mock providers.

## 4. Users / Actors

- AI application developer: migrates existing promptfooconfig files.
- CI maintainer: consumes JSONL/JUnit/SARIF and exit codes.

## 5. Behavior Contract

For P0 eval fixtures, promptfoo-rs must produce upstream-matching or classified stdout/stderr/exit code/result artifacts after normalization. Cache/resume/retry state must be deterministic and inspectable.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/decisions/adr-003-streaming-jsonl-sqlite-store.md
- docs/decisions/adr-004-cli-output-schema-protocol.md
- docs/specs/tasks/task-12.2-executable-upstream-rs-runner.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`serde_json`、`serde_yaml`、内部模块 `eval`、`cache`、`output`、`results`。

### 5.3 函数签名

- `run_eval_cli(args: EvalCliArgs) -> Result<CliRunArtifacts, CliError>`
- `write_requested_outputs(envelope: &EvalEnvelope, outputs: &[OutputTarget]) -> Result<Vec<OutputArtifact>, OutputError>`
- `resume_eval_from_cache(config: &EvalConfig, cache: &CacheStore) -> Result<EvalEnvelope, EvalError>`

## 6. Acceptance Criteria

- [ ] **AC1** (PRD §User Flow): `eval -c` supports output targets used by JSONL/JUnit/CSV/SARIF/HTML P0/P1 fixtures.
- [ ] **AC2** (PRD §Compatibility Matrix): cache/resume/retry/concurrency/delay fixture artifacts match upstream or are classified.
- [ ] **AC3** (ADR-004): stdout/stderr/exit code are stable and golden diffed for success, assertion failure, provider failure, invalid config.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-13.2.1 | TEST-13.2.1 | tests/eval_output_cache_parity.rs | install, typecheck, unit-test, manual | Not Started |
| AC2 | SCEN-13.2.1 | TEST-13.2.2 | tests/eval_output_cache_parity.rs | install, typecheck, unit-test, manual | Not Started |
| AC3 | SCEN-13.2.1 | TEST-13.2.3 | tests/eval_output_cache_parity.rs | install, typecheck, unit-test, manual | Not Started |

## 8. Risks

- Upstream output formats may include dynamic fields; normalization must be shared with Phase 12.
- Large eval performance must be validated later in Phase 15.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Manual**: inspect at least one eval fixture artifact set from Phase 12 runner.

## 10. Completion Notes

- **完成日期**：<TBD-after-impl>
- **改动文件**：<TBD-after-impl>
- **commit 列表**：<TBD-after-impl>
- **§9 Verification 结果**：<TBD-after-impl>
- **剩余风险 / 未做项**：<TBD-after-impl>
- **下游 task 影响**：<TBD-after-impl>
