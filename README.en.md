# promptfoo-rs

<div align="right">

[中文](README.md) | **English**

</div>

`promptfoo-rs` is a Rust-first promptfoo-compatible CLI, library, and local viewer implementation. It focuses on local eval workflows, config parsing, result output, compatibility evidence, and release gates that are auditable and testable by default.

Current status: local S2V verification passes, and the implemented scope builds, tests, and runs. The project does not yet claim full 1:1 replacement parity with the latest promptfoo GitHub HEAD, and public release-channel authority is not complete.

[Quickstart](docs/QUICKSTART.en.md) | [Project Overview](docs/PROJECT-OVERVIEW.md) | [Architecture](docs/architecture.md) | [Release Boundary](docs/release.md) | [Contributing](CONTRIBUTING.md)

## What this project is

promptfoo-rs is designed for:

- Running `promptfoo-rs eval -c promptfooconfig.yaml` locally or in CI with stable JSON/JSONL/JUnit/SARIF/CSV/HTML output.
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
| Public stable publication | Not complete. Real credentials, legal/brand approval, external URLs, and digests are still required. |

The strongest supported wording is: no known release-blocking defects under the declared gates. The project does not claim bug-free behavior or complete live-provider parity without matching gate evidence.

## Quickstart

### Prerequisites

- Rust stable toolchain
- Node.js 20+, Corepack, and pnpm for `viewer/` and `npm/`
- On Windows, use Git for Windows Bash for S2V helper scripts: `C:\Program Files\Git\bin\bash.exe`

### Build the CLI

```bash
cargo build --workspace --release
target/release/promptfoo-rs --help
```

Windows PowerShell:

```powershell
cargo build --workspace --release
.\target\release\promptfoo-rs.exe --help
```

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
target/release/promptfoo-rs eval -c promptfooconfig.yaml --output results.json
```

See [docs/QUICKSTART.en.md](docs/QUICKSTART.en.md) for more examples.

## CLI surface

The CLI currently exposes:

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

promptfoo-rs is an independent Rust reimplementation and is not affiliated with the upstream promptfoo project. The `promptfoo` name is used to describe the compatibility target and configuration format.
