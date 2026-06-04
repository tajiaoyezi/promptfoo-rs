# Quickstart

This guide builds `promptfoo-rs` from source and runs a minimal local eval through the local `promptfoo` command.

## 1. Prerequisites

You need:

- Rust stable toolchain
- Node.js 20+, Corepack, and pnpm for `viewer/` and `npm/`
- Git
- On Windows, use Git for Windows Bash for S2V helper scripts: `C:\Program Files\Git\bin\bash.exe`

Check the toolchain:

```bash
rustc --version
cargo --version
node --version
corepack --version
```

## 2. Clone and build

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

The preferred local entrypoint is `promptfoo --help`. The Rust release also keeps `promptfoo-rs`, and the npm wrapper's local bin shim supports `pf`.

## 3. Run a minimal eval

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
target/release/promptfoo eval -c promptfooconfig.yaml
```

Windows PowerShell:

```powershell
.\target\release\promptfoo.exe eval -c promptfooconfig.yaml
```

The default output is a JSON envelope. A successful run should contain `status: "ok"` and `summary.total_cases: 1`.

## 4. Write output files

```bash
target/release/promptfoo eval -c promptfooconfig.yaml --output results.json
target/release/promptfoo view .
```

CI-facing output contracts include JSON, JSONL, JUnit, SARIF, CSV, and HTML; support is covered by `src/output/` and release-gate tests.

## 5. Use env files and prompt files

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
target/release/promptfoo eval -c promptfooconfig.yaml --env-file .env
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
