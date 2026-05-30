# Phase 13: cli-output-eval-parity

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

补齐 promptfoo-compatible CLI/flag、eval runtime、cache/resume/retry 和 output 行为，使常见迁移命令可通过 golden diff 验证。依据 PRD §Core Capabilities、§User Flow、§Compatibility Matrix、ADR-004。

## 2. Business Value

现有用户迁移的第一感知面是 CLI。该阶段将 no-op/skeleton 命令变为有行为、有错误、有输出协议的兼容命令。

## 3. Scope / Modules

src/cli.rs、src/eval/、src/cache/、src/output/、src/config/、tests/cli_parity.rs、compatibility/fixtures/cli/

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 13.1 | command-flag-parity | ../tasks/task-13.1-command-flag-parity.md | Ready | 实现 upstream command/flag inventory 的 CLI 行为边界 |
| 13.2 | eval-output-cache-parity | ../tasks/task-13.2-eval-output-cache-parity.md | Ready | 补齐 eval/output/cache/resume/retry 的 P0 golden diff 行为 |

## 5. Dependencies

依赖 Phase 11 inventory 和 Phase 12 executable runner。

## 6. Phase Acceptance Criteria

- [ ] `view/cache/import/export` 不再是空成功占位；每个命令有兼容行为或明确 unsupported/later 错误。
- [ ] `eval` 支持 PRD happy path 中的 output flags、cache/resume/retry 和 CI exit code。
- [ ] CLI stdout/stderr/exit code 进入 golden diff fixture artifact。

## 7. Phase Risks

- upstream CLI flags 存在别名和默认配置加载副作用；必须由 inventory 固定解析规则。
- 输出格式字段漂移会破坏用户脚本；必须使用 schema snapshot 和 golden diff 双重验证。

## 8. Definition of Done

- Phase 13 smoke gate 运行 CLI help inventory check、P0 eval fixture diff、output schema diff 和 no-placeholder-command check。
