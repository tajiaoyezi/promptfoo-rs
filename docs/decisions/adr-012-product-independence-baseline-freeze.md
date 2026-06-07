# ADR-012: Product Independence and Compatibility Baseline Freeze

**Status**: Accepted
**Date**: 2026-06-07
**Category**: 兼容性
**Decided By**: leafiellune
**Related**: docs/prds/promptfoo-rs.prd.md §Product Independence Strategy, ADR-007, ADR-009, ADR-011 (superseded for product roadmap), Phase 48 task 48.1

## Context

promptfoo-rs began as a Rust reimplementation of promptfoo. Between 2026-06-01 and 2026-06-07, S2V phases 24–48 added a **current-latest rebaseline track** (ADR-011): whenever npm latest or GitHub default branch HEAD moved, a new target-refresh phase was planned to re-lock downstream compatibility evidence.

On 2026-06-07, the maintainer clarified product strategy:

1. promptfoo-rs is a **one-time** Rust reimplementation anchored to a **fixed** promptfoo snapshot at kickoff, then an **independent product line**.
2. The project **does not** continuously align with promptfoo releases, npm `latest`, or GitHub HEAD after that snapshot is frozen.
3. Waiting for upstream drift (e.g. a hypothetical Phase 49) is **not** part of the product roadmap.

Phase 48 (task 48.1) already captured the final upstream observation packet:

| Field | Value |
|---|---|
| npm package | `promptfoo@0.121.15` |
| npm gitHead | `4805856060d026521794d4e69decb938155580ad` |
| GitHub latest release | `refs/tags/0.121.15` / same commit |
| GitHub default branch HEAD | `c54a30668ad8319d76c20ae96e6680ad6c51a2c6` |
| Lock artifacts | `docs/compatibility/current-latest.lock.md`, `compatibility/inventory/current-latest-target.json` |

The earlier Phase 1 frozen baseline (`promptfoo@0.121.13` / `4860e990c7e9a2f8f677173fb92cf9867b34d03f`) remains valid historical harness evidence. The **product compatibility baseline** for all future roadmap work is the Phase 48 packet above.

## Decision

1. **Freeze the product compatibility baseline** at the Phase 48 observation packet (`promptfoo@0.121.15`). No scheduled or default S2V phases will refresh this target when upstream moves.
2. **Declare product independence**: promptfoo-rs is not a living fork that tracks promptfoo HEAD. Compatibility work measures progress against the frozen baseline and promptfoo-rs's own roadmap—not live upstream.
3. **Retire the drift-refresh backlog**: Phases 38–48 historical target-refresh work stays as completed audit history. **Phase 49 and later upstream drift-refresh phases are out of scope** unless a future ADR explicitly reopens upstream tracking.
4. **Preserve existing gate machinery**: `current-latest-*` artifacts, scripts, and tests remain as **frozen-baseline evidence generators**. The technical status `locked-with-drift` (npm release ref ≠ GitHub HEAD) is an observation fact at freeze time, not a signal to schedule another refresh.
5. **Public wording**: README, PRD, and release docs must state that the project does **not** claim to track or replace promptfoo latest releases.

## Rationale

Continuous upstream tracking turned the roadmap into an open-ended chase of npm and GitHub HEAD. That conflicts with the stated goal: ship an independent Rust toolchain with auditable compatibility boundaries at a known snapshot. Freezing at Phase 48 keeps the richest locked evidence already paid for (source inventory, matrix, golden corpus, quality gates) without binding future releases to upstream cadence.

## Alternatives

- **Keep ADR-011 drift-refresh cadence** (new phase whenever npm/HEAD moves). Rejected: contradicts independent product strategy and blocks a finite roadmap.
- **Revert to Phase 1 baseline only (`0.121.13`)**. Rejected: discards Phase 24–48 evidence; Phase 48 packet is the authoritative kickoff snapshot the team already validated.
- **Delete current-latest gates entirely**. Rejected in this ADR: large test and release-gate surface; frozen artifacts remain useful for burndown without live observation.

## Consequences

- PRD gains §Product Independence Strategy; §Current Latest Rebaseline Addendum is marked superseded for roadmap planning (historical phases remain documented).
- Adapter and README stop implying "must refresh when upstream drifts."
- Next product work: golden/fixture burndown, publication channels, and promptfoo-rs features—**not** waiting for `promptfoo@0.121.16+`.
- ADR-011 rollback path ("return to frozen-only") is satisfied by this ADR for product strategy; gate rename/refactor may follow in a separate task if desired.

## Rollback Or Migration Plan

Reopen upstream tracking only via a superseding ADR that updates PRD, adapter indexes, and explicitly schedules target-refresh tasks. Do not silently resume drift phases.

## Follow-ups

- Optional later task: rename `current-latest` gate labels to `product-baseline` in code/tests for clarity (cosmetic; not required for strategy alignment).
- Continue burndown on frozen baseline blockers (`perfect_refactor_claim_allowed=false`, publication authority) per existing phase 18–22 / 43–44 scope.