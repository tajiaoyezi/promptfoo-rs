# promptfoo-rs Baseline Lock

**Status**: Ready
**Baseline candidate**: promptfoo 0.121.13 + commit 4860e99

Readiness evidence captured on 2026-05-30 using public upstream sources. Task 1.1 still owns executable validation and §10 completion notes before Phase 1 can be Done.

| Artifact | Expected | Evidence | Status |
|---|---|---|---|
| Git tag | refs/tags/0.121.13 | `git ls-remote --tags https://github.com/promptfoo/promptfoo.git refs/tags/0.121.13*` -> `4860e990c7e9a2f8f677173fb92cf9867b34d03f refs/tags/0.121.13` | Verified |
| Git commit | 4860e990c7e9a2f8f677173fb92cf9867b34d03f | Same `git ls-remote` output confirms the tag points at the frozen commit. | Verified |
| npm artifact | promptfoo@0.121.13 | `npm view promptfoo@0.121.13 version gitHead dist.tarball dist.integrity dist.shasum --json` -> tarball `https://registry.npmjs.org/promptfoo/-/promptfoo-0.121.13.tgz`, integrity `sha512-DBPSixUophzcD7S7lML6SqVwnVtrhK5A3HsZ03IG9Xrw0t24r5imG7nLj+YMb0vlAjbdFtE7yFG+rsqDpfYp6g==`, shasum `564f31e1d7f6f1e5918ec55681167f8ae7eec726`, gitHead `4860e990c7e9a2f8f677173fb92cf9867b34d03f`. | Verified |
| container artifact | ghcr.io/promptfoo/promptfoo:0.121.13 | GHCR registry manifest `docker-content-digest` for tag `0.121.13`: `sha256:3993e7c105bcbc1c8f763309552728dd2bf30ff5c9c2e14ec69297b42d096f80`; linux/amd64 manifest `sha256:58ff1ec5b5b2463e4782ac8f038c75fc9504412028f3261333ce21cd42a5eddf`; linux/arm64 manifest `sha256:c04273f2b074f46c883abd319226219e4cefb281c913282d2dc91ebb68b68819`. Commands: GHCR token + manifest request, `docker manifest inspect ghcr.io/promptfoo/promptfoo:0.121.13`. | Verified |

## Release Gate Interpretation

- If any artifact above cannot be reproduced by task-1.1 validation, the release gate status is `blocked`.
- This lock intentionally avoids floating references; use `0.121.13`, the full Git SHA, npm integrity, and OCI digest only.
- Basis: PRD §Upstream Baseline Freeze Strategy, ADR-006, ADR-007, ADR-009.
