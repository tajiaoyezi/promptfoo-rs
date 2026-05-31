# Task 18.4: publication-authority-release-gate

**Status**: Done
**Priority**: P1
**Owner**: leafiellune
**Related Phase**: Phase 18 — perfect-refactor-blocker-burndown
**Dependencies**: task-18.1-source-inventory-ledger-closure, task-18.3-current-upstream-rebaseline-gate, task-17.5-release-installability-publication-readiness

## 1. Background

Phase 17 证明 GitHub archive、cargo package dry-run、npm pack、Docker/Homebrew/GitHub Action installability evidence 可生成，但真实公开发布仍为 `credential-blocked`，所有渠道 `published=false`。完美重构若包含可发布承诺，必须区分本地可安装、凭据阻塞、法律/品牌阻塞和真实已发布。依据 PRD §Release constraints、ADR-008、task 17.5 §10。

## 2. Goal

实现 publication authority release gate：每个发布渠道都有 authority status、credential probe、legal/brand requirement、published evidence URL 或 explicit blocker；没有真实凭据时保持 blocked，不伪造 published。

## 3. Scope

### In Scope

- `scripts/release/installability.sh`
- `scripts/release/runtime-smoke.sh`
- `target/release-gates/installability.json`
- `target/release-gates/release-candidate.json`
- `docs/release.md`
- `tests/publication_authority_release_gate.rs`

### Out Of Scope

- 不上传真实 release artifact，除非用户之后提供明确凭据和发布授权。
- 不处理品牌/legal 文案批准本身；只记录是否需要批准和 blocker。
- 不修改已经通过 dry-run 的本地 artifact build 逻辑。

## 4. Users / Actors

- Release maintainer：需要知道哪些渠道可本地安装，哪些需要凭据。
- Enterprise reviewer：需要确认没有把未发布渠道写成已发布。
- Future publisher：需要最小凭据/授权清单。

## 5. Behavior Contract

Installability report 必须为 GitHub Releases、Cargo、npm wrapper、Docker、Homebrew、GitHub Action 输出 `installability_status`、`authority_status`、`credential_probe`、`published`、`published_evidence`、`blocker`。`published=true` 必须有真实外部 URL/digest 证据；否则保持 false。Release candidate summary 必须把 publication state 纳入最终 claim。

### 5.1 Required Reading

- docs/specs/tasks/task-17.5-release-installability-publication-readiness.md
- docs/decisions/adr-008-binary-first-multi-channel-release.md
- docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md
- docs/release.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`serde_json`、`std::fs`、内部 release evidence structs。
- Tooling commands：adapter §Commands Install / Typecheck / Unit Test / Integration tests / Runtime smoke / Build。

### 5.3 函数签名

- `collect_publication_authority(channels: &[ReleaseChannel]) -> PublicationAuthorityReport`
- `validate_publication_evidence(report: &PublicationAuthorityReport) -> PublicationGateDecision`
- `write_publication_authority_report(report: &PublicationAuthorityReport, path: &Path) -> Result<(), ReleaseEvidenceError>`

## 6. Acceptance Criteria

- [x] **AC1** (ADR-008): every release channel records installability status separately from authority/credential status.
- [x] **AC2** (PRD §Release constraints): `published=true` requires external evidence; dry-run artifacts alone cannot set it.
- [x] **AC3** (docs/audits §P1 Public release): missing credentials or Homebrew tooling produce explicit blockers and keep release candidate publication state credential-blocked.
- [x] **AC4** (ADR-009): release docs show the exact remaining publication blockers and do not claim stable public availability when channels are unpublished.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-18.4.1 | TEST-18.4.1 | tests/publication_authority_release_gate.rs | install, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-18.4.1 | TEST-18.4.2 | tests/publication_authority_release_gate.rs | install, typecheck, unit-test, coverage, build | Done |
| AC3 | SCEN-18.4.1 | TEST-18.4.3 | tests/publication_authority_release_gate.rs | install, typecheck, unit-test, runtime-smoke, build | Done |
| AC4 | SCEN-18.4.1 | TEST-18.4.4 | tests/publication_authority_release_gate.rs | install, typecheck, unit-test, e2e, build | Done |

## 8. Risks

- Tool availability differs by developer machine; record tool-unavailable separately from credential-blocked.
- Real publication is high-risk; require explicit user authorization before any upload.
- A local dry-run can be mistaken for release availability; JSON field names must make that impossible.

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
  - `tests/publication_authority_release_gate.rs`
  - `src/release.rs`
  - `scripts/release/installability.sh`
  - `scripts/release/runtime-smoke.sh`
  - `scripts/release/integration.sh`
  - `docs/release.md`
  - `docs/compatibility/matrix.md`
  - `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`
  - `docs/s2v-adapter.md`
  - `docs/specs/phases/phase-18-perfect-refactor-blocker-burndown.md`
  - `docs/specs/tasks/task-18.4-publication-authority-release-gate.md`
- **commit 列表**：
  - `c620c20` `test(release): add SCEN-18.4.1 publication authority RED tests`
  - `8c37c7a` `feat(release): add publication authority gate evidence`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-18.4.1 ~ TEST-18.4.4 通过。
  - integration: PASS — `bash scripts/release/integration.sh` 通过，包含 `publication_authority_release_gate`。
  - e2e: PASS — `bash scripts/release/e2e.sh` 通过。
  - coverage: PASS — `bash scripts/release/coverage.sh` 通过；`s2v_coverage_threshold_guard` 通过。
  - build: PASS — helper 执行 `cargo build --workspace`、viewer/npm `pnpm build` 通过。
  - runtime-smoke: PASS — `bash scripts/release/runtime-smoke.sh` 通过；`target/release-gates/publication-authority.json` schema=`promptfoo-rs.publication-authority.v1`，publication_ready=`credential-blocked`，credential_blocked=true，legal_brand_blocked=true，blockers=6，所有 channel `published=false` 且 `published_evidence=null`；`target/release-gates/release-candidate.json.publication_authority` 同步 credential-blocked 状态并引用 `target/release-gates/publication-authority.json`。
- **剩余风险 / 未做项**：真实 GitHub Release、crates.io、npm、Docker registry、Homebrew tap、GitHub Action 发布仍需要真实凭据、账号权限和发布授权；本 task 只实现证据 gate，继续禁止把 dry-run installability 伪装为公开发布。
- **下游 task 影响**：Phase 18 smoke 可以把 `publication-authority.json` 与 `release-candidate.json.publication_authority` 作为公开发布边界证据；后续若要把 publication_ready 推到 Ready，必须先新增有授权的真实发布 task/ADR 并提供外部 URL/digest evidence。
