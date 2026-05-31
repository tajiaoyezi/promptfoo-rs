# Upstream Distribution Target Disambiguation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a release-gate artifact that separates npm `promptfoo` core package metadata from GitHub repository HEAD and non-core GitHub release drift.

**Architecture:** Extend the existing compatibility inventory module with serializable distribution-target structs and a parser for npm package metadata. Add one Bash/Node release script that writes the artifact, then wire runtime smoke and release-candidate summaries to consume it without changing the existing perfect-refactor claim blocker semantics.

**Tech Stack:** Rust stable, serde/serde_json, Bash release scripts through Git for Windows Bash, Node JSON processing inside release scripts, S2V helper scripts from `docs/s2v/scripts/lib/`.

---

### Task 1: Distribution Target Gate

**Files:**
- Create: `tests/upstream_distribution_target_gate.rs`
- Modify: `src/compatibility/inventory.rs`
- Create: `scripts/release/upstream-distribution-target.sh`
- Modify: `scripts/release/runtime-smoke.sh`
- Modify: `docs/compatibility/target-policy.md`
- Modify: `docs/compatibility/matrix.md`
- Modify: `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`

- [ ] **Step 1: Write the failing tests**

Create `tests/upstream_distribution_target_gate.rs` with four tests:

```rust
#[test]
fn test_21_1_1_npm_core_package_metadata_matches_frozen_baseline() {
    /* TEST-21.1.1 */
}

#[test]
fn test_21_1_2_github_head_and_release_are_separate_observations() {
    /* TEST-21.1.2 */
}

#[test]
fn test_21_1_3_non_core_github_release_cannot_allow_current_perfect_claim() {
    /* TEST-21.1.3 */
}

#[test]
fn test_21_1_4_runtime_smoke_wires_distribution_target_artifact() {
    /* TEST-21.1.4 */
}
```

- [ ] **Step 2: Verify RED**

Run:

```bash
cargo test --test upstream_distribution_target_gate
```

Expected: FAIL because `parse_npm_package_observation`, `build_upstream_distribution_target`, `write_upstream_distribution_target`, and `scripts/release/upstream-distribution-target.sh` are missing.

- [ ] **Step 3: Implement Rust distribution target structs and functions**

Add `NpmPackageObservation`, `UpstreamDistributionTarget`, `DistributionTargetError`, `parse_npm_package_observation`, `build_upstream_distribution_target`, and `write_upstream_distribution_target` to `src/compatibility/inventory.rs`.

- [ ] **Step 4: Add the release script**

Create `scripts/release/upstream-distribution-target.sh`. It reads `UPSTREAM_NPM_VIEW_FILE` and `UPSTREAM_LS_REMOTE_FILE` fixtures when provided; otherwise it runs `npm view promptfoo version gitHead dist.tarball dist.integrity time.modified --json` and `git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.13 refs/tags/code-scan-action-0.1.7`.

- [ ] **Step 5: Wire runtime smoke and docs**

Call the new script from `scripts/release/runtime-smoke.sh`, validate `target/release-gates/upstream-distribution-target.json`, add distribution target summary fields to `release-candidate.json`, and document the distinction in `docs/compatibility/target-policy.md`, `docs/compatibility/matrix.md`, and the current audit.

- [ ] **Step 6: Verify GREEN**

Run:

```bash
cargo test --test upstream_distribution_target_gate
```

Expected: PASS with all four TEST-21.1 cases green.

- [ ] **Step 7: Run S2V task verification**

Run with Git for Windows Bash:

```bash
source docs/s2v/scripts/lib/preflight.sh
source docs/s2v/scripts/lib/verify.sh
s2v_verify_full "$(s2v_extract_verify_keys docs/specs/tasks/task-21.1-upstream-distribution-target-gate.md)"
s2v_coverage_threshold_guard docs/specs/tasks/task-21.1-upstream-distribution-target-gate.md
```

Expected: all listed task §9 keys pass.
