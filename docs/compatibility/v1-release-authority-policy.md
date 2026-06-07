# v1 Release Authority Policy

Maintainer approval recorded on 2026-06-06 (leafiellune). This document is the policy reference for Phase 44 authority and publication evidence manifests. It does not store credentials.

## Positioning

- promptfoo-rs is an **independent reimplementation**, not the official promptfoo project.
- v1 targets **local-first CLI** usage with deterministic compatibility gates.
- v1 does **not** claim perfect-refactor completion, bug-free behavior, or full upstream feature parity.

## current-latest-target

Tracked target: `promptfoo@0.121.15` with GitHub default branch HEAD recorded in `target/release-gates/current-latest-target.json`.

Approval: `approval:v1-current-latest-target-2026-06-06`

## v1-publication-channels

| Channel | v1 decision |
|---|---|
| GitHub Releases | Authorized (`approval:v1-github-releases-only-2026-06-06`) |
| Cargo | Deferred (waiver) |
| npm wrapper | Deferred (waiver; avoid upstream `promptfoo` npm brand confusion) |
| Docker | Deferred (waiver) |
| Homebrew | Deferred (waiver) |
| GitHub Action | Deferred (waiver) |

Brand/legal approval: `approval:legal-brand-independent-reimplementation-2026-06-06` — public copy must state independent reimplementation and must not imply upstream endorsement.

Publication credentials remain outside the repository. `published=true` requires a real tagged GitHub Release URL and checksum.

## Config / cloud / server modules

v1 waives cloud sync, hosted server, account-linked config, and related parity claims. Users should expect local CLI workflows only.

## Longtail provider modules

v1 waives live parity claims for Assistants, Agents, Realtime, Claude Code auth, and related longtail provider surfaces. Users may supply their own API keys; the project does not claim dedicated live fixtures or product authority for these modules in v1.

## Evaluator in-memory store

Phase 47 adds dedicated eval-runner fixture evidence for `eval-runner:src-evaluator-inmemorystore`. This is a local eval-runner golden burndown item, not an external authority decision row in `authority-decisions.json`. `perfect_refactor_claim_allowed` still remains false while other golden blockers exist.