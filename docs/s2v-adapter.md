# Project Development Adapter

> S2V Development 项目适配层。AI agent 进入项目的第一份必读文件。
> 加载顺序：AGENTS.md（协作）→ 本文件（结构）→ task spec（业务）。

---

## Project

- **Name**: promptfoo-rs
- **Type**: Infrastructure / CLI / Library / Web local viewer / Compatibility runtime
- **Primary users / actors**: AI 应用开发者；AI infra / 平台工程团队；安全红队团队；企业安全 / 合规团队；开源 contributor
- **Critical workflows**: 1) promptfoo-rs eval -c promptfooconfig.yaml 在 CI 中生成 JSONL/JUnit/SARIF 并返回稳定 exit code；2) compatibility harness 对 upstream promptfoo 0.121.13 与 promptfoo-rs 做 golden diff；3) redteam init/generate/eval/run/report 本地可审计执行；4) 显式 --allow-scripts 后通过 JS/Python/Shell bridge 运行 custom provider/assertion

---

## Specification Locations

- **SDD home**: docs/specs/
- **Master spec**: docs/prds/promptfoo-rs.prd.md
- **Phase spec pattern**: docs/specs/phases/phase-{N}-{name}.md
- **Task spec pattern**: docs/specs/tasks/task-{phase}.{seq}-{name}.md
- **BDD acceptance home**: test/features/*.feature
- **ADR home**: docs/decisions/adr-{N}-{title}.md

---

## Source And Test Areas

### Source areas

- src/
- crates/
- viewer/
- npm/
- compatibility/

### Unit test areas

- src/
- crates/
- tests/
- viewer/
- npm/
- compatibility/

### Integration test areas

- tests/
- compatibility/fixtures/

### E2E test areas

- test/features/

### Other locations

- **BDD feature**: test/features/*.feature
- **Fixture areas**: test/fixtures/ and compatibility/fixtures/

### Test File Naming（Default Profile，可覆盖）

| 测试类型 | 文件名 | 示例 |
|---|---|---|
| Rust 单元测试 | #[cfg(test)] mod tests 同源文件 | src/cli.rs 内部测试 |
| Rust 集成测试 | tests/scenario.rs | tests/eval_cli.rs |
| Web/npm 测试 | module.test.ts | viewer/src/results.test.ts |
| Compatibility fixture | compatibility/fixtures/domain/case-name/ | compatibility/fixtures/cli/eval-basic/ |
| BDD feature | module.feature | test/features/cli.feature |

### Fixture 约定（避免多 agent drift）

| Fixture 大小 / 用途 | 落地位置 | 示例 |
|---|---|---|
| 小 (<20 行) | inline test literal | let yaml = "prompts: ..."; |
| 中 (20-100 行) | test/fixtures/module/case-name.ext | test/fixtures/config-loader/basic.yaml |
| 兼容性 golden | compatibility/fixtures/domain/case-name/ | compatibility/fixtures/providers/openai-chat-basic/ |
| 跨 task 复用 | test/fixtures/shared/purpose.ext | test/fixtures/shared/mock-openai-response.json |

### TEST-ID 落地约定（Default Profile，可覆盖）

- Rust: #[test] fn ...() { /* TEST-X.Y.Z */ } 或 rstest case 名含 TEST-X.Y.Z
- TypeScript: test("TEST-X.Y.Z: ...", ...)
- Compatibility harness: fixture metadata 写入 test_id: TEST-X.Y.Z

---

## Commands

> 这些命令使用 POSIX shell 语法。Windows 上用 Git for Windows Bash 执行 S2V helper/adapter commands。

- **Install**: cargo fetch && if [ -f viewer/package.json ]; then (cd viewer && (command -v corepack >/dev/null 2>&1 && corepack enable || true) && pnpm install --frozen-lockfile); fi && if [ -f npm/package.json ]; then (cd npm && (command -v corepack >/dev/null 2>&1 && corepack enable || true) && pnpm install --frozen-lockfile); fi
- **Lint**: bash scripts/release/lint.sh
- **Typecheck**: cargo check --workspace && if [ -f viewer/package.json ]; then (cd viewer && pnpm typecheck); fi && if [ -f npm/package.json ]; then (cd npm && pnpm typecheck); fi
- **Unit Test**: cargo test --workspace && if [ -f viewer/package.json ]; then (cd viewer && pnpm test); fi && if [ -f npm/package.json ]; then (cd npm && pnpm test); fi
- **Integration tests**: bash scripts/release/integration.sh
- **E2E tests**: bash scripts/release/e2e.sh
- **Build**: cargo build --workspace && if [ -f viewer/package.json ]; then (cd viewer && pnpm build); fi && if [ -f npm/package.json ]; then (cd npm && pnpm build); fi
- **Coverage**: bash scripts/release/coverage.sh
- **Runtime smoke**: bash scripts/release/runtime-smoke.sh

### Coverage 判读规则（Default Profile，可覆盖）

Coverage 命令当前执行 release-critical S2V traceability coverage gate：task-15.2 的 lint / integration / e2e / runtime smoke / security / performance 追踪测试必须存在且为绿，并输出 `target/release-gates/coverage.json`。本项目暂不引入 line coverage 外部工具；如后续声明 line coverage 阈值，须先补 ADR 并把本命令升级为真实阈值 gate。依据 PRD §Success Metrics 与 task-15.2 AC1。

---

## Constraints

- **Runtime target**: Rust stable toolchain；Windows 执行 S2V commands 使用 Git for Windows Bash；启用 viewer/npm wrapper 时使用 Node 20+、Corepack、pnpm；启用 Python bridge 时使用 Python 3.10+
- **Supported platforms**: Linux x64/arm64、macOS x64/arm64、Windows x64、Docker、GitHub Actions CI
- **Security requirements**: 默认 local-first；默认不执行 JS/Python/Shell custom code；API key/token/env/provider headers/share payload 必须 redaction；script bridge 需要显式授权和子进程隔离
- **Performance requirements**: CLI 冷启动 < 300ms；1000 条 mock eval case 本地调度与 assertion 执行 < 5s；内存基线 < 100MB；大型结果 JSONL/SQLite 流式写入
- **Compatibility requirements**: baseline 固定 promptfoo 0.121.13 + commit 4860e99；最终以 tag、commit、npm artifact、container artifact 四者可追溯校验为准；P0 golden diff 不通过不得发布 stable；P1 需 snapshot；P2 必须登记 known gap
- **Release constraints**: stable release 必须通过 compatibility release gate；失败只能发 prerelease/nightly；发布渠道包括 GitHub Releases、Homebrew、Cargo、Docker、npm wrapper、GitHub Action 示例

---

## Workflow

- **Collaboration Tier**: solo
  Overrides:
    - git-flow: direct trunk commits; no worktree; no PR required
    - task-mode: AUTO generated Draft specs

---

## Phase 状态索引

| # | Phase | Phase Spec | Status | Tasks | Worktree（仅 team）|
|---|---|---|---|---|---|
| 1 | baseline-freeze | docs/specs/phases/phase-1-baseline-freeze.md | Done | 2 | N/A (solo) |
| 2 | config-cli-core | docs/specs/phases/phase-2-config-cli-core.md | Done | 3 | N/A (solo) |
| 3 | eval-runner-cache | docs/specs/phases/phase-3-eval-runner-cache.md | Done | 2 | N/A (solo) |
| 4 | providers-assertions | docs/specs/phases/phase-4-providers-assertions.md | Done | 3 | N/A (solo) |
| 5 | output-ci | docs/specs/phases/phase-5-output-ci.md | Done | 2 | N/A (solo) |
| 6 | compatibility-harness | docs/specs/phases/phase-6-compatibility-harness.md | Done | 2 | N/A (solo) |
| 7 | redteam-core | docs/specs/phases/phase-7-redteam-core.md | Done | 2 | N/A (solo) |
| 8 | mcp-scan-audit | docs/specs/phases/phase-8-mcp-scan-audit.md | Done | 2 | N/A (solo) |
| 9 | script-bridges-node-api | docs/specs/phases/phase-9-script-bridges-node-api.md | Done | 2 | N/A (solo) |
| 10 | web-viewer-release | docs/specs/phases/phase-10-web-viewer-release.md | Done | 2 | N/A (solo) |
| 11 | upstream-inventory-baseline | docs/specs/phases/phase-11-upstream-inventory-baseline.md | Done | 3 | N/A (solo) |
| 12 | compatibility-fixtures-golden-diff | docs/specs/phases/phase-12-compatibility-fixtures-golden-diff.md | Done | 3 | N/A (solo) |
| 13 | cli-output-eval-parity | docs/specs/phases/phase-13-cli-output-eval-parity.md | Done | 2 | N/A (solo) |
| 14 | provider-assertion-redteam-parity | docs/specs/phases/phase-14-provider-assertion-redteam-parity.md | Done | 2 | N/A (solo) |
| 15 | release-hardening-performance | docs/specs/phases/phase-15-release-hardening-performance.md | Done | 2 | N/A (solo) |
| 16 | parity-proof-hardening | docs/specs/phases/phase-16-parity-proof-hardening.md | Done | 3 | N/A (solo) |
| 17 | deep-upstream-parity-proof | docs/specs/phases/phase-17-deep-upstream-parity-proof.md | Done | 5 | N/A (solo) |
| 18 | perfect-refactor-blocker-burndown | docs/specs/phases/phase-18-perfect-refactor-blocker-burndown.md | Done | 4 | N/A (solo) |
| 19 | source-accounting-native-burndown | docs/specs/phases/phase-19-source-accounting-native-burndown.md | Done | 4 | N/A (solo) |
| 20 | cross-ledger-perfect-claim-closure | docs/specs/phases/phase-20-cross-ledger-perfect-claim-closure.md | Done | 2 | N/A (solo) |
| 21 | upstream-distribution-target-disambiguation | docs/specs/phases/phase-21-upstream-distribution-target-disambiguation.md | Done | 1 | N/A (solo) |
| 22 | perfect-refactor-unblock-packet | docs/specs/phases/phase-22-perfect-refactor-unblock-packet.md | Done | 1 | N/A (solo) |
| 23 | dynamic-upstream-release-observation | docs/specs/phases/phase-23-dynamic-upstream-release-observation.md | Done | 1 | N/A (solo) |
| 24 | current-latest-perfect-refactor | docs/specs/phases/phase-24-current-latest-perfect-refactor.md | Done | 4 | N/A (solo) |
| 25 | current-latest-source-taxonomy-burndown | docs/specs/phases/phase-25-current-latest-source-taxonomy-burndown.md | Done | 1 | N/A (solo) |
| 26 | current-latest-viewer-config-reclassification | docs/specs/phases/phase-26-current-latest-viewer-config-reclassification.md | Done | 1 | N/A (solo) |
| 27 | current-latest-core-config-burndown | docs/specs/phases/phase-27-current-latest-core-config-burndown.md | Done | 1 | N/A (solo) |
| 28 | current-latest-provider-fixture-burndown | docs/specs/phases/phase-28-current-latest-provider-fixture-burndown.md | Done | 1 | N/A (solo) |
| 29 | current-latest-eval-runner-burndown | docs/specs/phases/phase-29-current-latest-eval-runner-burndown.md | Done | 1 | N/A (solo) |
| 30 | current-latest-prompt-processing-burndown | docs/specs/phases/phase-30-current-latest-prompt-processing-burndown.md | Done | 1 | N/A (solo) |
| 31 | current-latest-cache-store-burndown | docs/specs/phases/phase-31-current-latest-cache-store-burndown.md | Done | 1 | N/A (solo) |
| 32 | current-latest-local-prompt-processor-burndown | docs/specs/phases/phase-32-current-latest-local-prompt-processor-burndown.md | Done | 1 | N/A (solo) |

## Task 总索引

| Task | 模块 | Spec 文件 | Status | 依赖 / Phase 内顺序 | Worktree（仅 team）|
|---|---|---|---|---|---|
| 1.1 | compatibility | docs/specs/tasks/task-1.1-baseline-lock.md | Done | Phase 1 AUTO order | N/A (solo) |
| 1.2 | compatibility | docs/specs/tasks/task-1.2-compatibility-matrix.md | Done | Phase 1 AUTO order | N/A (solo) |
| 2.1 | cli | docs/specs/tasks/task-2.1-workspace-cli-skeleton.md | Done | Phase 2 AUTO order | N/A (solo) |
| 2.2 | config-loader | docs/specs/tasks/task-2.2-config-loader.md | Done | Phase 2 AUTO order | N/A (solo) |
| 2.3 | eval-runner | docs/specs/tasks/task-2.3-eval-command-smoke.md | Done | Phase 2 AUTO order | N/A (solo) |
| 3.1 | eval-runner | docs/specs/tasks/task-3.1-scheduler-runtime.md | Done | Phase 3 AUTO order | N/A (solo) |
| 3.2 | cache-resume-store | docs/specs/tasks/task-3.2-cache-resume-retry.md | Done | Phase 3 AUTO order | N/A (solo) |
| 4.1 | provider-registry | docs/specs/tasks/task-4.1-p0-provider-registry.md | Done | Phase 4 AUTO order | N/A (solo) |
| 4.2 | assertion-engine | docs/specs/tasks/task-4.2-assertion-engine.md | Done | Phase 4 AUTO order | N/A (solo) |
| 4.3 | assertion-engine | docs/specs/tasks/task-4.3-custom-assertion-contracts.md | Done | Phase 4 AUTO order | N/A (solo) |
| 5.1 | output-writers | docs/specs/tasks/task-5.1-result-store-schema.md | Done | Phase 5 AUTO order | N/A (solo) |
| 5.2 | output-writers | docs/specs/tasks/task-5.2-output-ci-contracts.md | Done | Phase 5 AUTO order | N/A (solo) |
| 6.1 | compatibility | docs/specs/tasks/task-6.1-upstream-harness-runner.md | Done | Phase 6 AUTO order | N/A (solo) |
| 6.2 | compatibility | docs/specs/tasks/task-6.2-golden-diff-release-gate.md | Done | Phase 6 AUTO order | N/A (solo) |
| 7.1 | redteam-engine | docs/specs/tasks/task-7.1-redteam-command-flow.md | Done | Phase 7 AUTO order | N/A (solo) |
| 7.2 | redteam-engine | docs/specs/tasks/task-7.2-redteam-registry-report.md | Done | Phase 7 AUTO order | N/A (solo) |
| 8.1 | mcp-runtime | docs/specs/tasks/task-8.1-mcp-runtime.md | Done | Phase 8 AUTO order | N/A (solo) |
| 8.2 | scan-engine | docs/specs/tasks/task-8.2-scan-audit-sarif.md | Done | Phase 8 AUTO order | N/A (solo) |
| 9.1 | script-bridge | docs/specs/tasks/task-9.1-script-bridge-sandbox.md | Done | Phase 9 AUTO order | N/A (solo) |
| 9.2 | node-api-wrapper | docs/specs/tasks/task-9.2-node-api-wrapper.md | Done | Phase 9 AUTO order | N/A (solo) |
| 10.1 | web-viewer | docs/specs/tasks/task-10.1-web-viewer.md | Done | Phase 10 AUTO order | N/A (solo) |
| 10.2 | release | docs/specs/tasks/task-10.2-release-docs-packaging.md | Done | Phase 10 AUTO order | N/A (solo) |
| 11.1 | compatibility | docs/specs/tasks/task-11.1-current-upstream-target-policy.md | Done | Phase 11 order 1 | N/A (solo) |
| 11.2 | compatibility-inventory | docs/specs/tasks/task-11.2-item-level-capability-inventory.md | Done | Phase 11 order 2 | N/A (solo) |
| 11.3 | compatibility-matrix | docs/specs/tasks/task-11.3-compatibility-matrix-expansion.md | Done | Phase 11 order 3 | N/A (solo) |
| 12.1 | compatibility-fixtures | docs/specs/tasks/task-12.1-p0-fixture-corpus.md | Done | Phase 12 order 1 | N/A (solo) |
| 12.2 | compatibility-harness | docs/specs/tasks/task-12.2-executable-upstream-rs-runner.md | Done | Phase 12 order 2 | N/A (solo) |
| 12.3 | release-gate | docs/specs/tasks/task-12.3-golden-diff-ci-release-gate.md | Done | Phase 12 order 3 | N/A (solo) |
| 13.1 | cli | docs/specs/tasks/task-13.1-command-flag-parity.md | Done | Phase 13 order 1 | N/A (solo) |
| 13.2 | eval-output-cache | docs/specs/tasks/task-13.2-eval-output-cache-parity.md | Done | Phase 13 order 2 | N/A (solo) |
| 14.1 | provider-assertion | docs/specs/tasks/task-14.1-provider-assertion-inventory-parity.md | Done | Phase 14 order 1 | N/A (solo) |
| 14.2 | redteam-engine | docs/specs/tasks/task-14.2-redteam-plugin-strategy-parity.md | Done | Phase 14 order 2 | N/A (solo) |
| 15.1 | release-viewer-node | docs/specs/tasks/task-15.1-viewer-node-packaging-release.md | Done | Phase 15 order 1 | N/A (solo) |
| 15.2 | release-gates | docs/specs/tasks/task-15.2-performance-security-observability-gates.md | Done | Phase 15 order 2 | N/A (solo) |
| 16.1 | cli | docs/specs/tasks/task-16.1-cli-command-behavior-closure.md | Done | Phase 16 order 1 | N/A (solo) |
| 16.2 | release-gates | docs/specs/tasks/task-16.2-measured-release-gate-reports.md | Done | Phase 16 order 2 | N/A (solo) |
| 16.3 | compatibility | docs/specs/tasks/task-16.3-source-extracted-inventory-real-upstream-smoke.md | Done | Phase 16 order 3 | N/A (solo) |
| 17.1 | compatibility-inventory | docs/specs/tasks/task-17.1-frozen-source-inventory-extractor.md | Done | Phase 17 order 1 | N/A (solo) |
| 17.2 | cli | docs/specs/tasks/task-17.2-cli-global-eval-redteam-parity.md | Done | Phase 17 order 2 | N/A (solo) |
| 17.3 | compatibility-harness | docs/specs/tasks/task-17.3-real-p0-golden-corpus-runner.md | Done | Phase 17 order 3 | N/A (solo) |
| 17.4 | provider-assertion-redteam | docs/specs/tasks/task-17.4-longtail-provider-assertion-redteam-classification.md | Done | Phase 17 order 4 | N/A (solo) |
| 17.5 | release | docs/specs/tasks/task-17.5-release-installability-publication-readiness.md | Done | Phase 17 order 5 | N/A (solo) |
| 18.1 | compatibility-inventory | docs/specs/tasks/task-18.1-source-inventory-ledger-closure.md | Done | Phase 18 order 1 | N/A (solo) |
| 18.2 | providers | docs/specs/tasks/task-18.2-p0-provider-module-fixture-burndown.md | Done | Phase 18 order 2 | N/A (solo) |
| 18.3 | compatibility-target | docs/specs/tasks/task-18.3-current-upstream-rebaseline-gate.md | Done | Phase 18 order 3 | N/A (solo) |
| 18.4 | release | docs/specs/tasks/task-18.4-publication-authority-release-gate.md | Done | Phase 18 order 4 | N/A (solo) |
| 19.1 | compatibility-inventory | docs/specs/tasks/task-19.1-viewer-config-source-reclassification.md | Done | Phase 19 order 1 | N/A (solo) |
| 19.2 | config | docs/specs/tasks/task-19.2-core-config-source-fixture-burndown.md | Done | Phase 19 order 2 | N/A (solo) |
| 19.3 | providers | docs/specs/tasks/task-19.3-provider-request-response-fixture-burndown.md | Done | Phase 19 order 3 | N/A (solo) |
| 19.4 | compatibility-authority | docs/specs/tasks/task-19.4-external-authority-blocker-waiver-gate.md | Done | Phase 19 order 4 | N/A (solo) |
| 20.1 | compatibility-inventory | docs/specs/tasks/task-20.1-source-provider-accounting-reconciliation.md | Done | Phase 20 order 1 | N/A (solo) |
| 20.2 | release | docs/specs/tasks/task-20.2-perfect-refactor-claim-contract.md | Done | Phase 20 order 2 | N/A (solo) |
| 21.1 | compatibility-target | docs/specs/tasks/task-21.1-upstream-distribution-target-gate.md | Done | Phase 21 order 1 | N/A (solo) |
| 22.1 | release | docs/specs/tasks/task-22.1-authority-unblock-packet-gate.md | Done | Phase 22 order 1 | N/A (solo) |
| 23.1 | compatibility-target | docs/specs/tasks/task-23.1-dynamic-github-latest-release-observation.md | Done | Phase 23 order 1 | N/A (solo) |
| 24.1 | compatibility-target | docs/specs/tasks/task-24.1-current-latest-upstream-authority-lock.md | Done | Phase 24 order 1 | N/A (solo) |
| 24.2 | compatibility-inventory | docs/specs/tasks/task-24.2-current-latest-source-inventory-reextract.md | Done | Phase 24 order 2 | N/A (solo) |
| 24.3 | compatibility-harness | docs/specs/tasks/task-24.3-current-latest-full-function-golden-corpus.md | Done | Phase 24 order 3 | N/A (solo) |
| 24.4 | release-quality | docs/specs/tasks/task-24.4-current-latest-exhaustive-quality-gate.md | Done | Phase 24 order 4 | N/A (solo) |
| 25.1 | compatibility-inventory | docs/specs/tasks/task-25.1-current-latest-source-taxonomy-burndown.md | Done | Phase 25 order 1 | N/A (solo) |
| 26.1 | compatibility-inventory | docs/specs/tasks/task-26.1-current-latest-viewer-config-reclassification.md | Done | Phase 26 order 1 | N/A (solo) |
| 27.1 | compatibility-inventory | docs/specs/tasks/task-27.1-current-latest-core-config-burndown.md | Done | Phase 27 order 1 | N/A (solo) |
| 28.1 | providers | docs/specs/tasks/task-28.1-current-latest-provider-fixture-burndown.md | Done | Phase 28 order 1 | N/A (solo) |
| 29.1 | eval-runner | docs/specs/tasks/task-29.1-current-latest-eval-runner-burndown.md | Done | Phase 29 order 1 | N/A (solo) |
| 30.1 | prompt-processing | docs/specs/tasks/task-30.1-current-latest-prompt-processing-burndown.md | Done | Phase 30 order 1 | N/A (solo) |
| 31.1 | cache-resume-store | docs/specs/tasks/task-31.1-current-latest-cache-store-burndown.md | Done | Phase 31 order 1 | N/A (solo) |
| 32.1 | prompt-processing | docs/specs/tasks/task-32.1-current-latest-local-prompt-processor-burndown.md | Done | Phase 32 order 1 | N/A (solo) |

## ADR 索引

| # | Title | Status | File |
|---|---|---|---|
| ADR-001 | Rust core with optional bridges | Accepted | docs/decisions/adr-001-rust-core-with-optional-bridges.md |
| ADR-002 | Stable Rust dependencies | Accepted | docs/decisions/adr-002-stable-rust-dependencies.md |
| ADR-003 | Streaming JSONL and SQLite store | Accepted | docs/decisions/adr-003-streaming-jsonl-sqlite-store.md |
| ADR-004 | CLI and output schema protocol | Accepted | docs/decisions/adr-004-cli-output-schema-protocol.md |
| ADR-005 | Explicit script authorization | Accepted | docs/decisions/adr-005-explicit-script-authorization.md |
| ADR-006 | Compatibility first test toolchain | Accepted | docs/decisions/adr-006-compatibility-first-test-toolchain.md |
| ADR-007 | Upstream golden diff release gate | Accepted | docs/decisions/adr-007-upstream-golden-diff-release-gate.md |
| ADR-008 | Binary first multi channel release | Accepted | docs/decisions/adr-008-binary-first-multi-channel-release.md |
| ADR-009 | P0 P1 P2 compatibility matrix | Accepted | docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md |
| ADR-010 | Node API wrapper contract boundary | Accepted | docs/decisions/adr-010-node-api-wrapper-contract-boundary.md |
| ADR-011 | Current Latest Full Refactor Target | Accepted | docs/decisions/adr-011-current-latest-full-refactor-target.md |

## BDD Feature 索引

| Task(s) | Feature 文件 |
|---|---|
| 4.2, 4.3 | test/features/assertion-engine.feature |
| 3.2 | test/features/cache-resume-store.feature |
| 2.1 | test/features/cli.feature |
| 1.1, 1.2, 6.1, 6.2 | test/features/compatibility.feature |
| 2.2 | test/features/config-loader.feature |
| 2.3, 3.1 | test/features/eval-runner.feature |
| 8.1 | test/features/mcp-runtime.feature |
| 9.2 | test/features/node-api-wrapper.feature |
| 5.1, 5.2 | test/features/output-writers.feature |
| 4.1 | test/features/provider-registry.feature |
| 7.1, 7.2 | test/features/redteam-engine.feature |
| 10.2 | test/features/release.feature |
| 8.2 | test/features/scan-engine.feature |
| 9.1 | test/features/script-bridge.feature |
| 10.1 | test/features/web-viewer.feature |
| 11.1, 11.2, 11.3, 12.1, 12.2, 12.3, 13.1, 13.2, 14.1, 14.2, 15.1, 15.2, 16.1, 16.2, 16.3, 17.1, 17.2, 17.3, 17.4, 17.5, 18.1, 18.2, 18.3, 18.4, 19.1, 19.2, 19.3, 19.4, 20.1, 20.2, 21.1, 22.1, 23.1, 24.1, 24.2, 24.3, 24.4, 25.1, 26.1, 27.1, 28.1, 29.1, 30.1 | test/features/perfect-refactor-parity.feature |
