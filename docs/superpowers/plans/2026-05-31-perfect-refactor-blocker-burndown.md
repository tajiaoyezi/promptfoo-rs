# Perfect Refactor Blocker Burndown Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Burn down the explicit blockers that prevent `promptfoo-rs` from honestly satisfying a perfect refactor claim for `promptfoo/promptfoo`.

**Architecture:** Keep S2V as the source of truth: each blocker class becomes a phase task, each task starts with RED tests, and release-gate JSON remains fail-closed. The first implementation narrows source inventory blockers from broad missing rows into actionable P0 implementation blockers without weakening parity claims.

**Tech Stack:** Rust stable, Bash release scripts through Git for Windows Bash, Node JSON processing inside release scripts, S2V helper scripts from `docs/s2v/scripts/lib/`.

---

### Task 1: Source Inventory Ledger Closure

**Files:**
- Create: `tests/source_inventory_ledger_closure.rs`
- Modify: `src/compatibility/inventory.rs`
- Modify: `scripts/release/source-inventory-evidence.sh`
- Modify: `scripts/release/runtime-smoke.sh`
- Modify: `docs/compatibility/matrix.md`
- Modify: `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`

- [ ] **Step 1: Write failing tests**

Create `tests/source_inventory_ledger_closure.rs` with tests that call `build_source_accounting_ledger`, assert every extracted item is represented, assert generated P0 rows remain blockers, and assert P1/P2 generated rows use snapshot/registration verification.

- [ ] **Step 2: Verify RED**

Run: `cargo test --test source_inventory_ledger_closure`

Expected: FAIL because `build_source_accounting_ledger`, `SourceAccountingLedger`, and `write_source_accounting_ledger` do not exist.

- [ ] **Step 3: Implement source accounting ledger**

Add serializable ledger structs and functions to `src/compatibility/inventory.rs`. Generated rows use `blocker:<item-id>` for P0, `snapshot:<item-id>` for P1, and `registration:<item-id>` for P2.

- [ ] **Step 4: Wire release scripts**

Update `scripts/release/source-inventory-evidence.sh` to write `target/release-gates/source-inventory-ledger.json`, set `missing_matrix_rows` only for truly unrepresented rows, and retain generated P0 rows in `release_blockers`.

- [ ] **Step 5: Verify GREEN**

Run: `cargo test --test source_inventory_ledger_closure`.

Expected: PASS.

- [ ] **Step 6: Run task verification**

Run with Git for Windows Bash:

```bash
source docs/s2v/scripts/lib/preflight.sh
source docs/s2v/scripts/lib/verify.sh
s2v_verify_full "$(s2v_extract_verify_keys docs/specs/tasks/task-18.1-source-inventory-ledger-closure.md)"
```

Expected: all listed keys pass.

### Task 2: P0 Provider Module Fixture Burndown

**Files:**
- Create: `tests/p0_provider_module_fixture_burndown.rs`
- Modify: `compatibility/inventory/upstream-items.json`
- Modify: `compatibility/fixtures/providers/`
- Modify: `src/compatibility/provider_assertion.rs`
- Modify: `scripts/release/longtail-classification.sh`
- Modify: `docs/compatibility/matrix.md`

- [ ] **Step 1: Write failing tests for current blocker rows**
- [ ] **Step 2: Add mock fixtures for provider helper modules that map to existing P0 providers**
- [ ] **Step 3: Preserve external-service rows as explicit blockers**
- [ ] **Step 4: Verify longtail report counts and reasons**
- [ ] **Step 5: Commit RED, GREEN, and docs completion notes**

### Task 3: Current Upstream Rebaseline Gate

**Files:**
- Create: `tests/current_upstream_rebaseline_gate.rs`
- Create: `scripts/release/current-upstream-policy.sh`
- Create: `compatibility/inventory/current-upstream-target.json`
- Modify: `docs/compatibility/target-policy.md`
- Modify: `scripts/release/runtime-smoke.sh`

- [ ] **Step 1: Write failing tests for target-mode policy**
- [ ] **Step 2: Parse `git ls-remote` output into frozen/current observations**
- [ ] **Step 3: Fail current-perfect claim in frozen mode when HEAD differs**
- [ ] **Step 4: Add runtime-smoke artifact and docs**
- [ ] **Step 5: Commit RED, GREEN, and docs completion notes**

### Task 4: Publication Authority Release Gate

**Files:**
- Create: `tests/publication_authority_release_gate.rs`
- Modify: `scripts/release/installability.sh`
- Modify: `scripts/release/runtime-smoke.sh`
- Modify: `docs/release.md`

- [ ] **Step 1: Write failing tests for authority vs installability fields**
- [ ] **Step 2: Add channel-level publication authority report**
- [ ] **Step 3: Require external evidence for `published=true`**
- [ ] **Step 4: Keep credential-blocked channels explicit**
- [ ] **Step 5: Commit RED, GREEN, and docs completion notes**
