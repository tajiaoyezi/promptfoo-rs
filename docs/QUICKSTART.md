# Quickstart

本指南用于把 `promptfoo-rs` 从源码构建起来，并运行一个最小本地 eval。

## 1. 准备环境

需要：

- Rust stable toolchain
- Node.js 20+、Corepack、pnpm，用于 `viewer/` 和 `npm/`
- Git
- Windows 用户运行 S2V helper 时使用 Git for Windows Bash：`C:\Program Files\Git\bin\bash.exe`

确认工具链：

```bash
rustc --version
cargo --version
node --version
corepack --version
```

## 2. 克隆和构建

```bash
git clone https://github.com/tajiaoyezi/promptfoo-rs.git
cd promptfoo-rs
cargo build --workspace --release
target/release/promptfoo-rs --help
```

Windows PowerShell:

```powershell
git clone https://github.com/tajiaoyezi/promptfoo-rs.git
cd promptfoo-rs
cargo build --workspace --release
.\target\release\promptfoo-rs.exe --help
```

## 3. 运行最小 eval

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
target/release/promptfoo-rs eval -c promptfooconfig.yaml
```

Windows PowerShell:

```powershell
.\target\release\promptfoo-rs.exe eval -c promptfooconfig.yaml
```

默认输出为 JSON envelope。成功时应看到 `status: "ok"` 和 `summary.total_cases: 1`。

## 4. 写入输出文件

```bash
target/release/promptfoo-rs eval -c promptfooconfig.yaml --output results.json
```

可用于 CI 的输出契约包括 JSON、JSONL、JUnit、SARIF、CSV 和 HTML；具体支持由 `src/output/` 和 release gate 测试覆盖。

## 5. 使用 env 文件和 prompt 文件

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
target/release/promptfoo-rs eval -c promptfooconfig.yaml --env-file .env
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
- 本地 gate 通过不等于已经公开发布，也不等于完整替代 promptfoo 当前 GitHub HEAD。
- 当前兼容边界以 [docs/compatibility/matrix.md](compatibility/matrix.md) 和 [docs/release.md](release.md) 为准。
