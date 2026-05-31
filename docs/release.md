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

The npm wrapper is a compatibility distribution channel over the Rust core boundary. The tracked `npm/package.json` supports local typecheck, test, build, smoke, and `pnpm pack` dry-run evidence. Real `npm publish` requires explicit credentials and must remain blocked in release evidence until those credentials and publication authority are provided.

### GitHub Action

Use `.github/workflows/release.yml` as the example shape for CI: checkout, Rust toolchain, S2V verification, build, and release artifact upload.

## Publication Authority Gate

`bash scripts/release/installability.sh` writes both `target/release-gates/installability.json` and `target/release-gates/publication-authority.json`. The first file proves local installability and dry-run packaging. The second file is the public publication authority gate: every channel records `installability_status`, `authority_status`, `credential_probe`, `legal_brand_requirement`, `published=false`, `published_evidence=null`, and an explicit blocker until real credentials, publication authority, and external URL/digest evidence exist.

Current remaining publication blockers:

- GitHub Releases: published=false; credential-blocked until a GitHub release publish token, release notes approval, and external artifact URL/checksum evidence exist.
- Cargo: published=false; credential-blocked until crates.io publish authority and external crate URL/digest evidence exist.
- npm wrapper: published=false; credential-blocked until npm publish authority and external package URL/digest evidence exist.
- Docker: published=false; credential-blocked until container registry credentials and immutable image digest evidence exist.
- Homebrew: published=false; credential-blocked/tool-unavailable when `brew` is absent; tap publication requires Homebrew tooling, tap authority, and formula URL/checksum evidence.
- GitHub Action: published=false; credential-blocked until workflow release permission and external run/artifact evidence exist.

`target/release-gates/release-candidate.json.publication_authority` must remain `credential-blocked` while any channel above is unpublished. Dry-run archives, `cargo package`, `pnpm pack`, Dockerfile checks, and Homebrew documentation checks are not public release evidence.

## External Authority Gate

Task 19.4 adds `target/release-gates/external-authority-blockers.json` and includes it in `target/release-gates/release-candidate.json.external_authority`. This artifact combines provider/product blockers from `longtail-classification.json` with publication blockers from `publication-authority.json`.

Each external authority item records `authority_type`, `required_decision`, `current_status`, `safe_local_fallback`, and `release_impact`. Provider rows may stay `waived-with-boundary` only for local mock or fixture accounting; publication rows stay `blocked` while `published=false`. No entry may become `ready` without real credentials, account or product authority, legal/brand approval where relevant, and external URL/digest evidence.

## S2V Release Checklist

1. Run the compatibility release gate.
2. Run `s2v_verify_full` for the release task verification keys.
3. Confirm README, architecture, Compatibility Matrix, contributing guide, GitHub Action example, and release gate notes are current.
4. Choose Stable only when the gate is Ready. Choose Prerelease or Nightly when the gate is Blocked.
5. Run `bash scripts/release/installability.sh` to generate `target/release-gates/installability.json`; local installability may be ready while public publication remains `credential-blocked`.
