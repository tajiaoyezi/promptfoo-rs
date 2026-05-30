# ADR-002: Stable Rust dependencies

**Status**: Accepted
**Date**: 2026-05-30
**Category**: 依赖
**Decided By**: leafiellune
**Related**: docs/prds/promptfoo-rs.prd.md §Decisions Log D2

## Context

promptfoo-rs is a Rust reimplementation of promptfoo 0.121.13. The project must preserve compatibility while reducing default Node/npm dependency exposure and making release readiness auditable through S2V artifacts.

## Decision

核心依赖采用 Rust 生态稳定库，避免自研通用基础设施。选择：Tokio、clap、serde、reqwest、axum、sqlx/libSQL、tracing。

## Rationale

自研基础设施会转移精力；复用 Node 包会保留 node_modules 和供应链问题。

## Alternatives

自研 async/runtime/HTTP/CLI / 继续复用 Node 包。

## Consequences

- Task specs that touch this decision must list this ADR in §5.1 Required Reading.
- Compatibility fixtures and release gates must preserve this decision unless a superseding ADR is accepted.
- Any implementation conflict must be recorded as SPEC-DRIFT or a follow-up ADR before changing behavior.

## Rollback Or Migration Plan

Create a superseding ADR, update docs/s2v-adapter.md constraints or commands if affected, then update the compatibility matrix and dependent task specs before implementation continues.

## Follow-ups

- Keep this ADR aligned with docs/prds/promptfoo-rs.prd.md and docs/compatibility/matrix.md.