# ADR-003: Streaming JSONL and SQLite store

**Status**: Accepted
**Date**: 2026-05-30
**Category**: 数据持久化
**Decided By**: leafiellune
**Related**: docs/prds/promptfoo-rs.prd.md §Decisions Log D3

## Context

promptfoo-rs is a Rust reimplementation of promptfoo 0.121.13. The project must preserve compatibility while reducing default Node/npm dependency exposure and making release readiness auditable through S2V artifacts.

## Decision

大型结果采用流式 JSONL 与 SQLite/libSQL 存储。选择：JSONL append + SQLite/libSQL query store。

## Rationale

单 JSON 和内存方案无法支撑大型 eval 与 resume；服务端数据库破坏 local-first 分发。

## Alternatives

单 JSON 文件 / 只存内存 / 专用服务端数据库。

## Consequences

- Task specs that touch this decision must list this ADR in §5.1 Required Reading.
- Compatibility fixtures and release gates must preserve this decision unless a superseding ADR is accepted.
- Any implementation conflict must be recorded as SPEC-DRIFT or a follow-up ADR before changing behavior.

## Rollback Or Migration Plan

Create a superseding ADR, update docs/s2v-adapter.md constraints or commands if affected, then update the compatibility matrix and dependent task specs before implementation continues.

## Follow-ups

- Keep this ADR aligned with docs/prds/promptfoo-rs.prd.md and docs/compatibility/matrix.md.