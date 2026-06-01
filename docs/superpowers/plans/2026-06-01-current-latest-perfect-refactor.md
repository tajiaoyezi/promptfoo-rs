# Current Latest Perfect Refactor Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rebaseline promptfoo-rs against the original promptfoo project's current latest observed target and enforce a large, auditable verification suite before any current-latest perfect-refactor claim.

**Architecture:** The work is split into four S2V tasks: lock the current-latest target, re-extract the complete source inventory, expand the golden corpus, and aggregate exhaustive quality gates. Every task follows RED/GREEN/verification and keeps impossible zero-bug wording out of release claims.

**Tech Stack:** Rust, serde_json, shell release gates, Git for Windows Bash, npm metadata, GitHub release metadata, compatibility fixtures, S2V helper scripts.

---

### Task 1: Current-Latest Target Lock

**Files:**
- Modify: `src/compatibility/inventory.rs`
- Create: `tests/current_latest_target_lock.rs`
- Create: `scripts/release/current-latest-target-lock.sh`
- Create: `docs/compatibility/current-latest.lock.md`
- Create: `compatibility/inventory/current-latest-target.json`
- Modify: `scripts/release/runtime-smoke.sh`

- [ ] **Step 1: Write RED tests**

Create `tests/current_latest_target_lock.rs` with TEST-24.1.1 through TEST-24.1.4. Use fixtures for npm latest `0.121.13`, GitHub HEAD `1d09dfeb5f0766905409117f923dd5c4b0838d9f`, and GitHub latest release `code-scan-action-0.1.7`.

- [ ] **Step 2: Verify RED**

Run:

```bash
cargo test --test current_latest_target_lock
```

Expected: fail because `CurrentLatestTargetLock` and `current-latest-target-lock.sh` do not exist.

- [ ] **Step 3: Implement GREEN**

Add `CurrentLatestTargetLock` parsing/writing in `src/compatibility/inventory.rs`, add the shell gate, and wire runtime smoke.

- [ ] **Step 4: Verify GREEN**

Run:

```bash
cargo test --test current_latest_target_lock
"C:\Program Files\Git\bin\bash.exe" -lc 'bash scripts/release/current-latest-target-lock.sh'
```

Expected: tests pass and `target/release-gates/current-latest-target.json` exists.

- [ ] **Step 5: Commit**

```bash
git add src/compatibility/inventory.rs tests/current_latest_target_lock.rs scripts/release/current-latest-target-lock.sh scripts/release/runtime-smoke.sh docs/compatibility/current-latest.lock.md compatibility/inventory/current-latest-target.json
git commit -m "feat(compatibility): lock current latest upstream target"
```

### Task 2: Current-Latest Source Inventory

**Files:**
- Modify: `src/compatibility/inventory.rs`
- Create: `tests/current_latest_source_inventory.rs`
- Create: `scripts/release/current-latest-source-inventory.sh`
- Create: `compatibility/inventory/current-latest-source-inventory.json`
- Create: `compatibility/matrix/current-latest-matrix.json`
- Modify: `docs/compatibility/matrix.md`

- [ ] **Step 1: Write RED tests**

Create TEST-24.2.1 through TEST-24.2.4 to require stable IDs and source references for command, flag, provider, assertion, redteam, config, output, viewer, Node API, example, and docs rows.

- [ ] **Step 2: Verify RED**

Run:

```bash
cargo test --test current_latest_source_inventory
```

Expected: fail because current-latest source inventory extraction is not implemented.

- [ ] **Step 3: Implement GREEN**

Implement extraction and reconciliation using the locked source target. Unknown rows become blockers.

- [ ] **Step 4: Verify GREEN**

Run:

```bash
cargo test --test current_latest_source_inventory
"C:\Program Files\Git\bin\bash.exe" -lc 'bash scripts/release/current-latest-source-inventory.sh'
```

Expected: tests pass and inventory/matrix artifacts are produced.

- [ ] **Step 5: Commit**

```bash
git add src/compatibility/inventory.rs tests/current_latest_source_inventory.rs scripts/release/current-latest-source-inventory.sh compatibility/inventory/current-latest-source-inventory.json compatibility/matrix/current-latest-matrix.json docs/compatibility/matrix.md
git commit -m "feat(compatibility): extract current latest source inventory"
```

### Task 3: Current-Latest Golden Corpus

**Files:**
- Modify: `src/compatibility/harness.rs`
- Create: `tests/current_latest_golden_corpus.rs`
- Create: `scripts/release/current-latest-golden-corpus.sh`
- Create: `compatibility/fixtures/current-latest/`
- Create: `compatibility/artifacts/current-latest/`

- [ ] **Step 1: Write RED tests**

Create TEST-24.3.1 through TEST-24.3.4. Tests must fail when P0 rows lack fixtures, P1 rows lack snapshots, P0 diffs are `bug`/unclassified, or the corpus has fewer than 250 fixtures while inventory has 250 or more rows.

- [ ] **Step 2: Verify RED**

Run:

```bash
cargo test --test current_latest_golden_corpus
```

Expected: fail because current-latest corpus evaluation is not implemented.

- [ ] **Step 3: Implement GREEN**

Implement corpus reporting and blocker evaluation. Use deterministic mock/recorded providers.

- [ ] **Step 4: Verify GREEN**

Run:

```bash
cargo test --test current_latest_golden_corpus
"C:\Program Files\Git\bin\bash.exe" -lc 'bash scripts/release/current-latest-golden-corpus.sh'
```

Expected: tests pass and corpus artifacts are produced.

- [ ] **Step 5: Commit**

```bash
git add src/compatibility/harness.rs tests/current_latest_golden_corpus.rs scripts/release/current-latest-golden-corpus.sh compatibility/fixtures/current-latest compatibility/artifacts/current-latest
git commit -m "feat(compatibility): expand current latest golden corpus"
```

### Task 4: Current-Latest Exhaustive Quality Gate

**Files:**
- Modify: `src/release.rs`
- Create: `tests/current_latest_quality_gate.rs`
- Create: `scripts/release/current-latest-quality-gate.sh`
- Modify: `scripts/release/runtime-smoke.sh`
- Modify: `docs/release.md`
- Modify: `docs/compatibility/matrix.md`

- [ ] **Step 1: Write RED tests**

Create TEST-24.4.1 through TEST-24.4.4. Tests must fail when quality report omits adapter verification, golden corpus, source inventory coverage, regression/stress/property results, runtime smoke, external authority, publication authority, or when claim text says no possible bugs.

- [ ] **Step 2: Verify RED**

Run:

```bash
cargo test --test current_latest_quality_gate
```

Expected: fail because the quality gate is not implemented.

- [ ] **Step 3: Implement GREEN**

Implement the quality report and claim evaluation. Permit only “no known release-blocking defects under declared gates”.

- [ ] **Step 4: Verify GREEN**

Run:

```bash
cargo test --test current_latest_quality_gate
"C:\Program Files\Git\bin\bash.exe" -lc 'bash scripts/release/current-latest-quality-gate.sh'
```

Expected: tests pass and `target/release-gates/current-latest-quality.json` is produced.

- [ ] **Step 5: Commit**

```bash
git add src/release.rs tests/current_latest_quality_gate.rs scripts/release/current-latest-quality-gate.sh scripts/release/runtime-smoke.sh docs/release.md docs/compatibility/matrix.md
git commit -m "feat(release): gate current latest perfect refactor claim"
```
