<div align="right">

[中文](README.md) · **English**

</div>

<div align="center">

# promptfoo-rs

**A Rust-first [promptfoo](https://github.com/promptfoo/promptfoo)-compatible CLI · library · local viewer**

*A local-first, auditable LLM eval toolchain that is honest — down to the gate level — about its compatibility boundary and release claims.*

[![license](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE)
[![release](https://img.shields.io/github/v/release/tajiaoyezi/promptfoo-rs?style=flat-square&color=brightgreen)](https://github.com/tajiaoyezi/promptfoo-rs/releases)
[![verify](https://github.com/tajiaoyezi/promptfoo-rs/actions/workflows/verify.yml/badge.svg?branch=master)](https://github.com/tajiaoyezi/promptfoo-rs/actions/workflows/verify.yml)
[![rust](https://img.shields.io/badge/rust-stable-orange?style=flat-square&logo=rust)](Cargo.toml)
![platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey?style=flat-square)
![compat](https://img.shields.io/badge/compat%20baseline-promptfoo%400.121.15%20frozen-blueviolet?style=flat-square)
[![last commit](https://img.shields.io/github/last-commit/tajiaoyezi/promptfoo-rs?style=flat-square)](https://github.com/tajiaoyezi/promptfoo-rs/commits/master)

<sub>🦀 Independent Rust reimplementation · not the official promptfoo project · no upstream endorsement</sub>

[⚡ Quickstart](docs/QUICKSTART.en.md) · [📦 Overview](docs/PROJECT-OVERVIEW.md) · [🧩 Compatibility](docs/compatibility/matrix.md) · [🚦 Release boundary](docs/release.md) · [🤝 Contributing](CONTRIBUTING.md) · [❓ Support](SUPPORT.md)

</div>

---

`promptfoo-rs` rewrites promptfoo's local eval loop and compatibility boundary in Rust: a promptfoo-compatible CLI and config parser → a deterministic eval engine (case scheduling / prompt rendering / assertions / cache / resume / retry) → stable CI output (JSON / JSONL / JUnit / CSV / HTML). Provider clients, richer assertions, and additional outputs are recorded as **P0/P1/P2 compatibility evidence** (matrix + golden corpus + S2V gates) — auditable and testable, not a marketing claim. It is local-first by default: no upload, no unapproved script execution, no remote writes.

> Current **v0.1.3** · GitHub Releases only · compatibility baseline frozen at `promptfoo@0.121.15` (ADR-012) · local S2V verification passes 🚧

## 🚦 Status

The project deliberately separates three things, and only draws a conclusion where gates and evidence support it:

| Area | Conclusion |
|---|---|
| ✅ Local build & tests | Ready. `install · lint · typecheck · unit-test · integration · e2e · coverage · build · runtime-smoke` all pass. |
| ✅ Use within implemented scope | Ready. CLI, core eval, output formats, viewer data contracts, Node wrapper smoke, and compatibility gates are covered. |
| 🚧 Full compatibility on frozen baseline | **Not claimed.** `promptfoo@0.121.15` gates still keep blockers and `perfect_refactor_claim_allowed=false`; upstream releases after the freeze are out of scope. |
| 🚧 Public stable publication (v1) | **GitHub Releases only** (current v0.1.3). Cargo / npm / Docker / Homebrew / GitHub Action are formally deferred for v1; aggregate `publication_ready=false`. |

**The strongest supported wording is:**

> no known release-blocking defects under the declared gates.

Honest caveats (kept visible, never auto-downgraded):

- No claim of bug-free behavior or complete live-provider parity without matching gate and external evidence.
- Live providers use your own API keys at your own risk; the frozen baseline still has waived compatibility items.
- It does not track promptfoo releases after `0.121.15` or GitHub HEAD — this is an **independent product line** (ADR-012), not an upstream subscription.

## ✨ Features

- 🧪 **Local eval engine** — schedule cases from `promptfooconfig.yaml`: prompt variable rendering + deterministic assertions + cache / resume / retry / bounded concurrency, emitting result records. The default path is deterministic render-and-assert and **does not issue live model calls**.
- ✅ **Assertions** — the eval config path supports `equals · contains · regex`; the assertion engine also implements `json · schema` and more (covered by fixture / matrix evidence; not all wired into eval config yet).
- 🔌 **Provider compatibility surface** — request normalization and clients for OpenAI-compatible / HTTP / Ollama / Anthropic, recorded as P0 matrix evidence; live calls use your own keys and are verified by snapshot / contract.
- 🧩 **Script bridge** — JS/TS (same JavaScript runtime) · Python · Shell · Ruby custom scripts, sandboxed default-deny.
- 📤 **CI output contracts** — stable JSON / JSONL / JUnit / CSV / HTML; SARIF findings are produced by the `scan` commands.
- 🖥️ **Local viewer** — `promptfoo view .` reads a local result store (`results.jsonl` / SQLite) and emits the viewer data contract (local data, not a hosted SaaS).
- 🛡️ **Redteam / scan** — local `redteam`, `code-scans`, `scan-model`, `model-audit` flows.
- 🧩 **Compatibility evidence** — P0/P1/P2 matrix + golden corpus + S2V gates recording native / bridge / blocked boundaries.
- 🔒 **Secure defaults & verified boundaries** — no upload, no remote writes; `share`/`auth`/cloud fail closed; script execution needs explicit authorization; Node API wrapper / Docker shape / GitHub Actions release are verified boundaries.

## 📦 Install

Recommended for v1: download the asset for your platform from [GitHub Releases](https://github.com/tajiaoyezi/promptfoo-rs/releases) and verify `SHA256SUMS`. The preferred command is `promptfoo`, with the explicit alias `promptfoo-rs` (the npm wrapper's local shim also exposes `pf`).

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
<summary>🍎 macOS (Apple Silicon arm64 / Intel x64)</summary>

```bash
# Apple Silicon (arm64)
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.3/promptfoo-rs-0.1.3-aarch64-apple-darwin.tar.gz
curl -LO https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.3/SHA256SUMS
shasum -a 256 -c SHA256SUMS --ignore-missing
tar -xzf promptfoo-rs-0.1.3-aarch64-apple-darwin.tar.gz
./promptfoo --help
```

Intel (x64): download `promptfoo-rs-0.1.3-x86_64-apple-darwin.tar.gz` instead, verify, extract, and add to `PATH`.

</details>

<details>
<summary>🪟 Windows x86_64</summary>

Download `promptfoo-rs-0.1.3-x86_64-pc-windows-msvc.zip`, extract it, add the folder to `PATH`, then run `promptfoo.exe --help`.

</details>

> Other channels (`cargo install` / npm registry / Docker / Homebrew / GitHub Marketplace Action) are formally deferred for v1; the repo claims only `local build/package smoke` and gate docs for them, and real `public registry publication` remains blocked. See [docs/release.md](docs/release.md) and [docs/release-notes/v0.1.3.md](docs/release-notes/v0.1.3.md).

## 🚀 Quick start

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

Run it and view the result:

```bash
promptfoo eval -c promptfooconfig.yaml --output results.jsonl
promptfoo view .
```

Build from source (developers):

```bash
cargo build --workspace --release
target/release/promptfoo --help          # Windows: .\target\release\promptfoo.exe --help
```

See [docs/QUICKSTART.en.md](docs/QUICKSTART.en.md) for more examples.

## 🧩 Compatibility & scope

promptfoo-rs aims for a **provable, well-defined functional boundary**, not byte-exact parity with upstream. Authoritative files:

- [docs/compatibility/matrix.md](docs/compatibility/matrix.md) — P0/P1/P2 compatibility matrix
- [docs/compatibility/current-latest.lock.md](docs/compatibility/current-latest.lock.md) — frozen product baseline lock (`promptfoo@0.121.15`, ADR-012; not a live subscription)
- [docs/compatibility/baseline.lock.md](docs/compatibility/baseline.lock.md) — Phase 1 historical harness baseline (`0.121.13`)
- [docs/release.md](docs/release.md) — release gate / publication authority / perfect-refactor claim contract

Grading: **P0** needs golden-diff or fixture evidence; **P1** needs snapshot or protocol evidence; **P2** must register an unsupported / later / bridge-backed reason. Unclassified, evidence-missing, and real external-authority gaps stay visible as blockers — never hidden, never auto-downgraded.

Explicitly **out of scope** (by design): tracking upstream versions / GitHub HEAD after `0.121.15`; claiming perfect-refactor or full upstream parity; live-provider result parity.

## 🔒 Security & privacy

- Local-first: no upload, no telemetry, no version check or auto-update by default; `share`/`auth`/cloud commands fail closed.
- Network requests happen only when you explicitly configure a provider, using your own API keys under that provider's terms.
- See [PRIVACY.md](PRIVACY.md) (privacy & telemetry) and [SECURITY.md](SECURITY.md) (security policy & vulnerability reporting).

## 🛠️ Development & verification

```bash
cargo check --workspace
cargo test  --workspace
cargo build --workspace
```

Full S2V verification (on Windows use Git for Windows Bash: `C:\Program Files\Git\bin\bash.exe`):

```bash
source docs/s2v/scripts/lib/preflight.sh
source docs/s2v/scripts/lib/verify.sh
s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"
```

## 📚 Documentation

| I want to... | Link |
|---|---|
| ⚡ Get running quickly | [docs/QUICKSTART.en.md](docs/QUICKSTART.en.md) |
| 🏗️ Understand layout & architecture | [docs/PROJECT-OVERVIEW.md](docs/PROJECT-OVERVIEW.md) · [docs/architecture.md](docs/architecture.md) |
| 🧩 Check compatibility | [docs/compatibility/matrix.md](docs/compatibility/matrix.md) |
| 🚦 Understand release gates | [docs/release.md](docs/release.md) |
| 📐 Architecture decisions (ADR) | [docs/decisions/](docs/decisions/) |
| 📋 PRD & specs | [docs/prds/promptfoo-rs.prd.md](docs/prds/promptfoo-rs.prd.md) · [docs/specs/](docs/specs/) |
| 🧭 S2V project entry | [AGENTS.md](AGENTS.md) · [docs/s2v-adapter.md](docs/s2v-adapter.md) |
| 🤝 Contribute | [CONTRIBUTING.md](CONTRIBUTING.md) |
| ❓ Get help / ask | [SUPPORT.md](SUPPORT.md) |
| 🔐 Privacy / security | [PRIVACY.md](PRIVACY.md) · [SECURITY.md](SECURITY.md) |
| 🏛️ Code of conduct | [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) |

## 🤝 Contributing

Code, docs, compatibility fixtures, bug reports, and audit feedback are welcome. Behavior changes follow S2V: read `AGENTS.md`, `docs/s2v-adapter.md`, and the relevant task spec first, then go through `RED → GREEN → verification → completion notes`.

Intake channels:

- 🐛 [Bug Report](.github/ISSUE_TEMPLATE/bug_report.yml) · 🧩 [Compatibility Report](.github/ISSUE_TEMPLATE/compatibility_report.yml) (gap vs the frozen baseline) · ✨ [Feature Request](.github/ISSUE_TEMPLATE/feature_request.yml)
- 🔀 Pull requests follow the S2V checklist in the [PR template](.github/PULL_REQUEST_TEMPLATE.md)
- ❓ Usage questions start at [SUPPORT.md](SUPPORT.md); 🔐 security issues use a [Security Advisory](https://github.com/tajiaoyezi/promptfoo-rs/security/advisories/new), not a public issue

✨ **MIT · no CLA** — contributions require no extra agreement. See [CONTRIBUTING.md](CONTRIBUTING.md).

## 📄 License

[MIT License](LICENSE) · see [LICENSE](LICENSE) and [NOTICE](NOTICE).

promptfoo-rs is an independent Rust reimplementation, not affiliated with the upstream promptfoo project, and does not imply official endorsement. The `promptfoo` name describes the compatibility target and config format only.

<div align="center">
<br />
<sub>Built with ❤️ and 🦀 · local-first · honest to the gate level</sub>
</div>
