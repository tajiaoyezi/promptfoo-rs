# Quickstart

本指南帮助你在几分钟内安装 `promptfoo-rs` 并运行一个最小 eval。v1 推荐路径是从 [GitHub Releases](https://github.com/tajiaoyezi/promptfoo-rs/releases) 下载预编译二进制；源码构建见文末「从源码构建」。

下文示例中的 `promptfoo` 表示已加入 `PATH` 的命令（GitHub Releases 解压目录，或源码构建的 `target/release/promptfoo`）。Windows 使用 `promptfoo.exe`。

## 1. 从 GitHub Releases 安装（推荐）

在 [Releases](https://github.com/tajiaoyezi/promptfoo-rs/releases) 下载对应平台资产，并校验同版本的 `SHA256SUMS`。

Linux x86_64：

```bash
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.3/promptfoo-rs-0.1.3-x86_64-unknown-linux-gnu.tar.gz
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.3/SHA256SUMS
sha256sum -c SHA256SUMS --ignore-missing
tar -xzf promptfoo-rs-0.1.3-x86_64-unknown-linux-gnu.tar.gz
./promptfoo --help
```

Linux arm64 (aarch64)：

```bash
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.3/promptfoo-rs-0.1.3-aarch64-unknown-linux-gnu.tar.gz
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.3/SHA256SUMS
sha256sum -c SHA256SUMS --ignore-missing
tar -xzf promptfoo-rs-0.1.3-aarch64-unknown-linux-gnu.tar.gz
./promptfoo --help
```


macOS Apple Silicon (arm64)：

```bash
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.3/promptfoo-rs-0.1.3-aarch64-apple-darwin.tar.gz
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.3/SHA256SUMS
shasum -a 256 -c SHA256SUMS --ignore-missing
tar -xzf promptfoo-rs-0.1.3-aarch64-apple-darwin.tar.gz
./promptfoo --help
```

macOS Intel (x64)：下载 `promptfoo-rs-0.1.3-x86_64-apple-darwin.tar.gz`，校验 `SHA256SUMS` 后解压并将目录加入 `PATH`。

Windows x86_64：下载 `promptfoo-rs-0.1.3-x86_64-pc-windows-msvc.zip`，解压后将目录加入 `PATH`，运行 `promptfoo.exe --help`。

首选本地入口是 `promptfoo --help`。Rust release 同时保留 `promptfoo-rs`；npm wrapper 的本地 bin shim 还支持 `pf`。

v1 已发布渠道仅为 **GitHub Releases**；`cargo install`、npm registry、Docker、Homebrew、GitHub Marketplace Action 在 v1 正式延期。详见 [docs/release.md](release.md)。

## 2. 运行最小 eval

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
promptfoo eval -c promptfooconfig.yaml
```

Windows PowerShell：

```powershell
.\promptfoo.exe eval -c promptfooconfig.yaml
```

默认输出为 JSON envelope。成功时应看到 `status: "ok"` 和 `summary.total_cases: 1`。

## 3. 写入输出文件

```bash
promptfoo eval -c promptfooconfig.yaml --output results.json
promptfoo view .
```

可用于 CI 的输出契约包括 JSON、JSONL、JUnit、SARIF、CSV 和 HTML；具体支持由 `src/output/` 和 release gate 测试覆盖。

## 4. 使用 env 文件和 prompt 文件

创建 `.env`：

```dotenv
MODEL=mock-model
```

创建 `prompts/hello.txt`：

```text
Hello {{name}} from ${MODEL}
```

配置：

```yaml
providers:
  - id: http
prompts:
  - file://prompts/hello.txt
tests:
  - vars:
      name: Grace
```

运行：

```bash
promptfoo eval -c promptfooconfig.yaml --env-file .env
```

## 5. 从源码构建（开发者）

需要 Rust stable、Node.js 20+、Corepack、pnpm（用于 `viewer/` 和 `npm/`）和 Git。Windows 用户运行 S2V helper 时使用 Git for Windows Bash：`C:\Program Files\Git\bin\bash.exe`。

```bash
git clone https://github.com/tajiaoyezi/promptfoo-rs.git
cd promptfoo-rs
cargo build --workspace --release
target/release/promptfoo --help
```

Windows PowerShell：

```powershell
git clone https://github.com/tajiaoyezi/promptfoo-rs.git
cd promptfoo-rs
cargo build --workspace --release
.\target\release\promptfoo.exe --help
```

## 6. 常用开发验证

```bash
cargo check --workspace
cargo test --workspace
cargo build --workspace
```

如果需要验证 viewer 和 npm wrapper：

```bash
corepack enable
cd viewer && pnpm install --frozen-lockfile && pnpm typecheck && pnpm test && pnpm build
cd ../npm && pnpm install --frozen-lockfile && pnpm typecheck && pnpm test && pnpm build
```

## 7. 完整 S2V 验证

Linux/macOS/Git Bash：

```bash
source docs/s2v/scripts/lib/preflight.sh
source docs/s2v/scripts/lib/verify.sh
s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"
```

Windows PowerShell：

```powershell
& 'C:\Program Files\Git\bin\bash.exe' -lc 'source docs/s2v/scripts/lib/preflight.sh; source docs/s2v/scripts/lib/verify.sh; s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"'
```

## 8. 注意边界

- `share`、`auth`、`delete` 等 cloud/SaaS 命令默认 fail-closed，不上传、不改远端。
- 脚本 bridge 默认关闭；需要显式授权和 sandbox/redaction 约束。
- `local build/package smoke` 通过不等于已经完成 `public registry publication`，也不等于完整替代 promptfoo 当前 GitHub HEAD。
- 当前兼容边界以 [docs/compatibility/matrix.md](compatibility/matrix.md) 和 [docs/release.md](release.md) 为准。