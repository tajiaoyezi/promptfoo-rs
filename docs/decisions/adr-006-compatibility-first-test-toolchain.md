# ADR-006: Compatibility first test toolchain

**Status**: Accepted
**Date**: 2026-05-30
**Category**: 测试工具链
**Decided By**: leafiellune
**Related**: docs/prds/promptfoo-rs.prd.md §Decisions Log D6

## Context

promptfoo-rs is a Rust reimplementation of promptfoo 0.121.13. The project must preserve compatibility while reducing default Node/npm dependency exposure and making release readiness auditable through S2V artifacts.

## Decision

兼容性测试优先于覆盖率数字。选择：fixture golden diff + schema snapshot + Rust unit/integration。

## Rationale

单元测试不能证明 promptfoo 行为兼容；手工对比不可审计、不可重复。

## Alternatives

只写 Rust 单元测试 / 手工对比。

## Consequences

- Task specs that touch this decision must list this ADR in §5.1 Required Reading.
- Compatibility fixtures and release gates must preserve this decision unless a superseding ADR is accepted.
- Any implementation conflict must be recorded as SPEC-DRIFT or a follow-up ADR before changing behavior.

## Rollback Or Migration Plan

Create a superseding ADR, update docs/s2v-adapter.md constraints or commands if affected, then update the compatibility matrix and dependent task specs before implementation continues.

## Follow-ups

- Keep this ADR aligned with docs/prds/promptfoo-rs.prd.md and docs/compatibility/matrix.md.