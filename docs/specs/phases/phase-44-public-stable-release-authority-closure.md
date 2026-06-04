# Phase 44: public-stable-release-authority-closure

**Status**: Draft
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Resolve the real external authority and publication evidence needed before `promptfoo-rs` can be considered for a public stable release or complete current-latest replacement claim. 依据 PRD §Release constraints / §Current Latest Rebaseline Addendum、ADR-008、ADR-009、ADR-011、task 37.1、Phase 43 planned intake gates。

## 2. Business Value

This is the final non-local evidence boundary. The project can be internally verified without this phase, but cannot honestly publish as a public stable complete replacement until authority decisions and publication evidence are provided.

## 3. Scope / Modules

Authority decision manifests, publication evidence manifests, release gate artifacts, compatibility matrix status, release docs, and final public release notes. This phase may require external systems and credentials; it must not be attempted without explicit maintainer authorization.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 44.1 | external-authority-evidence-application | ../tasks/task-44.1-external-authority-evidence-application.md | Draft | 应用真实 provider/config/current-target authority evidence 或正式 waiver |
| 44.2 | public-publication-evidence-application | ../tasks/task-44.2-public-publication-evidence-application.md | Draft | 应用真实 GitHub/Cargo/npm/Docker/Homebrew/GitHub Action 发布证据 |

## 5. Dependencies

Depends on Phase 42 refreshed current-latest target, Phase 43 intake gates, and real user/maintainer evidence. This phase is intentionally Draft until credentials, publication authority, legal/brand approval, and product/service decisions are available.

## 6. Phase Acceptance Criteria

- [ ] All non-auto-resolvable current-latest decision items are backed by real evidence or formal waivers with owner/date/scope/expiration/risk.
- [ ] Publication channels intended for stable release have external URLs, digests/checksums, release notes, credential authority references, and legal/brand approval references.
- [ ] Current-latest quality gate reports `local_current_latest_ready=true` only if all source/matrix/golden/external/publication/current-target gates agree.
- [ ] Public release docs use only allowed claim wording and do not promise "no potential bugs" or complete live-provider parity without evidence.

## 7. Phase Risks

- This phase can require real secrets or private services; secrets must never be committed.
- Legal/brand approval cannot be inferred from code.
- Waivers can reduce release scope and must be visible to users.

## 8. Definition of Done

Tasks 44.1 and 44.2 are Done, all §9 verification passes, external evidence artifacts are referenced without secrets, publication evidence is real rather than dry-run-only, and final release claims match the gate outputs.

## 9. Phase Completion Notes

- **完成日期**：无（Draft，等待真实外部 evidence）
- **Phase smoke**：未执行（等待 Phase 43 intake gates 和用户/maintainer evidence）
- **Artifact evidence**：无（Draft）
- **Remaining boundaries**：真实凭据、账号权限、私有服务、法律/品牌确认、发布授权、外部 URL/digest 证据。
