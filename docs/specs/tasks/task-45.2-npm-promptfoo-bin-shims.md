# Task 45.2: npm-promptfoo-bin-shims

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 45 - promptfoo-drop-in-cli-entrypoints
**Dependencies**: task-45.1-rust-promptfoo-binary-alias, task-9.2-node-api-wrapper

## 1. Background

Upstream `promptfoo` publishes npm bin entries `promptfoo` and `pf`. The current `npm/package.json` is a private thin wrapper with no `bin` field, so installing or packing the wrapper cannot expose `promptfoo`, `promptfoo-rs`, or `pf` commands for local smoke. 依据 `npm view promptfoo bin`, upstream README Quick Start, ADR-008, task 9.2, task 17.5, and task 18.4.

## 2. Goal

Add npm wrapper bin shims for `promptfoo`, `promptfoo-rs`, and `pf` that delegate to the verified Rust CLI boundary in local/package smoke tests without performing a real npm publish.

## 3. Scope

### In Scope

- `npm/package.json` `bin` field.
- `npm/src/` or `npm/bin/` shim files.
- npm build/test/smoke scripts.
- Node wrapper tests proving local bin invocation and argument forwarding.
- Release installability dry-run evidence if needed.

### Out Of Scope

- Real `npm publish`.
- Claiming ownership of the public `promptfoo` npm package.
- Registry credential storage.
- Changing the Node wrapper from a thin boundary into a business-logic implementation.

## 4. Users / Actors

- JavaScript user: wants a package-installed `promptfoo` command.
- CI maintainer: wants `npx`/npm-style bin smoke without installing the upstream package.
- Release maintainer: needs dry-run installability evidence before public publication authority.

## 5. Behavior Contract

The npm wrapper must declare local bin shims named `promptfoo`, `promptfoo-rs`, and `pf`. Each shim forwards argv to the same Rust CLI implementation or verified local binary boundary. Bin smoke must prove `--help` and at least one `eval -c` flow works through the shim. The package remains unpublished/private unless publication authority is separately provided by release gates.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- npm/package.json
- npm/src/
- docs/specs/tasks/task-9.2-node-api-wrapper.md
- docs/specs/tasks/task-17.5-release-installability-publication-readiness.md
- docs/specs/tasks/task-18.4-publication-authority-release-gate.md
- docs/decisions/adr-008-binary-first-multi-channel-release.md
- test/features/node-api-wrapper.feature
- test/features/cli.feature

### 5.2 Imports

- Node module/files: `node:child_process`, `node:path`, `node:process`, existing npm wrapper source.
- Rust binary artifacts: `target/release/promptfoo`, `target/release/promptfoo-rs` or platform `.exe` equivalents.
- Shell/tooling commands: adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke; npm `pnpm build`, `pnpm pack --dry-run`, bin smoke script.

### 5.3 函数签名

- Node shim contract: `runPromptfooBin(argv: string[], env?: NodeJS.ProcessEnv) -> Promise<number> | number`
- Test helper contract: `assertBinShim(binName: "promptfoo" | "promptfoo-rs" | "pf", args: string[]) -> void`
- Package contract: `package.json.bin["promptfoo"]`, `package.json.bin["promptfoo-rs"]`, `package.json.bin["pf"]`

## 6. Acceptance Criteria

- [ ] **AC1** (`npm view promptfoo bin`): `npm/package.json` declares `promptfoo` and `pf` bin entries, and also exposes `promptfoo-rs` for explicit reimplementation usage.
- [ ] **AC2** (ADR-008): npm bin shims delegate to the Rust CLI boundary and do not reimplement eval/provider/assertion business logic.
- [ ] **AC3** (task 17.5 / task 18.4): npm pack/build smoke proves bin installability locally while keeping `published=false` and publication authority blocked without real credentials.
- [ ] **AC4** (PRD §Security): bin shims do not store tokens, upload results, or execute unapproved scripts by default.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-45.2.1 | TEST-45.2.1 | tests/npm_promptfoo_bin_shims.rs or npm scripts/test.mjs | install, lint, typecheck, unit-test, build | Not Started |
| AC2 | SCEN-45.2.1 | TEST-45.2.2 | tests/npm_promptfoo_bin_shims.rs or npm scripts/test.mjs | install, typecheck, unit-test, integration, build | Not Started |
| AC3 | SCEN-45.2.1 | TEST-45.2.3 | npm scripts/node-smoke.mjs / installability gate | install, lint, typecheck, unit-test, runtime-smoke, build | Not Started |
| AC4 | SCEN-45.2.1 | TEST-45.2.4 | npm scripts/test.mjs | install, lint, typecheck, unit-test, e2e, build | Not Started |

## 8. Risks

- A shim can pass help smoke while failing argument forwarding; tests need an eval fixture.
- Public npm package name authority is separate from local bin shape; docs must not imply publication.
- Cross-platform binary lookup must handle `.exe` on Windows and release/debug paths.

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
