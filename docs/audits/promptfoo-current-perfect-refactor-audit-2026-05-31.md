# promptfoo current perfect-refactor audit - 2026-05-31

**Status**: Current audit verdict after Phase 18 completion
**Objective**: determine whether the current `promptfoo-rs` worktree completely satisfies a perfect refactor of `promptfoo/promptfoo`.
**Verdict**: Not fully satisfied.

## Scope

This audit supersedes the 2026-05-30 negative audit and the earlier 2026-05-31 pre-Phase-17 audit for current-state purposes.

The compatibility target in project specs is the frozen upstream baseline:

- npm: `promptfoo@0.121.13`
- git tag: `refs/tags/0.121.13`
- commit: `4860e990c7e9a2f8f677173fb92cf9867b34d03f`

Fresh external checks during this audit:

- `npm view promptfoo version gitHead dist.tarball dist.integrity time.modified --json` reports `version=0.121.13`, `gitHead=4860e990c7e9a2f8f677173fb92cf9867b34d03f`, and npm modified time `2026-05-28T23:59:40.582Z`.
- `git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.13 refs/tags/code-scan-action-0.1.7` reports GitHub `HEAD=ff8eafd743cf6d63dd85b790ad8a4c73ede5828d`, tag `0.121.13=4860e990c7e9a2f8f677173fb92cf9867b34d03f`, and tag `code-scan-action-0.1.7=1c743afe0e4807882e858c4f322fc064fa5f0770`.
- The GitHub releases page currently marks `code-scan-action: 0.1.7` as the latest release dated 2026-05-29, after `0.121.13` dated 2026-05-28.

Therefore any stable compatibility claim must remain scoped to the frozen npm baseline unless a future rebaseline task explicitly changes the target.

## Current Verification Evidence

Full local S2V verification was rerun on 2026-05-31 with Git for Windows Bash:

```bash
source docs/s2v/scripts/lib/preflight.sh
source docs/s2v/scripts/lib/verify.sh
s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"
```

Observed result:

- `install`: PASS
- `lint`: PASS
- `typecheck`: PASS
- `unit-test`: PASS
- `integration`: PASS
- `e2e`: PASS
- `coverage`: PASS
- `build`: PASS
- `runtime-smoke`: PASS
- helper summary: `§9 Verification 全套通过（共 9 项）`

Additional current-state checks:

- `git status --short --branch`: clean `master...origin/master` before this audit edit.
- `git rev-parse HEAD origin/master`: both refs were `81a9060c274bd2692c4a7f321f4f516af733b7f8`.
- Task status scan: 42 task specs are `Done`.
- Phase status scan: 17 phase specs are `Done`.
- Project specs scan: no real `<TBD-by-user>` remains in `docs/specs/tasks`, `docs/specs/phases`, `docs/decisions`, or `docs/compatibility`.
- `s2v_preflight_phase` passes for every phase spec.

This proves the current local gate chain is executable and green. It does not by itself prove a complete or perfect refactor of the full upstream repository.

## Improvements Since The Previous Audit

The project is materially stronger than the earlier audit snapshots:

- All phase and task specs are `Done` through Phase 18.
- Adapter verification commands are executable; the full 9-key local gate runs.
- `viewer/` and `npm/` include package metadata, lockfiles, build/test scripts, and smoke scripts.
- Phase 16 removed the previous no-op CLI behavior for `view`, `cache`, `import`, and `export`.
- Runtime smoke produces measured `performance.json`, `security.json`, and `release-candidate.json`.
- Phase 17 added source-tree extraction evidence, CLI/flag closure evidence, a 50-fixture real upstream P0 corpus, long-tail classification, and release installability evidence.
- Phase 18.2 added provider-module burndown evidence: 37 P0 provider module blockers are now split into 13 existing P0 fixture-covered rows and 24 explicit remaining blockers with item-level reasons.
- Phase 18.3 added a current-upstream target-mode gate: `current-upstream-policy.json` and `release-candidate.json.target_policy` now make frozen/current claim scope machine-readable.
- Phase 18.4 added publication authority evidence: `publication-authority.json` and `release-candidate.json.publication_authority` now separate local installability, credential blockers, legal/brand blockers, and actual published evidence.
- Phase 18 smoke passed `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"` after rustfmt drift was corrected.

These are real progress items. They support a scoped frozen-baseline compatibility implementation, not a literal perfect refactor claim.

## Blocking Findings

### P0 - Scope is frozen baseline, not current upstream repository

The project specs intentionally target `promptfoo@0.121.13` and commit `4860e990c7e9a2f8f677173fb92cf9867b34d03f`. Current GitHub `HEAD` is `ff8eafd743cf6d63dd85b790ad8a4c73ede5828d`, and GitHub releases show a later `code-scan-action: 0.1.7` release after `0.121.13`.

Verdict: the project can claim only frozen-baseline compatibility. It cannot honestly claim to be a complete refactor of current `promptfoo/promptfoo`.

Task 18.3 now makes this boundary enforceable in release evidence:

- `target/release-gates/current-upstream-policy.json` reports `target_mode=frozen`, `status=ready`, `current_perfect_claim_allowed=false`, frozen git commit `4860e990c7e9a2f8f677173fb92cf9867b34d03f`, and observed current HEAD `ff8eafd743cf6d63dd85b790ad8a4c73ede5828d`.
- `target/release-gates/release-candidate.json.target_policy` repeats the target mode, frozen commit, current HEAD, and `current_perfect_claim_allowed=false`.

This does not complete current-upstream parity; it prevents ambiguous or overstated “perfect refactor” claims.

### P0 - Source inventory ledger closes silent missing rows, but P0 accounting blockers remain

`target/release-gates/source-inventory-evidence.json` reports:

- `schema`: `promptfoo-rs.source-inventory-evidence.v2`
- `status`: `ready-with-blockers`
- `source_extracted_item_count`: 2549
- `missing_matrix_rows`: 0
- `release_blockers`: 74
- `source_accounting_ledger`: `target/release-gates/source-inventory-ledger.json`
- `p0_accounting_blocker_count`: 111

`target/release-gates/source-inventory-ledger.json` reports 2549 ledger rows and 2116 generated accounting rows. This removes the previous silent missing-row problem, but it still directly contradicts a "complete upstream surface implemented" claim because generated P0 accounting rows remain release-blocking until they get native fixture, bridge fixture, explicit waiver, or external blocker evidence.

### P0 - Long-tail classification still has P0 release blockers

`target/release-gates/longtail-classification.json` reports:

- `schema`: `promptfoo-rs.longtail-classification.v1`
- `status`: `ready-with-blockers`
- `source_extracted_item_count`: 433
- `tracked_longtail_item_count`: 433
- `p0_provider_module_burndown.initial_blocker_count`: 37
- `p0_provider_module_burndown.resolved_by_fixture_count`: 13
- `p0_provider_module_burndown.remaining_blocker_count`: 24
- `p0_release_blocker_count`: 24
- `unresolved_rows`: 0
- `missing_reason_rows`: 0
- `p0_release_blockers[]`: lists every remaining provider module blocker by item id, source reference, reason, verification, and external-authority flag

This is better auditability than Phase 17: existing OpenAI/Anthropic/HTTP/Ollama P0 fixtures now cover 13 source provider modules, while Codex, Claude Code auth, billing, ChatKit, Agents, Realtime, Assistant, and endpoint-specific unfixtureed modules remain explicit blockers. It is still not perfect/native parity because 24 P0 provider module blockers remain release-blocking.

### P1 - Compatibility matrix intentionally contains later, unsupported, and partial-parity rows

The project correctly registers non-goals and deferred areas, including:

- `Other documented providers`: P1/P2 with `native/bridge/later`.
- `Python custom provider/assertion`: Python runtime discovery fixture remains follow-up.
- `Shell/Ruby custom scripts`: Ruby depends on upstream documentation inventory.
- `Local Web viewer`: P1 data-contract parity; pixel-level upstream UI parity is out of scope.
- `MCP provider / promptfoo mcp`: P1 until protocol coverage is complete.
- `code-scans / scan-model / model-audit`: false-positive rate is a known limitation, not a 1.0 gate.
- `Node API wrapper`: npm package scaffold depends on a Corepack-enabled packaging environment.
- `promptfoo cloud/share`: P2 unsupported/later; brand/legal copy needs review before public release.

This is acceptable for the PRD's P0/P1/P2 compatibility policy. It contradicts a literal "perfect refactor" claim unless the phrase is redefined as "complete according to the frozen PRD compatibility policy with documented P2 gaps."

### P1 - Public release is still credential-blocked and unpublished

`target/release-gates/installability.json`, `target/release-gates/publication-authority.json`, and `target/release-gates/release-candidate.json` report:

- `installability_ready`: true
- `publication_ready`: `credential-blocked`
- `credential_blocked`: true
- publication authority `legal_brand_blocked`: true
- publication authority `blockers`: 6
- release-candidate `published`: false
- channel-level `published`: false for GitHub Releases, Cargo, npm wrapper, Docker, Homebrew, and GitHub Action
- channel-level `published_evidence`: null for every public release channel
- channel-level `installability_status` is recorded separately from `authority_status`
- Homebrew status: `tool-unavailable`, blocker `Homebrew CLI unavailable; tap publication requires credentials`

Local dry-run installability and publication authority blockers are proven. Real multi-channel publication is not proven and cannot be claimed without credentials, legal/brand approval, release authority, and external URL/digest evidence.

## Requirement Verdict

| Requirement | Current evidence | Verdict |
|---|---|---|
| All local S2V phase/task specs complete | 42 task specs `Done`; 17 phase specs `Done`; phase preflight passes | Met |
| Local verification gates executable and green | Full 9-key S2V verification passed on 2026-05-31 | Met |
| Frozen baseline traceable | npm version/gitHead/integrity, git tag, baseline lock, and artifacts point to `0.121.13` / `4860e99` | Met for frozen target |
| Target-mode claim separation | `current-upstream-policy.json` and `release-candidate.json.target_policy` record frozen mode and `current_perfect_claim_allowed=false` | Met |
| Current upstream repository parity | GitHub `HEAD=ff8eafd...` differs from frozen tag; later `code-scan-action: 0.1.7` release exists; current mode lacks same-ref inventory/matrix/fixtures/golden/release evidence | Not met |
| 100% source-extracted upstream item accounting | 2549 source-extracted items now have 2549 ledger rows and missing matrix rows are 0 | Met as accounting, not implementation parity |
| P0 real upstream golden diff corpus | 50 real upstream P0 fixtures are recorded and smoke metadata is ready | Met for recorded corpus |
| Provider/assertion/redteam long-tail parity | 433 tracked long-tail rows; provider module burndown resolves 13 of 37 via fixture evidence, but 24 explicit P0 provider module release blockers remain | Not met |
| Compatibility matrix honesty | P1/P2/later/unsupported rows are explicit and reasoned | Met as auditability, not perfect parity |
| Viewer/npm package local build smoke | Viewer and npm package smoke scripts pass in full local gate | Met for local smoke |
| Multi-channel public release | Dry-run artifacts and publication authority blockers exist, but all channels are unpublished, publication evidence is null, and publication is credential-blocked/legal-brand-blocked | Not proven |

## Conclusion

The current project is a substantially improved, locally verified, S2V-complete Rust compatibility implementation for the frozen `promptfoo@0.121.13` baseline. It is not yet a complete or perfect refactor of `promptfoo/promptfoo`.

The remaining gap is no longer basic project readiness. It is deep parity and release proof:

1. resolve the 111 P0 source accounting blockers now exposed by the ledger,
2. resolve the 24 remaining P0 long-tail provider module blockers,
3. decide whether to implement and verify current-mode evidence for current upstream `HEAD`,
4. either implement or formally scope out the current upstream surfaces that are outside the frozen PRD target,
5. complete real publication evidence for the intended release channels once credentials and release authority exist.

Until those are resolved, the honest project claim is "auditable frozen-baseline compatibility implementation with explicit blockers," not "perfect refactor."

## Recommended Next S2V Work

Add follow-up S2V work focused on blocker burn-down rather than smoke hardening:

- source inventory P0 accounting blocker burn-down from the new ledger;
- fixture or waiver decisions for the 24 remaining P0 long-tail provider module blockers;
- explicit rebaseline ADR if the target changes from `0.121.13` to current upstream `HEAD`;
- real publication evidence for GitHub Releases, Cargo, npm, Docker, Homebrew, and GitHub Action after credentials, authority, and legal/brand approval exist;
- compatibility matrix update that separates "frozen-baseline complete" from "current-upstream complete."

Phase 19 now captures the first two follow-up tracks as Ready S2V task specs: viewer config source reclassification, core config fixture burndown, provider request/response fixture burndown, and external authority blocker gate.

Task 19.1 implementation adds viewer config scope correction for `src/app/**` generated config rows. This is not a claim of complete upstream React UI parity; it keeps Local Web viewer under the existing P1 boundary and leaves non-app runtime config rows as P0 blockers for task 19.2.
