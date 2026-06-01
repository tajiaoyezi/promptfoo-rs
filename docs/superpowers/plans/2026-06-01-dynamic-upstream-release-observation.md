# Dynamic Upstream Release Observation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace hard-coded GitHub latest release observation in the upstream distribution target gate with dynamic latest release metadata.

**Architecture:** Keep the existing Rust distribution target model and shell gate. Add fixture-driven latest release tag resolution in `scripts/release/upstream-distribution-target.sh`, then verify the generated `github.source` and release classification from Rust integration tests.

**Tech Stack:** Rust integration tests, Bash, Node.js JSON parsing / HTTPS, existing S2V release gate scripts.

---

### Task 1: Spec And Baseline

**Files:**
- Modify: `docs/prds/promptfoo-rs.prd.md`
- Modify: `docs/s2v-adapter.md`
- Create: `docs/specs/phases/phase-23-dynamic-upstream-release-observation.md`
- Create: `docs/specs/tasks/task-23.1-dynamic-github-latest-release-observation.md`
- Modify: `test/features/perfect-refactor-parity.feature`

- [ ] **Step 1: Run phase/task preflight**

Run:

```bash
"C:\Program Files\Git\bin\bash.exe" -lc 'source docs/s2v/scripts/lib/preflight.sh; s2v_preflight_ready docs/specs/tasks/task-23.1-dynamic-github-latest-release-observation.md; s2v_preflight_phase docs/specs/phases/phase-23-dynamic-upstream-release-observation.md'
```

Expected: both preflight checks pass.

- [ ] **Step 2: Commit spec**

Run:

```bash
git add docs/prds/promptfoo-rs.prd.md docs/s2v-adapter.md docs/specs/phases/phase-23-dynamic-upstream-release-observation.md docs/specs/tasks/task-23.1-dynamic-github-latest-release-observation.md test/features/perfect-refactor-parity.feature docs/superpowers/plans/2026-06-01-dynamic-upstream-release-observation.md
git commit -m "docs(spec): add phase 23 dynamic upstream release observation"
```

Expected: docs-only S2V spec commit.

### Task 2: RED Test

**Files:**
- Modify: `tests/upstream_distribution_target_gate.rs`

- [ ] **Step 1: Add failing fixture test**

Add a test that writes:

```json
{"tag_name":"0.122.0","target_commitish":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}
```

and an `ls-remote` fixture containing `refs/tags/0.122.0`. The test runs `UPSTREAM_GITHUB_RELEASE_FILE`, `UPSTREAM_NPM_VIEW_FILE`, and `UPSTREAM_LS_REMOTE_FILE` through `scripts/release/upstream-distribution-target.sh`.

- [ ] **Step 2: Assert RED**

Run:

```bash
cargo test --test upstream_distribution_target_gate test_23_1_1_script_uses_dynamic_latest_release_metadata -- --nocapture
```

Expected before implementation: FAIL because `github.source` still mentions `refs/tags/code-scan-action-0.1.7` instead of the dynamic latest release ref.

- [ ] **Step 3: Commit RED**

Run:

```bash
git add tests/upstream_distribution_target_gate.rs
git commit -m "test(compatibility): add SCEN-23.1.1 dynamic release observation RED test"
```

Expected: failing test committed.

### Task 3: GREEN Implementation

**Files:**
- Modify: `scripts/release/upstream-distribution-target.sh`
- Modify: `docs/compatibility/target-policy.md`
- Modify: `docs/compatibility/matrix.md`
- Modify: `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`
- Modify: `BLOCKED-task-22.1-perfect-refactor-external-authority.md`

- [ ] **Step 1: Implement latest release tag resolution**

Change the script to:

```bash
release_tmp="$(mktemp)"
if [ -n "${UPSTREAM_GITHUB_RELEASE_FILE:-}" ]; then
  cp "$UPSTREAM_GITHUB_RELEASE_FILE" "$release_tmp"
else
  node <<'NODE' > "$release_tmp"
const https = require('https');
https.get(
  'https://api.github.com/repos/promptfoo/promptfoo/releases/latest',
  { headers: { 'user-agent': 'promptfoo-rs-release-gate' } },
  (res) => {
    let body = '';
    res.on('data', (chunk) => { body += chunk; });
    res.on('end', () => {
      if (res.statusCode < 200 || res.statusCode >= 300) {
        console.error(`GitHub latest release lookup failed: ${res.statusCode}`);
        process.exit(1);
      }
      process.stdout.write(body);
    });
  },
).on('error', (error) => {
  console.error(error.message);
  process.exit(1);
});
NODE
fi
```

Then parse `tagName` or `tag_name`, query `git ls-remote ... "refs/tags/$latest_release_tag"`, and write that dynamic ref into `github.source`.

- [ ] **Step 2: Run GREEN target**

Run:

```bash
cargo test --test upstream_distribution_target_gate test_23_1_1_script_uses_dynamic_latest_release_metadata -- --nocapture
```

Expected: PASS.

- [ ] **Step 3: Commit GREEN**

Run:

```bash
git add scripts/release/upstream-distribution-target.sh docs/compatibility/target-policy.md docs/compatibility/matrix.md docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md BLOCKED-task-22.1-perfect-refactor-external-authority.md
git commit -m "feat(compatibility): observe dynamic upstream latest release"
```

Expected: implementation commit.

### Task 4: Verification And Completion

**Files:**
- Modify: `docs/specs/tasks/task-23.1-dynamic-github-latest-release-observation.md`
- Modify: `docs/specs/phases/phase-23-dynamic-upstream-release-observation.md`
- Modify: `docs/s2v-adapter.md`

- [ ] **Step 1: Run task §9**

Run:

```bash
"C:\Program Files\Git\bin\bash.exe" -lc 'source docs/s2v/scripts/lib/preflight.sh; source docs/s2v/scripts/lib/verify.sh; TASK_SPEC="docs/specs/tasks/task-23.1-dynamic-github-latest-release-observation.md"; VERIFY_KEYS="$(s2v_extract_verify_keys "$TASK_SPEC")"; s2v_verify_full "$VERIFY_KEYS"; s2v_coverage_threshold_guard "$TASK_SPEC"'
```

Expected: all listed task verification keys pass.

- [ ] **Step 2: Backfill task and phase**

Update task §10 with date, files, commits, verification result, residual blockers, and downstream impact. Mark task Done. Run phase smoke and mark phase Done.

- [ ] **Step 3: Commit docs completion and push**

Run:

```bash
git add docs/specs/tasks/task-23.1-dynamic-github-latest-release-observation.md docs/specs/phases/phase-23-dynamic-upstream-release-observation.md docs/s2v-adapter.md
git commit -m "docs(spec): complete task 23.1 dynamic release observation"
git push origin master
```

Expected: clean `master` synced with `origin/master`.
