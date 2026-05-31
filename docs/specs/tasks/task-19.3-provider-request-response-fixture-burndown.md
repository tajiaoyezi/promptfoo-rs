# Task 19.3: provider-request-response-fixture-burndown

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 19 — source-accounting-native-burndown
**Dependencies**: task-18.2-p0-provider-module-fixture-burndown, task-19.2-core-config-source-fixture-burndown

## 1. Background

Task 18.2 把 37 个 P0 provider module blockers 拆成 13 个已有 fixture 覆盖和 24 个 remaining blockers。剩余 24 个中，一部分是可用 mock/recorded request-response fixture 证明的普通 provider module（如 Anthropic/OpenAI completion、embedding、image、moderation、responses、transcription、video、HTTP multipart），另一部分需要真实产品/账号授权。依据 docs/compatibility/matrix.md `Task 18.2` 段和 `longtail-classification.json.p0_release_blockers[]`。

## 2. Goal

为不需要真实账号授权的 remaining provider module blockers 补 dedicated request/response fixtures，并把仍需账号/产品授权的项留给 task 19.4 external authority gate。

## 3. Scope

### In Scope

- `src/providers/`
- `src/compatibility/provider_assertion.rs`
- `scripts/release/longtail-classification.sh`
- `compatibility/fixtures/providers/`
- `tests/provider_request_response_fixture_burndown.rs`
- `docs/compatibility/matrix.md`
- `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`

### Out Of Scope

- 不调用真实 OpenAI/Anthropic/HTTP 外部服务。
- 不处理 Codex/Agents/Assistant/Billing/ChatKit/Realtime/Claude Code 等需要产品授权的模块。
- 不改变已有 provider aggregate fixture 的含义。

## 4. Users / Actors

- Provider maintainer：需要每个可本地证明的 provider module 有 dedicated fixture。
- Release reviewer：需要 P0 provider blocker count 的下降有 item-level evidence。
- Security reviewer：需要确认 mock fixtures 不泄漏真实 provider secrets。

## 5. Behavior Contract

Provider module burndown 必须把 remaining blockers 分成 `fixture-covered`、`external-authority-blocker`、`blocked` 三类。新增 fixture 必须包含 source item id、request shape、response shape、redaction/no-real-secret evidence，并写入 `longtail-classification.json.p0_provider_module_burndown`。

### 5.1 Required Reading

- docs/specs/tasks/task-18.2-p0-provider-module-fixture-burndown.md
- docs/compatibility/matrix.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`promptfoo_rs::compatibility::provider_assertion`、`promptfoo_rs::providers`、`serde_json`、`std::fs`。
- Tooling commands：adapter §Commands Install / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke。

### 5.3 函数签名

- `resolve_provider_request_response_fixture(item_id: &str) -> ProviderModuleResolution`
- `validate_provider_fixture_burndown(report: &LongtailClassificationReport) -> ProviderFixtureBurndownReport`
- `write_provider_fixture_burndown(report: &ProviderFixtureBurndownReport, path: &Path) -> Result<(), CompatibilityEvidenceError>`

## 6. Acceptance Criteria

- [x] **AC1** (ADR-009): every non-external provider module blocker has a dedicated request/response fixture or remains release-blocking with a specific reason.
- [x] **AC2** (PRD §Security): new fixtures use mock/recorded data and contain no real provider secrets.
- [x] **AC3** (Phase 18 §9): `longtail-classification.json` reports updated resolved/remaining provider blocker counts and lists item-level evidence.
- [x] **AC4** (docs/audits): audit and compatibility matrix distinguish fixture-covered provider modules from external-authority modules.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-19.3.1 | TEST-19.3.1 | tests/provider_request_response_fixture_burndown.rs | install, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-19.3.1 | TEST-19.3.2 | tests/provider_request_response_fixture_burndown.rs | install, typecheck, unit-test, coverage, build | Done |
| AC3 | SCEN-19.3.1 | TEST-19.3.3 | tests/provider_request_response_fixture_burndown.rs | install, typecheck, unit-test, runtime-smoke, build | Done |
| AC4 | SCEN-19.3.1 | TEST-19.3.4 | tests/provider_request_response_fixture_burndown.rs | install, typecheck, unit-test, e2e, build | Done |

## 8. Risks

- Some provider modules may look mockable but depend on product-specific semantics; if evidence is insufficient, keep blocker rather than inventing parity.
- Fixture count can hide item-level gaps; reports must retain item ids.
- No real provider credentials are available in this repo.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **E2E tests**: adapter §Commands E2E tests
- **Coverage**: adapter §Commands Coverage
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：2026-05-31
- **改动文件**：
  - `tests/provider_request_response_fixture_burndown.rs`
  - `tests/p0_fixture_corpus.rs`
  - `src/compatibility/provider_assertion.rs`
  - `scripts/release/longtail-classification.sh`
  - `scripts/release/integration.sh`
  - `compatibility/fixtures/providers/anthropic-completion/fixture.yaml`
  - `compatibility/fixtures/providers/http-multipart/fixture.yaml`
  - `compatibility/fixtures/providers/openai-completion/fixture.yaml`
  - `compatibility/fixtures/providers/openai-embedding/fixture.yaml`
  - `compatibility/fixtures/providers/openai-image/fixture.yaml`
  - `compatibility/fixtures/providers/openai-moderation/fixture.yaml`
  - `compatibility/fixtures/providers/openai-responses/fixture.yaml`
  - `compatibility/fixtures/providers/openai-transcription/fixture.yaml`
  - `compatibility/fixtures/providers/openai-video/fixture.yaml`
  - `docs/compatibility/matrix.md`
  - `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`
  - `docs/s2v-adapter.md`
  - `docs/specs/phases/phase-19-source-accounting-native-burndown.md`
  - `docs/specs/tasks/task-19.3-provider-request-response-fixture-burndown.md`
- **commit 列表**：
  - `543a37a` `test(providers): add SCEN-19.3.1 request response fixture RED tests`
  - `a136c01` `feat(providers): add dedicated provider request response fixtures`
  - `8e5854f` `refactor(compatibility): accept task-scoped fixture test ids`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-19.3.1 ~ TEST-19.3.4 通过。
  - integration: PASS — `bash scripts/release/integration.sh` 通过，包含 `provider_request_response_fixture_burndown`。
  - e2e: PASS — `bash scripts/release/e2e.sh` 通过。
  - build: PASS — helper 执行 `cargo build --workspace`、viewer/npm `pnpm build` 通过。
  - coverage: PASS — `bash scripts/release/coverage.sh` 通过；`s2v_coverage_threshold_guard` 通过。
  - runtime-smoke: PASS — `bash scripts/release/runtime-smoke.sh` 通过；`longtail-classification.json` reports `initial_blocker_count=37`, `resolved_by_fixture_count=22`, `new_dedicated_request_response_fixture_count=9`, `remaining_blocker_count=15`, `external_authority_blocker_count=15`, `generic_blocker_count=0`, `p0_release_blocker_count=15`。
- **剩余风险 / 未做项**：15 个 Codex/Agents/Assistant/Billing/ChatKit/Realtime/Claude Code auth 等 provider module rows 仍需要真实产品/账号/授权判断；它们继续作为 external-authority blockers，不声明 provider-module parity 完成。
- **下游 task 影响**：task 19.4 可聚焦 15 个 external-authority provider blockers 以及真实发布/授权边界，Phase 19 smoke 应检查 provider generic blocker 为 0、remaining provider blocker 为 15。
