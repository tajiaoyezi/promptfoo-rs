# Phase 43: authority-evidence-intake-gates

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Create local, auditable intake gates for the remaining 31 non-auto-resolvable decisions so future agents can apply real external authority evidence or formal waivers without weakening current-latest quality gates. 依据 PRD §Current Latest Rebaseline Addendum、ADR-008、ADR-009、ADR-011、task 19.4、task 22.1、task 37.1、task 41.1。

## 2. Business Value

The project cannot reach public stable / perfect-refactor status while real provider/config authority, current-target policy, and publication evidence are informal chat items. This phase turns the remaining decisions into validated artifacts with strict fail-closed semantics.

## 3. Scope / Modules

`target/release-gates/perfect-refactor-unblock-packet.json`, `target/release-gates/external-authority-blockers.json`, `target/release-gates/publication-authority.json`, new authority-decision evidence files under `docs/compatibility/`, optional validation scripts under `scripts/release/`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 43.1 | authority-decision-manifest-gate | ../tasks/task-43.1-authority-decision-manifest-gate.md | Ready | 建立 current-latest authority decision manifest schema 和校验 gate |
| 43.2 | publication-evidence-manifest-gate | ../tasks/task-43.2-publication-evidence-manifest-gate.md | Ready | 建立 publication URL/digest/credential/legal evidence manifest schema 和校验 gate |

## 5. Dependencies

Depends on task 37.1 current-latest unblock packet, task 18.4 publication authority gate, task 19.4 external authority blocker gate, and task 41.1 target drift refresh. This phase does not require real credentials because it only validates intake schema and fail-closed behavior.

## 6. Phase Acceptance Criteria

- [ ] A current-latest authority decision manifest schema exists and every unblock-packet decision item is either unresolved, backed by real evidence, or explicitly waived with owner/date/scope/risk.
- [ ] Publication evidence intake requires channel, credential authority, legal/brand approval, artifact URL, digest, and no-upload provenance before any channel can become `published=true`.
- [ ] Missing, partial, expired, or mock-only evidence keeps `perfect_refactor_claim_allowed=false`, `publication_ready=credential-blocked`, or equivalent blocked status.
- [ ] Runtime smoke or coverage gate consumes the new manifests without requiring real secrets in the repository.

## 7. Phase Risks

- A schema-only task can be misread as external approval; docs and gates must state that unresolved entries remain blockers.
- Evidence manifests must not store secrets; only references, approval IDs, URLs, digests, and redacted proof are allowed.
- Waivers can hide real gaps if they lack owner, scope, expiration, and release impact.

## 8. Definition of Done

Tasks 43.1 and 43.2 are Done, phase §6 smoke passes with task §9 verification, unresolved decision items remain blocked by default, and the repository is clean and pushed.

## 9. Phase Completion Notes

- **完成日期**：<TBD-after-impl>
- **Phase smoke**：<TBD-after-impl>
- **Artifact evidence**：<TBD-after-impl>
- **Remaining boundaries**：Real external authority, publication credentials, legal/brand approval, and public artifact evidence still require user or maintainer action before Phase 44 can be completed.
