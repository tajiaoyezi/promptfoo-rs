<div align="right">

**中文** · [English](README.en.md)

</div>

<div align="center">

# promptfoo-rs

**Rust-first 的 [promptfoo](https://github.com/promptfoo/promptfoo) 兼容 CLI · 库 · 本地查看器**

*本地优先、可审计、对兼容边界与发布主张「诚实到 gate 级」的 LLM eval 工具链。*

[![license](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE)
[![release](https://img.shields.io/github/v/release/tajiaoyezi/promptfoo-rs?style=flat-square&color=brightgreen)](https://github.com/tajiaoyezi/promptfoo-rs/releases)
[![verify](https://github.com/tajiaoyezi/promptfoo-rs/actions/workflows/verify.yml/badge.svg?branch=master)](https://github.com/tajiaoyezi/promptfoo-rs/actions/workflows/verify.yml)
[![rust](https://img.shields.io/badge/rust-stable-orange?style=flat-square&logo=rust)](Cargo.toml)
![platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey?style=flat-square)
![compat](https://img.shields.io/badge/compat%20baseline-promptfoo%400.121.15%20frozen-blueviolet?style=flat-square)
[![last commit](https://img.shields.io/github/last-commit/tajiaoyezi/promptfoo-rs?style=flat-square)](https://github.com/tajiaoyezi/promptfoo-rs/commits/master)

<sub>🦀 独立 Rust 重实现 · 非 promptfoo 官方项目 · 不代表 upstream 背书</sub>

[⚡ 快速上手](docs/QUICKSTART.md) · [📦 项目概览](docs/PROJECT-OVERVIEW.md) · [🧩 兼容矩阵](docs/compatibility/matrix.md) · [🚦 发布边界](docs/release.md) · [🤝 贡献](CONTRIBUTING.md) · [❓ 获取帮助](SUPPORT.md)

</div>

---

`promptfoo-rs` 用 Rust 重写了 promptfoo 的本地 eval 闭环与兼容边界：promptfoo 兼容的 CLI 与配置解析 → 确定性 eval 引擎（用例调度 / prompt 渲染 / 断言 / 缓存 / resume / 重试）→ 稳定的 CI 输出（JSON / JSONL / JUnit / CSV / HTML）。provider 客户端、更丰富的断言与输出作为 **P0/P1/P2 兼容证据**（matrix + golden corpus + S2V gate）记录——可审计、可测试，而不是营销话术。它默认本地优先：不上传、不执行未授权脚本、不写远端。

> 当前 **v0.1.3** · 仅 GitHub Releases 发布 · 兼容基线冻结在 `promptfoo@0.121.15`（ADR-012）· 本地 S2V verification 通过 🚧

## 🚦 当前状态

本项目刻意把三件事分开说清楚，且只在 gate 与证据支持时才下结论：

| 维度 | 结论 |
|---|---|
| ✅ 本地构建与测试 | 可用。`install · lint · typecheck · unit-test · integration · e2e · coverage · build · runtime-smoke` 全绿。 |
| ✅ 已声明范围内的使用 | 可用。CLI、核心 eval、输出、viewer 数据契约、Node wrapper smoke、compatibility gates 均有测试覆盖。 |
| 🚧 冻结基线完整兼容 | **尚未声明**。`promptfoo@0.121.15` gate 仍保留 blocker，`perfect_refactor_claim_allowed=false`；冻结之后的 upstream 版本不在范围内。 |
| 🚧 公开稳定发布（v1） | **仅 GitHub Releases**（当前 v0.1.3）。Cargo / npm / Docker / Homebrew / GitHub Action 在 v1 正式延期，聚合 `publication_ready=false`。 |

**允许的质量表述**：

> 在已声明 gate 下，没有已知 release-blocking defect。

诚实边界（不会被隐藏，也不会自动降级）：

- 不承诺「无任何潜在 bug」或「完整 live-provider parity」，除非对应 gate 与外部证据闭合。
- live provider 用你自己的 API key，自负风险；冻结基线上仍有被 waive 的兼容项。
- 不跟踪 promptfoo 后续版本 / GitHub HEAD——这是**独立产品线**（见 ADR-012），不是 upstream 订阅。

## ✨ 功能

- 🧪 **本地 eval 引擎** — 从 `promptfooconfig.yaml` 调度用例：prompt 变量渲染 + 确定性断言 + 缓存 / 断点 resume / 重试 / 并发，并 emit 结果记录。默认路径是确定性 render-and-assert，**不代发 live 模型请求**。
- ✅ **Assertions** — eval 配置路径支持 `equals · contains · regex`；断言引擎另实现 `json · schema` 等（以 fixture / matrix 证据覆盖，暂未全部接入 eval 配置）。
- 🔌 **Provider 兼容面** — OpenAI 兼容 / HTTP / Ollama / Anthropic 的请求规范化与客户端，记为 P0 matrix 证据；live 调用用你自己的 key，按 snapshot / 契约验证。
- 🧩 **脚本 bridge** — JS/TS（同一 JavaScript 运行时）· Python · Shell · Ruby 自定义脚本，sandbox 默认 default-deny。
- 📤 **CI 输出契约** — JSON / JSONL / JUnit / CSV / HTML 稳定契约；SARIF findings 由 `scan` 命令产出。
- 🖥️ **本地 viewer** — `promptfoo view .` 读取本地结果库（`results.jsonl` / SQLite）并输出 viewer 数据契约（本地数据，非托管 SaaS）。
- 🛡️ **Redteam / scan** — `redteam`、`code-scans`、`scan-model`、`model-audit` 等本地流程。
- 🧩 **兼容证据** — P0/P1/P2 矩阵 + golden corpus + S2V gate，记录原生 / bridge / 阻塞边界。
- 🔒 **安全默认 & 可验证边界** — 默认不上传、不写远端；`share`/`auth`/cloud fail-closed；脚本执行需显式授权；Node API wrapper / Docker shape / GitHub Actions release 均为可验证边界。

## 📦 安装

v1 推荐：从 [GitHub Releases](https://github.com/tajiaoyezi/promptfoo-rs/releases) 下载对应平台资产并校验 `SHA256SUMS`。安装后首选命令 `promptfoo`，显式别名 `promptfoo-rs`（npm wrapper 的本地 shim 还支持 `pf`）。

<details open>
<summary>🐧 <strong>Linux x86_64</strong></summary>

```bash
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.3/promptfoo-rs-0.1.3-x86_64-unknown-linux-gnu.tar.gz
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.3/SHA256SUMS
sha256sum -c SHA256SUMS --ignore-missing
tar -xzf promptfoo-rs-0.1.3-x86_64-unknown-linux-gnu.tar.gz
./promptfoo --help
```

</details>

<details>
<summary>🐧 Linux arm64 (aarch64)</summary>

```bash
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.3/promptfoo-rs-0.1.3-aarch64-unknown-linux-gnu.tar.gz
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.3/SHA256SUMS
sha256sum -c SHA256SUMS --ignore-missing
tar -xzf promptfoo-rs-0.1.3-aarch64-unknown-linux-gnu.tar.gz
./promptfoo --help
```

</details>

<details>
<summary>🍎 macOS（Apple Silicon arm64 / Intel x64）</summary>

```bash
# Apple Silicon (arm64)
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.3/promptfoo-rs-0.1.3-aarch64-apple-darwin.tar.gz
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.3/SHA256SUMS
shasum -a 256 -c SHA256SUMS --ignore-missing
tar -xzf promptfoo-rs-0.1.3-aarch64-apple-darwin.tar.gz
./promptfoo --help
```

Intel (x64)：改下载 `promptfoo-rs-0.1.3-x86_64-apple-darwin.tar.gz`，校验后解压并加入 `PATH`。

</details>

<details>
<summary>🪟 Windows x86_64</summary>

下载 `promptfoo-rs-0.1.3-x86_64-pc-windows-msvc.zip`，解压后将目录加入 `PATH`，运行 `promptfoo.exe --help`。

</details>

> 其余渠道（`cargo install` / npm registry / Docker / Homebrew / GitHub Marketplace Action）在 v1 正式延期，仓库对它们只声明 `local build/package smoke` 与 gate 文档；真实 `public registry publication` 仍 blocked。详见 [docs/release.md](docs/release.md) 与 [docs/release-notes/v0.1.3.md](docs/release-notes/v0.1.3.md)。

## 🚀 快速开始

创建 `promptfooconfig.yaml`：

```yaml
providers:
  - id: echo
prompts:
  - "Hello {{name}}"
tests:
  - vars:
      name: Ada
    assert:
      - type: contains
        value: Ada
```

运行并查看结果：

```bash
promptfoo eval -c promptfooconfig.yaml --output results.jsonl
promptfoo view .
```

从源码构建（开发者）：

```bash
cargo build --workspace --release
target/release/promptfoo --help          # Windows: .\target\release\promptfoo.exe --help
```

更多示例见 [docs/QUICKSTART.md](docs/QUICKSTART.md)。

## 🧩 兼容性与边界

promptfoo-rs 追求的是**功能边界清晰可证**，而不是对 upstream 的逐字节 parity。权威文件：

- [docs/compatibility/matrix.md](docs/compatibility/matrix.md) — P0/P1/P2 兼容矩阵
- [docs/compatibility/current-latest.lock.md](docs/compatibility/current-latest.lock.md) — 冻结产品基线锁（`promptfoo@0.121.15`，ADR-012；非 live 订阅）
- [docs/compatibility/baseline.lock.md](docs/compatibility/baseline.lock.md) — Phase 1 历史 harness 基线（`0.121.13`）
- [docs/release.md](docs/release.md) — release gate / publication authority / perfect-refactor claim contract

分级规则：**P0** 需 golden-diff 或 fixture 证据；**P1** 需 snapshot 或协议证据；**P2** 必须登记 unsupported / later / bridge 理由。未分类、缺证据或真实外部权限缺口都会作为 blocker 保留，不隐藏、不自动降级。

明确**不在范围**（by design）：跟踪 `0.121.15` 之后的 upstream 版本 / GitHub HEAD；对外宣称 perfect-refactor 或完整 upstream parity；live provider 的真实结果 parity。

## 🔒 安全与隐私

- 本地优先：默认不上传、不回传遥测、不做版本检查或自动更新；`share`/`auth`/cloud 命令默认 fail-closed。
- 网络请求只在你显式配置 provider 时发生，使用你自己的 API key，受该 provider 条款约束。
- 详见 [PRIVACY.md](PRIVACY.md)（隐私与遥测）与 [SECURITY.md](SECURITY.md)（安全策略与漏洞报告）。

## 🛠️ 开发与验证

```bash
cargo check --workspace
cargo test  --workspace
cargo build --workspace
```

完整 S2V 验证（Windows 用 Git for Windows Bash：`C:\Program Files\Git\bin\bash.exe`）：

```bash
source docs/s2v/scripts/lib/preflight.sh
source docs/s2v/scripts/lib/verify.sh
s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"
```

## 📚 文档导航

| 我想了解 | 文档 |
|---|---|
| ⚡ 5 分钟跑起来 | [docs/QUICKSTART.md](docs/QUICKSTART.md) |
| 🏗️ 仓库结构与架构 | [docs/PROJECT-OVERVIEW.md](docs/PROJECT-OVERVIEW.md) · [docs/architecture.md](docs/architecture.md) |
| 🧩 兼容矩阵 | [docs/compatibility/matrix.md](docs/compatibility/matrix.md) |
| 🚦 发布规则 | [docs/release.md](docs/release.md) |
| 📐 架构决策 (ADR) | [docs/decisions/](docs/decisions/) |
| 📋 PRD 与规格 | [docs/prds/promptfoo-rs.prd.md](docs/prds/promptfoo-rs.prd.md) · [docs/specs/](docs/specs/) |
| 🧭 S2V 项目入口 | [AGENTS.md](AGENTS.md) · [docs/s2v-adapter.md](docs/s2v-adapter.md) |
| 🤝 贡献指南 | [CONTRIBUTING.md](CONTRIBUTING.md) |
| ❓ 获取帮助 / 提问 | [SUPPORT.md](SUPPORT.md) |
| 🔐 隐私 / 安全 | [PRIVACY.md](PRIVACY.md) · [SECURITY.md](SECURITY.md) |
| 🏛️ 行为准则 | [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) |

## 🤝 贡献

欢迎贡献代码、文档、兼容性 fixture、bug 报告与审计意见。行为变更遵守 S2V：先读 `AGENTS.md`、`docs/s2v-adapter.md` 和相关 task spec，再按 `RED → GREEN → verification → completion notes` 推进。

提交入口：

- 🐛 [Bug Report](.github/ISSUE_TEMPLATE/bug_report.yml) · 🧩 [Compatibility Report](.github/ISSUE_TEMPLATE/compatibility_report.yml)（与冻结基线的差异）· ✨ [Feature Request](.github/ISSUE_TEMPLATE/feature_request.yml)
- 🔀 Pull Request 遵循 [PR 模板](.github/PULL_REQUEST_TEMPLATE.md) 的 S2V 检查项
- ❓ 使用问题先看 [SUPPORT.md](SUPPORT.md)；🔐 安全漏洞走 [Security Advisory](https://github.com/tajiaoyezi/promptfoo-rs/security/advisories/new)，不要开公开 issue

✨ **MIT · 无 CLA**，提交无需签署额外协议。详见 [CONTRIBUTING.md](CONTRIBUTING.md)。

## 📄 许可证

[MIT License](LICENSE) · 详见 [LICENSE](LICENSE) 与 [NOTICE](NOTICE)。

promptfoo-rs 是独立的 Rust reimplementation，不隶属于 promptfoo upstream 项目，也不暗示官方背书；`promptfoo` 名称仅用于描述兼容目标与配置格式。

<div align="center">
<br />
<sub>用 ❤️ 与 🦀 打造 · 本地优先 · 诚实到 gate 级</sub>
</div>
