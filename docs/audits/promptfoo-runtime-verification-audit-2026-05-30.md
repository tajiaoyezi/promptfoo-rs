# promptfoo runtime verification audit - 2026-05-30

**Status**: Audit attachment
**Parent audit**: `docs/audits/promptfoo-perfect-refactor-audit-2026-05-30.md`
**Purpose**: record runtime verification evidence for the current `promptfoo-rs` worktree and distinguish local green tests from upstream parity.

## Snapshot

| Item | Value |
|---|---|
| Local branch before this attachment | `master` |
| Local HEAD before this attachment | `e0c77f488228058f1e9057ddc0b9703f6ad5870c` |
| Node | `v26.1.0` |
| npm | `11.13.0` |
| Cargo | `cargo 1.95.0` |
| rustc | `rustc 1.95.0` |

## Local S2V Verification

Command:

```powershell
& 'C:\Program Files\Git\bin\bash.exe' -lc 'cd /h/devlopment/code/promptfoo-rs && export CARGO_INCREMENTAL=0 && source docs/s2v/scripts/lib/preflight.sh && source docs/s2v/scripts/lib/verify.sh && s2v_verify_full "install typecheck unit-test"'
```

Result:

| Check | Result |
|---|---|
| install | passed |
| typecheck | passed |
| unit-test | passed |
| S2V helper summary | `✅ §9 Verification 全套通过（共 3 项）` |

Observed test scope:

| Test class | Observed result |
|---|---|
| `src/lib.rs` unit tests | 0 tests |
| `src/main.rs` unit tests | 0 tests |
| Rust integration test files | 22 files |
| Rust integration tests | 66 passed, 0 failed |
| Doc tests | 0 tests |

Audit interpretation: the current local implementation is internally green for the adapter's active verification keys. This does not prove upstream promptfoo parity because lint, integration, E2E, coverage, runtime smoke, and full compatibility golden diff are not global executable gates in `docs/s2v-adapter.md`.

## Local CLI Runtime Smoke

Commands were run with `CARGO_INCREMENTAL=0` to avoid Windows incremental-compilation warning noise.

| Command | Exit | stdout | stderr | Audit interpretation |
|---|---:|---|---|---|
| `cargo run --quiet -- --help` | 0 | shows `Promptfoo-compatible Rust CLI skeleton` and local commands | empty | CLI is explicitly still a skeleton |
| `cargo run --quiet -- view` | 0 | empty | empty | no-op success, not upstream-equivalent viewer behavior |
| `cargo run --quiet -- cache` | 0 | empty | empty | no-op success, not upstream-equivalent cache management |
| `cargo run --quiet -- import` | 0 | empty | empty | no-op success, not upstream-equivalent import behavior |
| `cargo run --quiet -- export` | 0 | empty | empty | no-op success, not upstream-equivalent export behavior |

This directly matches `src/cli.rs`, where `View`, `Cache`, `Import`, and `Export` return `ExitCode::SUCCESS` without behavior.

## Upstream Runtime Evidence Attempt

Attempted command group:

```powershell
npx --yes promptfoo@0.121.13 --help
npx --yes promptfoo@0.121.13 view --help
npx --yes promptfoo@0.121.13 cache --help
npx --yes promptfoo@0.121.13 import --help
npx --yes promptfoo@0.121.13 export --help
```

Result: timed out after 184 seconds, so this audit does not use the `npx` help attempt as semantic upstream runtime evidence.

Usable npm manifest evidence:

```json
{
  "bin": {
    "promptfoo": "dist/src/entrypoint.js",
    "pf": "dist/src/entrypoint.js"
  },
  "version": "0.121.13",
  "gitHead": "4860e990c7e9a2f8f677173fb92cf9867b34d03f"
}
```

Usable upstream source evidence remains the command registration in `src/main.ts` and the source inventory recorded in `docs/audits/promptfoo-upstream-inventory-gap-2026-05-30.md`.

## Runtime Verdict

The runtime audit strengthens the parent conclusion:

- Local S2V verification is green for the active local test suite.
- Several user-visible CLI commands that exist in local help still behave as empty success placeholders.
- The successful local tests prove the current scoped implementation, not a complete promptfoo refactor.
- No runtime evidence was obtained that proves upstream-equivalent behavior for `view`, `cache`, `import`, `export`, provider long tail, assertion long tail, redteam long tail, or compatibility golden diff.

Therefore the active goal must remain incomplete.
