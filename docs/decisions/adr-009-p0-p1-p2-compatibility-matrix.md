# ADR-009: P0 P1 P2 compatibility matrix

**Status**: Accepted
**Date**: 2026-05-30
**Category**: 兼容性
**Decided By**: leafiellune
**Related**: docs/prds/promptfoo-rs.prd.md §Decisions Log D9

## Context

promptfoo-rs is a Rust reimplementation of promptfoo 0.121.13. The project must preserve compatibility while reducing default Node/npm dependency exposure and making release readiness auditable through S2V artifacts.

## Decision

1.0 兼容目标覆盖全部已文档化能力域，但按 P0/P1/P2 分级验收。选择：全量登记 + 分级 release gate。

## Rationale

只登记已实现项会沉默遗漏；全部 Rust native 会把长尾 provider 范围拖垮。

## Alternatives

只登记已实现项 / 承诺全部 Rust native。

## Consequences

- Task specs that touch this decision must list this ADR in §5.1 Required Reading.
- Compatibility fixtures and release gates must preserve this decision unless a superseding ADR is accepted.
- Any implementation conflict must be recorded as SPEC-DRIFT or a follow-up ADR before changing behavior.

## Rollback Or Migration Plan

Create a superseding ADR, update docs/s2v-adapter.md constraints or commands if affected, then update the compatibility matrix and dependent task specs before implementation continues.

## Follow-ups

- Keep this ADR aligned with docs/prds/promptfoo-rs.prd.md and docs/compatibility/matrix.md.