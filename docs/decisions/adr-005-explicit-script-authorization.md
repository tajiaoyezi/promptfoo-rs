# ADR-005: Explicit script authorization

**Status**: Accepted
**Date**: 2026-05-30
**Category**: 安全
**Decided By**: leafiellune
**Related**: docs/prds/promptfoo-rs.prd.md §Decisions Log D5

## Context

promptfoo-rs is a Rust reimplementation of promptfoo 0.121.13. The project must preserve compatibility while reducing default Node/npm dependency exposure and making release readiness auditable through S2V artifacts.

## Decision

custom scripts 默认禁用，必须显式授权。选择：--allow-scripts 或配置开启，子进程隔离 env/stdio/timeout/redaction。

## Rationale

默认执行扩大任意代码执行和 CI secret 泄露面；完全移除会断现有生态。

## Alternatives

默认执行 / 完全移除脚本兼容。

## Consequences

- Task specs that touch this decision must list this ADR in §5.1 Required Reading.
- Compatibility fixtures and release gates must preserve this decision unless a superseding ADR is accepted.
- Any implementation conflict must be recorded as SPEC-DRIFT or a follow-up ADR before changing behavior.

## Rollback Or Migration Plan

Create a superseding ADR, update docs/s2v-adapter.md constraints or commands if affected, then update the compatibility matrix and dependent task specs before implementation continues.

## Follow-ups

- Keep this ADR aligned with docs/prds/promptfoo-rs.prd.md and docs/compatibility/matrix.md.