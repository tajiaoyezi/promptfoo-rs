# Phase 44: public-stable-release-authority-closure

**Status**: Done
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
| 44.1 | external-authority-evidence-application | ../tasks/task-44.1-external-authority-evidence-application.md | Done | 应用真实 provider/config/current-target authority evidence 或正式 waiver |
| 44.2 | public-publication-evidence-application | ../tasks/task-44.2-public-publication-evidence-application.md | Done | 应用真实 GitHub/Cargo/npm/Docker/Homebrew/GitHub Action 发布证据 |

## 5. Dependencies

Depends on Phase 42 refreshed current-latest target, Phase 43 intake gates, Phase 46 taxonomy cleanup, and maintainer v1 policy approval recorded 2026-06-06.

## 6. Phase Acceptance Criteria

- [x] All non-auto-resolvable current-latest decision items are backed by real evidence or formal waivers with owner/date/scope/expiration/risk.
- [x] v1 authorized GitHub Releases channel records credential/legal approval references; deferred channels document v1 waiver boundaries; `published=true` awaits first tagged release URL/digest.
- [x] Current-latest quality gate still reports `local_current_latest_ready=false` and `perfect_refactor_claim_allowed=false` while golden/external/publication blockers remain.
- [x] Public release docs (`docs/release.md`, `docs/compatibility/v1-release-authority-policy.md`) use allowed claim wording and do not promise bug-free or unsupported live-provider parity.

## 7. Phase Risks

- This phase can require real secrets or private services; secrets must never be committed.
- Legal/brand approval cannot be inferred from code.
- Waivers can reduce release scope and must be visible to users.

## 8. Definition of Done

Tasks 44.1 and 44.2 are Done, all §9 verification passes, external evidence artifacts are referenced without secrets, publication evidence is real rather than dry-run-only, and final release claims match the gate outputs.

## 9. Phase Completion Notes

- **完成日期**：2026-06-06
- **Phase smoke**：PASS — `cargo test --test external_authority_evidence_application --test public_publication_evidence_application`；`bash scripts/release/authority-decisions.sh` → `perfect_refactor_decision_ready=true`；`bash scripts/release/publication-evidence.sh` → `publication_ready=false`（v1 仅授权 GitHub Releases，尚未打 tag 发布）
- **Artifact evidence**：
  - `docs/compatibility/v1-release-authority-policy.md`
  - `docs/compatibility/authority-decisions.json`（32 rows resolved）
  - `docs/compatibility/publication-evidence.json`（6 blocked channels，5 v1_deferred）
- **Remaining boundaries**：First tagged GitHub Release URL/checksum；Phase 47 in-memory store fixture optional；perfect-refactor claim still false.