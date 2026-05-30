# ADR-008: Binary first multi channel release

**Status**: Accepted
**Date**: 2026-05-30
**Category**: 部署发布
**Decided By**: leafiellune
**Related**: docs/prds/promptfoo-rs.prd.md §Decisions Log D8

## Context

promptfoo-rs is a Rust reimplementation of promptfoo 0.121.13. The project must preserve compatibility while reducing default Node/npm dependency exposure and making release readiness auditable through S2V artifacts.

## Decision

二进制是一等产物，npm wrapper 是兼容和分发补充。选择：GitHub Releases/Homebrew/Cargo/Docker/npm wrapper/GitHub Action。

## Rationale

只发 npm 不能解决 Node 依赖痛点；只发 Cargo 对非 Rust 用户和 CI 不友好。

## Alternatives

只发布 npm / 只发布 Cargo。

## Consequences

- Task specs that touch this decision must list this ADR in §5.1 Required Reading.
- Compatibility fixtures and release gates must preserve this decision unless a superseding ADR is accepted.
- Any implementation conflict must be recorded as SPEC-DRIFT or a follow-up ADR before changing behavior.

## Rollback Or Migration Plan

Create a superseding ADR, update docs/s2v-adapter.md constraints or commands if affected, then update the compatibility matrix and dependent task specs before implementation continues.

## Follow-ups

- Keep this ADR aligned with docs/prds/promptfoo-rs.prd.md and docs/compatibility/matrix.md.