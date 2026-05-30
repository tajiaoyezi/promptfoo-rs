# ADR-007: Upstream golden diff release gate

**Status**: Accepted
**Date**: 2026-05-30
**Category**: 兼容性
**Decided By**: leafiellune
**Related**: docs/prds/promptfoo-rs.prd.md §Decisions Log D7

## Context

promptfoo-rs is a Rust reimplementation of promptfoo 0.121.13. The project must preserve compatibility while reducing default Node/npm dependency exposure and making release readiness auditable through S2V artifacts.

## Decision

upstream golden diff 是 1.0 stable release gate。选择：P0 golden diff 不通过不得发布 stable。

## Rationale

兼容是 1.0 的核心承诺；非阻断 diff 会让用户在 CI 中遇到不可预测迁移失败。

## Alternatives

把 diff 当非阻断报告 / 只在 nightly 跑。

## Consequences

- Task specs that touch this decision must list this ADR in §5.1 Required Reading.
- Compatibility fixtures and release gates must preserve this decision unless a superseding ADR is accepted.
- Any implementation conflict must be recorded as SPEC-DRIFT or a follow-up ADR before changing behavior.

## Rollback Or Migration Plan

Create a superseding ADR, update docs/s2v-adapter.md constraints or commands if affected, then update the compatibility matrix and dependent task specs before implementation continues.

## Follow-ups

- Keep this ADR aligned with docs/prds/promptfoo-rs.prd.md and docs/compatibility/matrix.md.