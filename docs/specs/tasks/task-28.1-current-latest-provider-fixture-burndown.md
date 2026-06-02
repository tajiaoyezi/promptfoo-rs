# Task 28.1: current-latest-provider-fixture-burndown

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 28 - current-latest-provider-fixture-burndown
**Dependencies**: task-27.1-current-latest-core-config-burndown, task-19.3-provider-request-response-fixture-burndown, task-19.4-external-authority-blocker-waiver-gate

## 1. Background

Phase 27 left `current-latest-golden-corpus.json` at 92 P0 golden blockers, including 38 `provider` blockers. Task 19.3 already proved that 22 frozen-baseline provider source rows are covered by aggregate or dedicated mock/recorded request-response fixtures, while task 19.4 proved that Codex/Agents/Assistant/Billing/ChatKit/Realtime/Claude Code auth style rows require external authority and must not be faked as ready. This task applies that same conservative split to current-latest artifacts. 依据 PRD §Core Capabilities / §Compatibility Matrix、ADR-009、ADR-011、task 19.3、task 19.4。

## 2. Goal

Reduce current-latest provider P0 golden blockers from 38 generic blockers to 16 explicit external authority blockers while preserving 22 fixture-covered provider rows as P0 native fixture evidence.

## 3. Scope

### In Scope

- `src/compatibility/inventory.rs`
- `scripts/release/current-latest-source-inventory.sh`
- `tests/current_latest_provider_fixture_burndown.rs`
- `target/release-gates/current-latest-source-inventory.json`
- `target/release-gates/current-latest-matrix.json`
- `target/release-gates/current-latest-golden-corpus.json`
- `target/release-gates/current-latest-quality.json`
- `docs/compatibility/matrix.md`
- `docs/s2v-adapter.md`
- `docs/prds/promptfoo-rs.prd.md`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不调用真实 OpenAI/Anthropic/Ollama/HTTP 外部服务。
- 不提供真实 API key、账号、Claude Code auth、Codex private SDK/server、Agents SDK product contract、ChatKit browser/session、Realtime streaming service contract 或 Billing authority。
- 不解决 eval-runner、prompt-processing、cache-store、config external、script-bridge、current-target 或 publication blockers。
- 不承诺“无任何潜在 bug”；claim 仍受 ADR-011 的 evidence boundary 约束。

## 4. Users / Actors

- Provider maintainer: needs current-latest provider rows to reuse existing mock/recorded fixture evidence when that evidence is sufficient.
- Release reviewer: needs provider blocker reduction to be item-level and not hide credential/product authority gaps.
- Security reviewer: needs fixture evidence separated from live provider credentials and real account authority.

## 5. Behavior Contract

Current-latest `category=provider` rows must be classified by stable id and source path. Fixture-covered rows must use `level=P0`, `implementation_status=native`, `verification_owner=provider-runtime`, `evidence_kind=fixture`, and `evidence_reference=fixture:<stable-id>`. External authority rows must use `level=P0`, `implementation_status=blocked`, `verification_owner=external-authority`, `evidence_kind=blocker`, and `evidence_reference=blocker:<stable-id>`. Long-tail provider rows outside the P0 allowlist stay P2 registration evidence. Rust and shell artifact generation must use equivalent provider rules.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/phases/phase-24-current-latest-perfect-refactor.md
- docs/specs/phases/phase-27-current-latest-core-config-burndown.md
- docs/specs/tasks/task-19.3-provider-request-response-fixture-burndown.md
- docs/specs/tasks/task-19.4-external-authority-blocker-waiver-gate.md
- docs/specs/tasks/task-24.4-current-latest-exhaustive-quality-gate.md
- docs/specs/tasks/task-27.1-current-latest-core-config-burndown.md
- docs/compatibility/current-latest.lock.md
- docs/compatibility/matrix.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `promptfoo_rs::compatibility::inventory::{extract_current_latest_inventory, write_current_latest_inventory_artifacts, CurrentLatestTargetLock}`, `promptfoo_rs::compatibility::harness::{build_current_latest_golden_corpus, evaluate_current_latest_release_blockers}`, `serde_json::Value`, `std::path::Path`, `std::process::Command`.
- Tooling commands: `bash scripts/release/current-latest-source-inventory.sh`, `bash scripts/release/current-latest-golden-corpus.sh`, `bash scripts/release/current-latest-quality-gate.sh`, `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `current_latest_provider_fixture_ids(stable_id: &str) -> &'static [&'static str]`
- `is_current_latest_fixture_provider(stable_id: &str, file: &str) -> bool`
- `is_current_latest_external_provider(file: &str) -> bool`
- `current_latest_provider_external_reason(stable_id: &str, file: &str) -> String`
- Shell contract: `currentLatestProviderFixtureIds(id)`, `isCurrentLatestFixtureProvider(id, file)`, `isCurrentLatestExternalProvider(file)`, `currentLatestProviderExternalReason(id, file)`, `metadata(category, id, file)`.

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Provider P0 / task 19.3): 22 current-latest mockable provider rows have P0 native fixture evidence and do not produce golden release blockers.
- [x] **AC2** (PRD §Security / task 19.4): 16 current-latest provider rows requiring credentials/account/private-service/product authority remain explicit external-authority P0 blockers.
- [x] **AC3** (ADR-009): Rust extractor and shell extractor emit equivalent provider classification, evidence kind, evidence reference, and owner values.
- [x] **AC4** (ADR-011 / task 24.4): source/matrix/golden/quality artifacts show provider blockers reduced to 16 and total blockers reduced to 70, while perfect-refactor completion remains false.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-28.1.1 | TEST-28.1.1 | tests/current_latest_provider_fixture_burndown.rs | install, lint, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-28.1.1 | TEST-28.1.2 | tests/current_latest_provider_fixture_burndown.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Done |
| AC3 | SCEN-28.1.1 | TEST-28.1.3 | tests/current_latest_provider_fixture_burndown.rs | install, lint, typecheck, unit-test, coverage, build | Done |
| AC4 | SCEN-28.1.1 | TEST-28.1.4 | tests/current_latest_provider_fixture_burndown.rs | install, lint, typecheck, unit-test, integration, e2e, runtime-smoke, build | Done |

## 8. Risks

- Provider helper/defaults/types/util rows can look like implementation internals, but task 19.3 already tied them to aggregate provider fixtures; this task must reuse that explicit allowlist rather than path-prefix demotion.
- External authority rows may later become fixture-covered only through new user-approved tasks with real authority or recorded protocol evidence.
- Reducing provider blockers does not prove eval-runner, prompt-processing, cache-store, config external, script-bridge, publication, or current-target readiness.

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

- **完成日期**：2026-06-02
- **改动文件**：
  - `docs/prds/promptfoo-rs.prd.md`
  - `docs/s2v-adapter.md`
  - `docs/specs/phases/phase-28-current-latest-provider-fixture-burndown.md`
  - `docs/specs/tasks/task-28.1-current-latest-provider-fixture-burndown.md`
  - `docs/compatibility/matrix.md`
  - `test/features/perfect-refactor-parity.feature`
  - `tests/current_latest_provider_fixture_burndown.rs`
  - `src/compatibility/inventory.rs`
  - `scripts/release/current-latest-source-inventory.sh`
- **commit 列表**：
  - `0ec6884` `docs(spec): add phase 28 current latest provider burndown`
  - `d347412` `docs(spec): task-28.1 enters implementation`
  - `e78b9b8` `test(providers): add current latest provider burndown RED tests`
  - `dd6ee14` `feat(providers): classify current latest provider evidence`
  - 本次 docs 回填提交：`docs(spec): complete task 28.1 current latest provider burndown`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - lint: PASS — helper 执行 `bash scripts/release/lint.sh` 通过。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-28.1.1 ~ TEST-28.1.4 通过。
  - integration: PASS — helper 执行 `bash scripts/release/integration.sh` 通过。
  - e2e: PASS — helper 执行 `bash scripts/release/e2e.sh` 通过。
  - build: PASS — helper 执行 `cargo build --workspace`、viewer/npm `pnpm build` 通过。
  - coverage: PASS — helper 执行 `bash scripts/release/coverage.sh` 通过；`s2v_coverage_threshold_guard` 通过。
  - runtime-smoke: PASS — helper 执行 `bash scripts/release/runtime-smoke.sh` 通过；真实 artifact 复核显示 `current-latest-source-inventory.json` rows=3858、provider native fixture=22、provider external blockers=16，`current-latest-golden-corpus.json` status=`ready-with-blockers`、blocker_count=70、provider_blockers=16，`current-latest-quality.json` status=`ready-with-blockers`、local_current_latest_ready=false、perfect_refactor_claim_allowed=false。
- **剩余风险 / 未做项**：本 task 只燃尽 current-latest provider generic blockers；仍有 70 个 current-latest P0 golden blockers（provider=16、eval-runner=18、prompt-processing=13、cache-store=9、config=7、script-bridge=7）、current-target claim boundary、external authority blockers，以及 publication `credential-blocked`。这些继续阻止“完美重构完成”声明。
- **下游 task 影响**：后续可优先处理 eval-runner=18、prompt-processing=13、cache-store=9 或 script-bridge=7 blockers；16 个 provider blockers 现为 explicit external authority rows，不能由本地代码或 mock fixture 伪造成完成。
