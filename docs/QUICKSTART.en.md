# Quickstart

This guide gets `promptfoo-rs` running a minimal eval in minutes. For v1, the recommended path is to download a prebuilt binary from [GitHub Releases](https://github.com/tajiaoyezi/promptfoo-rs/releases); building from source is covered at the end under "Build from source".

Examples below use `promptfoo` to mean the command on your `PATH` (from an extracted GitHub Release archive, or `target/release/promptfoo` after a source build). On Windows, use `promptfoo.exe`.

## 1. Install from GitHub Releases (recommended)

Download the asset for your platform from [Releases](https://github.com/tajiaoyezi/promptfoo-rs/releases) and verify `SHA256SUMS` for the same version.

Linux x86_64:

```bash
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.2/promptfoo-rs-0.1.2-x86_64-unknown-linux-gnu.tar.gz
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.2/SHA256SUMS
sha256sum -c SHA256SUMS --ignore-missing
tar -xzf promptfoo-rs-0.1.2-x86_64-unknown-linux-gnu.tar.gz
./promptfoo --help
```

Linux arm64 (aarch64):

```bash
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.2/promptfoo-rs-0.1.2-aarch64-unknown-linux-gnu.tar.gz
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.2/SHA256SUMS
sha256sum -c SHA256SUMS --ignore-missing
tar -xzf promptfoo-rs-0.1.2-aarch64-unknown-linux-gnu.tar.gz
./promptfoo --help
```


macOS Apple Silicon (arm64):

```bash
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.2/promptfoo-rs-0.1.2-aarch64-apple-darwin.tar.gz
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.2/SHA256SUMS
shasum -a 256 -c SHA256SUMS --ignore-missing
tar -xzf promptfoo-rs-0.1.2-aarch64-apple-darwin.tar.gz
./promptfoo --help
```

macOS Intel (x64): download `promptfoo-rs-0.1.2-x86_64-apple-darwin.tar.gz`, verify `SHA256SUMS`, extract, and add the directory to `PATH`.

Windows x86_64: download `promptfoo-rs-0.1.2-x86_64-pc-windows-msvc.zip`, extract, add the directory to `PATH`, and run `promptfoo.exe --help`.

The preferred local entrypoint is `promptfoo --help`. The Rust release also keeps `promptfoo-rs`, and the npm wrapper's local bin shim supports `pf`.

v1 publishes **GitHub Releases** only; `cargo install`, npm registry, Docker, Homebrew, and GitHub Marketplace Action are formally deferred for v1. See [docs/release.md](release.md).

## 2. Run a minimal eval

Create `promptfooconfig.yaml`:

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

Run:

```bash
promptfoo eval -c promptfooconfig.yaml
```

Windows PowerShell:

```powershell
.\promptfoo.exe eval -c promptfooconfig.yaml
```

The default output is a JSON envelope. A successful run should contain `status: "ok"` and `summary.total_cases: 1`.

## 3. Write output files

```bash
promptfoo eval -c promptfooconfig.yaml --output results.json
promptfoo view .
```

CI-facing output contracts include JSON, JSONL, JUnit, SARIF, CSV, and HTML; support is covered by `src/output/` and release-gate tests.

## 4. Use env files and prompt files

Create `.env`:

```dotenv
MODEL=mock-model
```

Create `prompts/hello.txt`:

```text
Hello {{name}} from ${MODEL}
```

Config:

```yaml
providers:
  - id: http
prompts:
  - file://prompts/hello.txt
tests:
  - vars:
      name: Grace
```

Run:

```bash
promptfoo eval -c promptfooconfig.yaml --env-file .env
```

## 5. Build from source (developers)

You need Rust stable, Node.js 20+, Corepack, pnpm (for `viewer/` and `npm/`), and Git. On Windows, use Git for Windows Bash for S2V helper scripts: `C:\Program Files\Git\bin\bash.exe`.

```bash
git clone https://github.com/tajiaoyezi/promptfoo-rs.git
cd promptfoo-rs
cargo build --workspace --release
target/release/promptfoo --help
```

Windows PowerShell:

```powershell
git clone https://github.com/tajiaoyezi/promptfoo-rs.git
cd promptfoo-rs
cargo build --workspace --release
.\target\release\promptfoo.exe --help
```

## 6. Common development checks

```bash
cargo check --workspace
cargo test --workspace
cargo build --workspace
```

To verify the viewer and npm wrapper:

```bash
corepack enable
cd viewer && pnpm install --frozen-lockfile && pnpm typecheck && pnpm test && pnpm build
cd ../npm && pnpm install --frozen-lockfile && pnpm typecheck && pnpm test && pnpm build
```

## 7. Full S2V verification

Linux/macOS/Git Bash:

```bash
source docs/s2v/scripts/lib/preflight.sh
source docs/s2v/scripts/lib/verify.sh
s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"
```

Windows PowerShell:

```powershell
& 'C:\Program Files\Git\bin\bash.exe' -lc 'source docs/s2v/scripts/lib/preflight.sh; source docs/s2v/scripts/lib/verify.sh; s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"'
```

## 8. Boundaries

- `share`, `auth`, `delete`, and related cloud/SaaS commands fail closed by default and do not upload or mutate remote state.
- Script bridges are disabled by default; they require explicit authorization and sandbox/redaction constraints.
- Passing `local build/package smoke` does not mean `public registry publication` is complete or that the project is a complete replacement for the latest promptfoo GitHub HEAD.
- The compatibility boundary is defined by [docs/compatibility/matrix.md](compatibility/matrix.md) and [docs/release.md](release.md).