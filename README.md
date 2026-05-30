# promptfoo-rs

`promptfoo-rs` is a Rust-first implementation of the promptfoo 0.121.13 local evaluation workflow. The default path is a single Rust binary; optional bridges keep compatibility with custom scripts and Node API users where explicitly enabled.

## Install

- GitHub Releases: download the platform binary from a tagged release.
- Cargo: `cargo install promptfoo-rs` once the crate is published.
- Docker: `docker run ghcr.io/leafiellune/promptfoo-rs:latest promptfoo-rs --help`.
- Homebrew: install through the release tap once the formula is published.
- npm wrapper: use the Node wrapper package when a JavaScript API boundary is required.
- GitHub Action: use the workflow example in `.github/workflows/release.yml` as the CI shape.

## Compatibility Release Gate

Stable releases require the compatibility release gate to pass. The gate uses `docs/compatibility/baseline.lock.md`, the Compatibility Matrix, and the golden diff summary from task 6.2. P0 bugs or unclassified diffs block stable releases; blocked builds may only become prerelease or nightly artifacts.

## S2V Evidence

Development follows S2V: task specs define acceptance criteria, tests prove the behavior, and completion notes record verification. Start with `AGENTS.md`, `docs/s2v-adapter.md`, and `docs/prds/promptfoo-rs.prd.md`.

## Local Development

```bash
cargo check --workspace
cargo test --workspace
```

When running S2V helper scripts on Windows, use Git for Windows Bash.
