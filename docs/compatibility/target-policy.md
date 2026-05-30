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
