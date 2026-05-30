# ADR-004: CLI and output schema protocol

**Status**: Accepted
**Date**: 2026-05-30
**Category**: 协议接口
**Decided By**: leafiellune
**Related**: docs/prds/promptfoo-rs.prd.md §Decisions Log D4

## Context

promptfoo-rs is a Rust reimplementation of promptfoo 0.121.13. The project must preserve compatibility while reducing default Node/npm dependency exposure and making release readiness auditable through S2V artifacts.

## Decision

CLI exit code、stdout/stderr、JSON/JUnit/SARIF 输出 schema 作为稳定兼容协议。选择：将 CLI 与输出 schema 纳入 P0/P1 snapshot 和 golden diff。

## Rationale

现有用户依赖 CI、JUnit、SARIF 和脚本消费；字段漂移会直接破坏迁移。

## Alternatives

只保证人类可读输出 / 输出字段随实现漂移。

## Consequences

- Task specs that touch this decision must list this ADR in §5.1 Required Reading.
- Compatibility fixtures and release gates must preserve this decision unless a superseding ADR is accepted.
- Any implementation conflict must be recorded as SPEC-DRIFT or a follow-up ADR before changing behavior.

## Rollback Or Migration Plan

Create a superseding ADR, update docs/s2v-adapter.md constraints or commands if affected, then update the compatibility matrix and dependent task specs before implementation continues.

## Follow-ups

- Keep this ADR aligned with docs/prds/promptfoo-rs.prd.md and docs/compatibility/matrix.md.