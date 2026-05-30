# Phase 15: release-hardening-performance

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

补齐 viewer/npm package、multi-channel release evidence、performance/security gates，并将 lint/integration/e2e/coverage/runtime smoke 从 N/A 升级为可执行命令。依据 PRD §Constraints / §Success Metrics / §Release、ADR-008。

## 2. Business Value

完美重构不能只停留在源码和文档层，必须可安装、可运行、可验证、可发布，并在性能和安全默认值上有自动化证据。

## 3. Scope / Modules

viewer/package.json、viewer/、npm/package.json、npm/、.github/workflows/release.yml、Dockerfile、docs/release.md、docs/s2v-adapter.md、tests/performance.rs、tests/security_defaults.rs

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 15.1 | viewer-node-packaging-release | ../tasks/task-15.1-viewer-node-packaging-release.md | Done | 补齐 viewer/npm package、browser smoke 和 release packaging |
| 15.2 | performance-security-observability-gates | ../tasks/task-15.2-performance-security-observability-gates.md | Done | 固化性能、安全、lint/integration/e2e/coverage/runtime gates |

## 5. Dependencies

依赖 Phase 12 release gate、Phase 13 CLI parity、Phase 14 capability parity。

## 6. Phase Acceptance Criteria

- [x] viewer 与 npm wrapper 有 package metadata、lockfile、typecheck/test/build/browser or node smoke。
- [x] release workflow 能执行 full compatibility release gate，并产生 checksums/container digest/npm/cargo/Homebrew dry-run evidence。
- [x] adapter 中 lint/integration/e2e/coverage/runtime smoke 不再为 N/A，且性能/安全默认值有自动化门禁。

## 7. Phase Risks

- 真实发布需要 credentials；缺密钥时必须走 dry-run 或写 BLOCKED release credential 记录。
- coverage/performance 工具在 Windows 与 Linux 差异大；adapter 需定义平台判读规则。

## 8. Definition of Done

- Phase 15 smoke gate 执行 full release candidate verification，不通过不得标记 stable-ready。
