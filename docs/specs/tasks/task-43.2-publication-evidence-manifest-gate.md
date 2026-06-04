# Task 43.2: publication-evidence-manifest-gate

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 43 - authority-evidence-intake-gates
**Dependencies**: task-18.4-publication-authority-release-gate, task-43.1-authority-decision-manifest-gate

## 1. Background

`publication-authority.json` currently reports `publication_ready=credential-blocked`, `credential_blocked=true`, `legal_brand_blocked=true`, six publication blockers, and `published=false` for GitHub Releases, Cargo, npm wrapper, Docker, Homebrew, and GitHub Action. The project needs an evidence intake gate before any future publication task can mark channels as ready. 依据 PRD §Release constraints、ADR-008、ADR-011、task 18.4、task 17.5。

## 2. Goal

Add a publication evidence manifest and validation gate that requires each publication channel to provide authorized credentials, legal/brand approval, artifact URL, digest/checksum, release notes reference, and no-upload provenance before the channel can be counted as published.

## 3. Scope

### In Scope

- New publication evidence manifest under `docs/compatibility/` or release-gate artifact schema.
- Validation logic under `scripts/release/` or Rust release gate tests.
- Tests proving dry-run installability is not enough for `published=true`.
- Updates to `publication-authority.json` or release candidate wiring only if needed for manifest consumption.

### Out Of Scope

- Executing real publication commands.
- Storing publish tokens or private credentials.
- Giving legal/brand approval.
- Claiming public stable availability without external URLs and digests.

## 4. Users / Actors

- Release maintainer: provides channel-specific publication evidence.
- Legal/brand approver: approves package metadata, release notes, naming, and public copy.
- Future implementation agent: validates evidence and keeps missing channels blocked.

## 5. Behavior Contract

Each publication channel must remain blocked unless its manifest row includes channel name, authority owner, credential authority reference, legal/brand approval reference, artifact URL, digest/checksum, release notes reference, and publication timestamp. Dry-run artifacts and local package builds can prove installability, but must not set `published=true` or `publication_ready=ready`.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/release.md
- target/release-gates/publication-authority.json
- target/release-gates/installability.json
- docs/specs/tasks/task-17.5-release-installability-publication-readiness.md
- docs/specs/tasks/task-18.4-publication-authority-release-gate.md
- docs/decisions/adr-008-binary-first-multi-channel-release.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `serde_json::Value`, `std::fs`, `std::collections::BTreeMap`.
- Shell/tooling commands: adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke; optional `bash scripts/release/publication-evidence.sh`.

### 5.3 函数签名

- `validate_publication_evidence(publication_authority: &Value, manifest: &Value) -> PublicationEvidenceReport`
- `PublicationEvidenceReport::publication_ready(&self) -> bool`
- Shell contract: `bash scripts/release/publication-evidence.sh`

## 6. Acceptance Criteria

- [ ] **AC1** (ADR-008): every release channel in `publication-authority.json.channels[]` has one publication evidence manifest row.
- [ ] **AC2** (task 18.4): dry-run installability evidence alone never sets channel `published=true`.
- [ ] **AC3** (PRD §Release constraints): a ready publication row requires artifact URL, digest/checksum, release notes reference, credential authority reference, legal/brand approval reference, and publication timestamp.
- [ ] **AC4** (PRD §Security): the manifest stores no publish tokens, API keys, or private credentials.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-43.2.1 | TEST-43.2.1 | tests/publication_evidence_manifest_gate.rs | install, lint, typecheck, unit-test, integration, build | Not Started |
| AC2 | SCEN-43.2.1 | TEST-43.2.2 | tests/publication_evidence_manifest_gate.rs | install, typecheck, unit-test, runtime-smoke, build | Not Started |
| AC3 | SCEN-43.2.1 | TEST-43.2.3 | tests/publication_evidence_manifest_gate.rs | install, lint, typecheck, unit-test, coverage, build | Not Started |
| AC4 | SCEN-43.2.1 | TEST-43.2.4 | tests/publication_evidence_manifest_gate.rs | install, lint, typecheck, unit-test, integration, e2e, build | Not Started |

## 8. Risks

- Local package dry-runs can be mistaken for public release; AC2 keeps publication blocked until external evidence exists.
- Legal/brand approval is not inferable from code; AC3 requires explicit reference.
- Tokens must not be committed; AC4 forbids credentials in manifests.

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

- **完成日期**：<TBD-after-impl>
- **改动文件**：<TBD-after-impl>
- **commit 列表**：<TBD-after-impl>
- **§9 Verification 结果**：
  - install: <TBD-after-impl>
  - lint: <TBD-after-impl>
  - typecheck: <TBD-after-impl>
  - unit-test: <TBD-after-impl>
  - integration: <TBD-after-impl>
  - e2e: <TBD-after-impl>
  - coverage: <TBD-after-impl>
  - build: <TBD-after-impl>
  - runtime-smoke: <TBD-after-impl>
- **剩余风险 / 未做项**：<TBD-after-impl>
- **下游 task 影响**：<TBD-after-impl>
