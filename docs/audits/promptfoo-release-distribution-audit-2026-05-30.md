# promptfoo release and distribution audit - 2026-05-30

**Status**: Audit attachment
**Parent audit**: `docs/audits/promptfoo-perfect-refactor-audit-2026-05-30.md`
**Purpose**: check whether release, distribution, local viewer, and Node wrapper evidence supports a complete `promptfoo/promptfoo` refactor claim.

## Snapshot

| Item | Value |
|---|---|
| Local branch before this attachment | `master` |
| Local HEAD before this attachment | `91ac725b5ec2611ab5f8a165fbf8a5b577a365fd` |
| Focus area | PRD release constraints, Phase 10, viewer, npm wrapper, release workflow |

## PRD Requirements

The PRD requires:

- local Web viewer reads JSONL/SQLite results
- release channels include GitHub Releases, Homebrew, Cargo, Docker, npm wrapper, and GitHub Action example
- stable releases require the compatibility release gate
- published artifacts must be backed by release evidence, checksums/digests, and install instructions

## Local Artifact Inventory

| Artifact | Current evidence | Audit status |
|---|---|---|
| GitHub Actions workflow | `.github/workflows/release.yml` exists | Example only |
| Dockerfile | `Dockerfile` exists | Build recipe only |
| Cargo package | `Cargo.toml` and `Cargo.lock` exist | Crate not proven published |
| GitHub Releases binary | no tagged release artifact in worktree | Missing evidence |
| Homebrew formula | release docs describe future formula | Missing artifact |
| npm wrapper source | `npm/src/index.ts`, `npm/src/rpc.ts` exist | Source slice only |
| npm package metadata | `npm/package.json` does not exist | Missing |
| npm lockfile | `npm/pnpm-lock.yaml` does not exist | Missing |
| viewer source | `viewer/src/App.tsx`, `viewer/src/results.ts` exist | Source slice only |
| viewer package metadata | `viewer/package.json` does not exist | Missing |
| viewer lockfile | `viewer/pnpm-lock.yaml` does not exist | Missing |
| release binary | `target/release/promptfoo-rs` not present at audit time | Not built in worktree |

## Phase 10 Evidence Boundary

`phase-10-web-viewer-release.md` is `Done`, but its phase acceptance criteria only say the phase smoke completed through task §9 or manual evidence. It does not identify a concrete browser smoke, npm publish dry-run, Docker build, Homebrew formula test, cargo package publish dry-run, or GitHub Release artifact upload.

### Task 10.1: Web Viewer

Task 10.1 lists `viewer/package.json`, `viewer/src/App.tsx`, `viewer/src/results.ts`, and `viewer/src/results.test.ts` in scope, but the current tracked files include only:

- `viewer/.gitkeep`
- `viewer/src/App.tsx`
- `viewer/src/results.ts`
- Rust-side `src/viewer_server.rs`
- Rust integration test `tests/web_viewer.rs`

The task completion notes explicitly say `viewer/package.json` and a Vitest harness were not added because the environment lacked Corepack. The current Rust tests verify data-contract loading/filtering/export; they do not prove a packaged Vite/React viewer or browser runtime.

### Task 10.2: Release Docs Packaging

Task 10.2 is `Done`, and the repository contains:

- `README.md`
- `docs/architecture.md`
- `docs/release.md`
- `docs/contributing.md`
- `.github/workflows/release.yml`
- `Dockerfile`
- `src/release.rs`
- `tests/release_docs_packaging.rs`

The completion notes explicitly say:

- `npm/package.json` was not added because the environment lacked Corepack
- the release workflow is an example
- real release keys and permissions are absent for Homebrew, crates.io, container registry, npm, and GitHub release publishing

This is a valid local documentation and contract milestone, but it is not evidence of complete distribution parity.

## Workflow Evidence

`.github/workflows/release.yml` performs:

- checkout
- Rust stable toolchain setup
- `s2v_verify_full "install typecheck unit-test"`
- `cargo build --workspace --release`
- a reminder that stable publishing requires the compatibility release gate

It does not perform:

- actual compatibility fixture golden diff
- artifact upload
- checksum publication
- container image push
- Homebrew formula update
- crates.io publish dry-run or publish
- npm package build/test/publish
- viewer build/test/browser smoke

## Release Contract Evidence

`src/release.rs` models release readiness, artifacts, channels, and stable/prerelease/nightly decisions. `tests/release_docs_packaging.rs` validates that the model contains release channels and docs.

This is a contract-level check. It does not verify real publication or installability from any channel.

## Verdict

The current repository has release documentation, a Dockerfile, a GitHub Actions example, release readiness modeling, npm wrapper source, and viewer source. It does not have enough evidence to claim that promptfoo-rs is fully releasable as a complete promptfoo replacement:

- no npm package metadata or lockfile
- no viewer package metadata or browser test harness
- no published GitHub Release artifact
- no Homebrew formula artifact
- no container image digest for promptfoo-rs
- no crates.io publication evidence
- no npm publication evidence
- no release workflow that executes the full compatibility release gate or publishes artifacts

Therefore Phase 10 `Done` is a local milestone, not proof that the project fully satisfies the PRD's multi-channel release and viewer compatibility promise.
