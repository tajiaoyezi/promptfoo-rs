# Perfect Refactor Unblock Packet Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a release gate artifact that turns remaining perfect-refactor blockers into a deduplicated, machine-readable external decision packet.

**Architecture:** Add typed packet builders and validators in `src/release.rs`, then wire a shell/Node runtime script into `scripts/release/runtime-smoke.sh`. Tests drive the Rust API, script wiring, docs, and release-candidate integration.

**Tech Stack:** Rust, serde/serde_json, Bash, Node.js JSON glue, S2V release helper commands.

---

### Task 1: Spec And RED Tests

**Files:**
- Create: `docs/specs/phases/phase-22-perfect-refactor-unblock-packet.md`
- Create: `docs/specs/tasks/task-22.1-authority-unblock-packet-gate.md`
- Modify: `docs/prds/promptfoo-rs.prd.md`
- Modify: `docs/s2v-adapter.md`
- Modify: `test/features/perfect-refactor-parity.feature`
- Create: `tests/perfect_refactor_unblock_packet.rs`

- [ ] **Step 1: Add Phase 22 and task 22.1 specs**

Use the phase/task specs to define a Ready S2V task with AC1 through AC5 mapped to `SCEN-22.1.1` and `TEST-22.1.1` through `TEST-22.1.5`.

- [ ] **Step 2: Write failing tests**

Create `tests/perfect_refactor_unblock_packet.rs` with imports for:

```rust
use promptfoo_rs::release::{
    build_perfect_refactor_claim_contract, build_perfect_refactor_unblock_packet,
    validate_perfect_refactor_unblock_packet, write_perfect_refactor_unblock_packet,
    PerfectRefactorClaimInputs, PerfectRefactorUnblockInputs, PerfectRefactorUnblockItem,
    PublicationReadiness,
};
```

The tests must assert the packet schema, `status=blocked`, `auto_resolvable=false`, source/external deduplication, publication evidence requirements, current-upstream rebaseline requirement, runtime script wiring, and docs wording.

- [ ] **Step 3: Verify RED**

Run: `cargo test --test perfect_refactor_unblock_packet`

Expected: FAIL because the `PerfectRefactorUnblock*` types and functions do not exist yet.

- [ ] **Step 4: Commit RED**

```bash
git add docs/prds/promptfoo-rs.prd.md docs/s2v-adapter.md docs/specs/phases/phase-22-perfect-refactor-unblock-packet.md docs/specs/tasks/task-22.1-authority-unblock-packet-gate.md docs/superpowers/plans/2026-05-31-perfect-refactor-unblock-packet.md test/features/perfect-refactor-parity.feature tests/perfect_refactor_unblock_packet.rs
git commit -m "test(release): add SCEN-22.1.1 perfect refactor unblock RED tests"
```

### Task 2: Green Implementation

**Files:**
- Modify: `src/release.rs`
- Create: `scripts/release/perfect-refactor-unblock-packet.sh`
- Modify: `scripts/release/runtime-smoke.sh`
- Modify: `docs/release.md`
- Modify: `docs/compatibility/matrix.md`
- Modify: `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`

- [ ] **Step 1: Add typed release packet API**

Add `PerfectRefactorUnblockInputs`, `PerfectRefactorUnblockItem`, `PerfectRefactorUnblockPacket`, `PerfectRefactorUnblockValidation`, `build_perfect_refactor_unblock_packet`, `validate_perfect_refactor_unblock_packet`, and `write_perfect_refactor_unblock_packet` to `src/release.rs`.

- [ ] **Step 2: Deduplicate provider blockers by item_id**

Build decision items from external authority items first, then add source-only blockers only when the item id is not already represented. Add current-upstream and publication decisions when not already represented.

- [ ] **Step 3: Wire runtime script**

Create `scripts/release/perfect-refactor-unblock-packet.sh` to read `perfect-refactor-claim.json`, `source-inventory-evidence.json`, `external-authority-blockers.json`, `publication-authority.json`, and `upstream-distribution-target.json`; write `target/release-gates/perfect-refactor-unblock-packet.json`.

- [ ] **Step 4: Include packet in runtime smoke and release candidate**

Call the new script after `perfect-refactor-claim.json` is generated and add `perfect_refactor_unblock_packet` to `release-candidate.json`.

- [ ] **Step 5: Verify GREEN**

Run: `cargo test --test perfect_refactor_unblock_packet`

Expected: PASS for TEST-22.1.1 through TEST-22.1.5.

- [ ] **Step 6: Commit GREEN**

```bash
git add src/release.rs scripts/release/perfect-refactor-unblock-packet.sh scripts/release/runtime-smoke.sh docs/release.md docs/compatibility/matrix.md docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md tests/perfect_refactor_unblock_packet.rs
git commit -m "feat(release): add perfect refactor unblock packet gate"
```

### Task 3: S2V Verification And Completion

**Files:**
- Modify: `docs/specs/tasks/task-22.1-authority-unblock-packet-gate.md`
- Modify: `docs/specs/phases/phase-22-perfect-refactor-unblock-packet.md`
- Modify: `docs/s2v-adapter.md`

- [ ] **Step 1: Run task §9 verification**

Run:

```bash
"C:\Program Files\Git\bin\bash.exe" -lc 'source docs/s2v/scripts/lib/preflight.sh; source docs/s2v/scripts/lib/verify.sh; TASK_SPEC="docs/specs/tasks/task-22.1-authority-unblock-packet-gate.md"; VERIFY_KEYS="$(s2v_extract_verify_keys "$TASK_SPEC")"; s2v_verify_full "$VERIFY_KEYS"; s2v_coverage_threshold_guard "$TASK_SPEC"'
```

Expected: all keys in task §9 pass.

- [ ] **Step 2: Backfill task §10 and mark task Done**

Record completion date, changed files, RED/GREEN commits, §9 results, remaining risks, and downstream impact.

- [ ] **Step 3: Commit task completion**

```bash
git add docs/specs/tasks/task-22.1-authority-unblock-packet-gate.md docs/specs/phases/phase-22-perfect-refactor-unblock-packet.md docs/s2v-adapter.md
git commit -m "docs(spec): complete task 22.1 authority unblock packet gate"
```

- [ ] **Step 4: Run phase smoke and mark phase Done**

Run:

```bash
"C:\Program Files\Git\bin\bash.exe" -lc 'source docs/s2v/scripts/lib/preflight.sh; source docs/s2v/scripts/lib/verify.sh; s2v_preflight_phase docs/specs/phases/phase-22-perfect-refactor-unblock-packet.md; s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"'
```

Expected: phase preflight passes and 9-key smoke passes.

- [ ] **Step 5: Commit phase completion and push**

```bash
git add docs/specs/phases/phase-22-perfect-refactor-unblock-packet.md docs/s2v-adapter.md
git commit -m "docs(spec): phase 22 smoke passed and status done"
git push origin master
```
