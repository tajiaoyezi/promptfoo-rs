# 隐私与遥测声明 · Privacy & Telemetry

promptfoo-rs 是 **local-first** 的命令行工具。本声明说明它收集什么、发送什么。

> 适用对象：promptfoo-rs（独立 Rust 重实现）。它**不隶属于** promptfoo upstream；upstream 产品可能有自己的遥测与隐私政策，与本项目无关。

## 简短结论

**默认情况下，promptfoo-rs 不收集、不上传、不回传任何遥测、使用统计或个人数据。** 没有 analytics SDK，没有 phone-home，没有静默的自动更新检查。

## 数据去向

- **本地优先**：配置解析、eval 执行、缓存、结果存储、viewer 数据都在你的机器本地完成与落盘。
- **网络请求仅在你显式配置时发生**：
  - 当你配置某个 provider 并运行 eval 时，prompt / 变量等内容会按你的配置发送到**你选择的** provider API（如 OpenAI、Anthropic、自建 HTTP 等），使用**你自己的** API key，受该 provider 的条款约束。promptfoo-rs 只是转发者，不在中间留存或转送到第三方。
  - `share` / `auth` / cloud 相关命令默认 **fail-closed**，不上传、不写远端状态，除非被显式授权实现。
- **不自动联网**：不做遥测上报、不做匿名统计、不做版本检查 / 自动更新。

## 凭据与密钥

- provider API key、token、账号凭据等通过你的环境 / 配置提供，**不会**被写入 release 证据或诊断产物。
- 报告、artifact 与 release evidence 中的密钥 material 会被脱敏。
- 请不要把真实 secret 提交到仓库，或附加到 issue / 日志。

## 你能控制的

- 不配置任何 provider → 不产生任何外发请求。
- 使用 `echo` 等本地 provider → 完全离线。
- 自定义脚本 bridge 默认禁用，需显式授权才执行。

## 相关文档

- 安全策略与漏洞报告：[SECURITY.md](SECURITY.md)
- 安全 / 发布边界：[docs/release.md](docs/release.md)、[docs/PROJECT-OVERVIEW.md](docs/PROJECT-OVERVIEW.md)

如本声明与实际行为不符，请按 [SECURITY.md](SECURITY.md) 报告——未声明的外发行为会被视为安全问题。
