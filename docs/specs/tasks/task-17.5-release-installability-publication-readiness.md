# Task 17.5: release-installability-publication-readiness

**Status**: Done
**Priority**: P1
**Owner**: leafiellune
**Related Phase**: Phase 17 — deep-upstream-parity-proof
**Dependencies**: task-17.3-real-p0-golden-corpus-runner, task-17.4-longtail-provider-assertion-redteam-classification, task-15.1-viewer-node-packaging-release, task-15.2-performance-security-observability-gates

## 1. Background

Phase 15/16 已有 viewer/npm local smoke、measured release gate 与 release candidate JSON，但当前审计仍认为 multi-channel public release/installability 没有真实证明。公开发布需要 GitHub/Homebrew/crates.io/Docker/npm 凭据；本 task 先把无需凭据的可安装 artifact dry-run 与凭据 blocker 明确化。依据 PRD §Release constraints、ADR-008、docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md §Requirement Verdict。

## 2. Goal

生成可审计的 release installability evidence：本地 release archive/checksum、cargo package dry-run、npm pack、viewer/npm smoke、Docker/Homebrew/GitHub Action dry-run 或显式 blocker，并让 stable/publication claim 不再依赖口头说明。

## 3. Scope

### In Scope

- `scripts/release/`
- `src/release.rs`
- `.github/workflows/release.yml`
- `Dockerfile`
- `npm/package.json`
- `viewer/package.json`
- `docs/release.md`
- `docs/compatibility/`
- `tests/release_installability_publication_readiness.rs`
- `target/release-gates/`

### Out Of Scope

- 不使用真实 GitHub Release、Homebrew tap、crates.io、Docker registry、npm publish credentials。
- 不执行会向公网发布 artifact 的命令。
- 不承诺 promptfoo cloud/share SaaS 功能。

## 4. Users / Actors

- Release maintainer：需要区分 local installability ready 与 external credentials missing。
- CI maintainer：需要 workflow 在 stable release 前跑 full gate 和 packaging dry-run。
- Enterprise reviewer：需要 checksum、archive、package manifest、install command、no-upload evidence。

## 5. Behavior Contract

Release readiness command 必须在本地生成 release archive、checksums、cargo package dry-run evidence、npm pack tarball evidence、viewer/npm build smoke evidence、Docker/Homebrew/GitHub Action dry-run evidence 或 explicit credential blocker。`release-candidate.json` 必须区分 `installability_ready`、`publication_ready`、`credential_blocked`，不得在缺少真实外部发布证据时宣称已公开发布。

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/release.md
- docs/specs/tasks/task-15.1-viewer-node-packaging-release.md
- docs/specs/tasks/task-15.2-performance-security-observability-gates.md
- docs/specs/tasks/task-16.2-measured-release-gate-reports.md
- docs/specs/tasks/task-17.3-real-p0-golden-corpus-runner.md
- docs/decisions/adr-008-binary-first-multi-channel-release.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`std::fs`、`std::path::PathBuf`、`std::process::Command`、`serde::{Deserialize, Serialize}`、`serde_json`、内部模块 `release`、`compatibility::release_gate`。
- Tooling commands：adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke；`cargo package --no-verify --allow-dirty` dry-run equivalent；`pnpm pack --pack-destination`；Docker/Homebrew commands only when available.

### 5.3 函数签名

- `ReleaseInstallabilityRunner::run(config: &ReleaseInstallabilityConfig) -> Result<ReleaseInstallabilityReport, ReleaseError>`
- `collect_channel_evidence(channel: ReleaseChannel, workspace: &Path) -> ChannelEvidence`
- `classify_publication_blockers(report: &ReleaseInstallabilityReport) -> PublicationReadiness`
- `write_release_installability_report(report: &ReleaseInstallabilityReport, path: &Path) -> Result<(), ReleaseError>`

## 6. Acceptance Criteria

- [x] **AC1** (ADR-008): release installability report records binary archive/checksum, cargo package dry-run, npm pack, viewer/npm smoke, GitHub Action workflow gate, and Docker/Homebrew dry-run evidence or explicit unavailable-tool blocker.
- [x] **AC2** (PRD §Release constraints): stable publication status is `credential_blocked` unless real GitHub/Homebrew/crates.io/Docker/npm credentials and external artifact URLs/digests are present; local dry-run evidence cannot be labeled as published.
- [x] **AC3** (ADR-007 / task-17.3): release workflow requires full 50+ P0 real golden corpus gate before any stable artifact build or publication job.
- [x] **AC4** (PRD §Security): release evidence records checksums, no-upload statement, redaction/security gate result, and excludes secrets from artifacts/logs.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-17.5.1 | TEST-17.5.1 | tests/release_installability_publication_readiness.rs | install, lint, typecheck, unit-test, integration, build, runtime-smoke | Done |
| AC2 | SCEN-17.5.1 | TEST-17.5.2 | tests/release_installability_publication_readiness.rs | install, lint, typecheck, unit-test, integration, build, runtime-smoke | Done |
| AC3 | SCEN-17.5.1 | TEST-17.5.3 | tests/release_installability_publication_readiness.rs | install, lint, typecheck, unit-test, integration, coverage, build, runtime-smoke | Done |
| AC4 | SCEN-17.5.1 | TEST-17.5.4 | tests/release_installability_publication_readiness.rs | install, lint, typecheck, unit-test, integration, e2e, build, runtime-smoke | Done |

## 8. Risks

- Docker/Homebrew tooling may be absent on a developer machine; report unavailable tools as blocker evidence, not pass.
- External publication requires real credentials and legal/brand confirmation; if user asks for actual publish later, follow BLOCKED protocol until credentials and authority are explicit.
- Checksums and archives must be reproducible enough for audit, but byte-for-byte reproducible builds are not introduced in this task.

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

- **完成日期**：2026-05-31
- **改动文件**：
  - `tests/release_installability_publication_readiness.rs`
  - `src/release.rs`
  - `scripts/release/installability.sh`
  - `scripts/release/runtime-smoke.sh`
  - `scripts/release/integration.sh`
  - `scripts/release/coverage.sh`
  - `scripts/release/real-upstream-smoke.sh`
  - `.github/workflows/release.yml`
  - `docs/release.md`
  - `tests/real_upstream_smoke_gate.rs`
  - `tests/cli_global_eval_redteam_parity.rs`
  - `tests/frozen_source_inventory_extractor.rs`
  - `tests/real_p0_golden_corpus_runner.rs`
  - `docs/compatibility/matrix.md`
  - `docs/s2v-adapter.md`
  - `docs/specs/phases/phase-17-deep-upstream-parity-proof.md`
  - `docs/specs/tasks/task-17.5-release-installability-publication-readiness.md`
- **commit 列表**：
  - `247d3cf` `test(release): add SCEN-17.5.1 installability RED tests`
  - `de44410` `feat(release): add installability publication readiness evidence`
  - `a2c7ff4` `refactor(test): apply rustfmt for release lint gate`
  - `98a4a4a` `test(release): require upstream smoke telemetry guard`
  - `115edf5` `fix(release): disable upstream telemetry in real smoke gate`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - lint: PASS — `bash scripts/release/lint.sh` 通过；仓库级 `cargo fmt --all -- --check` 与 `cargo clippy --workspace --all-targets -- -D warnings` 通过。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-17.5.1 ~ TEST-17.5.5 与 real-upstream smoke telemetry guard 通过。
  - integration: PASS — `bash scripts/release/integration.sh` 通过，包含 `release_installability_publication_readiness` 与 real upstream smoke contract tests。
  - e2e: PASS — `bash scripts/release/e2e.sh` 通过。
  - build: PASS — helper 执行 `cargo build --workspace`、viewer/npm `pnpm build` 通过。
  - coverage: PASS — `bash scripts/release/coverage.sh` 通过；`s2v_coverage_threshold_guard` 通过。
  - runtime-smoke: PASS — `bash scripts/release/runtime-smoke.sh` 通过；`target/release-gates/installability.json` schema=`promptfoo-rs.release-installability.v1`，installability_ready=`true`，publication_ready=`credential-blocked`，credential_blocked=`true`，所有 channel evidence 均为 published=`false`，artifacts/checksums=6；`target/release-gates/release-candidate.json` published=`false`；`target/release-gates/real-upstream-corpus/index.json` status=`ready`，fixture_count=50；`target/release-gates/real-upstream-smoke/latest/metadata.json` status=`ready`，upstream_exit_code=0，rs_exit_code=0。
- **剩余风险 / 未做项**：真实 GitHub Release、Homebrew tap、crates.io、Docker registry、npm publish 仍需真实凭据、账号权限和发布授权；本 task 只证明 no-upload dry-run installability，并以 `credential-blocked`、channel-level published=`false`、release-candidate published=`false` 防止把本地证据伪装成公开发布。`longtail-classification.json` 仍如实记录 37 个 P0 provider module blocker，`source-inventory-evidence.json` 仍如实记录非 provider/assertion/redteam source rows 的 release blockers。
- **下游 task 影响**：Phase 17 收尾 smoke 可直接引用 `installability.json`、`release-candidate.json`、50+ real corpus artifacts 与 real upstream smoke metadata；后续真实公开发布 task 必须先取得凭据和授权，并保留当前 no-upload/credential-blocked 证据边界。
