# promptfoo S2V parity-claim audit - 2026-05-30

**Status**: Audit attachment
**Parent audit**: `docs/audits/promptfoo-perfect-refactor-audit-2026-05-30.md`
**Purpose**: compare S2V `Done` evidence against the stronger PRD claim of upstream promptfoo parity.

## Snapshot

| Item | Value |
|---|---|
| Local branch before this attachment | `master` |
| Local HEAD before this attachment | `f77c13a85e52bf0411a4865aca77b1d36b992fe7` |
| Focus area | Phase 6 compatibility harness and golden diff release gate |

## Why This Audit Exists

The project can be locally S2V-complete while still failing the "perfect promptfoo refactor" objective. This attachment checks whether the S2V completion record proves actual upstream execution, fixture parity, golden diff persistence, and release-gate coverage.

## Phase 6 Completion Claim

`docs/specs/phases/phase-6-compatibility-harness.md` says Phase 6 is `Done` and that the phase goal is:

- upstream and promptfoo-rs P0 golden diff automation
- P1 snapshot automation
- release gate automation

The phase acceptance evidence is `s2v_preflight_phase ... && cargo test --workspace`. That proves local tests passed. It does not, by itself, prove that upstream promptfoo was executed or that a fixture corpus exists.

## Task 6.1: Harness Runner

### Spec evidence

`task-6.1-upstream-harness-runner.md` is `Done`, but its traceability table marks integration/E2E tests as `N/A until integration harness exists`.

Its completion notes explicitly leave this residual risk:

- real upstream Node execution remains future work
- artifact persistence remains future work
- task 6.2 / later CI integration must extend the runner

### Implementation evidence

`src/compatibility/harness.rs` defines `HarnessRunner::run_fixture`, but the implementation constructs two in-memory artifacts:

- one with `ArtifactEngine::UpstreamPromptfoo`
- one with `ArtifactEngine::PromptfooRs`

It does not spawn `promptfoo`, `npx`, `node`, `cargo run`, or any CLI process. It does not write an upstream artifact to `compatibility/artifacts/`.

### Test evidence

`tests/upstream_harness_runner.rs` verifies:

- pinned baseline references are accepted
- `latest` / `HEAD` are rejected
- one inline fixture produces paired artifact structs
- normalization rules replace timestamps, paths, random IDs, and latency

These are useful contract tests, but they are not upstream-vs-rs golden diff tests.

## Task 6.2: Golden Diff Release Gate

### Spec evidence

`task-6.2-golden-diff-release-gate.md` is `Done`, but its traceability table also marks integration/E2E tests as `N/A until integration harness exists`.

Its completion notes explicitly leave these residual risks:

- real CI job wiring remains future work
- artifact persistence remains future work

### Implementation evidence

`src/compatibility/diff.rs` classifies two already-normalized in-memory payloads. If payloads differ, it trusts `/compatibility/classification` embedded in the rs payload and defaults to `Bug`.

`src/compatibility/release_gate.rs` evaluates a release-gate summary from a supplied matrix and supplied findings. It does not discover fixtures, execute upstream promptfoo, execute promptfoo-rs, or persist release-gate reports.

### Test evidence

`tests/golden_diff_release_gate.rs` verifies:

- the enum covers diff classes
- P0 `Bug` and `Unclassified` findings block stable release
- P1/P2 coverage counters appear in the summary

These are useful release-gate unit tests, but they do not prove that the project has run the PRD-required 50 P0 fixtures or that upstream and rs outputs match.

## Filesystem Evidence

| Path | Evidence |
|---|---|
| `compatibility/fixtures/` | contains only `.gitkeep` |
| `compatibility/artifacts/` | directory does not exist |
| `src/compatibility/harness.rs` | constructs in-memory placeholder artifacts |
| `tests/upstream_harness_runner.rs` | uses inline JSON fixtures, not upstream promptfoo execution |
| `tests/golden_diff_release_gate.rs` | uses inline normalized JSON payloads |

## Verdict

S2V `Done` for Phase 6 proves a local contract layer exists for compatibility harness concepts. It does not prove the PRD's stronger compatibility promise:

- no executable corpus of at least 50 P0 fixtures is present
- no upstream promptfoo CLI execution is implemented in the harness
- no promptfoo-rs CLI execution is implemented in the harness
- no persisted upstream/rs/normalized/diff artifacts are present
- no full compatibility release gate evidence exists

Therefore Phase 6's `Done` state cannot be used as evidence that `promptfoo-rs` is a perfect or complete refactor of `promptfoo/promptfoo`.
