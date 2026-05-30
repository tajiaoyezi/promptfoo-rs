# Architecture

promptfoo-rs is organized around a Rust core with optional compatibility bridges. The architecture follows ADR-001 and keeps the default evaluation path independent of Node, Python, or shell runtimes.

## Core Modules

- `config`: promptfoo config normalization.
- `eval`: scheduling, provider execution, retry, cache, and result aggregation.
- `providers` and `assertions`: deterministic contracts for common provider and assertion behavior.
- `results` and `output`: JSONL, SQLite, JUnit, SARIF, CSV, HTML data contracts, and CI-facing output.
- `compatibility`: Compatibility Matrix parsing, baseline lock validation, golden diff normalization, and the compatibility release gate.
- `viewer_server`: local viewer data contract for JSONL/SQLite result loading, table filtering, and export.
- `release`: S2V release checklist and stable/prerelease/nightly decision contract.

## Compatibility Boundary

The Compatibility Matrix is the public map of P0, P1, and P2 behavior. P0 behavior needs runnable golden diff evidence. P1 behavior needs a snapshot or protocol contract. P2 behavior must be registered as unsupported, later, or bridge-backed with a reason.

## Release Boundary

Stable release decisions are data driven. A blocked compatibility release gate disables stable artifacts and allows only prerelease or nightly output. Multi-channel publishing follows ADR-008: GitHub Releases, Homebrew, Cargo, Docker, npm wrapper, and a GitHub Action example.
