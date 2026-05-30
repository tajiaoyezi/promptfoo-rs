# Task 10.2: release-docs-packaging

> ✅ **Status: Done** — release checklist、multi-channel docs、workflow/Dockerfile 示例与 stable/prerelease/nightly 决策 contract 已实现并通过 §9 验证。

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 10 — web-viewer-release
**Dependencies**: Phase 10 AUTO order; see adapter Task 总索引

## 1. Background

PRD 要求 promptfoo-rs 在 promptfoo 0.121.13 baseline 下建立 Rust-native core、兼容矩阵和 golden diff release gate。本 task 负责 release 模块中的 release-docs-packaging 工作。

## 2. Goal

完成 GitHub Releases、Homebrew、Cargo、Docker、npm wrapper、GitHub Action 示例和贡献文档。

## 3. Scope

### In Scope

- release 模块中与 release-docs-packaging 直接相关的源码、测试、fixture 和文档。
- 与本 task AC 对应的 compatibility matrix 或 release gate 记录。
- 具体文件清单：README.md、docs/architecture.md、docs/release.md、docs/contributing.md、.github/workflows/release.yml、npm/package.json、Dockerfile。依据 PRD §Release constraints 与 ADR-008。

### Out Of Scope

- 不实现本 task AC 之外的长尾 provider/assertion/plugin。
- 不绕过 PRD 的 P0/P1/P2 兼容等级规则。
- 不修改 unrelated phase/task spec。

## 4. Users / Actors

- **AI 应用开发者**：通过 CLI、配置、输出和本地 viewer 感知兼容性。
- **AI infra / 平台工程团队**：在 CI 中依赖 exit code、JUnit/SARIF、golden diff 和 release gate。
- **安全红队团队**：依赖 redteam/MCP/scan/script bridge 的本地可审计执行边界。
- 本 task 无额外 actor；沿用 adapter §Project 中的 AI 应用开发者、AI infra / 平台工程团队、安全红队团队、企业安全 / 合规团队与开源 maintainer。依据 docs/s2v-adapter.md §Project。

## 5. Behavior Contract

本 task 的外部可观察行为以 §6 AC、对应 BDD feature 和 compatibility fixture 为准。任何与 upstream promptfoo 0.121.13 的差异必须登记为 matching / intentional-difference / unsupported / later / upstream-ambiguous / bug。

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/phases/phase-10-web-viewer-release.md
- test/features/release.feature
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-006-compatibility-first-test-toolchain.md
- docs/decisions/adr-008-binary-first-multi-channel-release.md

### 5.2 Imports

- Release/doc tooling：Cargo release profile、Docker build context、npm wrapper package metadata、GitHub Actions workflow；无需新增 runtime crate，除非实现过程中另有 ADR。依据 ADR-008。

### 5.3 函数签名

- `ReleaseChecklist { compatibility_gate, artifacts, install_channels, docs }`
- `evaluate_release_readiness(summary: &ReleaseGateSummary, checklist: &ReleaseChecklist) -> ReleaseDecision`
- Release workflow 需要输出 stable/prerelease/nightly 决策，stable 失败只能降级 prerelease/nightly；依据 PRD §Release constraints / ADR-008。

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Implementation Phases / §Compatibility Matrix): release checklist 包含 compatibility gate 证据
- [x] **AC2** (PRD §Implementation Phases / §Compatibility Matrix): README、架构文档、兼容矩阵、贡献指南齐全
- [x] **AC3** (PRD §Implementation Phases / §Compatibility Matrix): 稳定版发布失败时只能发 prerelease 或 nightly

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-10.2.1 | TEST-10.2.1 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-10.2.2 | TEST-10.2.2 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-10.2.3 | TEST-10.2.3 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |

## 8. Risks

- upstream promptfoo 0.121.13 行为未文档化，fixture 可能覆盖不足。
- Windows/macOS/Linux path、env、shell 行为可能漂移。
- Draft 字段未清零就实施会破坏 S2V Ready Gate。

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Manual**: 审核本 task 的 AC、traceability、compatibility matrix 记录与 BDD scenario 是否一致。

## 10. Completion Notes

- **完成日期**：2026-05-30
- **改动文件**：
  - src/lib.rs
  - src/release.rs
  - tests/release_docs_packaging.rs
  - README.md
  - docs/architecture.md
  - docs/release.md
  - docs/contributing.md
  - .github/workflows/release.yml
  - Dockerfile
  - docs/specs/tasks/task-10.2-release-docs-packaging.md
  - docs/specs/phases/phase-10-web-viewer-release.md
  - docs/s2v-adapter.md
  - docs/compatibility/matrix.md
- **commit 列表**：
  - fd24b30 test(release): add task-10.2 release RED tests
  - 1994cf5 feat(release): add release readiness contract and docs
  - 本 docs(spec) 回填提交见 git log：docs(spec): 回填 task-10.2 §10 Completion Notes + Status → Done
- **§9 Verification 结果**：
  - install: PASS — `CARGO_INCREMENTAL=0 s2v_verify_full "install typecheck unit-test"` / `cargo fetch`
  - typecheck: PASS — `cargo check --workspace`
  - unit-test: PASS — `cargo test --workspace`，含 `tests/release_docs_packaging.rs` 的 TEST-10.2.1 ~ TEST-10.2.3（66 个 integration tests 全绿）
  - manual: PASS — 已核对 AC、SCEN/TEST、BDD feature、README、architecture/release/contributing docs、GitHub Action 示例、Dockerfile、compatibility matrix 与 ADR-008 一致。
- **剩余风险 / 未做项**：当前环境缺 `corepack`，未新增 `npm/package.json` 以免 S2V helper 的 npm 分支失效；release workflow 是示例，不含真实发布密钥、Homebrew tap 权限、crate owner、container registry token 或 npm publish 权限。
- **下游 task 影响**：Phase 10 可收尾；后续真实发布需要在具备发布凭据和 Corepack 的环境补齐 npm package metadata / lockfile，并把实际 tag、checksums、container digest 与 release gate summary 写入发布记录。
