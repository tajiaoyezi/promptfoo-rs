# ADR-011: Current Latest Full Refactor Target

**Status**: Accepted (product roadmap superseded by ADR-012 on 2026-06-07)
**Date**: 2026-06-01
**Category**: Compatibility / Release Quality
**Decided By**: leafiellune
**Related**: docs/prds/promptfoo-rs.prd.md §Current Latest Rebaseline Addendum

## Context

The original PRD froze `promptfoo@0.121.13` / `4860e990c7e9a2f8f677173fb92cf9867b34d03f` as the stable 1.0 compatibility baseline. Later release gates correctly blocked any "perfect refactor" claim because GitHub repository HEAD, GitHub latest release, npm latest package, source inventory, publication authority, and external provider authority did not all point to the same complete target.

On 2026-06-01, the user clarified the intended "perfect refactor" target: rebuild the complete functionality of the original promptfoo project at its current latest version, with a large verification suite intended to eliminate known defects. Fresh public observations at decision time are:

- npm `promptfoo@latest`: `0.121.13`, gitHead `4860e990c7e9a2f8f677173fb92cf9867b34d03f`
- GitHub default branch HEAD: `1d09dfeb5f0766905409117f923dd5c4b0838d9f`
- GitHub latest release: `code-scan-action-0.1.7`, target commit `1c743afe0e4807882e858c4f322fc064fa5f0770`

## Decision

Promptfoo-rs will add a current-latest rebaseline track. The current-latest track treats the upstream target as an immutable observation packet captured at S2V task runtime, not as the floating words `latest`, `main`, `master`, or `HEAD`.

The track must distinguish three upstream facts:

1. npm latest stable package evidence.
2. GitHub default branch source HEAD evidence.
3. GitHub latest release channel evidence.

A current-latest perfect-refactor claim is only allowed when source inventory, compatibility matrix, fixture corpus, golden diff artifacts, release gates, publication evidence, and external authority evidence all refer to the same locked target packet or explicitly classified channel boundary.

The project must not claim "no potential bugs". The only permitted claim is "no known release-blocking defects under the declared verification gates", because unbounded absence of all possible bugs is not empirically provable. The verification suite must still be intentionally large: full P0 golden diff coverage, P1 snapshots, fixture coverage for source inventory rows, stress/regression/property tests for deterministic core behavior, and explicit live/recorded evidence boundaries for external providers.

## Rationale

Floating latest references are not auditable and conflict with ADR-007 / ADR-009. The user's clarified goal resolves the previous "which upstream target?" decision, but it does not remove the need for immutable evidence. Separating npm latest, GitHub HEAD, and GitHub release channel prevents a non-core release or an unreleased commit from being misrepresented as a stable package release.

## Alternatives

- Keep only the frozen `0.121.13` target. Rejected because it does not match the clarified goal.
- Treat GitHub HEAD directly as "latest" without lock artifacts. Rejected because it is non-reproducible and would violate the existing floating-reference safeguards.
- Promise zero bugs. Rejected because no finite test suite can prove the absence of all possible defects.

## Consequences

- New S2V tasks must use a locked current-latest observation packet.
- Existing frozen-baseline evidence remains valid for local stable compatibility, but it is not enough for the clarified perfect-refactor goal.
- Release gates must keep external authority, publication authority, and live-provider evidence explicit.
- Task specs touching current-latest claims must list this ADR in §5.1 Required Reading.

## Rollback Or Migration Plan

If the target needs to return to frozen-only compatibility, add a superseding ADR and update the PRD, adapter indexes, current-latest phase specs, and claim contract before changing release gate behavior.

## Amendment (2026-06-07)

ADR-012 supersedes the **ongoing drift-refresh roadmap** implied by this ADR. Phase 48 (`promptfoo@0.121.15`) is the final product compatibility baseline; `current-latest-*` gates remain as frozen evidence machinery. See docs/prds/promptfoo-rs.prd.md §Product Independence Strategy.

## Follow-ups

- Phase 24 implements the current-latest target lock, source inventory re-extraction, golden corpus expansion, and exhaustive quality gate.
