# 获取帮助 · Support

感谢使用 promptfoo-rs。本文件说明遇到不同问题时该走哪条通道。

> promptfoo-rs 是独立的 Rust 重实现，**不隶属于** promptfoo upstream 项目。upstream 的支持渠道不负责本项目的问题。

## 先看文档

| 我想… | 去 |
|---|---|
| 5 分钟跑起来 | [docs/QUICKSTART.md](docs/QUICKSTART.md) |
| 了解仓库结构 / 边界 | [docs/PROJECT-OVERVIEW.md](docs/PROJECT-OVERVIEW.md) |
| 看兼容矩阵 / 分级 | [docs/compatibility/matrix.md](docs/compatibility/matrix.md) |
| 了解发布 / 安装渠道状态 | [docs/release.md](docs/release.md)、[README.md](README.md) |
| 参与贡献 | [CONTRIBUTING.md](CONTRIBUTING.md) |

## 按问题类型选通道

- **使用问题**（安装、配置、CLI 用法）：先查 QUICKSTART 与 README；仍无解再开 issue。
- **缺陷（bug）**：用 **Bug Report** 模板，附最小复现与脱敏日志。
- **与 promptfoo@0.121.15 的兼容差异**：用 **Compatibility Report** 模板。
- **功能建议**：用 **Feature Request** 模板，注意 [ADR-012](docs/decisions/adr-012-product-independence-baseline-freeze.md) 的冻结基线范围。
- **安全漏洞**：**不要**开公开 issue → 走 [GitHub Security Advisory](https://github.com/tajiaoyezi/promptfoo-rs/security/advisories/new)，详见 [SECURITY.md](SECURITY.md)。

## 响应预期

这是一个小型维护者项目（当前协作层级 `solo`），尽力而为：

| 事项 | 目标 |
|---|---|
| Issue 初次回应 | 数日内（best-effort） |
| 安全漏洞确认 | 见 [SECURITY.md](SECURITY.md)（72 小时致谢目标） |

## 不在支持范围内

- live provider 的真实结果 parity（用户自带 API key，自负风险）。
- 把兼容基线升级到 promptfoo 0.121.15 之后的版本（不追 upstream drift，见 ADR-012）。
- cargo / npm registry / Docker / Homebrew / GitHub Action 等 v1 已延期的发布渠道（当前仅 GitHub Releases 为授权渠道）。
