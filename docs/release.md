# Release

This document records the release surface required by PRD release constraints and ADR-008. It is release guidance, not a live publishing credential store.

## Compatibility Release Gate

Stable releases require:

- `docs/compatibility/baseline.lock.md` points to the frozen promptfoo 0.121.13 baseline.
- `docs/compatibility/matrix.md` registers all P0/P1/P2 capability rows.
- Task 6.2 golden diff release gate reports zero P0 bug or unclassified findings.
- Task 12.3 full compatibility gate writes `compatibility/artifacts/release-gate/summary.json` or an equivalent CI artifact before stable build/upload.
- Task specs for implemented release-surface work have Done status and completion notes.

If the compatibility release gate is blocked, the release channel must be prerelease or nightly. Stable is disabled until the blocker is fixed or classified according to the compatibility policy.

The machine-readable gate summary records:

- requested release channel: stable, prerelease, or nightly.
- stable decision: `stable_allowed=true|false`.
- P0 fixture coverage count.
- blocking P0 bug/unclassified findings.
- persisted artifact paths and missing artifact paths.

## Channels

### GitHub Releases

Attach platform binaries, checksums, release notes, and the compatibility release gate summary to the tag.

### Homebrew

Publish a formula that downloads the GitHub Releases binary and checksum. The formula must point to immutable tag artifacts.

### Cargo

Publish the crate only after `cargo test --workspace` and the S2V release checklist pass.

### Docker

Build from `Dockerfile` and publish an immutable image digest. The container is a convenience wrapper around the release binary.

### npm Wrapper

The npm wrapper is a compatibility distribution channel over the Rust core boundary. In this workspace, `npm/src` is present but `npm/package.json` is intentionally deferred until a Corepack-enabled release environment is available.

### GitHub Action

Use `.github/workflows/release.yml` as the example shape for CI: checkout, Rust toolchain, S2V verification, build, and release artifact upload.

## S2V Release Checklist

1. Run the compatibility release gate.
2. Run `s2v_verify_full` for the release task verification keys.
3. Confirm README, architecture, Compatibility Matrix, contributing guide, GitHub Action example, and release gate notes are current.
4. Choose Stable only when the gate is Ready. Choose Prerelease or Nightly when the gate is Blocked.
