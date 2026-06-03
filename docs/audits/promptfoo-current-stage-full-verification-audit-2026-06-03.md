# promptfoo-rs current-stage full verification audit - 2026-06-03

**Status**: Current-stage audit after Phase 41 completion.
**Objective**: Review all completed work at the current stage, including full functional verification.
**Verdict**: Local completed-task verification passes; full current-latest perfect-refactor parity is not proven.

## Scope

This audit uses the current worktree and fresh external target checks as authority.

Current S2V inventory:

- Phase specs: 41, all `Done`.
- Task specs: 76, all `Done`.
- BDD feature files: 16.
- ADRs: 11.
- Tracked source/spec/compatibility files in reviewed areas: 382.
- Phase preflight: 41/41 phase specs passed `s2v_preflight_phase`.
- Source/test area tracking guard: passed for `src crates tests viewer npm compatibility`.

No task spec is currently `Draft`, `Ready`, or `In Progress`.

## Verification Commands

All S2V helper commands were run through Git for Windows Bash:

```bash
source docs/s2v/scripts/lib/preflight.sh
source docs/s2v/scripts/lib/verify.sh
s2v_baseline_green "src/ crates/ tests/ viewer/ npm/ compatibility/"
s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"
for f in docs/specs/tasks/*.md; do s2v_coverage_threshold_guard "$f"; done
```

Fresh verification evidence:

- Baseline green: PASS.
- Full verification: PASS.
- Full verification log: `target/audit/2026-06-03-full-verification.log`.
- `s2v_verify_full_rc=0`.
- Observed verification keys: `install`, `lint`, `typecheck`, `unit-test`, `integration`, `e2e`, `coverage`, `build`, `runtime-smoke`.
- Coverage threshold guard: 76/76 task specs checked.
- Cargo result-line pass count parsed from the full verification log: 398 passed tests, 0 failed in the parsed `test result: ok` lines.
- Audit window: `2026-06-03T22:07:41+08:00` to `2026-06-03T22:16:28+08:00`.

The coverage gate is project-specific S2V release-gate traceability coverage, not line coverage. `target/release-gates/coverage.json` reports:

- `status=ready`.
- `covered_acceptance_criteria=16`.
- `minimum_covered_acceptance_criteria=16`.
- Covered tests include release-critical `TEST-15.2.*`, `TEST-17.5.*`, `TEST-24.3.*`, and `TEST-24.4.*`.

## Functional Gate Evidence

Release and compatibility artifacts after the full verification run report:

- `target/release-gates/real-upstream-corpus/summary.json`: `status=ready`, `required_p0_fixture_count=50`, `observed_p0_fixture_count=50`, `used_test_binary_count=0`, `blocking_finding_count=0`.
- `target/release-gates/current-latest-source-inventory.json`: `status=ready`, `unclassified_rows=[]`.
- `target/release-gates/current-latest-matrix.json`: `status=ready`, `unclassified_rows=[]`.
- `target/release-gates/current-latest-golden-corpus.json`: `status=ready-with-blockers`, `fixture_case_count=94`, `p0_total=94`, `p1_total=2367`, `p2_total=1417`, `blocker_count=24`.
- `target/release-gates/current-latest-quality.json`: `status=ready-with-blockers`, `local_stable_allowed=true`, `local_current_latest_ready=false`, `perfect_refactor_claim_allowed=false`, `blockers=4`.
- `target/release-gates/perfect-refactor-unblock-packet.json`: `status=blocked`, `auto_resolvable=false`, `required_user_decision_count=31`.
- `target/release-gates/publication-authority.json`: `publication_ready=credential-blocked`, `credential_blocked=true`, `legal_brand_blocked=true`.
- `target/release-gates/release-candidate.json`: `stable_allowed=true`, `published=false`.

The local functional test suite proves the current local gates. It does not prove real external provider/account behavior, public publication, or bug-free behavior for all future inputs.

## Fresh Upstream Target Check

Fresh external checks during this audit:

```bash
npm view promptfoo version gitHead dist.tarball dist.integrity --json
git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.14 refs/tags/0.121.15 refs/tags/0.121.13
gh api repos/promptfoo/promptfoo/releases/latest --jq '{tag_name,name,target_commitish,published_at,html_url}'
```

Observed current external state:

- npm latest: `promptfoo@0.121.14`.
- npm `gitHead`: `7a48c5fce614bee617efbb3b7fc93d404c75b628`.
- npm tarball: `https://registry.npmjs.org/promptfoo/-/promptfoo-0.121.14.tgz`.
- npm integrity: `sha512-YUeBMqwfv3xZC7HJ3ohwk2e0i3DdCitOrvWZPijCOMywp/S+CZEjyqVh1pUzR1PgDo9eBBn9WXyw2wbDBihcpA==`.
- GitHub latest release: `0.121.14`, target `7a48c5fce614bee617efbb3b7fc93d404c75b628`, published `2026-06-02T13:49:18Z`.
- GitHub default branch HEAD: `313d996991f33fd0e7b5f90d68ffed8f1b18cd23`.
- Tag `0.121.13`: `4860e990c7e9a2f8f677173fb92cf9867b34d03f`.
- Tag `0.121.14`: `7a48c5fce614bee617efbb3b7fc93d404c75b628`.
- No `0.121.15` tag was returned by the checked refs.

The repository's current lock artifact still records GitHub default branch HEAD `9d7d810c2118c63cb537bf05ea2d34c12bd22066` in `target/release-gates/current-latest-target.json`.

This is a new live HEAD drift:

- locked GitHub HEAD: `9d7d810c2118c63cb537bf05ea2d34c12bd22066`
- live GitHub HEAD: `313d996991f33fd0e7b5f90d68ffed8f1b18cd23`

The npm package and latest GitHub release still agree on `0.121.14` / `7a48c5fce614bee617efbb3b7fc93d404c75b628`, but repository HEAD has moved again.

## Findings

### P0 - Current-stage local verification is green

All adapter verification keys pass in one fresh full verification run. Baseline green also passed before the full run. This supports the claim that the current local completed-task implementation is testable and locally stable under the declared S2V gates.

### P0 - Current-latest repository HEAD parity is stale

The full verification run used the tracked current-latest target fixture, whose GitHub default branch HEAD is `9d7d810c2118c63cb537bf05ea2d34c12bd22066`. Fresh external state shows the live default branch HEAD is now `313d996991f33fd0e7b5f90d68ffed8f1b18cd23`.

This does not invalidate the local frozen and package-release evidence. It does invalidate any claim that the repository has just proven full parity against the live GitHub default branch HEAD as of this audit.

Recommended next S2V work: add a new current-latest GitHub HEAD drift refresh task if the target policy continues to require repository HEAD evidence, then rerun the source inventory, matrix, golden corpus, quality, runtime-smoke, and unblock packet gates against the refreshed target.

### P0 - Perfect-refactor completion remains blocked by external authority

The unblock packet still requires 31 user or external decisions:

- `current-latest-golden, product-authority`: 13
- `current-latest-golden, product-service-authority`: 7
- `current-latest-golden, private-service`: 2
- `current-latest-golden, credential`: 1
- `current-latest-golden, account`: 1
- `current-target, target-policy`: 1
- `publication-authority, publication-authority`: 6

These are not local test failures. They require real credentials, account authority, private-service access, product/service owner confirmation, legal/brand approval, public publication evidence, or formal waivers.

### P1 - Public publication is not proven

Publication remains `credential-blocked` and `legal_brand_blocked`. All publication channels remain dry-run/local evidence only, with no external URL/digest proof and `published=false`.

### P1 - Bug-free wording is impossible to prove

The current quality gate intentionally forbids wording such as "no potential bugs", "zero possible bugs", "bug-free", or "no possible bugs". The strongest supported wording is: "no known release-blocking defects under declared gates".

## Requirement Verdict

| Requirement | Evidence | Verdict |
|---|---|---|
| Completed-task status review | 76 task specs and 41 phase specs are `Done`; no `Draft`, `Ready`, or `In Progress` task specs found | Met |
| Phase-level readiness review | 41/41 phase specs pass `s2v_preflight_phase` | Met |
| Source/test area audit | `s2v_guard_areas_tracked src crates tests viewer npm compatibility` passes | Met |
| Baseline green | `s2v_baseline_green "src/ crates/ tests/ viewer/ npm/ compatibility/"` exits 0 | Met |
| Full local functional verification | `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"` exits 0 | Met |
| Coverage/traceability guard | 76/76 task specs pass `s2v_coverage_threshold_guard`; `coverage.json` is `ready` | Met |
| Frozen-baseline real upstream corpus | 50/50 real P0 fixtures ready, no test binary use, no blocking findings | Met |
| Current-latest source/matrix accounting | source inventory and matrix are `ready`, with no unclassified rows | Met for locked artifact |
| Current-latest full-function parity | golden corpus has 24 blockers; quality has 4 blockers; `local_current_latest_ready=false` | Not met |
| Live GitHub default branch parity | live HEAD `313d996...` differs from locked HEAD `9d7d810...` | Not met |
| Public release proof | publication is credential/legal-brand blocked and unpublished | Not met |
| Perfect refactor claim | `perfect_refactor_claim_allowed=false` | Not met |
| "No potential bugs" guarantee | quality gate forbids this claim | Impossible to prove by finite testing |

## Conclusion

The current stage passes a comprehensive local S2V functional verification audit. The project is locally stable under the declared adapter gates, and all completed tasks have a green verification path.

The project still cannot honestly be called a complete perfect refactor of the live `promptfoo/promptfoo` repository. The strongest current claim is:

> Under the declared local S2V gates and locked evidence artifacts, there are no known release-blocking defects in the locally verified scope.

The remaining work is not another ordinary local smoke test. It is target-refresh plus external authority closure: refresh the moved GitHub HEAD target if repository HEAD remains in scope, then resolve or formally waive the 31 external decision items and provide publication authority evidence.
