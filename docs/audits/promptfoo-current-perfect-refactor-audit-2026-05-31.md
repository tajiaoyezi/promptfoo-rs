# promptfoo current perfect-refactor audit - 2026-05-31

**Status**: Current audit verdict
**Objective**: determine whether the current `promptfoo-rs` worktree completely satisfies a perfect refactor of `promptfoo/promptfoo`.
**Verdict**: Not fully satisfied.

## Scope

This audit supersedes the 2026-05-30 negative audit for current-state purposes only. The older audit remains useful history, but Phase 11-16 have since changed the repository substantially.

The compatibility target in project specs is the frozen upstream baseline:

- npm: `promptfoo@0.121.13`
- git tag: `refs/tags/0.121.13`
- commit: `4860e990c7e9a2f8f677173fb92cf9867b34d03f`

The current npm `latest` value checked during this audit is also `0.121.13`, with npm `gitHead=4860e990c7e9a2f8f677173fb92cf9867b34d03f`. Current GitHub `main` is ahead of the frozen tag at `6e4ba60eba69ff696d0404aabf5453311214c500`; project stable claims must therefore remain scoped to the frozen tag unless a future rebaseline task changes the target.

## Current Verification Evidence

Full local S2V verification passed on 2026-05-31 with Git for Windows Bash:

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

This proves the current local gate chain is executable and green. It does not by itself prove full upstream semantic parity.

## Improvements Since The Previous Audit

The project is materially stronger than the 2026-05-30 audit snapshot:

- All phase and task specs are `Done` through Phase 16.
- Adapter verification commands are no longer `N/A`; full local gates run.
- `viewer/` and `npm/` now include package metadata, lockfiles, build/test scripts, and smoke scripts.
- `compatibility/fixtures/` contains 53 tracked fixture manifest files.
- Phase 16 removed the previous no-op CLI behavior for `view`, `cache`, `import`, and `export`.
- Runtime smoke now produces measured `performance.json`, `security.json`, and `release-candidate.json`.
- Runtime smoke includes one real upstream execution path using `npx --yes promptfoo@0.121.13 eval`.

These are real progress items and should not be confused with the older audit's pre-Phase-16 state.

## Blocking Findings

### P0 - Local CLI surface is still much smaller than upstream

`promptfoo-rs --help` currently exposes these top-level commands:

`eval`, `view`, `cache`, `redteam`, `mcp`, `code-scans`, `scan-model`, `model-audit`, `import`, `export`.

`promptfoo@0.121.13 --help` exposes additional user-visible top-level commands including:

`init`, `share`, `auth`, `config`, `debug`, `delete`, `generate`, `feedback`, `list`, `logs`, `optimize`, `retry`, `validate`, and `show`.

`promptfoo-rs eval --help` exposes only:

`--config`, `--output`, and `--max-concurrency`.

`promptfoo@0.121.13 eval --help` exposes a much broader flag/subcommand surface, including assertions/prompts/providers/tests/vars, model outputs, prompt prefix/suffix, tags, repeat, delay, no-cache, filters, table/no-table, share/no-share, resume, retry-errors, no-write, grader, suggest-prompts, watch, extension hooks, description, progress bar control, env-file/env-path, and `setup`.

`promptfoo-rs redteam --help` models redteam stages as one `--stage` option. Upstream `promptfoo redteam --help` exposes subcommands such as `init`, `eval`, `discover`, `generate`, `run`, `poison`, `report`, `setup`, and `plugins`.

Verdict: the project has a useful compatible CLI subset, but not a perfect CLI refactor.

### P0 - Item-level upstream inventory is not complete enough to support a perfect-refactor claim

Current local inventory:

- `compatibility/inventory/upstream-items.json`: 44 items
- categories: 9 command, 3 flag, 6 provider, 7 assertion, 6 redteam, 5 output, 3 config, 1 node-api, 1 viewer, 3 release
- one unresolved P2 row remains: `provider:dynamic-registry`

Direct frozen-tag upstream tree counts for `4860e990c7e9a2f8f677173fb92cf9867b34d03f`:

- tracked files: 5290
- `src/` files: 1595
- command-related TS/JS files: 85
- provider TS/JS files: 219
- assertion TS/JS files: 56
- redteam TS/JS files: 217
- redteam plugin files: 125
- redteam strategy files: 32
- app/viewer files: 701
- example files: 1220

The current local inventory is a curated matrix seed, not a source-extracted complete inventory of all documented and source-visible upstream capability items.

### P0 - The real upstream smoke is too narrow

`target/release-gates/real-upstream-smoke/latest/metadata.json` proves one real upstream eval:

- fixture: `real-upstream-smoke-echo`
- upstream command: `npx --yes promptfoo@0.121.13 eval -c promptfooconfig.yaml --output raw/upstream.json`
- rs command: `target/release/promptfoo-rs.exe eval -c promptfooconfig.yaml --output raw/rs.json`
- upstream exit code: 0
- rs exit code: 0
- diff status: ready

This is valuable, but it is one minimal echo fixture. It does not prove the PRD's broader P0 gate of at least 50 core fixtures with upstream artifact, rs artifact, normalization output, and diff report for the real CLI/runtime surface.

### P1 - Source inventory evidence is too weak

`target/release-gates/source-inventory-evidence.json` reports:

- `inventory_item_count`: 44
- `package_file_count`: 536
- `package_file_counts.assertion`: 0
- `package_file_counts.redteam`: 0
- `package_file_counts.config`: 0
- `status`: ready

The ready status only proves the current script found category coverage and source-reference strings. It does not prove AST-level or source-level extraction of all upstream commands, providers, assertions, redteam plugins/strategies, config features, examples, and viewer/API surfaces.

### P1 - Some compatibility matrix rows intentionally remain later or unsupported

The project correctly registers some non-goals and deferred areas, for example:

- `promptfoo cloud/share`: P2 unsupported/later by PRD scope
- `redteam-plugin:medical`: later
- `redteam-strategy:agentic-chain`: later
- `provider:dynamic-registry`: unresolved/P2 seed item

This is acceptable for a scoped 1.0 compatibility plan, but it contradicts a literal "perfect refactor" claim unless the phrase is explicitly redefined as "complete according to the frozen PRD P0/P1/P2 policy, with documented P2 gaps".

## Requirement Verdict

| Requirement | Current evidence | Verdict |
|---|---|---|
| All local S2V phase/task specs complete | Adapter indexes Phase 1-16 as `Done`; status scan found no Draft/Ready/In Progress task specs | Met |
| Local verification gates executable and green | Full 9-key S2V verification passed | Met |
| Frozen baseline traceable | npm, tag, commit, integrity, and container digest recorded; npm latest still 0.121.13 | Met for frozen target |
| CLI command/flag parity | Local top-level commands and eval/redteam flags are much narrower than upstream help | Not met |
| 100% upstream item-level inventory | Local inventory has 44 curated items; frozen tag has 85 command files, 219 provider files, 56 assertion files, 217 redteam files | Not met |
| P0 real upstream golden diff corpus | One real upstream smoke fixture exists; the 50-fixture corpus is manifest-level/local-gate evidence, not 50 real upstream runs | Not proven |
| Provider/assertion/redteam full parity | P0 subsets are covered; long-tail source surface is not fully enumerated or implemented | Not met |
| Viewer/npm package local build smoke | Viewer and npm package smoke scripts pass in full local gate | Met for local smoke |
| Multi-channel public release | Packaging model and dry-run evidence exist; real external credentials/publication are outside current evidence | Not proven |

## Conclusion

The current project is a substantially improved, locally verified, S2V-complete Rust compatibility implementation for a frozen promptfoo baseline. It is not yet a complete or perfect refactor of `promptfoo/promptfoo`.

The remaining gap is no longer basic project readiness. It is deep parity proof:

1. source-extracted complete upstream inventory,
2. broader CLI command/flag behavior parity,
3. real upstream-vs-rs golden diff execution across the required P0 corpus,
4. complete provider/assertion/redteam long-tail classification or implementation evidence,
5. final release publication/installability evidence where credentials and public channels are required.

## Recommended Next S2V Work

Add a new phase after Phase 16 focused on deep parity proof rather than smoke hardening:

- source-extracted upstream inventory from `refs/tags/0.121.13`;
- CLI global/eval/redteam command and flag parity expansion;
- real upstream golden corpus runner over at least 50 P0 fixtures;
- provider/assertion/redteam long-tail extractor and classifier;
- release publication dry-run/installability artifact hardening.

