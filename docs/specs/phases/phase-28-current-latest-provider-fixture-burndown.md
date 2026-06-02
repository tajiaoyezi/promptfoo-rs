# Phase 28: current-latest-provider-fixture-burndown

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Burn down current-latest `provider` P0 golden blockers by applying the task 19.3/19.4 provider decision model to the locked current-latest inventory: provider files already covered by mock/recorded request-response fixtures become P0 native fixture evidence, and provider files requiring credentials, accounts, private services, SDK product authority, or live session authority remain explicit external blockers. 依据 PRD §Core Capabilities / §Compatibility Matrix、ADR-009、ADR-011、task 19.3、task 19.4。

## 2. Business Value

After Phase 27, release artifacts still report 38 current-latest provider blockers. Reusing existing OpenAI-compatible, HTTP, Ollama, and Anthropic fixtures prevents locally proven provider behavior from being double-counted as missing, while preserving external-authority provider gaps that cannot be solved safely by local code or mock data.

## 3. Scope / Modules

`src/compatibility/inventory.rs`, `scripts/release/current-latest-source-inventory.sh`, `tests/current_latest_provider_fixture_burndown.rs`, `target/release-gates/current-latest-source-inventory.json`, `target/release-gates/current-latest-matrix.json`, `target/release-gates/current-latest-golden-corpus.json`, `target/release-gates/current-latest-quality.json`, `docs/compatibility/matrix.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 28.1 | current-latest-provider-fixture-burndown | ../tasks/task-28.1-current-latest-provider-fixture-burndown.md | Ready | 将 current-latest 38 个 provider blockers 分解为 22 个 fixture-covered rows 和 16 个 explicit external authority blockers |

## 5. Dependencies

依赖 Phase 24 current-latest target artifacts、Phase 25 taxonomy burndown、Phase 27 core config burndown、task 19.3 provider fixture precedent、task 19.4 external authority gate。该 phase 不需要真实 provider credentials、私有服务账号、法律/品牌确认或 publication credentials；external rows 必须继续阻塞。

## 6. Phase Acceptance Criteria

- [ ] current-latest OpenAI-compatible、HTTP、Ollama、Anthropic mockable provider rows no longer appear as generic P0 provider blockers and carry `fixture:` evidence references.
- [ ] current-latest Codex、Agents、Assistant、Billing、ChatKit、Realtime、Claude Code auth provider rows remain explicit P0 external-authority blockers.
- [ ] `current-latest-golden-corpus.json` provider blocker count drops from 38 to 16, and total blocker count drops from 92 to 70.
- [ ] `current-latest-quality.json` still keeps `perfect_refactor_claim_allowed=false` while eval-runner, prompt-processing, cache-store, config, script-bridge, current-target, external-authority, and publication blockers remain.

## 7. Phase Risks

- Broadly marking all `src/providers/openai/**` or `src/providers/anthropic/**` native would hide SDK/account/product authority gaps. Classification must use an explicit provider fixture allowlist plus external-authority allowlist.
- Shell and Rust extractors can drift; both must emit identical metadata and evidence references.
- This phase reduces current-latest provider fixture blockers only; it does not prove live provider parity or remove publication/current-target blockers.

## 8. Definition of Done

Task 28.1 spec is Done, phase §6 smoke passes with `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`, provider fixture-covered rows stop generating P0 golden findings, provider external-authority rows remain visible, and perfect-refactor claim remains blocked by remaining evidence gaps.

## 9. Phase Completion Notes

- **完成日期**：待实施
- **Phase smoke**：待实施
- **Artifact evidence**：待实施
- **保留边界**：待实施
