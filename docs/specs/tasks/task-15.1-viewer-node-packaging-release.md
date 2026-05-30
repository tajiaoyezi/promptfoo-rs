# Task 15.1: viewer-node-packaging-release

**Status**: Done
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

- [x] **AC1** (PRD §Core Capabilities / §Release): `viewer/` has package metadata, lockfile, typecheck, test, build, and browser smoke scripts that run from adapter commands.
- [x] **AC2** (ADR-010): `npm/` has package metadata, lockfile, exported API contract, and Node smoke that exercises Rust core transport without duplicating business logic.
- [x] **AC3** (ADR-008): release dry-run records generated viewer/npm artifacts, expected package names, versions, checksums, and no-publish evidence.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-15.1.1 | TEST-15.1.1 | tests/viewer_node_packaging_release.rs | install, typecheck, unit-test, build, manual | Done |
| AC2 | SCEN-15.1.1 | TEST-15.1.2 | tests/viewer_node_packaging_release.rs | install, typecheck, unit-test, build, manual | Done |
| AC3 | SCEN-15.1.1 | TEST-15.1.3 | tests/viewer_node_packaging_release.rs | install, typecheck, unit-test, build, manual | Done |

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

- **完成日期**：2026-05-30
- **改动文件**：
  - `.github/workflows/release.yml`
  - `docs/s2v-adapter.md`
  - `docs/specs/tasks/task-15.1-viewer-node-packaging-release.md`
  - `src/release.rs`
  - `tests/viewer_node_packaging_release.rs`
  - `viewer/package.json`
  - `viewer/pnpm-lock.yaml`
  - `viewer/scripts/typecheck.mjs`
  - `viewer/scripts/test.mjs`
  - `viewer/scripts/build.mjs`
  - `viewer/scripts/browser-smoke.mjs`
  - `npm/package.json`
  - `npm/pnpm-lock.yaml`
  - `npm/scripts/typecheck.mjs`
  - `npm/scripts/test.mjs`
  - `npm/scripts/build.mjs`
  - `npm/scripts/node-smoke.mjs`
- **commit 列表**：
  - `9e6c6d1` `docs(spec): task-15.1 进入实施 (Status: Ready → In Progress)`
  - `b1d26c1` `test(release): 加 SCEN-15.1.1 的 3 个 RED 测试`
  - `f8b026d` `feat(release): 补齐 viewer npm packaging smoke`
- **§9 Verification 结果**：
  - install: PASS — `s2v_verify_full "install typecheck unit-test build"` 的 install 分支通过；本地 `corepack` 缺失时 adapter fallback 使用已安装 `pnpm 10.33.0`，`viewer/` 与 `npm/` 均执行 `pnpm install --frozen-lockfile`。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、`viewer pnpm typecheck`、`npm pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、`viewer pnpm test`、`npm pnpm test` 通过；新增 `cargo test --test viewer_node_packaging_release` 覆盖 TEST-15.1.1 ~ TEST-15.1.3。
  - build: PASS — helper 执行 `cargo build --workspace`、`viewer pnpm build`、`npm pnpm build` 通过；viewer build 串联 `smoke:browser`，npm build 串联 `smoke:node`。
  - manual: PASS — dry-run artifacts 位于 `target/package-smoke/`，`viewer-dist.json` 与 `npm-wrapper-dist.json` 均记录 `publish:false`；package names 为 `@promptfoo-rs/viewer` / `@promptfoo-rs/node`；SHA256 分别为 `A5B84DD0B81E3D4B7C496747391EA7FE8A8DDD2F1F5EB515F85DAEF0D761B114` / `F250DA0AC60F034CCB621DD3F9F8F49E450BEDADE7877115ABFA932108693BA7`。非交互环境中 helper 的 manual `/dev/tty` 确认不可用，因此手工记录 dry-run 证据。
- **剩余风险 / 未做项**：真实 npm/GitHub/Cargo/Homebrew 发布仍属 Out Of Scope，未使用真实发布凭据；viewer/browser smoke 为本地 deterministic smoke，不承诺像素级 UI parity；后续 task-15.2 继续收紧性能、安全与观测 gate。
- **下游 task 影响**：task-15.2 可直接复用 viewer/npm package scripts 与 release dry-run report，把 release hardening gate 扩展到 adapter/CI 验证路径。
