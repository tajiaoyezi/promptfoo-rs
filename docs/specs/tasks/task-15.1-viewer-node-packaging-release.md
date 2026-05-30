# Task 15.1: viewer-node-packaging-release

**Status**: In Progress
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 15 — release-hardening-performance
**Dependencies**: task-14.2-redteam-plugin-strategy-parity

## 1. Background

Audit found `viewer/` and `npm/` exist in source areas and adapter commands, but package metadata, lockfiles, browser smoke, Node wrapper smoke, and release dry-run evidence are incomplete. A "perfect refactor" must prove the advertised viewer and npm wrapper channels are buildable, testable, and release-gated. Basis: PRD §Core Capabilities, PRD §Release, ADR-008, ADR-010, docs/audits/promptfoo-release-distribution-audit-2026-05-30.md.

## 2. Goal

Make local viewer and npm wrapper packaging executable in normal verification, with browser smoke, Node API smoke, and release artifact dry-run evidence.

## 3. Scope

### In Scope

- viewer/package.json
- viewer/pnpm-lock.yaml
- viewer/src/
- npm/package.json
- npm/pnpm-lock.yaml
- npm/src/
- tests/viewer_node_packaging_release.rs
- docs/release/
- .github/workflows/release.yml

### Out Of Scope

- Publishing real packages to npm, Homebrew, Cargo, Docker, or GitHub Releases.
- Pixel-perfect upstream UI cloning; viewer scope remains PRD P1 local result inspection.
- Changing Rust eval semantics except where needed for wrapper smoke transport.

## 4. Users / Actors

- Promptfoo-rs maintainer: needs reproducible release packaging evidence.
- Node ecosystem user: needs npm wrapper and programmatic API compatibility.
- AI application developer: needs local viewer to inspect result artifacts.

## 5. Behavior Contract

Viewer and npm wrapper packages must have deterministic package metadata, lockfiles, test/typecheck/build scripts, and smoke tests. Release dry-run must produce auditable artifacts without publishing anything.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/s2v-adapter.md
- docs/audits/promptfoo-release-distribution-audit-2026-05-30.md
- docs/decisions/adr-008-binary-first-multi-channel-release.md
- docs/decisions/adr-010-node-api-wrapper-contract-boundary.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`std::process`、`serde_json`、内部模块 `release`、`node_api_wrapper`、`web_viewer`。
- TypeScript package：viewer package scripts、npm wrapper package scripts、Node 20+ Corepack/pnpm runtime。

### 5.3 函数签名

- `verify_viewer_package(root: &Path) -> Result<PackageCheck, PackageError>`
- `verify_npm_wrapper_package(root: &Path) -> Result<PackageCheck, PackageError>`
- `run_release_packaging_smoke(config: &PackagingSmokeConfig) -> Result<PackagingSmokeReport, PackageError>`

## 6. Acceptance Criteria

- [ ] **AC1** (PRD §Core Capabilities / §Release): `viewer/` has package metadata, lockfile, typecheck, test, build, and browser smoke scripts that run from adapter commands.
- [ ] **AC2** (ADR-010): `npm/` has package metadata, lockfile, exported API contract, and Node smoke that exercises Rust core transport without duplicating business logic.
- [ ] **AC3** (ADR-008): release dry-run records generated viewer/npm artifacts, expected package names, versions, checksums, and no-publish evidence.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-15.1.1 | TEST-15.1.1 | tests/viewer_node_packaging_release.rs | install, typecheck, unit-test, build, manual | Not Started |
| AC2 | SCEN-15.1.1 | TEST-15.1.2 | tests/viewer_node_packaging_release.rs | install, typecheck, unit-test, build, manual | Not Started |
| AC3 | SCEN-15.1.1 | TEST-15.1.3 | tests/viewer_node_packaging_release.rs | install, typecheck, unit-test, build, manual | Not Started |

## 8. Risks

- pnpm lockfiles can drift if Node/Corepack versions are not pinned in docs and CI.
- npm wrapper can accidentally reimplement Rust behavior; contract tests must keep wrapper thin.
- Browser smoke may need platform-specific timeouts on Windows CI.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Build**: adapter §Commands Build
- **Manual**: inspect release dry-run report and verify it does not publish artifacts.

## 10. Completion Notes

- **完成日期**：<TBD-after-impl>
- **改动文件**：<TBD-after-impl>
- **commit 列表**：<TBD-after-impl>
- **§9 Verification 结果**：<TBD-after-impl>
- **剩余风险 / 未做项**：<TBD-after-impl>
- **下游 task 影响**：<TBD-after-impl>
