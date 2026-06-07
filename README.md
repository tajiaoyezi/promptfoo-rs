# promptfoo-rs

<div align="right">

**中文** | [English](README.en.md)

</div>

`promptfoo-rs` 是一个 Rust-first 的 promptfoo 兼容 CLI、库和本地查看器实现。项目目标是把 promptfoo 的本地 eval、配置解析、结果输出、兼容性验证和 release gate 做成可审计、可测试、默认本地优先的 Rust 工具链。

**独立重实现**：promptfoo-rs 不是 [promptfoo](https://github.com/promptfoo/promptfoo) 官方项目，也不代表 upstream 背书；`promptfoo` 名称仅用于描述兼容目标与配置格式。

当前状态：**v0.1.2** 在 [GitHub Releases](https://github.com/tajiaoyezi/promptfoo-rs/releases) 提供 Linux（x64 与 arm64）/ Windows / macOS（arm64 与 x64）二进制（v1 唯一授权公开渠道）。本地 S2V verification 通过。

**产品战略（2026-06-07）**：promptfoo-rs 是对 promptfoo 的**一次性** Rust 重实现，兼容基线已冻结在 **`promptfoo@0.121.15`**（Phase 48 观测包，见 ADR-012）。项目是**独立产品线**，**不**跟踪 promptfoo 新版本或 GitHub HEAD。尚不宣称在冻结基线上完成全部兼容或 perfect-refactor。

[快速上手](docs/QUICKSTART.md) | [项目概览](docs/PROJECT-OVERVIEW.md) | [架构](docs/architecture.md) | [发布边界](docs/release.md) | [贡献指南](CONTRIBUTING.md)

## 项目定位

promptfoo-rs 关注这些场景：

- 在 CI 或本地运行 `promptfoo eval -c promptfooconfig.yaml`，输出稳定 JSON/JSONL/JUnit/SARIF/CSV/HTML 结果。
- 用 Rust 实现 eval runner、调度、缓存、结果存储、断点恢复、provider/assertion 契约和安全默认值。
- 用 compatibility matrix、golden corpus、real upstream corpus 和 S2V gate 记录与 upstream promptfoo 的兼容边界。
- 默认不上传、不执行未授权脚本、不写远程云端状态；需要脚本 bridge 时必须显式授权。
- 为 Node API wrapper、本地 web viewer、Docker/GitHub Actions release shape 提供可验证边界。

## 当前状态

本项目区分三个概念：

| 状态 | 当前结论 |
|---|---|
| 本地构建和测试 | 可用。`install`、`lint`、`typecheck`、`unit-test`、`integration`、`e2e`、`coverage`、`build`、`runtime-smoke` 已通过。 |
| 已声明范围内的正常使用 | 可用。CLI、核心 eval、输出、viewer 数据契约、Node wrapper smoke 和 compatibility gates 均有测试覆盖。 |
| 冻结基线（`promptfoo@0.121.15`）完整兼容 | 尚未声明。冻结基线 gate 仍保留 blocker，`perfect_refactor_claim_allowed=false`。不跟踪 promptfoo 后续版本。 |
| 公开稳定发布（v1 范围） | **GitHub Releases**（当前 **v0.1.2**：Linux x64/arm64、Windows、macOS）。Cargo、npm、Docker、Homebrew、GitHub Action 在 v1 正式延期；聚合 `publication_ready` 仍为 false。 |

允许的质量表述是：在已声明 gate 下，没有已知 release-blocking defect。项目不会承诺“无任何潜在 bug”或“完整 live-provider parity”，除非对应 gate 和外部证据闭合。

## 快速开始

### 前置条件

- Rust stable toolchain
- Node.js 20+、Corepack、pnpm，用于 `viewer/` 和 `npm/` 包
- Windows 上运行 S2V helper 时使用 Git for Windows Bash：`C:\Program Files\Git\bin\bash.exe`

### 构建 CLI

```bash
cargo build --workspace --release
target/release/promptfoo --help
```

Windows PowerShell:

```powershell
cargo build --workspace --release
.\target\release\promptfoo.exe --help
```

构建后首选本地命令是 `promptfoo --help`。`promptfoo-rs` 仍作为显式 Rust 别名保留；npm wrapper 的本地 bin shim 还支持 `pf`。

### 运行一个最小 eval

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

运行：

```bash
target/release/promptfoo eval -c promptfooconfig.yaml --output results.json
```

如果需要打开本地结果目录：

```bash
target/release/promptfoo view .
```

更多示例见 [docs/QUICKSTART.md](docs/QUICKSTART.md)。

### 从 GitHub Releases 安装

v1 推荐安装方式：在 [Releases](https://github.com/tajiaoyezi/promptfoo-rs/releases) 下载对应平台资产并校验 `SHA256SUMS`。

Linux x86_64：

```bash
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.2/promptfoo-rs-0.1.2-x86_64-unknown-linux-gnu.tar.gz
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.2/SHA256SUMS
sha256sum -c SHA256SUMS --ignore-missing
tar -xzf promptfoo-rs-0.1.2-x86_64-unknown-linux-gnu.tar.gz
./promptfoo --help
```

Linux arm64 (aarch64)：

```bash
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.2/promptfoo-rs-0.1.2-aarch64-unknown-linux-gnu.tar.gz
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.2/SHA256SUMS
sha256sum -c SHA256SUMS --ignore-missing
tar -xzf promptfoo-rs-0.1.2-aarch64-unknown-linux-gnu.tar.gz
./promptfoo --help
```


macOS Apple Silicon (arm64)：

```bash
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.2/promptfoo-rs-0.1.2-aarch64-apple-darwin.tar.gz
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.2/SHA256SUMS
shasum -a 256 -c SHA256SUMS --ignore-missing
tar -xzf promptfoo-rs-0.1.2-aarch64-apple-darwin.tar.gz
./promptfoo --help
```

macOS Intel (x64)：下载 `promptfoo-rs-0.1.2-x86_64-apple-darwin.tar.gz`，校验 `SHA256SUMS` 后解压并将目录加入 `PATH`。

Windows x86_64：下载 `promptfoo-rs-0.1.2-x86_64-pc-windows-msvc.zip`，解压后将目录加入 `PATH`，运行 `promptfoo.exe --help`。

### 发布与安装渠道状态

v1 已发布：**GitHub Releases**（见上）。其余渠道（`cargo install`、npm registry、Docker registry、Homebrew、GitHub Marketplace Action）在 v1 正式延期；仓库对这些渠道只声明 `local build/package smoke` 与 gate 文档，真实 `public registry publication` 仍 blocked。详见 [docs/release.md](docs/release.md) 与 [docs/release-notes/v0.1.2.md](docs/release-notes/v0.1.2.md)。

## CLI 能力

当前 CLI 暴露以下命令族：

- 首选本地入口：`promptfoo --help`、`promptfoo eval -c promptfooconfig.yaml`、`promptfoo view .`。
- 兼容别名：Rust release 同时生成 `promptfoo-rs`；npm wrapper 本地 bin shim 暴露 `promptfoo`、`promptfoo-rs` 和 `pf`。
- `eval`：从 promptfoo config 运行本地 eval。
- `view`：读取本地结果目录，输出 viewer 数据契约。
- `cache`：管理本地 eval cache。
- `redteam`：运行 redteam init/generate/eval/run/report 等本地流程。
- `mcp`：输出 MCP compatibility 工具信息。
- `code-scans`、`scan-model`、`model-audit`：输出 scan/audit 数据契约。
- `import`、`export`：导入/导出 promptfoo artifacts。
- `share`、`auth`、`list`、`logs`、`delete` 等 cloud 相关命令：保留兼容入口，但默认 fail-closed，不上传、不操作远端。

## 兼容性与边界

权威兼容性文件：

- [docs/compatibility/matrix.md](docs/compatibility/matrix.md)：P0/P1/P2 兼容矩阵。
- [docs/compatibility/baseline.lock.md](docs/compatibility/baseline.lock.md)：冻结 baseline。
- [docs/compatibility/current-latest.lock.md](docs/compatibility/current-latest.lock.md)：当前 latest 目标锁。
- [docs/release.md](docs/release.md)：release gate、publication authority 和 perfect-refactor claim contract。

兼容性策略：

- P0：需要 golden diff 或 fixture 证据。
- P1：需要 snapshot 或协议证据。
- P2：必须登记 unsupported/later/bridge-backed reason。
- 未分类、缺证据或真实外部权限缺口不会被隐藏，也不会自动降级。

## 开发与验证

常用本地命令：

```bash
cargo check --workspace
cargo test --workspace
cargo build --workspace
```

完整 S2V 验证：

```bash
source docs/s2v/scripts/lib/preflight.sh
source docs/s2v/scripts/lib/verify.sh
s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"
```

Windows:

```powershell
& 'C:\Program Files\Git\bin\bash.exe' -lc 'source docs/s2v/scripts/lib/preflight.sh; source docs/s2v/scripts/lib/verify.sh; s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"'
```

## 文档导航

| 我想了解 | 文档 |
|---|---|
| 5 分钟跑起来 | [docs/QUICKSTART.md](docs/QUICKSTART.md) |
| 仓库结构和架构 | [docs/PROJECT-OVERVIEW.md](docs/PROJECT-OVERVIEW.md)、[docs/architecture.md](docs/architecture.md) |
| 兼容矩阵 | [docs/compatibility/matrix.md](docs/compatibility/matrix.md) |
| 发布规则 | [docs/release.md](docs/release.md) |
| S2V 项目入口 | [AGENTS.md](AGENTS.md)、[docs/s2v-adapter.md](docs/s2v-adapter.md) |
| PRD 和规格 | [docs/prds/promptfoo-rs.prd.md](docs/prds/promptfoo-rs.prd.md)、[docs/specs/](docs/specs/) |
| 架构决策 | [docs/decisions/](docs/decisions/) |
| 贡献 | [CONTRIBUTING.md](CONTRIBUTING.md) |
| 安全报告 | [SECURITY.md](SECURITY.md) |

## 贡献

欢迎贡献代码、文档、兼容性 fixture、bug 报告和审计意见。行为变更必须遵守 S2V：先读 `AGENTS.md`、`docs/s2v-adapter.md` 和相关 task spec，再按 RED -> GREEN -> verification -> completion notes 的顺序推进。

详见 [CONTRIBUTING.md](CONTRIBUTING.md)。

## 许可证

MIT License。详见 [LICENSE](LICENSE) 和 [NOTICE](NOTICE)。

promptfoo-rs 是独立的 Rust reimplementation，不隶属于 promptfoo upstream 项目，也不暗示官方背书。`promptfoo` 名称用于描述兼容目标和配置格式。
