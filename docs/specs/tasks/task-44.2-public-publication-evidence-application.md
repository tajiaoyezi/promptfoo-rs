# Task 44.2: public-publication-evidence-application

**Status**: Draft
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 44 - public-stable-release-authority-closure
**Dependencies**: task-43.2-publication-evidence-manifest-gate, task-44.1-external-authority-evidence-application

## 1. Background

Publication remains `credential-blocked` and `legal_brand_blocked` across GitHub Releases, Cargo, npm wrapper, Docker, Homebrew, and GitHub Action. A public stable release requires real external publication evidence and explicit approval; dry-run installability is not sufficient. 依据 PRD §Release constraints、ADR-008、task 18.4、task 43.2。

## 2. Goal

Apply real publication evidence for authorized public release channels, including external URL, digest/checksum, release notes, credential authority reference, and legal/brand approval reference.

## 3. Scope

### In Scope

- Publication evidence manifest rows for selected release channels.
- External URLs and digests/checksums for published artifacts.
- Release notes and package metadata approval references.
- Release gate updates that consume validated publication evidence.

### Out Of Scope

- Publishing without explicit user/maintainer authorization.
- Storing publish tokens or private credentials.
- Claiming channels as published from local dry-runs only.
- Changing compatibility scope without updating PRD/ADR/spec evidence.

## 4. Users / Actors

- Release maintainer: authorizes and executes publication.
- Legal/brand approver: approves public copy and package metadata.
- Package registry owner: provides channel credentials and access.

## 5. Behavior Contract

No publication channel can become `published=true` unless Phase 43's publication evidence validator accepts real external evidence. If any credential, legal/brand approval, external URL, digest, or release note reference is missing, that channel remains blocked and public stable release remains unavailable.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/release.md
- target/release-gates/publication-authority.json
- docs/specs/tasks/task-43.2-publication-evidence-manifest-gate.md
- docs/specs/tasks/task-18.4-publication-authority-release-gate.md
- docs/decisions/adr-008-binary-first-multi-channel-release.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `serde_json::Value`, `std::fs`.
- Shell/tooling commands: adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke; Phase 43 publication evidence validator; authorized registry publication commands only after user approval.

### 5.3 函数签名

- `apply_publication_evidence(manifest: &Value, publication_authority: &Value) -> PublicationApplicationReport`
- `PublicationApplicationReport::published_channels(&self) -> Vec<String>`

## 6. Acceptance Criteria

- [ ] **AC1** (ADR-008): every channel marked `published=true` has external URL, digest/checksum, release notes, credential authority reference, legal/brand approval reference, and timestamp.
- [ ] **AC2** (task 18.4): channels without complete evidence remain `credential-blocked`, `legal-brand-blocked`, or equivalent blocked state.
- [ ] **AC3** (PRD §Release constraints): release candidate evidence distinguishes local installability from public availability.
- [ ] **AC4** (PRD §Current Latest Rebaseline Addendum): public release notes use only allowed claim wording and do not promise bug-free or unsupported live-provider parity.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-44.2.1 | TEST-44.2.1 | tests/public_publication_evidence_application.rs | install, lint, typecheck, unit-test, integration, build | Not Started |
| AC2 | SCEN-44.2.1 | TEST-44.2.2 | tests/public_publication_evidence_application.rs | install, typecheck, unit-test, runtime-smoke, build | Not Started |
| AC3 | SCEN-44.2.1 | TEST-44.2.3 | tests/public_publication_evidence_application.rs | install, lint, typecheck, unit-test, coverage, build | Not Started |
| AC4 | SCEN-44.2.1 | TEST-44.2.4 | tests/public_publication_evidence_application.rs | install, lint, typecheck, unit-test, e2e, runtime-smoke, build | Not Started |

## 8. Risks

- This task requires real credentials and publication authority; do not convert Draft to Ready until user explicitly authorizes publication.
- External URLs and digests can drift if artifacts are republished; evidence must be immutable.
- Release copy can overclaim; AC4 keeps wording bound to gate evidence.

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

- **完成日期**：无（Draft，等待真实发布授权与凭据）
- **改动文件**：无（未实施）
- **commit 列表**：无（未实施）
- **§9 Verification 结果**：
  - install: 未执行（Draft）
  - lint: 未执行（Draft）
  - typecheck: 未执行（Draft）
  - unit-test: 未执行（Draft）
  - integration: 未执行（Draft）
  - e2e: 未执行（Draft）
  - coverage: 未执行（Draft）
  - build: 未执行（Draft）
  - runtime-smoke: 未执行（Draft）
- **剩余风险 / 未做项**：需要真实发布凭据、registry 权限、法律/品牌确认、外部 URL/digest 证据。
- **下游 task 影响**：最终 public stable release claim 等待本 task 的真实 evidence。
