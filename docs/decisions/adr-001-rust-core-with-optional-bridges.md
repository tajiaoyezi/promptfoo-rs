# ADR-001: Rust core with optional bridges

**Status**: Accepted
**Date**: 2026-05-30
**Category**: 架构
**Decided By**: leafiellune
**Related**: docs/prds/promptfoo-rs.prd.md §Decisions Log D1

## Context

promptfoo-rs is a Rust reimplementation of promptfoo 0.121.13. The project must preserve compatibility while reducing default Node/npm dependency exposure and making release readiness auditable through S2V artifacts.

## Decision

默认执行路径采用 Rust core，脚本运行时只作为兼容桥。选择：模块化 Rust core + optional bridges。

## Rationale

纯 Rust 会断已有 custom provider/assertion；Node 主体无法解决单二进制和供应链目标。

## Alternatives

纯 Rust 无 bridge / Node 主体重写。

## Consequences

- Task specs that touch this decision must list this ADR in §5.1 Required Reading.
- Compatibility fixtures and release gates must preserve this decision unless a superseding ADR is accepted.
- Any implementation conflict must be recorded as SPEC-DRIFT or a follow-up ADR before changing behavior.

## Rollback Or Migration Plan

Create a superseding ADR, update docs/s2v-adapter.md constraints or commands if affected, then update the compatibility matrix and dependent task specs before implementation continues.

## Follow-ups

- Keep this ADR aligned with docs/prds/promptfoo-rs.prd.md and docs/compatibility/matrix.md.