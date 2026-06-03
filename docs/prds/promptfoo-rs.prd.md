# promptfoo-rs · 产品需求文档（PRD）

> 本 PRD 由 `/s2v-prd` 生成。
>
> ⚠️ **不要手工重命名章节标题**（`/s2v-init` 按"中文｜English"双语锚点解析）。修改章节内容随时可以；改章节名会让 init 漏读字段。
>
> 解析逻辑：`## Vision` 或 `## 愿景` 任一命中即认；但 `## 产品愿景` / `## Product Vision` 会失败（不在双语模板内）。

**生成日期**：2026-05-30
**作者**：leafiellune
**版本**：v1.0

---

## Vision｜愿景

`promptfoo-rs` 要在 3 个月内成为 `promptfoo 0.121.13` 的可审计 Rust reimplementation：现有 promptfoo 用户可以拿已有 `promptfooconfig.yaml/json`、CI 命令、输出消费脚本和核心 redteam 工作流，在默认不依赖 Node/npm/node_modules 的 Rust 单二进制路径上运行；所有兼容能力、缺口、差异和发布阻断项都通过 compatibility matrix 与 upstream golden diff 追踪。

1.0 不是轻量替代品，也不是重新设计 LLM eval DSL。它的价值来自两个约束同时成立：第一，兼容 promptfoo 0.121.13 已文档化能力域；第二，把默认执行路径收敛到 Rust-native core、可复现 fixture、可审计输出和明确的脚本执行授权边界。

---

## Problem Statement｜问题陈述

**谁有这个问题**：
已经在用 promptfoo 做 LLM eval、redteam、CI 回归测试的 AI 应用开发者、AI infra 团队、平台工程团队和安全红队团队。他们通常在企业内网、CI/CD、离线环境或安全敏感环境运行评测，需要稳定 exit code、可审计结果文件、可控依赖链和可迁移配置。

**痛点**：
promptfoo 当前功能完整，但 Node/TypeScript 运行时、npm 依赖、node_modules 体积、供应链依赖和 CI 冷启动成本，对企业安全环境和基础设施场景不够理想。复杂 eval 涉及 provider 调用、缓存、并发、重试、resume、输出格式和 CI 集成，现有用户还沉淀了大量 `promptfooconfig.yaml`、assertions、自定义 provider、自定义脚本和输出消费脚本；迁移成本主要来自行为细节不一致，而不是 Rust 语言本身。

**现状**：
现有选择包括继续使用 promptfoo 原项目、写一个轻量 Rust eval runner、或只做 provider/assertion 局部高性能模块。继续使用原项目能获得最完整功能，但保留 Node/npm 依赖链；轻量 runner 可以快速落地，但无法承接现有配置和 CI 输出；局部模块风险较低，但不能形成完整开源差异化，也无法验证 promptfoo 行为兼容。

竞品和对标包括 promptfoo 原项目、Ragas、DeepEval、LangSmith、Langfuse、Phoenix、自研脚本加 CI、以及轻量 LLM eval runners。`promptfoo-rs` 的核心对标对象是 promptfoo upstream；其他工具主要用于定位边界：Ragas/DeepEval 更偏框架，LangSmith/Langfuse/Phoenix 更偏平台和观测，自研脚本无法标准化，轻量 runner 缺少兼容生态。

**为什么是现在**：
AI eval、LLM regression testing、redteam、MCP、Agent 安全测试正在变成 AI 应用工程基础设施。promptfoo 已证明需求真实存在；企业对 AI 工具链的供应链安全、内网部署、离线运行、可重复 CI、结果可审计和本地执行边界更敏感。Rust 重构的差异化集中在部署形态、单二进制、并发调度、缓存/resume、低依赖和安全默认值。

---

## Users & Context｜用户与场景

**主要用户**：
- **AI 应用开发者**：维护 prompts、providers、test cases 和 assertions，需要本地与 CI 一致运行 eval。
- **AI infra / 平台工程团队**：把 eval 接入 CI/CD、模型网关、内部 provider、缓存和结果归档系统。
- **安全红队团队**：运行 redteam 生成、攻击策略、风险评分和报告输出，需要本地可审计执行。
- **企业安全 / 合规团队**：审查工具链依赖、脚本执行、密钥脱敏、离线运行和 share/cloud 边界。

**次要用户 / 利益相关者**：
- **promptfoo 深度用户和社区 contributor**：关注兼容完整性、品牌边界、license notice 和行为差异解释。
- **DevOps / CI 维护者**：关注安装方式、冷启动时间、exit code、JUnit/SARIF 输出和失败可定位性。
- **开源维护者**：关注 issue triage、兼容矩阵更新成本、fixture 归档和 release gate 可执行性。

**关键使用场景**：
1. **CI 回归评测**：团队在 GitHub Actions 或内部 CI 执行 `promptfoo-rs eval -c promptfooconfig.yaml --output junit.xml --output results.jsonl`，用 exit code 和 JUnit 作为合并阻断。
2. **企业内网离线评测**：开发者在不能安装 npm 依赖或不能访问公网 registry 的环境，用单二进制运行本地 HTTP/Ollama/OpenAI-compatible provider。
3. **红队安全测试**：安全团队从 `redteam.yaml` 初始化、生成、运行、评分并输出报告，同时要求 prompts、vars、outputs 默认不上传。
4. **兼容迁移评估**：promptfoo 用户把现有 fixtures 同时交给 upstream promptfoo 和 `promptfoo-rs`，用 golden diff 判断能否迁移。
5. **自定义扩展保留**：已有 JS/Python/Shell custom provider/assertion 的团队显式开启脚本 bridge，在隔离子进程中继续运行兼容扩展。

---

## Core Capabilities｜核心能力

> ≤ 5 条。多于 5 条说明范围还没收敛 — 拆 v1.0 / v1.1 / v2.0。

1. **promptfoo-compatible CLI/runtime**：覆盖 promptfoo 0.121.13 的全部已文档化能力域，1.0 至少跑通 `eval`、`view`、`cache`、`redteam`、`mcp`、`code-scans`、`scan-model`、`import/export` 的兼容闭环与常用 flags。
2. **compatibility harness**：冻结 upstream baseline 后，对同一 fixture 分别运行 promptfoo upstream 与 `promptfoo-rs`，在 mock provider 下做 golden diff；P0 能力不通过不得发布 stable。
3. **Rust-native eval core**：配置解析、eval 调度、provider 调用、assertion 执行、cache/resume、retry/backoff、限速、流式结果写入默认由 Rust core 承担。
4. **provider/assertion/script bridge**：OpenAI-compatible、HTTP、Ollama、Anthropic 作为 P0 provider；JS/TS、Python、Shell custom provider/assertion 通过显式授权 bridge 保留兼容。
5. **local viewer 与多渠道分发**：本地 Web viewer 读取 JSONL/SQLite 结果；输出 JSON、JSONL、CSV、YAML、HTML、JUnit XML、SARIF；发布 GitHub Releases、Homebrew、Cargo、Docker、npm wrapper 和 GitHub Action 示例。

**明确不做（Out of Scope，至少列 3 项）**：
- 不做 promptfoo cloud/share SaaS 的替代服务；任何 share/cloud 相关 API 只登记兼容边界、错误行为和品牌风险，不把数据上传作为默认能力。
- 不承诺所有长尾 provider 都有 Rust 原生实现；但 1.0 必须在兼容矩阵中登记所有 promptfoo 0.121.13 已文档化 provider/assertion/redteam/plugin/CLI 能力，并标明 `native` / `bridge` / `unsupported` / `later`。
- 不默认执行 JS/Python/Shell/Ruby custom code；脚本扩展必须通过 `--allow-scripts` 或配置显式开启。
- 不重新设计 promptfoo 配置格式、assertion DSL、输出格式或 CLI 语义；新增 Rust-specific 选项不得破坏 promptfoo-compatible 默认路径。
- 不把非稳定 Web UI 像素级还原作为 1.0 gate；1.0 关注结果可读、筛选、导出和与稳定结果 schema 的一致性。

---

## User Flow｜用户流程

**主流程（happy path）**：
1. 用户在已有项目中执行 `promptfoo-rs eval -c promptfooconfig.yaml --output results.jsonl --output junit.xml`。
2. 系统加载 `.env`、配置文件、file prompts、CSV/JSON/YAML tests，解析 providers、prompts、tests、assertions 和 CLI flags。
3. 系统构建 eval graph，按 `max-concurrency`、provider rate limit、delay、retry/backoff 和 cache 策略调度执行。
4. 系统把每条结果流式写入 JSONL/SQLite，生成终端摘要、JUnit/CSV/SARIF/HTML 等请求的输出格式。
5. 用户在 CI 中读取 exit code 与 JUnit/SARIF，或执行 `promptfoo-rs view` 打开本地 viewer 检查失败样本。

**异常流（≥ 2 项）**：
- **provider 超时 / 限流**：按配置的 `retry-errors`、退避、delay 和 provider-scoped 限速重试；最终失败写入结构化错误、保留 partial results，并按 promptfoo-compatible exit code 退出。
- **脚本扩展未授权**：配置中引用 JS/Python/Shell custom provider/assertion 但未开启 `--allow-scripts` 时，系统拒绝执行，stderr 指出具体配置路径、脚本类型和启用方式。
- **compatibility diff 失败**：harness 输出 fixture ID、字段路径、upstream 值、rs 值、差异类别和是否 release-blocking；P0 未解释差异阻断 stable release。
- **缓存/resume 文件损坏**：系统跳过不可解析条目并记录损坏位置；默认不删除用户缓存，提供明确命令用于重建。
- **share/cloud 能力被调用**：默认返回本地不支持或需显式替代配置的错误，不上传 payload；错误文案必须避免暗示本项目提供 promptfoo cloud 服务。

**边界场景（≥ 1 项）**：
- **大型 eval**：10k+ cases 或大模型输出时，runner 必须流式写入 JSONL/SQLite，终端摘要只保留聚合统计和失败索引，避免完整结果集常驻内存。
- **非确定性 model-graded assertion**：golden diff 不比较原始 LLM 文本的 byte-level 一致性，而比较归一化后的评分、阈值判断、metadata schema 和可解释差异标签。
- **跨平台路径与 shell quoting**：Windows x64、macOS arm64、Linux x64 对 `.env`、文件路径、脚本参数和换行的解析必须有 fixture 覆盖。

---

## Technical Approach｜技术方案

- **项目类型**：Infrastructure / CLI / Library / Web local viewer / Compatibility runtime。
- **技术栈**：Rust + Tokio + clap + serde + reqwest + axum + sqlx/libSQL + tracing；Web viewer 使用 TypeScript + React + Vite 或 Next.js + Tailwind + shadcn/ui + TanStack Table；JS/TS 兼容使用 Node worker 或 napi-rs；Python/Shell/Ruby 兼容使用隔离 subprocess；MCP 使用 Rust MCP client/server 或协议自实现。
- **关键模块边界**（≥ 3 个，越具体越好）：
  - `cli`：解析 promptfoo-compatible commands、flags、exit code、stdout/stderr 协议。
  - `config-loader`：加载 `promptfooconfig.yaml/json`、`redteam.yaml`、`.env`、file prompts、CSV/JSON/YAML tests，并保留 upstream 解析差异记录。
  - `eval-runner`：构建 eval graph，执行并发调度、delay、retry/backoff、rate limit、partial failure 和 cancellation。
  - `provider-registry`：注册 native provider、bridge provider、request normalization、response normalization 和 provider-scoped config/env。
  - `assertion-engine`：执行 deterministic assertions、model-graded assertions、custom assertions 和评分聚合。
  - `cache-resume-store`：管理 cache key、resume cursor、SQLite/libSQL schema、JSONL append 和损坏恢复策略。
  - `output-writers`：生成 JSON、JSONL、CSV、YAML、HTML、JUnit XML、SARIF 和 terminal summary。
  - `redteam-engine`：实现 redteam init/generate/eval/run/report、插件/strategy registry、风险评分和报告输出。
  - `mcp-runtime`：实现 `promptfoo mcp`、MCP provider、MCP target materialization、client/server protocol adapter。
  - `scan-engine`：实现 code-scans、scan-model、model-audit 兼容命令和 SARIF 输出。
  - `compat-harness`：调用 upstream promptfoo baseline 与 `promptfoo-rs`，做 fixture orchestration、golden diff、差异分类和 release gate 汇总。
  - `script-bridge`：隔离执行 JS/TS、Python、Shell/Ruby custom provider/assertion，控制 env、stdio、timeout 和 redaction。
  - `web-viewer`：读取 SQLite/JSONL 结果，展示 eval table、filter、diff、失败样本和导出入口。
  - `node-api-wrapper`：提供 npm package 和 Node API wrapper，把 JS programmatic usage 桥接到 Rust core。
- **架构风格**：模块化单体。Rust core 作为稳定内部 API；CLI、viewer、Node wrapper、script bridge 和 compatibility harness 通过明确边界调用 core。
- **数据流（如适用）**：配置/env/test files → config-loader → eval graph → provider-registry/assertion-engine/script-bridge → eval-runner → cache-resume-store → output-writers/web-viewer/CI artifacts。Compatibility harness 额外执行 upstream promptfoo → upstream artifacts → normalized diff。

---

## Constraints｜约束

- **运行时**：默认路径为 Rust 单二进制，无 Node/Python 运行时要求；启用 JS/TS bridge 时要求 Node 20+，启用 Python bridge 时要求 Python 3.10+；npm wrapper 仅作为分发和 Node API 兼容层。
- **平台**：Linux x64/arm64、macOS x64/arm64、Windows x64、Docker、GitHub Actions CI。
- **性能**：CLI 冷启动 < 300ms，不含网络模型调用；1000 条 mock eval case 的本地调度与 assertion 执行 < 5s；内存基线 < 100MB，不含 Web viewer 大型结果加载；大型结果使用 JSONL/SQLite 流式写入。
- **安全**：默认 local-first，不上传 prompts、vars、outputs；默认不执行 custom scripts；API key/token/env/provider headers/share payload 日志必须 redaction；threat model 覆盖配置任意代码执行、provider 请求泄露、CI secret 泄露、share payload 泄露、插件供应链风险。
- **兼容性**：1.0 目标是覆盖 promptfoo 0.121.13 的全部已文档化能力域，并建立完整兼容矩阵。provider/assertion/redteam/plugin 按 P0/P1/P2 标注兼容等级：P0 必须可运行并通过 golden diff；P1 必须有协议、请求、输出快照测试；P2 至少登记为 known gap，不能沉默遗漏。
- **发布**：GitHub Releases 二进制、Homebrew tap、`cargo install`、Docker image、npm wrapper 包、GitHub Action 示例。稳定版发布必须通过 compatibility release gate；失败时只能发 prerelease 或 nightly。

---

## Upstream Baseline Freeze Strategy｜上游基线冻结策略

1. **候选冻结基线**：`promptfoo 0.121.13 + commit 4860e99`。
2. **最终冻结条件**：以 tag、commit、npm artifact、container artifact 四者可追溯校验为准。四者必须写入 `docs/compatibility/baseline.lock.md` 或等价 lock artifact，包含版本号、commit SHA、npm tarball integrity、container digest、采集时间、采集命令和来源 URL。
3. **不可变引用**：PRD 和 phase/task specs 只引用冻结基线，不引用 `latest`。发现 upstream 新版本时，只能通过新 PRD 或兼容矩阵变更流程纳入。
4. **证据来源**：GitHub Releases、npm package artifact、GitHub Container Package、upstream repository tag/commit。任一来源缺失或不一致时，Phase 1 不得完成。
5. **差异政策**：所有与 upstream 不一致的行为必须标为 `matching`、`intentional-difference`、`unsupported`、`later`、`upstream-ambiguous` 或 `bug`；P0 的 `bug` / 未分类差异阻断 stable release。

---

## Current Latest Rebaseline Addendum｜当前最新重基线补充

2026-06-01 用户明确“完美重构”目标应基于原始 promptfoo 项目当前最新版本的完整功能，并要求大量测试来尽可能排除潜在缺陷。该目标新增一条 current-latest rebaseline track，不删除原有 frozen baseline track。

**当前最新目标定义**：
- “当前最新”必须被锁定为 task runtime 观测到的 immutable target packet；不得把浮动 `latest`、`main`、`master`、`HEAD` 字符串直接当作完成证据。
- 观测包必须同时记录 npm latest stable package、GitHub default branch HEAD、GitHub latest release channel、采集命令、完整 SHA、artifact URL / integrity 和采集时间。
- 2026-06-01 本地观测为：npm `promptfoo@latest=0.121.13` / `4860e990c7e9a2f8f677173fb92cf9867b34d03f`，GitHub default branch HEAD `1d09dfeb5f0766905409117f923dd5c4b0838d9f`，GitHub latest release `code-scan-action-0.1.7` / `1c743afe0e4807882e858c4f322fc064fa5f0770`。

**质量声明边界**：
- 项目不得承诺数学意义上的“无任何潜在 bug”；有限测试无法证明所有未来输入、平台、provider、网络和并发条件下不存在缺陷。
- 可发布声明必须改写为：“在声明的 S2V verification gates、current-latest golden diff、source inventory coverage、stress/regression/property tests、external authority 和 publication evidence 下，无已知 release-blocking 缺陷。”
- 若真实 provider credentials、账号、私有服务、法律/品牌授权或发布渠道证据缺失，则必须继续显示为 blocker 或 formal waiver，不能用 mock/recorded fixtures 伪装成 live parity。

依据：用户 2026-06-01 澄清、ADR-007、ADR-009、ADR-011。

---

## Compatibility Matrix｜兼容矩阵

**兼容等级定义**：
- **P0**：1.0 必须可运行并通过 upstream golden diff；不通过不得发布 stable。
- **P1**：1.0 必须有协议、请求、输出或 schema snapshot 测试；允许不做全量 golden diff，但不能无测试。
- **P2**：1.0 至少登记 known gap、unsupported reason、later target 或 bridge 计划；不能沉默遗漏。

**实现状态定义**：
- **native**：Rust core 原生实现。
- **bridge**：通过 Node/Python/Shell/Ruby/npm wrapper 等兼容桥实现。
- **unsupported**：1.0 明确不支持，必须有用户可见错误和迁移说明。
- **later**：已登记但推迟到后续版本，必须有理由和验证缺口。

| 能力域 | 1.0 等级 | 目标状态 | 验证要求 | 备注 |
|---|---|---|---|---|
| CLI command/flag inventory | P0 | native | 全部已文档化命令和常用 flags 进入矩阵；P0 命令 golden diff stdout/stderr/exit code | 覆盖 `eval`、`view`、`cache`、`redteam`、`mcp`、`code-scans`、`scan-model`、`import/export` |
| `promptfooconfig.yaml/json` | P0 | native | fixture golden diff config normalization、vars、prompts、tests、providers、assertions | 不重新设计格式 |
| `redteam.yaml` | P0 | native | redteam init/generate/eval/run/report fixture golden diff | 插件/strategy 按 P0/P1/P2 子矩阵登记 |
| `.env` 与 file prompts/tests | P0 | native | path/env/newline fixture 覆盖 Linux/macOS/Windows | CSV/JSON/YAML tests 均覆盖 |
| Eval runner | P0 | native | mock provider 下结果、metadata、latency shape、error shape golden diff | 网络 provider 不比较真实延迟 |
| Cache/resume/retry/concurrency/delay | P0 | native | cache key、resume cursor、partial results、retry 行为 fixture | Azure/assistant 等特殊 key 进入矩阵 |
| OpenAI-compatible provider | P0 | native | request/response snapshot + golden diff | 支持 env/header/model/options |
| HTTP provider | P0 | native | request template、headers、body、response transform snapshot | 覆盖常用 auth/header 场景 |
| Ollama provider | P0 | native | local mock server snapshot + golden diff | 不要求真实模型下载 |
| Anthropic provider | P0 | native | request/response snapshot + golden diff | 网络调用用 mock |
| 其他已文档化 providers | P1/P2 | native/bridge/later | 全量登记；P1 至少请求/输出 snapshot；P2 known gap | Phase 1 生成完整 provider 子矩阵 |
| Deterministic assertions | P0 | native | assertion result golden diff | equals/contains/regex/json/schema 等核心断言进入 P0 |
| Model-graded assertions | P1 | native/bridge | 评分协议、prompt、threshold、metadata snapshot | 因 LLM 非确定性，不要求原始文本 byte-level golden diff |
| JS/TS custom provider/assertion | P0 | bridge | `--allow-scripts` fixture，stdio/env/timeout/error snapshot | 默认禁用也是 P0 行为 |
| Python custom provider/assertion | P0 | bridge | subprocess fixture，stdio/env/timeout/error snapshot | 默认禁用也是 P0 行为 |
| Shell/Ruby custom scripts | P1 | bridge | subprocess snapshot + security gate | Ruby 若 upstream 文档覆盖则登记 |
| JSON/JSONL/CSV/YAML output | P0 | native | schema + golden diff | 大结果必须流式写入 |
| HTML/JUnit XML/SARIF output | P0/P1 | native | JUnit/SARIF schema snapshot；HTML stable data contract snapshot | SARIF 与 scan phase 绑定 |
| Local Web viewer | P1 | native web | 读取 result schema、filter、失败样本、导出 smoke test | 不做像素级 upstream UI 复刻 |
| Redteam plugins/strategies | P0/P1/P2 | native/later | 全量登记；核心插件 P0 golden diff；其他 P1/P2 标注 | 不沉默遗漏 |
| MCP provider / `promptfoo mcp` | P1 | native | protocol/request/response snapshot | 以已文档化命令和 provider 能力为准 |
| code-scans / scan-model / model-audit | P1 | native | CLI protocol、SARIF、finding schema snapshot | 安全扫描误报率另列非 1.0 gate |
| Node API wrapper | P1 | bridge | JS API contract snapshot 与 Rust core 行为一致性测试 | 防止 wrapper/core 漂移 |
| promptfoo cloud/share | P2 | unsupported/later | 能力登记、错误行为、品牌说明、无上传测试 | 1.0 不提供 SaaS |

Phase 1 必须生成更细粒度的 compatibility matrix artifact，逐项列出 promptfoo 0.121.13 已文档化 provider、assertion、redteam plugin/strategy、CLI command/flag、output format 和 config feature。PRD 级矩阵定义覆盖政策；完整项级矩阵是 release gate 输入。

---

## Compatibility Harness Design｜兼容性测试设计

1. **Fixture source**：从 upstream examples/docs、最小手写 fixtures、回归 issue fixtures 和用户提供真实配置裁剪样本组成。每个 fixture 必须标注能力域、P0/P1/P2、是否使用 mock provider、是否需要 script bridge。
2. **Execution model**：harness 固定执行 `upstream promptfoo@0.121.13` 与当前 `promptfoo-rs`；同一输入目录、同一 env fixture、同一 mock provider 响应、同一时间/随机数 seed。
3. **Normalization**：对时间戳、绝对路径、随机 ID、latency、平台换行、对象 key 顺序和非确定性 model output 做归一化；归一化规则本身需要 snapshot。
4. **Diff classes**：`matching`、`intentional-difference`、`unsupported`、`later`、`upstream-ambiguous`、`bug`。P0 中 `bug`、未分类差异和缺 fixture 都是 release blocker。
5. **Artifacts**：每次运行输出 upstream artifact、rs artifact、normalized artifact、diff report、matrix coverage report 和 release gate summary。
6. **Model-graded policy**：model-graded assertions 不用真实 LLM 输出做稳定 golden；使用 mock grader 或 recorded response，比较 prompt construction、threshold、score parsing、pass/fail decision 和 metadata schema。
7. **CI policy**：PR 必跑快速 P0 smoke；release candidate 必跑完整 P0 golden diff 与 P1 snapshot；P2 必校验登记完整性。

---

## Implementation Phases｜实施阶段

> `/s2v-init` 会读这张表批量生成 phase spec 和 task spec。要求：
> - `description` 列写"完成后能做什么"，不写 TODO 风格
> - `scope` 列要列出**具体模块名 / 文件名**（不写"全部代码"）
> - `depends_on` 用 phase 编号；零依赖写 `-`
> - `parallel` 标"是 / 否"；写"是"时必须说明"可与谁并行"

| # | Phase 名称（kebab）| 描述（完成后能做什么）| 范围（涉及模块 / 文件）| 依赖 | 可并行 |
|---|---|---|---|---|---|
| 1 | baseline-freeze | 冻结 `promptfoo 0.121.13 + 4860e99`，生成 baseline lock 和完整兼容矩阵骨架 | `compat-harness` + `docs/compatibility/baseline.lock.md` + `docs/compatibility/matrix.md` | - | 否 |
| 2 | config-cli-core | `promptfoo-rs eval -c promptfooconfig.yaml` 能解析基础配置、env、prompts、tests 并进入 runner | `cli` + `config-loader` + `eval-runner` | 1 | 否 |
| 3 | eval-runner-cache | runner 支持并发、retry、delay、cache、resume、partial result 和 cancellation | `eval-runner` + `cache-resume-store` + integration tests | 2 | 否 |
| 4 | providers-assertions | P0 provider 与核心 assertions 可在 mock provider 下通过 golden diff | `provider-registry` + `assertion-engine` + fixtures | 2 | 是（可与 phase 5 并行）|
| 5 | output-ci | JSON/JSONL/CSV/YAML/JUnit/SARIF/HTML 输出和 CLI exit code 协议稳定 | `output-writers` + `cli` + schema snapshots | 2 | 是（可与 phase 4 并行）|
| 6 | compatibility-harness | upstream 与 `promptfoo-rs` 的 P0 golden diff、P1 snapshot 和 release gate 自动化可运行 | `compat-harness` + fixtures + CI scripts | 1, 3, 4, 5 | 否 |
| 7 | redteam-core | redteam init/generate/eval/run/report 最小兼容闭环、核心插件/strategy registry、风险评分和 report 输出可运行 | `redteam-engine` + `config-loader` + `output-writers` + redteam fixtures | 4, 5, 6 | 是（可与 phase 8 和 9 并行）|
| 8 | mcp-scan-audit | `promptfoo mcp`、MCP provider、code-scans、scan-model、model-audit 和 SARIF 输出形成兼容闭环 | `mcp-runtime` + `scan-engine` + `output-writers` + SARIF snapshots | 4, 5, 6 | 是（可与 phase 7 和 9 并行）|
| 9 | script-bridges-node-api | JS/TS、Python、Shell/Ruby custom provider/assertion bridge 与 npm Node API wrapper 可运行并有 drift 测试 | `script-bridge` + `node-api-wrapper` + bridge fixtures | 4, 5, 6 | 是（可与 phase 7 和 8 并行）|
| 10 | web-viewer-release | 本地 viewer 可读取结果并完成跨平台发布、安装、文档和 release gate 汇总 | `web-viewer` + release scripts + README + docs + GitHub Actions | 6, 7, 8, 9 | 否 |
| 11 | upstream-inventory-baseline | 审计后的当前 upstream 目标政策、能力项 inventory、兼容矩阵扩展可追溯，避免“完美重构”目标停留在旧 baseline 或粗粒度矩阵 | `docs/compatibility/` + `compatibility/inventory/` + `docs/audits/` + matrix tests | 10 | 否 |
| 12 | compatibility-fixtures-golden-diff | P0 fixture corpus、upstream/rs 可执行 runner、golden diff CI release gate 覆盖审计发现的缺口 | `compatibility/fixtures/` + `compatibility/harness/` + `tests/` + release gate scripts | 11 | 否 |
| 13 | cli-output-eval-parity | CLI commands/flags、eval outputs、cache/resume/retry 行为达到 item-level parity 或有明确分类证据 | `src/cli` + `src/output` + `src/cache` + `tests/` + compatibility fixtures | 12 | 否 |
| 14 | provider-assertion-redteam-parity | provider/assertion/redteam plugin/strategy inventory 对齐 upstream，P0 有 fixture，P2/later 有原因和用户可见行为 | `src/providers` + `src/assertions` + `src/redteam` + `docs/compatibility/` | 13 | 否 |
| 15 | release-hardening-performance | viewer/npm packaging、lint/integration/e2e/coverage/runtime-smoke、性能、安全、观测 release gates 均可执行 | `viewer/` + `npm/` + `scripts/release/` + `.github/workflows/` + adapter commands | 14 | 否 |
| 16 | parity-proof-hardening | 移除审查中仍可观察到的 explicit later CLI 命令、合成 release 证据和非真实 upstream smoke 证据 | `src/cli` + `src/viewer_server.rs` + `src/cache` + `scripts/release/` + `compatibility/inventory/` + `compatibility/artifacts/` + `tests/` | 15 | 否 |
| 17 | deep-upstream-parity-proof | 完整 frozen upstream source inventory、CLI/global/eval/redteam parity、50+ 真实 upstream golden corpus、长尾 capability 分类与发布安装证据均可执行 | `compatibility/inventory/` + `compatibility/matrix/` + `compatibility/fixtures/` + `compatibility/artifacts/` + `src/cli` + `src/providers` + `src/assertions` + `src/redteam` + `scripts/release/` + `.github/workflows/` | 16 | 否 |
| 18 | perfect-refactor-blocker-burndown | 将 Phase 17 后仍阻断“完美重构”的 source missing rows、P0 provider blockers、current-upstream target 和 publication authority 转成可燃尽 release gate | `compatibility/inventory/` + `compatibility/matrix/` + `compatibility/fixtures/` + `src/compatibility/` + `src/providers` + `scripts/release/` + `docs/audits/` + `tests/` | 17 | 否 |
| 19 | source-accounting-native-burndown | 继续燃尽 Phase 18 暴露的 P0 source accounting/provider blockers：纠正 viewer config 分类、补 core config/provider fixtures、集中 external authority blockers | `src/compatibility/` + `src/config/` + `src/providers/` + `scripts/release/` + `compatibility/fixtures/` + `docs/compatibility/` + `docs/audits/` + `tests/` | 18 | 否 |
| 20 | cross-ledger-perfect-claim-closure | 关闭 Phase 19 后 source accounting/provider burndown/release claim 的跨 artifact 口径差异：fixture-covered provider rows 不再重复计入 source blocker，并新增 perfect-refactor claim contract | `src/compatibility/` + `src/release.rs` + `scripts/release/` + `target/release-gates/` + `docs/compatibility/` + `docs/audits/` + `tests/` | 19 | 否 |
| 21 | upstream-distribution-target-disambiguation | 区分 npm core package 最新发布、GitHub repository HEAD、GitHub latest release tag 与 frozen baseline 的关系，防止把 non-core release 或 unreleased HEAD 漂移误读为 current promptfoo 完成或缺口 | `src/compatibility/inventory.rs` + `scripts/release/upstream-distribution-target.sh` + `scripts/release/runtime-smoke.sh` + `target/release-gates/` + `docs/compatibility/` + `docs/audits/` + `tests/` | 20 | 否 |
| 22 | perfect-refactor-unblock-packet | 将 Phase 21 后仍阻止 perfect-refactor claim 的 source/external/current/publication blockers 聚合成最小用户/维护者决策包，明确哪些项无法由 agent 自动解决 | `src/release.rs` + `scripts/release/perfect-refactor-unblock-packet.sh` + `scripts/release/runtime-smoke.sh` + `target/release-gates/` + `docs/release.md` + `docs/compatibility/` + `docs/audits/` + `tests/` | 21 | 否 |
| 23 | dynamic-upstream-release-observation | 修正 upstream distribution target gate 的 GitHub latest release 观测方式：不再把 `code-scan-action-0.1.7` 当固定 latest，而是从 GitHub latest release metadata 动态解析 release tag 并写入 source evidence | `scripts/release/upstream-distribution-target.sh` + `scripts/release/runtime-smoke.sh` + `target/release-gates/` + `tests/upstream_distribution_target_gate.rs` + `docs/compatibility/target-policy.md` + `docs/compatibility/matrix.md` + `docs/audits/` | 22 | 否 |
| 24 | current-latest-perfect-refactor | 依据用户澄清，把目标切到原始 promptfoo 当前最新完整功能：锁定 current-latest target、重抽 source inventory、扩展 250+ 或全量 golden corpus，并新增大规模质量 gate | `docs/compatibility/current-latest.lock.md` + `compatibility/inventory/` + `compatibility/matrix/` + `compatibility/fixtures/current-latest/` + `compatibility/artifacts/current-latest/` + `src/compatibility/` + `src/release.rs` + `scripts/release/` + `tests/` | 23 | 否 |
| 25 | current-latest-source-taxonomy-burndown | 消费 Phase 24 质量门暴露的 318 个 current-latest unclassified source/matrix rows：按 PRD 模块边界和 ADR-009 分级规则给出 deterministic taxonomy、owner、evidence kind，并保留真实 P0/external/publication blockers | `src/compatibility/inventory.rs` + `scripts/release/current-latest-source-inventory.sh` + `scripts/release/current-latest-golden-corpus.sh` + `scripts/release/current-latest-quality-gate.sh` + `target/release-gates/` + `tests/current_latest_source_taxonomy_burndown.rs` + `test/features/perfect-refactor-parity.feature` | 24 | 否 |
| 26 | current-latest-viewer-config-reclassification | 将 current-latest target 中 `src/app/**` viewer config source 的 duplicate P0 config blockers 纠正为 P1 viewer evidence，同时保留 non-app core config P0 blockers | `src/compatibility/inventory.rs` + `scripts/release/current-latest-source-inventory.sh` + `scripts/release/current-latest-golden-corpus.sh` + `scripts/release/current-latest-quality-gate.sh` + `target/release-gates/` + `tests/current_latest_viewer_config_reclassification.rs` + `test/features/perfect-refactor-parity.feature` | 25 | 否 |
| 27 | current-latest-core-config-burndown | 将 current-latest 18 个 non-app config blockers 拆为 runtime/redteam fixture evidence、auxiliary snapshot evidence 和 explicit external authority blockers，消除 generic config blocker | `src/compatibility/inventory.rs` + `scripts/release/current-latest-source-inventory.sh` + `scripts/release/current-latest-golden-corpus.sh` + `scripts/release/current-latest-quality-gate.sh` + `target/release-gates/` + `tests/current_latest_core_config_burndown.rs` + `test/features/perfect-refactor-parity.feature` | 26 | 否 |
| 28 | current-latest-provider-fixture-burndown | 将 current-latest 38 个 provider blockers 拆为 fixture-covered provider rows 和 explicit external authority blockers，复用 task 19.3/19.4 的 provider 决策模型 | `src/compatibility/inventory.rs` + `scripts/release/current-latest-source-inventory.sh` + `scripts/release/current-latest-golden-corpus.sh` + `scripts/release/current-latest-quality-gate.sh` + `target/release-gates/` + `tests/current_latest_provider_fixture_burndown.rs` + `docs/compatibility/matrix.md` + `test/features/perfect-refactor-parity.feature` | 27 | 否 |
| 29 | current-latest-eval-runner-burndown | 将 current-latest 18 个 eval-runner blockers 拆为已有 eval/scheduler fixture evidence、P1 snapshot evidence 和保留 P0 rate-limit/adaptive/provider-wrapper blockers | `src/compatibility/inventory.rs` + `scripts/release/current-latest-source-inventory.sh` + `scripts/release/current-latest-golden-corpus.sh` + `scripts/release/current-latest-quality-gate.sh` + `target/release-gates/` + `tests/current_latest_eval_runner_burndown.rs` + `docs/compatibility/matrix.md` + `test/features/perfect-refactor-parity.feature` | 28 | 否 |
| 30 | current-latest-prompt-processing-burndown | 将 current-latest prompt-processing blockers 拆为已有 prompt fixture evidence、P1 snapshot evidence 和保留 P0 processor blockers | `src/compatibility/inventory.rs` + `scripts/release/current-latest-source-inventory.sh` + `target/release-gates/` + `tests/current_latest_prompt_processing_burndown.rs` + `docs/compatibility/matrix.md` + `test/features/perfect-refactor-parity.feature` | 29 | 否 |
| 31 | current-latest-cache-store-burndown | 将 current-latest cache-store blockers 拆为 cache/result-store fixture evidence、P1 helper snapshot evidence 和保留 eval deletion blocker | `src/compatibility/inventory.rs` + `scripts/release/current-latest-source-inventory.sh` + `target/release-gates/` + `tests/current_latest_cache_store_burndown.rs` + `docs/compatibility/matrix.md` + `test/features/perfect-refactor-parity.feature` | 30 | 否 |
| 32 | current-latest-local-prompt-processor-burndown | 将 current-latest JSON/Markdown/Jinja prompt processor blockers 转为 local fixture evidence，保留 JS/Python/executable script-bridge blockers | `src/compatibility/inventory.rs` + `scripts/release/current-latest-source-inventory.sh` + `target/release-gates/` + `tests/current_latest_local_prompt_processor_burndown.rs` + `docs/compatibility/matrix.md` + `test/features/perfect-refactor-parity.feature` | 31 | 否 |
| 33 | current-latest-eval-deletion-burndown | 实现并证明 SQLite eval 删除/断言级联语义，将剩余 cache-store evalDeletion blocker 转为 native fixture evidence | `src/results/sqlite.rs` + `src/compatibility/inventory.rs` + `scripts/release/current-latest-source-inventory.sh` + `target/release-gates/` + `tests/current_latest_eval_deletion_burndown.rs` + `docs/compatibility/matrix.md` + `test/features/perfect-refactor-parity.feature` | 32 | 否 |
| 34 | current-latest-eval-scheduler-rate-limit-burndown | 实现 deterministic scheduler rate-limit/adaptive/provider-wrapper evidence，将剩余 eval-runner scheduler blockers 转为 native fixture evidence | `src/eval/rate_limit.rs` + `src/compatibility/inventory.rs` + `scripts/release/current-latest-source-inventory.sh` + `target/release-gates/` + `tests/current_latest_eval_scheduler_rate_limit_burndown.rs` + `docs/compatibility/matrix.md` + `test/features/perfect-refactor-parity.feature` | 33 | 否 |
| 35 | current-latest-script-prompt-python-bridge-burndown | 实现 JS/Python/executable prompt processor 与 Python bridge deterministic subprocess evidence，将本地可证明 script blockers 转为 native fixture evidence 并保留 Ruby blockers | `src/script_bridge/` + `src/compatibility/inventory.rs` + `scripts/release/current-latest-source-inventory.sh` + `target/release-gates/` + `tests/current_latest_script_prompt_python_bridge_burndown.rs` + `docs/compatibility/matrix.md` + `test/features/perfect-refactor-parity.feature` | 34 | 否 |
| 36 | current-latest-ruby-bridge-burndown | 实现 Ruby bridge deterministic subprocess evidence，将最后 2 个本地 script-bridge blockers 转为 native fixture evidence | `src/script_bridge/` + `src/compatibility/inventory.rs` + `scripts/release/current-latest-source-inventory.sh` + `target/release-gates/` + `tests/current_latest_ruby_bridge_burndown.rs` + `docs/compatibility/matrix.md` + `test/features/perfect-refactor-parity.feature` | 35 | 否 |
| 37 | current-latest-unblock-packet-refresh | 刷新 perfect-refactor unblock packet，使其以 Phase 36 后 current-latest 23 个剩余 blockers 为权威决策源，而不是旧 frozen/source-accounting 口径 | `scripts/release/perfect-refactor-unblock-packet.sh` + `scripts/release/runtime-smoke.sh` + `target/release-gates/` + `tests/current_latest_unblock_packet.rs` + `docs/compatibility/matrix.md` + `test/features/perfect-refactor-parity.feature` | 36 | 否 |
| 38 | current-latest-0.121.14-target-refresh | 刷新 current-latest target lock 到 2026-06-03 观测到的 promptfoo 0.121.14，并修复 npm tag 与 GitHub latest release 同 ref 时的解析 | `scripts/release/current-latest-target-lock.sh` + `src/compatibility/inventory.rs` + `compatibility/inventory/current-latest-target.json` + `docs/compatibility/current-latest.lock.md` + `tests/current_latest_target_drift_refresh.rs` + `docs/compatibility/matrix.md` + `test/features/perfect-refactor-parity.feature` | 37 | 否 |
| 39 | current-latest-evaluator-runtime-classification | 将 promptfoo 0.121.14 新增的 `src/evaluator/runtime.ts` 从 unclassified 转为明确 eval-runner P0 blocker，消除 source/matrix unknown taxonomy blocker 且不伪造 native parity | `src/compatibility/inventory.rs` + `scripts/release/current-latest-source-inventory.sh` + `tests/current_latest_evaluator_runtime_classification.rs` + `docs/compatibility/matrix.md` + `test/features/perfect-refactor-parity.feature` | 38 | 否 |
| 40 | current-latest-evaluator-runtime-fixture-burndown | 将 `eval-runner:src-evaluator-runtime` 从 P0 blocker 推进为 native fixture evidence，减少一个本地 current-latest golden blocker | `src/compatibility/inventory.rs` + `scripts/release/current-latest-source-inventory.sh` + `tests/current_latest_evaluator_runtime_fixture.rs` + `docs/compatibility/matrix.md` + `test/features/perfect-refactor-parity.feature` | 39 | 否 |

> Phase 11-15 是 2026-05-30 审计后的补强链路，依据 `docs/audits/promptfoo-final-audit-index-2026-05-30.md`、PRD §Compatibility Matrix、ADR-007、ADR-008、ADR-009、ADR-010。它们不替换 Phase 1-10 的已完成履迹，而是把“promptfoo 完整重构”从初版可运行实现推进到 item-level parity、可执行 release gate 和可发布证据。
>
> Phase 16 是 2026-05-31 复审后的证据硬化链路，依据 PRD §Core Capabilities / §Success Metrics、ADR-004、ADR-007、ADR-009，以及 task-13.1 / task-15.2 §10 中登记的剩余风险。它不改写既有 Done 履迹，而是把已登记为 `later` 或合成证据的关键项推进到可执行、可审计的 stable release 证据。
>
> Phase 17 是 2026-05-31 当前态审计后的深度 parity proof 链路，依据 `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`、PRD §Compatibility Matrix / §Success Metrics、ADR-004、ADR-007、ADR-008、ADR-009。它承认 Phase 16 已让本地 S2V gate 变强，但继续补齐仍未证明的完整 source inventory、CLI surface、50+ 真实 golden corpus、长尾分类与发布安装证据。
>
> Phase 18 是 2026-05-31 Phase 17 复审后的 blocker burn-down 链路，依据 `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md` 中仍未满足“完美重构”的 2116 source inventory missing rows、37 个 P0 provider module blockers、current upstream 差异和 public publication credential blockers。它不把 blocker 改名为完成，而是把 silent omission、implementation blocker、rebaseline blocker、publication authority blocker 拆成独立可验证 task。

> Phase 19 是 2026-05-31 Phase 18 完成后的 native burndown 链路，依据 Phase 18 §9 artifact evidence 中仍保留的 111 个 generated P0 source accounting blockers、24 个 P0 provider module blockers 和 publication/current-upstream 边界。它优先修正已知分级错误（`src/app/**` viewer config 应按 Local Web viewer=P1 处理），再对剩余 core config/provider blockers 补 fixture 或 external authority gate。
>
> Phase 20 是 2026-05-31 Phase 19 smoke 后的 cross-ledger closure 链路，依据 Phase 19 §9 artifact evidence 中 `source-inventory-evidence.json` 仍有 44 个 P0 accounting blockers、而 `longtail-classification.json` 已证明 22 个 provider rows fixture-covered 且 15 个 provider rows 属 external authority。它不把 external/current/publication blocker 改名为完成，而是统一 source/provider/release claim 口径。

> Phase 21 是 2026-05-31 Phase 20 smoke 后的 upstream target disambiguation 链路，依据 task 18.3 / task 20.2 仍保留的 current-upstream blocker。它把 npm core package latest、GitHub repo HEAD、GitHub latest release tag 和 frozen baseline 拆成独立证据，避免把 non-core release drift 或 unreleased HEAD 漂移误读成 promptfoo core package rebaseline。
>
> Phase 22 是 2026-05-31 Phase 21 后的 unblock handoff 链路，依据 task 20.2 / 21.1 仍保留的 source、external-authority、publication-authority 和 current-upstream blockers。它不把外部授权、真实凭据、法律/品牌确认或公开发布证据伪造成完成，而是生成机器可审计的最小决策包，让 perfect-refactor claim 的剩余条件可交接、可验证、可阻塞。
>
> Phase 23 是 2026-06-01 恢复审计发现的 upstream evidence freshness 链路，依据 task 21.1 的 “GitHub latest release tag” 语义和 BLOCKED-task-22.1 的 resumed audit。它不解除 current-upstream blocker，而是让 latest release 观测不再依赖硬编码 tag，避免过期 release-channel evidence 影响 perfect-refactor blocker 判断。
>
> Phase 25 是 2026-06-02 Phase 24 smoke 后的 current-latest taxonomy burndown 链路，依据 `target/release-gates/current-latest-quality.json` 中 source-inventory/matrix/golden blockers。它不把 P0 fixture、external authority 或 publication blocker 改名为完成，而是先消除 318 个 unknown rows，让后续 native/bridge/external burndown 有可审计 owner 和 evidence kind。
>
> Phase 26 是 2026-06-02 Phase 25 后的 current-latest viewer config scope correction 链路，依据 Phase 25 §10 中仍保留的 current-latest P0 golden blockers、task 19.1 的 frozen-baseline viewer config reclassification precedent、PRD §Compatibility Matrix、ADR-009、ADR-011。它只消除 `src/app/**` duplicate config blockers，不降低 non-app config/runtime parity 要求。
>
> Phase 27 是 2026-06-02 Phase 26 后的 current-latest core config burndown 链路，依据 Phase 26 §10 中剩余 18 个 non-app config blockers、task 19.2 的 frozen-baseline config fixture/external/auxiliary decision precedent、PRD §Core Capabilities、ADR-009、ADR-011。它不把外部服务配置伪造成完成，只消除 generic blocker。
>
> Phase 28 是 2026-06-02 Phase 27 后的 current-latest provider fixture burndown 链路，依据 Phase 27 §10 中剩余 38 个 provider blockers、task 19.3 的 provider request/response fixture precedent、task 19.4 的 external authority gate、PRD §Provider P0、ADR-009、ADR-011。它只把已有 mock/recorded fixture 可证明的 provider rows 计为 fixture evidence，Codex/Agents/Assistant/Billing/ChatKit/Realtime/Claude Code auth 等仍保持 external-authority blocker。
>
> Phase 29 是 2026-06-02 Phase 28 后的 current-latest eval-runner burndown 链路，依据 Phase 28 §10 中剩余 18 个 eval-runner blockers、task 3.1 的 scheduler fixture、task 3.2 的 retry/backoff precedent、task 13.2 的 eval/output/cache parity、PRD §Eval runner、ADR-009、ADR-011。它只把已有 deterministic fixture 可证明的 eval/scheduler rows 计为 fixture evidence，optimizer/events/test synthesis 降为 P1 snapshot，adaptive/rate-limit/provider-wrapper rows 继续保持 P0 blocker。
>
> Phase 30 是 2026-06-02 Phase 29 后的 current-latest prompt-processing burndown 链路，依据 Phase 29 §10 中仍保留的 13 个 prompt-processing blockers、task 2.2 的 config/file prompt fixture、task 2.3 的 eval prompt rendering fixture、task 4.2 的 model-graded prompt evidence、task 9.1 的 script bridge boundary、PRD §Core Capabilities、ADR-009、ADR-011。它只把已有 deterministic fixture 可证明的 prompt index/string/text/utils rows 计为 fixture evidence，constants/grading/Ragas helper 降为 P1 snapshot，JSON/Markdown/Jinja/JS/Python/executable processors 继续保持 P0 blocker。
>
> Task 30.1 完成后，current-latest 证据记录 prompt-processing 行为为 4 fixture / 3 snapshot / 6 blocker，且 `perfect_refactor_claim_allowed=false`。live task §9 target `96e556507e4bbee5110d94286d500c4605ccc38b` 的总 blocker 数为 53；最终 Phase 30 smoke 使用 tracked target `1d09dfeb5f0766905409117f923dd5c4b0838d9f` 的 deterministic fixture fallback，总 blocker 数为 52。剩余 JSON/Markdown/Jinja/JS/Python/executable prompt processor 行仍需后续专用 fixture 与安全边界验证。
>
> Phase 31 是 2026-06-02 Phase 30 后的 current-latest cache-store burndown 链路，依据 Phase 30 §9 中仍保留的 9 个 cache-store blockers、task 3.2 的 cache/resume/retry fixture、task 5.1 的 JSONL/SQLite result-store schema、task 13.2 的 eval output/cache parity、PRD §Technical Approach、ADR-003、ADR-009、ADR-011。它只把已有 deterministic fixture 可证明的 cache/database/storage local rows 计为 fixture evidence，database testing/signal helper 降为 P1 snapshot，eval deletion 继续保持 P0 blocker。
>
> Task 31.1 完成后，current-latest 证据记录 cache-store 行为为 6 fixture / 2 snapshot / 1 blocker；tracked target `1d09dfeb5f0766905409117f923dd5c4b0838d9f` 的 `current-latest-golden-corpus.blocker_count=44`，分组为 `cache-store=1, config=7, eval-runner=7, prompt-processing=6, provider=16, script-bridge=7`，且 `perfect_refactor_claim_allowed=false`。§9 runtime-smoke 包含 50 个真实 upstream P0 corpus fixture 并保持 `real-upstream-corpus.summary.status=ready`；剩余 eval deletion、external authority、script bridge、processor、config、publication/current-target blockers 仍需后续 task 或外部决策。
>
> Phase 32 是 2026-06-02 Phase 31 后的 current-latest local prompt processor burndown 链路，依据 Phase 31 §9 中仍保留的 6 个 prompt-processing blockers、task 2.2 的 config loader、task 2.3 的 eval prompt rendering、task 9.1 的 script bridge boundary、PRD §Core Capabilities、ADR-009、ADR-011。它只把无需脚本执行权限的 JSON/Markdown/Jinja processor rows 计为 fixture evidence，JS/Python/executable processor rows 继续保持 P0 script-bridge blocker。
>
> Task 32.1 完成后，current-latest 证据记录 prompt-processing 行为为 7 fixture / 3 snapshot / 3 blocker；tracked target `1d09dfeb5f0766905409117f923dd5c4b0838d9f` 的 `current-latest-golden-corpus.blocker_count=41`，分组为 `cache-store=1, config=7, eval-runner=7, prompt-processing=3, provider=16, script-bridge=7`，且 `perfect_refactor_claim_allowed=false`。剩余 JS/Python/executable processor rows 仍需 script bridge subprocess fixture 证明，不能由本地 JSON/Markdown/Jinja parser fixture 间接视为 native。
>
> Phase 33 是 2026-06-02 Phase 32 后的 current-latest eval deletion burndown 链路，依据 Phase 32 §9 中仍保留的 1 个 cache-store blocker、task 5.1 的 SQLite result schema、task 13.2 的 eval output/cache parity、PRD §Technical Approach、ADR-003、ADR-009、ADR-011。它只实现本地 SQLite eval deletion lifecycle evidence，不改变 remote/cloud delete command policy，也不处理外部授权、provider、script bridge、eval-runner 或 publication blockers。
>
> Task 33.1 完成后，current-latest 证据记录 cache-store 行为为 7 fixture / 2 snapshot / 0 blocker；tracked target `1d09dfeb5f0766905409117f923dd5c4b0838d9f` 的 `current-latest-golden-corpus.blocker_count=40`，分组为 `config=7, eval-runner=7, prompt-processing=3, provider=16, script-bridge=7`，且 `perfect_refactor_claim_allowed=false`。剩余 blockers 均不属于 cache-store；需要后续 script bridge、eval-runner rate-limit、external authority、publication 或 current-target task/决策处理。
>
> Phase 34 是 2026-06-02 Phase 33 后的 current-latest eval scheduler rate-limit burndown 链路，依据 Phase 33 §9 中仍保留的 7 个 eval-runner blockers、task 3.1 的 scheduler runtime、task 3.2 的 retry/backoff、task 13.2 的 eval output/cache parity、task 29.1 的 blocker split、PRD §Eval runner、ADR-009、ADR-011。它只实现本地 deterministic rate-limit/adaptive/provider-wrapper scheduler contract，不调用真实 provider、账号、private SDK 或外部服务，也不解除 config/provider external authority、script bridge、publication 或 current-target blockers。
>
> Task 34.1 完成后，current-latest 证据记录 eval-runner 行为为 15 fixture / 3 snapshot / 0 blocker；tracked target `1d09dfeb5f0766905409117f923dd5c4b0838d9f` 的 `current-latest-golden-corpus.blocker_count=33`，分组为 `config=7, prompt-processing=3, provider=16, script-bridge=7`，且 `perfect_refactor_claim_allowed=false`。剩余 blockers 均不属于 eval-runner；需要后续 script bridge / prompt processor、external authority、publication 或 current-target task/决策处理。
>
> Phase 35 是 2026-06-02 Phase 34 后的 current-latest script prompt/Python bridge burndown 链路，依据 Phase 34 §9 中仍保留的 3 个 script-backed prompt processor blockers、7 个 script-bridge blockers、task 9.1 的 explicit authorization sandbox、task 32.1 的 blocker split、PRD §Core Capabilities、ADR-005、ADR-009、ADR-011。它只实现本地 deterministic Node/Python subprocess contract，不安装或声称 Ruby parity，也不解除 config/provider external authority、publication、current-target 或“无任何潜在 bug”不可证明承诺。
>
> Task 35.1 完成后，current-latest 证据记录 prompt-processing 行为为 10 fixture / 3 snapshot / 0 blocker，script-bridge 行为为 5 fixture / 2 blocker；tracked target `1d09dfeb5f0766905409117f923dd5c4b0838d9f` 的 `current-latest-golden-corpus.blocker_count=25`，分组为 `config=7, provider=16, script-bridge=2`，且 `perfect_refactor_claim_allowed=false`。剩余 script-bridge blockers 仅为 Ruby runtime rows；config/provider external-authority、publication、current-target 和“无任何潜在 bug”承诺仍需外部证据或正式 waiver。
>
> Phase 36 是 2026-06-03 Phase 35 后的 current-latest Ruby bridge burndown 链路，依据 Phase 35 §9 中仍保留的 2 个 Ruby script-bridge blockers、task 9.1 的 explicit authorization sandbox、PRD §Core Capabilities、ADR-005、ADR-009、ADR-011。它只实现本地 deterministic Ruby subprocess contract，不解除 config/provider external authority、publication、current-target 或“无任何潜在 bug”不可证明承诺。
>
> Task 36.1 完成后，current-latest 证据记录 script-bridge 行为为 7 fixture / 0 blocker；tracked target `1d09dfeb5f0766905409117f923dd5c4b0838d9f` 的 `current-latest-golden-corpus.blocker_count=23`，分组为 `config=7, provider=16`，且 `perfect_refactor_claim_allowed=false`。剩余 blockers 均为外部权限/服务/发布/当前目标/不可证明质量声明边界，不能由本地 Ruby subprocess fixture 间接解除。
>
> Phase 37 是 2026-06-03 Phase 36 后的 current-latest unblock packet refresh 链路，依据 Phase 36 §9 中剩余的 23 个 current-latest golden blockers、task 22.1 的 unblock packet gate、task 24.4 的 current-latest quality gate、PRD §Success Metrics、ADR-008、ADR-009、ADR-011。它只刷新用户/维护者决策包，不解除外部权限、发布、current-target 或“无任何潜在 bug”不可证明边界。
>
> Task 37.1 完成后，`perfect-refactor-unblock-packet.json` 使用 current-latest artifacts 作为权威来源，记录 `target_scope=current-latest`、23 个 current-latest golden blockers、30 个 required decision items，并保持 `perfect_refactor_claim_allowed=false` / `status=blocked`。剩余工作仍是 config/provider external authority、current-target、publication 和质量声明边界，需要真实外部证据或正式 waiver。
>
> Phase 38 是 2026-06-03 live upstream drift 后的 current-latest target refresh 链路，依据 PRD §Current Latest Rebaseline Addendum、ADR-007、ADR-009、ADR-011、task 24.1 和 task 37.1。实时观测显示 npm latest 已变为 `promptfoo@0.121.14` / `7a48c5fce614bee617efbb3b7fc93d404c75b628`，GitHub default branch HEAD 为 `4d22e57f5f9b4c7cdde494f00558d9afde8b4975`，GitHub latest release 为 `refs/tags/0.121.14` / `7a48c5fce614bee617efbb3b7fc93d404c75b628`。该 phase 只刷新 target lock 与同 ref 解析，不解除外部权限、发布或“无任何潜在 bug”不可证明边界。
>
> Task 38.1 完成后，tracked current-latest target lock 已刷新为 `promptfoo@0.121.14`，Rust 与 shell target-lock parser 均支持 npm tag 与 GitHub latest release 共享 `refs/tags/0.121.14` 的合法情况。runtime smoke 继续保持 fail-closed：current-latest golden corpus `blocker_count=25`、quality `blocker_count=6`、release candidate `publication_ready=credential-blocked`，因此 `perfect_refactor_claim_allowed=false` 仍是正确状态。
>
> Phase 39 针对 Phase 38 runtime-smoke 暴露的一个本地 taxonomy regression：`src/evaluator/runtime.ts` 在 `0.121.14` target 下变为 `unclassified:src-evaluator-runtime`。依据 Phase 25 / Phase 29，agent 可保守地将其归入 eval-runner P0 blocker，但必须保留 dedicated fixture 要求，不能把 unknown cleanup 等同于 eval runtime native parity。
>
> Task 39.1 完成后，current-latest source inventory 与 matrix 在 `0.121.14` target 下均为 `status=ready` 且 `unclassified_rows=[]`。`src/evaluator/runtime.ts` 被记录为 `eval-runner:src-evaluator-runtime`、P0 blocked、`evidence_kind=blocker`；quality gate 因此从 6 个 blockers 降到 4 个 blockers，但 `perfect_refactor_claim_allowed=false` 仍正确。
>
> Phase 40 继续消化唯一剩余的本地 current-latest golden blocker：`eval-runner:src-evaluator-runtime`。该 phase 只能把该单行推进为 native fixture evidence；config/provider external authority、current-target、publication authority、以及“无任何潜在 bug”的不可证明边界仍不在本地实现范围内。

---

## Decisions Log｜决策日志

> `/s2v-init` 阶段 9.1 会把每条决策转成一份 ADR（默认 Status=Accepted）。
> 至少 3 条；至少覆盖 S2V 8 类决策中的任 3 类。
> 完整 8 类决策见 S2V `full-standard.md` §16.1。
> **`类别`列取值约束**：从 full-standard.md §16.1「8 类决策类别（唯一权威）」表的 8 个字面值中选其一 —— `架构` / `依赖` / `数据持久化` / `协议接口` / `安全` / `测试工具链` / `部署发布` / `兼容性`（**逐字照抄、勿用同义词**；下游 `/s2v-init` 渲染 ADR `Category` + 做"8 类是否都覆盖"审计按字符串相等匹配，写法不一致会让已覆盖类别被误判为未覆盖）。

| ID (D1, D2...) | 类别 | 决策（一句话）| 选择 | 候选方案 | 拒绝候选的理由 |
|---|---|---|---|---|---|
| D1 | 架构 | 默认执行路径采用 Rust core，脚本运行时只作为兼容桥 | 模块化 Rust core + optional bridges | 纯 Rust 无 bridge / Node 主体重写 | 纯 Rust 会断已有 custom provider/assertion；Node 主体无法解决单二进制和供应链目标 |
| D2 | 依赖 | 核心依赖采用 Rust 生态稳定库，避免自研通用基础设施 | Tokio、clap、serde、reqwest、axum、sqlx/libSQL、tracing | 自研 async/runtime/HTTP/CLI / 继续复用 Node 包 | 自研基础设施会转移精力；复用 Node 包会保留 node_modules 和供应链问题 |
| D3 | 数据持久化 | 大型结果采用流式 JSONL 与 SQLite/libSQL 存储 | JSONL append + SQLite/libSQL query store | 单 JSON 文件 / 只存内存 / 专用服务端数据库 | 单 JSON 和内存方案无法支撑大型 eval 与 resume；服务端数据库破坏 local-first 分发 |
| D4 | 协议接口 | CLI exit code、stdout/stderr、JSON/JUnit/SARIF 输出 schema 作为稳定兼容协议 | 将 CLI 与输出 schema 纳入 P0/P1 snapshot 和 golden diff | 只保证人类可读输出 / 输出字段随实现漂移 | 现有用户依赖 CI、JUnit、SARIF 和脚本消费；字段漂移会直接破坏迁移 |
| D5 | 安全 | custom scripts 默认禁用，必须显式授权 | `--allow-scripts` 或配置开启，子进程隔离 env/stdio/timeout/redaction | 默认执行 / 完全移除脚本兼容 | 默认执行扩大任意代码执行和 CI secret 泄露面；完全移除会断现有生态 |
| D6 | 测试工具链 | 兼容性测试优先于覆盖率数字 | fixture golden diff + schema snapshot + Rust unit/integration | 只写 Rust 单元测试 / 手工对比 | 单元测试不能证明 promptfoo 行为兼容；手工对比不可审计、不可重复 |
| D7 | 兼容性 | upstream golden diff 是 1.0 stable release gate | P0 golden diff 不通过不得发布 stable | 把 diff 当非阻断报告 / 只在 nightly 跑 | 兼容是 1.0 的核心承诺；非阻断 diff 会让用户在 CI 中遇到不可预测迁移失败 |
| D8 | 部署发布 | 二进制是一等产物，npm wrapper 是兼容和分发补充 | GitHub Releases/Homebrew/Cargo/Docker/npm wrapper/GitHub Action | 只发布 npm / 只发布 Cargo | 只发 npm 不能解决 Node 依赖痛点；只发 Cargo 对非 Rust 用户和 CI 不友好 |
| D9 | 兼容性 | 1.0 兼容目标覆盖全部已文档化能力域，但按 P0/P1/P2 分级验收 | 全量登记 + 分级 release gate | 只登记已实现项 / 承诺全部 Rust native | 只登记已实现项会沉默遗漏；全部 Rust native 会把长尾 provider 范围拖垮 |
| D10 | 协议接口 | Node API wrapper 通过稳定 JSON-RPC/stdio 或 FFI 边界调用 Rust core | wrapper contract tests 固定 API、参数、错误和结果 schema | JS wrapper 复写业务逻辑 / 只暴露 CLI subprocess | 复写业务逻辑会产生 wrapper/core 漂移；只暴露 CLI 会破坏 programmatic usage 体验 |

---

## Success Metrics｜成功指标

**主要指标**（Primary，≥ 1 个，必须可测量）：
- **P0 兼容 release gate**：至少 50 个核心 fixtures 在 mock provider 下 upstream promptfoo 0.121.13 与 `promptfoo-rs` 输出一致或差异可解释；P0 未分类差异数为 0。
- **常见 eval 可迁移**：`promptfoo-rs eval -c promptfooconfig.yaml` 能运行覆盖 prompts、vars、tests、providers、assertions、cache、resume、retry 和 output 的常见配置。
- **兼容矩阵完整性**：promptfoo 0.121.13 已文档化 provider/assertion/redteam/plugin/CLI/output/config 能力 100% 登记，均有 P0/P1/P2、状态、验证方式和 owner。

**次要指标**（Secondary，≥ 2 个）：
- **Provider P0**：OpenAI-compatible、HTTP、Ollama、Anthropic 4 类 provider 可运行并有 request/response snapshot。
- **Output P0/P1**：JSON、JSONL、JUnit XML、CSV 至少 4 类输出可用于 CI；SARIF 对 scan 能力有 schema snapshot。
- **性能基线**：CLI 冷启动 < 300ms；1000 条 mock eval case 本地调度与 assertion 执行 < 5s；内存基线 < 100MB。
- **安全默认值**：未显式开启 `--allow-scripts` 时，所有 custom script fixture 均拒绝执行并产生可定位错误；日志 redaction fixture 通过。
- **文档可用性**：README、架构文档、兼容矩阵、贡献指南、GitHub Action 示例和 release gate 说明齐全。

**反指标**（Anti-metrics — 优化主指标时不能牺牲的，≥ 1 项）：
- 不能为了 Rust-native 比例牺牲 promptfoo-compatible 配置、CLI、输出和 script bridge 迁移路径。
- 不能为了冷启动指标删除必要的 redaction、cache integrity 或 compatibility diff 证据。
- 不能把未实现能力从矩阵中省略来制造“高兼容率”。
- 不能为了稳定 golden diff 对真实非确定性 LLM 输出做伪确定性断言；必须用 mock/recorded response 或明确 P1 snapshot。

---

## Open Questions｜开放问题

> ≥ 1 项。零 open question 通常是危险信号 — 说明思考还没到位。

- [ ] **完整能力清单提取方法**：Phase 1 需要确认从 upstream docs、CLI help、source registry、examples 和 release notes 中提取能力项的脚本/人工复核流程，避免矩阵漏项。
- [ ] **baseline artifact 校验细节**：npm artifact integrity、container digest、GitHub release tag 和 commit 的采集命令需要在 Phase 1 固化。
- [ ] **长尾 provider 分级规则**：P0/P1/P2 的 provider/assertion/redteam plugin 分级需要按使用频率、文档稳定性、实现复杂度和安全风险形成公开规则。
- [ ] **promptfoo cloud/share 文案边界**：README、CLI 错误和兼容矩阵需要法律/社区视角复核，避免品牌混淆或暗示云服务兼容。
- [ ] **Web viewer 技术栈最终选择**：Vite/React 与 Next.js 的取舍需以本地静态分发、结果加载体积和维护成本决定。
- [ ] **Node API wrapper API 面**：需要确认 1.0 支持哪些 promptfoo programmatic usage，哪些只通过 CLI subprocess 暴露。

---

## Technical Risks｜技术风险

> ≥ 3 项。

| # | 风险 | 概率 | 影响 | 缓解策略 |
|---|---|---|---|---|
| R1 | upstream 行为细节未文档化，导致 fixture 之外的配置迁移失败 | 高 | 高 | 以 docs、examples、source registry 和用户 fixtures 扩展矩阵；差异必须分类并进入 release gate |
| R2 | provider 长尾过多，全部 native 实现会拖垮 1.0 | 高 | 高 | 全量登记但分 P0/P1/P2；长尾允许 `bridge`、`unsupported` 或 `later`，但不能沉默遗漏 |
| R3 | CLI 输出字段、exit code 或 JUnit/SARIF schema 漂移破坏 CI | 中 | 高 | 把 stdout/stderr、exit code、JSON/JUnit/SARIF schema 纳入 snapshot 和 golden diff |
| R4 | script bridge 引入任意代码执行、env 泄露或 CI secret 泄露 | 中 | 高 | 默认禁用、显式授权、子进程隔离、env allowlist、timeout、redaction、审计日志 |
| R5 | redteam、MCP、code-scan/model-audit 范围过宽，影响核心 eval 交付 | 高 | 中 | 拆成 `redteam-core` 与 `mcp-scan-audit` phase；每个 phase 定义最小兼容闭环和 later 项 |
| R6 | Windows 路径、换行、shell quoting 与 Unix 行为不一致 | 中 | 中 | Windows x64 CI fixture 覆盖 `.env`、file prompts、script args、path normalization 和 line endings |
| R7 | Web viewer 直接耦合 runner 内部结构，导致结果 schema 漂移 | 中 | 中 | viewer 只读取稳定 result schema；schema 变更必须通过 output-writers snapshot |
| R8 | Node API wrapper 与 Rust core 行为漂移 | 中 | 高 | wrapper 不复写业务逻辑；建立 contract tests，对参数、错误、输出 schema 和 exit behavior 做 drift 检测 |
| R9 | promptfoo cloud/share 相关 API 边界不清，引发用户误解或品牌风险 | 中 | 中 | 1.0 明确不提供 SaaS；矩阵标 `unsupported/later`；README、CLI 错误和 license notice 明确 reimplementation 边界 |
| R10 | model-graded assertions 因 LLM 非确定性导致 golden diff 不稳定 | 高 | 中 | 使用 mock grader 或 recorded response；比较 prompt construction、threshold、score parsing 和 metadata schema，不比较真实模型原始文本 |
| R11 | cache key 与 resume cursor 与 upstream 不一致，导致重复请求或漏跑 | 中 | 高 | 为 cache key、resume 文件、partial failure 和 retry-errors 建 P0 fixtures；损坏恢复行为单独测试 |
| R12 | license/copyright notice 处理不完整 | 低 | 高 | 保留 promptfoo 原 MIT license/copyright notice；新增文件明确本项目 license；release checklist 审查 |

---

## Next Steps｜后续步骤

1. **审本 PRD 内容**（重点：阶段表 / 决策日志 / 风险 / 兼容矩阵 / baseline 冻结策略）
2. **后续路径**：当前项目尚未初始化 S2V；审完 PRD 后可接 `/s2v-init` 生成 adapter、phase spec、task spec、ADR 和 BDD feature
3. **Phase 1 先行**：先冻结 `promptfoo 0.121.13 + 4860e99` 的 tag、commit、npm artifact、container artifact，再生成完整项级 compatibility matrix

> ⚠️ task spec 实施完后留在原地不归档（SDD 单一事实源核心要求）。
