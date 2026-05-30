# ADR-010: Node API wrapper contract boundary

**Status**: Accepted
**Date**: 2026-05-30
**Category**: 协议接口
**Decided By**: leafiellune
**Related**: docs/prds/promptfoo-rs.prd.md §Decisions Log D10

## Context

promptfoo-rs is a Rust reimplementation of promptfoo 0.121.13. The project must preserve compatibility while reducing default Node/npm dependency exposure and making release readiness auditable through S2V artifacts.

## Decision

Node API wrapper 通过稳定 JSON-RPC/stdio 或 FFI 边界调用 Rust core。选择：wrapper contract tests 固定 API、参数、错误和结果 schema。

## Rationale

复写业务逻辑会产生 wrapper/core 漂移；只暴露 CLI 会破坏 programmatic usage 体验。

## Alternatives

JS wrapper 复写业务逻辑 / 只暴露 CLI subprocess。

## Consequences

- Task specs that touch this decision must list this ADR in §5.1 Required Reading.
- Compatibility fixtures and release gates must preserve this decision unless a superseding ADR is accepted.
- Any implementation conflict must be recorded as SPEC-DRIFT or a follow-up ADR before changing behavior.

## Rollback Or Migration Plan

Create a superseding ADR, update docs/s2v-adapter.md constraints or commands if affected, then update the compatibility matrix and dependent task specs before implementation continues.

## Follow-ups

- Keep this ADR aligned with docs/prds/promptfoo-rs.prd.md and docs/compatibility/matrix.md.