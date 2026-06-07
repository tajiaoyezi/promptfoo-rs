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

The npm wrapper is a compatibility distribution channel over the Rust core boundary. The tracked `npm/package.json` exposes local bin shims for `promptfoo`, `promptfoo-rs`, and `pf`, and supports local typecheck, test, build, smoke, and `pnpm pack` dry-run evidence. This is `local build/package smoke`; real `public registry publication` through `npm publish` requires explicit credentials and must remain blocked in release evidence until those credentials and publication authority are provided.

### GitHub Action

Use `.github/workflows/release.yml` as the example shape for CI: checkout, Rust toolchain, S2V verification, build, and release artifact upload.

## Publication Authority Gate

`bash scripts/release/installability.sh` writes both `target/release-gates/installability.json` and `target/release-gates/publication-authority.json`. The first file proves local installability and dry-run packaging. The second file is the public publication authority gate: every channel records `installability_status`, `authority_status`, `credential_probe`, `legal_brand_requirement`, `published=false`, `published_evidence=null`, and an explicit blocker until real credentials, publication authority, and external URL/digest evidence exist.

## v1 Publication Scope (2026-06-06)

Maintainer policy: `docs/compatibility/v1-release-authority-policy.md`

- **Authorized for v1**: GitHub Releases only (`approval:v1-github-releases-only-2026-06-06`).
- **Deferred for v1 (formal waiver)**: Cargo, npm wrapper, Docker, Homebrew, GitHub Action.
- **Brand/legal**: independent reimplementation wording required (`approval:legal-brand-independent-reimplementation-2026-06-06`).

Current publication evidence (`docs/compatibility/publication-evidence.json`) records **github-releases** as `published` for tag `v0.1.1` (external URL + sha256). The five v1-deferred channels remain `blocked`. Aggregate `publication_ready` stays `false` until policy requires more than the single authorized channel or deferred channels are waived/published.

## v0.1.0 GitHub Release (shipped 2026-06-06)

Live release: https://github.com/tajiaoyezi/promptfoo-rs/releases/tag/v0.1.0

Assets: Linux tarball, Windows zip, merged `SHA256SUMS`. Publication evidence backfilled in `docs/compatibility/publication-evidence.json` (PR #9).

## v0.1.1 GitHub Release (macOS matrix; shipped 2026-06-07)

Live release: https://github.com/tajiaoyezi/promptfoo-rs/releases/tag/v0.1.1

Adds macOS `aarch64-apple-darwin` and `x86_64-apple-darwin` archives to the release build matrix alongside existing Linux and Windows artifacts. Tag `v0.1.1` publishes all four platform archives plus combined `SHA256SUMS`. Publication evidence backfilled in `docs/compatibility/publication-evidence.json` (PR #15).


## v0.1.2 GitHub Release (current; ships with tag)

Live release: https://github.com/tajiaoyezi/promptfoo-rs/releases/tag/v0.1.2

Adds Linux arm64 `aarch64-unknown-linux-gnu` archive on the `ubuntu-24.04-arm` runner alongside existing Linux x64, Windows, and macOS artifacts. Tag `v0.1.2` publishes five platform archives plus combined `SHA256SUMS`. Backfill publication evidence after the release upload completes.

### Repeat procedure for future tags

1. Merge changes to `master`; `verify` workflow on push/PR should be green.
2. Push an annotated tag: `git tag vX.Y.Z && git push origin vX.Y.Z`.
3. `.github/workflows/release.yml` runs S2V verification, packages linux/windows/macos archives (including linux arm64), and uploads assets to the GitHub Release using `docs/release-notes/vX.Y.Z.md`.
4. Backfill publication evidence for the published asset:

```bash
node scripts/release/backfill-github-release-evidence.mjs \
  --artifact-url https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.2/promptfoo-rs-0.1.2-x86_64-unknown-linux-gnu.tar.gz \
  --digest sha256:<checksum-from-SHA256SUMS> \
  --timestamp 2026-06-07T02:33:32Z \
  --release-notes docs/release-notes/v0.1.2.md
```

5. Run `bash scripts/release/publication-evidence.sh` and commit the updated manifest. Aggregate `publication_ready` may remain `false` while v1-deferred channels stay blocked; that is expected.

`target/release-gates/release-candidate.json.publication_authority` remains `credential-blocked` while any channel is unpublished. Dry-run archives, `cargo package`, `pnpm pack`, Dockerfile checks, and Homebrew documentation checks are not public release evidence.

## External Authority Gate

Task 19.4 adds `target/release-gates/external-authority-blockers.json` and includes it in `target/release-gates/release-candidate.json.external_authority`. This artifact combines provider/product blockers from `longtail-classification.json` with publication blockers from `publication-authority.json`.

Each external authority item records `authority_type`, `required_decision`, `current_status`, `safe_local_fallback`, and `release_impact`. Provider rows may stay `waived-with-boundary` only for local mock or fixture accounting; publication rows stay `blocked` while `published=false`. No entry may become `ready` without real credentials, account or product authority, legal/brand approval where relevant, and external URL/digest evidence.

## Perfect Refactor Claim Contract

Task 20.2 adds `target/release-gates/perfect-refactor-claim.json` and links it from `target/release-gates/release-candidate.json.perfect_refactor_claim`. This is the authority for any statement that the project fully satisfies the promptfoo perfect-refactor target.

Current local stable release gates can pass for the frozen baseline while `perfect_refactor_claim_allowed=false`. The claim stays false until source accounting blockers are zero, current-upstream evidence is ready, external authority blockers are resolved, publication authority is ready, and the publication flag is true with external URL/digest evidence. Local stable means the local frozen-baseline release gate is ready; it is not a public or perfect-refactor completion claim.

## Perfect Refactor Unblock Packet

Task 22.1 adds `target/release-gates/perfect-refactor-unblock-packet.json` and links it from `target/release-gates/release-candidate.json.perfect_refactor_unblock_packet`. This packet is a blocker handoff artifact: it lists the minimum user, maintainer, product owner, account owner, service owner, legal/brand reviewer, or release maintainer decisions still required before a perfect-refactor claim can become true.

Every unblock item records `required_actor`, `required_evidence`, `source_artifact`, `release_impact`, and `auto_resolvable=false`. Dry-run installability, local fixture coverage, and frozen-baseline local stable readiness do not satisfy these items. The packet must remain `status=blocked` and `auto_resolvable=false` while credentials, account/product authority, current-upstream same-ref evidence, legal/brand approval, or external publication URL/digest evidence are absent.

## Current Latest Quality Gate

Task 24.4 adds `target/release-gates/current-latest-quality.json` and links it from `target/release-gates/release-candidate.json.current_latest_quality`. The report aggregates adapter verification, current-latest source inventory, current-latest matrix, golden corpus, deterministic regression/stress/property checks, runtime smoke, external authority, publication authority, and claim wording.

The only permitted quality wording is `no known release-blocking defects under declared gates`. The gate rejects phrases such as `no potential bugs`, `zero possible bugs`, and `bug-free`; local current-latest readiness can become true only when all local current-latest gates pass, while public perfect-refactor completion remains blocked until external authority and publication authority are ready.

## S2V Release Checklist

1. Run the compatibility release gate.
2. Run `s2v_verify_full` for the release task verification keys.
3. Confirm README, architecture, Compatibility Matrix, contributing guide, GitHub Action example, and release gate notes are current.
4. Choose Stable only when the gate is Ready. Choose Prerelease or Nightly when the gate is Blocked.
5. Run `bash scripts/release/installability.sh` to generate `target/release-gates/installability.json`; local installability may be ready while public publication remains `credential-blocked`.
