# promptfoo-rs

<div align="right">

[中文](README.md) | **English**

</div>

`promptfoo-rs` is a Rust-first promptfoo-compatible CLI, library, and local viewer implementation. It focuses on local eval workflows, config parsing, result output, compatibility evidence, and release gates that are auditable and testable by default.

**Independent reimplementation**: promptfoo-rs is not the official [promptfoo](https://github.com/promptfoo/promptfoo) project and does not imply upstream endorsement. The `promptfoo` name describes compatibility targets and config formats only.

Current status: **v0.1.1** ships on [GitHub Releases](https://github.com/tajiaoyezi/promptfoo-rs/releases) with Linux, Windows, and macOS (arm64 and x64) binaries (the only v1-authorized public channel). Local S2V verification passes. The project does not claim full 1:1 replacement with the latest promptfoo GitHub HEAD or perfect-refactor completion.

[Quickstart](docs/QUICKSTART.en.md) | [Project Overview](docs/PROJECT-OVERVIEW.md) | [Architecture](docs/architecture.md) | [Release Boundary](docs/release.md) | [Contributing](CONTRIBUTING.md)

## What this project is

promptfoo-rs is designed for:

- Running `promptfoo eval -c promptfooconfig.yaml` locally or in CI with stable JSON/JSONL/JUnit/SARIF/CSV/HTML output.
- Implementing the eval runner, scheduler, cache, result store, retry behavior, provider/assertion contracts, and security defaults in Rust.
- Recording promptfoo compatibility through a compatibility matrix, golden corpus, real upstream corpus, and S2V release gates.
- Staying local-first by default: no upload, no unapproved script execution, and no remote cloud mutation.
- Providing verified boundaries for the Node API wrapper, local web viewer, Docker image shape, and GitHub Actions release workflow.

## Current status

| Area | Status |
|---|---|
| Local build and tests | Ready. `install`, `lint`, `typecheck`, `unit-test`, `integration`, `e2e`, `coverage`, `build`, and `runtime-smoke` pass. |
| Normal use within implemented scope | Ready. CLI, core eval, output formats, viewer data contracts, Node wrapper smoke, and compatibility gates are covered. |
| Full replacement for current latest promptfoo | Not claimed. Current-latest gates still keep blockers and `perfect_refactor_claim_allowed=false`. |
| Public stable publication (v1 scope) | **GitHub Releases** (v0.1.0 Linux/Windows; macOS from v0.1.1). Cargo, npm, Docker, Homebrew, and GitHub Action are formally deferred for v1; aggregate `publication_ready` remains false. |

The strongest supported wording is: no known release-blocking defects under the declared gates. The project does not claim bug-free behavior or complete live-provider parity without matching gate evidence.

## Quickstart

### Prerequisites

- Rust stable toolchain
- Node.js 20+, Corepack, and pnpm for `viewer/` and `npm/`
- On Windows, use Git for Windows Bash for S2V helper scripts: `C:\Program Files\Git\bin\bash.exe`

### Build the CLI

```bash
cargo build --workspace --release
target/release/promptfoo --help
```

Windows PowerShell:

```powershell
cargo build --workspace --release
.\target\release\promptfoo.exe --help
```

After building, the preferred local command is `promptfoo --help`. `promptfoo-rs` remains available as the explicit Rust alias, and the npm wrapper's local bin shims also support `pf`.

### Run a minimal eval

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
target/release/promptfoo eval -c promptfooconfig.yaml --output results.json
```

To open a local result directory:

```bash
target/release/promptfoo view .
```

See [docs/QUICKSTART.en.md](docs/QUICKSTART.en.md) for more examples.

### Install from GitHub Releases

Recommended v1 install path: download platform assets from [Releases](https://github.com/tajiaoyezi/promptfoo-rs/releases) and verify `SHA256SUMS`.

Linux x86_64:

```bash
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.1/promptfoo-rs-0.1.1-x86_64-unknown-linux-gnu.tar.gz
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.1/SHA256SUMS
sha256sum -c SHA256SUMS --ignore-missing
tar -xzf promptfoo-rs-0.1.1-x86_64-unknown-linux-gnu.tar.gz
./promptfoo --help
```

macOS Apple Silicon (arm64):

```bash
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.1/promptfoo-rs-0.1.1-aarch64-apple-darwin.tar.gz
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.1/SHA256SUMS
shasum -a 256 -c SHA256SUMS --ignore-missing
tar -xzf promptfoo-rs-0.1.1-aarch64-apple-darwin.tar.gz
./promptfoo --help
```

macOS Intel (x64): download `promptfoo-rs-0.1.1-x86_64-apple-darwin.tar.gz`, verify `SHA256SUMS`, extract, and add the directory to `PATH`.

Windows x86_64: download `promptfoo-rs-0.1.1-x86_64-pc-windows-msvc.zip`, extract it, add the folder to `PATH`, then run `promptfoo.exe --help`.

### Release and install channel status

v1 published channel: **GitHub Releases** (above). Other channels (`cargo install`, npm registry, Docker registry, Homebrew, GitHub Marketplace Action) are formally deferred for v1; the repo claims only `local build/package smoke` and gate documentation for those channels, and real `public registry publication` remains blocked. See [docs/release.md](docs/release.md) and [docs/release-notes/v0.1.0.md](docs/release-notes/v0.1.0.md).

## CLI surface

The CLI currently exposes:

- Preferred local entrypoints: `promptfoo --help`, `promptfoo eval -c promptfooconfig.yaml`, and `promptfoo view .`.
- Supported aliases: the Rust release also builds `promptfoo-rs`; the npm wrapper local bin shims expose `promptfoo`, `promptfoo-rs`, and `pf`.
- `eval`: run a local eval from a promptfoo config.
- `view`: read local result artifacts and emit the viewer data contract.
- `cache`: manage local eval cache state.
- `redteam`: run local redteam init/generate/eval/run/report flows.
- `mcp`: list MCP compatibility tool metadata.
- `code-scans`, `scan-model`, `model-audit`: emit scan/audit data contracts.
- `import`, `export`: import/export promptfoo artifacts.
- `share`, `auth`, `list`, `logs`, `delete`, and related cloud commands: compatibility entrypoints that fail closed by default; they do not upload or mutate remote state.

## Compatibility boundary

Authoritative compatibility files:

- [docs/compatibility/matrix.md](docs/compatibility/matrix.md)
- [docs/compatibility/baseline.lock.md](docs/compatibility/baseline.lock.md)
- [docs/compatibility/current-latest.lock.md](docs/compatibility/current-latest.lock.md)
- [docs/release.md](docs/release.md)

Compatibility policy:

- P0 behavior requires golden-diff or fixture evidence.
- P1 behavior requires snapshot or protocol evidence.
- P2 behavior must be registered with an unsupported/later/bridge-backed reason.
- Unclassified behavior, missing evidence, and real external authority gaps remain visible blockers.

## Development and verification

```bash
cargo check --workspace
cargo test --workspace
cargo build --workspace
```

Full S2V verification:

```bash
source docs/s2v/scripts/lib/preflight.sh
source docs/s2v/scripts/lib/verify.sh
s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"
```

Windows:

```powershell
& 'C:\Program Files\Git\bin\bash.exe' -lc 'source docs/s2v/scripts/lib/preflight.sh; source docs/s2v/scripts/lib/verify.sh; s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"'
```

## Documentation

| I want to... | Link |
|---|---|
| Get running quickly | [docs/QUICKSTART.en.md](docs/QUICKSTART.en.md) |
| Understand repository layout | [docs/PROJECT-OVERVIEW.md](docs/PROJECT-OVERVIEW.md), [docs/architecture.md](docs/architecture.md) |
| Check compatibility | [docs/compatibility/matrix.md](docs/compatibility/matrix.md) |
| Understand release gates | [docs/release.md](docs/release.md) |
| Read S2V project rules | [AGENTS.md](AGENTS.md), [docs/s2v-adapter.md](docs/s2v-adapter.md) |
| Read the PRD and specs | [docs/prds/promptfoo-rs.prd.md](docs/prds/promptfoo-rs.prd.md), [docs/specs/](docs/specs/) |
| Contribute | [CONTRIBUTING.md](CONTRIBUTING.md) |
| Report a vulnerability | [SECURITY.md](SECURITY.md) |

## Contributing

Code, docs, compatibility fixtures, bug reports, and audit feedback are welcome. Behavior changes must follow S2V: read `AGENTS.md`, `docs/s2v-adapter.md`, and the relevant task spec first, then proceed through RED -> GREEN -> verification -> completion notes.

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT License. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

promptfoo-rs is an independent Rust reimplementation and is not affiliated with the upstream promptfoo project, nor does it imply official endorsement. The `promptfoo` name is used to describe the compatibility target and configuration format.
