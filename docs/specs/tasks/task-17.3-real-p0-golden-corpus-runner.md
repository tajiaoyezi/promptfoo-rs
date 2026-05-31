# Task 17.3: real-p0-golden-corpus-runner

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 17 — deep-upstream-parity-proof
**Dependencies**: task-17.1-frozen-source-inventory-extractor, task-17.2-cli-global-eval-redteam-parity, task-12.2-executable-upstream-rs-runner

## 1. Background

Phase 16 增加了一个真实 `promptfoo@0.121.13` echo smoke，但当前审计指出这不足以证明 PRD 的至少 50 个 P0 fixture release gate。现有 53 个 fixture manifest 证明“有元数据”，但不是 50 个真实 upstream-vs-rs run artifacts。依据 PRD §Success Metrics / §Compatibility Harness Design、ADR-007、docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md §P0 real upstream smoke。

## 2. Goal

把 P0 fixture corpus 从 manifest/local gate 推进到真实 upstream-vs-rs golden diff execution：至少 50 个 P0 fixture 均由 frozen upstream npm artifact 与当前 promptfoo-rs 二进制执行，并持久化可审计 artifacts。

## 3. Scope

### In Scope

- `compatibility/fixtures/`
- `compatibility/artifacts/`
- `src/compatibility/harness.rs`
- `src/compatibility/release_gate.rs`
- `src/compatibility/fixtures.rs`
- `scripts/release/real-upstream-corpus.sh`
- `scripts/release/runtime-smoke.sh`
- `scripts/release/integration.sh`
- `tests/real_p0_golden_corpus_runner.rs`
- `.github/workflows/release.yml`

### Out Of Scope

- 不使用真实 API key；fixtures 必须使用 echo/mock HTTP/recorded response。
- 不比较真实非确定性 LLM 文本 byte-level 输出；按 PRD 使用 normalization 和 recorded/mock grader。
- 不要求所有 P1/P2 rows 进入 full golden diff；本 task gate 聚焦 P0。

## 4. Users / Actors

- Release maintainer：需要 stable gate 的 50+ P0 corpus 证据。
- Enterprise reviewer：需要 raw/normalized/diff artifacts 而不是测试口头结论。
- Contributor：需要 fixture failure 报告定位 upstream 值、rs 值、diff class 和 matrix item。

## 5. Behavior Contract

Corpus runner 必须读取 P0 fixture manifests，逐个执行 `npx --yes promptfoo@0.121.13` 与当前 `promptfoo-rs` binary，写入 `metadata.json`、`raw/upstream.*`、`raw/rs.*`、`normalized/*.json`、`diff/findings.json`、corpus index 和 release summary。任何 fixture 未运行、使用本地 test binary 代替 upstream、缺 artifact、P0 bug/unclassified diff、缺 normalization rule 都阻断 stable。

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/tasks/task-12.1-p0-fixture-corpus.md
- docs/specs/tasks/task-12.2-executable-upstream-rs-runner.md
- docs/specs/tasks/task-12.3-golden-diff-ci-release-gate.md
- docs/specs/tasks/task-16.3-source-extracted-inventory-real-upstream-smoke.md
- docs/specs/tasks/task-17.1-frozen-source-inventory-extractor.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`std::process::Command`、`std::time::Duration`、`serde_json`、内部模块 `compatibility::fixtures`、`compatibility::harness`、`compatibility::diff`、`compatibility::release_gate`。
- Tooling commands：adapter §Commands Install / Typecheck / Unit Test / Integration tests / Runtime smoke / Build；`npx --yes promptfoo@0.121.13`。

### 5.3 函数签名

- `RealCorpusRunner::run_all(fixtures: &[FixtureManifest]) -> Result<CorpusRunSummary, HarnessError>`
- `run_real_fixture_pair(fixture: &FixtureManifest, upstream: &CommandSpec, rs: &CommandSpec) -> Result<FixtureArtifacts, HarnessError>`
- `validate_corpus_artifacts(summary: &CorpusRunSummary) -> ReleaseGateSummary`
- `write_corpus_index(summary: &CorpusRunSummary, path: &Path) -> Result<(), HarnessError>`

## 6. Acceptance Criteria

- [ ] **AC1** (PRD §Success Metrics): at least 50 P0 fixtures execute through real upstream `promptfoo@0.121.13` and current `promptfoo-rs` binary; metadata proves `used_test_binary=false` for every upstream run.
- [ ] **AC2** (ADR-007): every corpus fixture persists upstream raw, rs raw, normalized upstream, normalized rs, diff findings, stdout/stderr, exit codes, duration, baseline version, baseline gitHead, and matrix item ids.
- [ ] **AC3** (PRD §Compatibility Harness Design): normalization handles timestamp/path/random/latency/platform newline differences and records rule names per fixture; missing normalization for unstable fields fails the fixture.
- [ ] **AC4** (ADR-007 / PRD §Release): stable release gate fails on P0 bug/unclassified diff, missing artifact, skipped fixture, upstream command failure, rs command failure, or P0 coverage below 50.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-17.3.1 | TEST-17.3.1 | tests/real_p0_golden_corpus_runner.rs | install, typecheck, unit-test, integration, build, runtime-smoke | Spec Ready |
| AC2 | SCEN-17.3.1 | TEST-17.3.2 | tests/real_p0_golden_corpus_runner.rs | install, typecheck, unit-test, integration, build, runtime-smoke | Spec Ready |
| AC3 | SCEN-17.3.1 | TEST-17.3.3 | tests/real_p0_golden_corpus_runner.rs | install, typecheck, unit-test, integration, build, runtime-smoke | Spec Ready |
| AC4 | SCEN-17.3.1 | TEST-17.3.4 | tests/real_p0_golden_corpus_runner.rs | install, typecheck, unit-test, integration, coverage, build, runtime-smoke | Spec Ready |

## 8. Risks

- Running 50+ real upstream fixtures can be slow or npm-cache sensitive; quick smoke may stay small, but full stable gate must run the complete corpus.
- Upstream promptfoo output schemas may include unstable fields; normalization must be explicit and reviewed, not broad deletion.
- Fixtures that require network providers or secrets must be rejected at corpus validation time.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **Coverage**: adapter §Commands Coverage
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：待实施后回填
- **改动文件**：待实施后回填
- **commit 列表**：待实施后回填
- **§9 Verification 结果**：待实施后回填
- **剩余风险 / 未做项**：待实施后回填
- **下游 task 影响**：待实施后回填
