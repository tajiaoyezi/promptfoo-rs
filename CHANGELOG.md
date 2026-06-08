# Changelog

This project follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and uses Semantic Versioning once public releases begin.

## [Unreleased]

### Added

- Community-health files: GitHub issue forms (`bug_report`, `feature_request`, `compatibility_report`), `ISSUE_TEMPLATE/config.yml`, `PULL_REQUEST_TEMPLATE.md`, `dependabot.yml`, `CODEOWNERS`, root `SUPPORT.md`, and root `PRIVACY.md` (privacy & telemetry statement).

### Fixed

- GitHub Releases install snippets in `README.md`, `README.en.md`, `docs/QUICKSTART.md`, and `docs/QUICKSTART.en.md` now use the correct `0.1.3` asset filenames and tag (previously referenced `0.1.2` filenames under the `v0.1.3` path, or the `v0.1.2` tag entirely).

### Changed

- README documentation navigation now links `SUPPORT.md`, `PRIVACY.md`, `CODE_OF_CONDUCT.md`, and the issue/PR templates.

## [0.1.3] - 2026-06-07

### Added

- Phase 49 product-baseline v1 gate alignment on the frozen `promptfoo@0.121.15` baseline: `current-upstream-policy.json` defaults to `target_mode=product-baseline` (`product_baseline_frozen=true`, `current_upstream_rebaseline_required=false`); runtime gates consume the v1 authority/publication waivers (`required_user_decision_count=0`, `v1_scope_ready=true`).
- Same GitHub Releases binary matrix as v0.1.2: Linux x64/arm64, Windows, macOS arm64/x64.

### Notes

- `perfect_refactor_claim_allowed=false` and aggregate `publication_ready=false` remain by design (ADR-012). See [docs/release-notes/v0.1.3.md](docs/release-notes/v0.1.3.md).

## [0.1.2] - 2026-06-07

### Added

- Linux arm64 GitHub Release archive (`aarch64-unknown-linux-gnu`) on `ubuntu-24.04-arm` runner in `.github/workflows/release.yml`.

## [0.1.1] - 2026-06-07

### Added

- macOS GitHub Release archives (`aarch64-apple-darwin`, `x86_64-apple-darwin`) in `.github/workflows/release.yml`.

### Changed

- `scripts/release/package-github-release.sh` builds with explicit `--target` for cross-compiled release archives.

## [0.1.0] - 2026-06-06

### Added

- Open-source documentation package:
  - Root `README.md` and `README.en.md`.
  - Quickstart guides in `docs/QUICKSTART.md` and `docs/QUICKSTART.en.md`.
  - Project overview in `docs/PROJECT-OVERVIEW.md`.
  - Root `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, `LICENSE`, and `NOTICE`.

### Changed

- README wording now distinguishes local usability from full current-latest promptfoo replacement and public publication authority.


