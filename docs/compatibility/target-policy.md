# Compatibility Target Policy

**Status**: Ready
**Basis**: PRD §Upstream Baseline Freeze Strategy, ADR-007, docs/audits/promptfoo-final-audit-index-2026-05-30.md

Stable release gates bind to exactly one immutable compatibility target. Moving upstream observations are append-only tracking evidence and never replace the frozen baseline without an explicit rebaseline task and updated S2V specs.

```json
{
  "stable_targets": [
    {
      "id": "promptfoo-0.121.13-frozen-baseline",
      "kind": "FrozenBaseline",
      "package_version": "0.121.13",
      "git_ref": "refs/tags/0.121.13",
      "git_commit": "4860e990c7e9a2f8f677173fb92cf9867b34d03f",
      "npm_integrity": "sha512-DBPSixUophzcD7S7lML6SqVwnVtrhK5A3HsZ03IG9Xrw0t24r5imG7nLj+YMb0vlAjbdFtE7yFG+rsqDpfYp6g==",
      "container_digest": "sha256:3993e7c105bcbc1c8f763309552728dd2bf30ff5c9c2e14ec69297b42d096f80"
    }
  ],
  "moving_upstream_observations": [
    {
      "head": "945fda5d965ed27abb302fe0f0910b7dddea5dde",
      "package_version": "0.121.13",
      "collected_at": "2026-05-30T00:00:00Z",
      "source": "upstream origin/main tracking",
      "modifies_frozen_baseline": false
    }
  ]
}
```

## Policy

- Stable releases must use the single `stable_targets[0]` entry above until an explicit rebaseline task updates this policy and all dependent compatibility evidence.
- References such as `latest`, `main`, `master`, or `HEAD` are never valid stable targets.
- Moving upstream observations can record drift in current upstream `origin/main`, but they are tracking-only and cannot modify `docs/compatibility/baseline.lock.md`.

## Current Upstream Gate

Task 18.3 adds `compatibility/inventory/current-upstream-target.json`, `scripts/release/current-upstream-policy.sh`, and runtime-smoke artifact `target/release-gates/current-upstream-policy.json`. The release candidate now carries `target_policy.target_mode`, `target_policy.current_perfect_claim_allowed`, frozen git commit, and observed current HEAD. In the default frozen mode, `current_perfect_claim_allowed=false` whenever observed current HEAD differs from `4860e990c7e9a2f8f677173fb92cf9867b34d03f`; current mode requires source inventory, matrix, fixtures, golden corpus, and release candidate evidence to share the same observed ref before any current-perfect claim is allowed.

## Upstream Distribution Target Gate

Task 21.1 adds `target/release-gates/upstream-distribution-target.json`. This artifact separates npm `promptfoo` core package metadata from GitHub repository HEAD and observed GitHub release tags. `npm_core_matches_frozen_baseline=true` means the published npm core package still matches the frozen baseline evidence; it does not mean repository HEAD or a non-core GitHub release is complete. `repository_head_matches_npm_core=false` or `github_latest_release_is_core_package=false` keeps `current_repository_perfect_claim_allowed=false` until a future rebaseline task produces same-ref inventory, matrix, fixture, golden corpus, and release candidate evidence.

Task 23.1 updates the same gate so `github.source` records a dynamic latest release ref resolved from GitHub latest release metadata rather than a hard-coded release tag. This dynamic latest release observation improves evidence freshness only; it does not change the frozen stable target, and it does not allow a promptfoo perfect-refactor claim while repository HEAD, latest release channel, source accounting, external authority, publication, or same-ref current rebaseline blockers remain unresolved.

## Current Latest Target Track

Task 24.1 records the immutable current-latest target packet at `compatibility/inventory/current-latest-target.json` and `docs/compatibility/current-latest.lock.md`. This packet distinguishes npm latest package evidence, GitHub default branch HEAD evidence, and GitHub latest release-channel evidence, and rejects floating values such as `latest`, `main`, `master`, or `HEAD` as completion proof.

Task 24.2 consumes that locked packet and writes current-latest inventory/matrix evidence at `compatibility/inventory/current-latest-source-inventory.json`, `compatibility/matrix/current-latest-matrix.json`, `target/release-gates/current-latest-source-inventory.json`, and `target/release-gates/current-latest-matrix.json`. These artifacts use the locked GitHub default branch SHA as the source reference. Unclassified rows or rows without evidence keep `perfect_refactor_claim_allowed=false`; this is an audit boundary, not a waiver or downgrade.

Task 24.3 consumes the current-latest matrix and writes `target/release-gates/current-latest-golden-corpus.json`. The corpus creates P0 fixture/artifact slots and P1/P2 evidence records against the locked target ref, then keeps P0 blocker or unclassified findings release-blocking. This expands test/evidence scale without converting mocked, recorded, or blocker evidence into live external provider authority.
