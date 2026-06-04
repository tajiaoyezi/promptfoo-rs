# Contributing to promptfoo-rs

Contributions are welcome: code, docs, tests, compatibility fixtures, audits, bug reports, and issue triage.

This project follows S2V. If a change affects behavior, compatibility, release gates, security, or public documentation claims, read the relevant spec before editing.

## First-time setup

```bash
git clone https://github.com/tajiaoyezi/promptfoo-rs.git
cd promptfoo-rs
cargo build --workspace
cargo test --workspace
```

For viewer and npm wrapper work:

```bash
corepack enable
cd viewer && pnpm install --frozen-lockfile
cd ../npm && pnpm install --frozen-lockfile
```

## Required reading

Before behavior changes:

1. [AGENTS.md](AGENTS.md)
2. [docs/s2v-adapter.md](docs/s2v-adapter.md)
3. Relevant task spec under [docs/specs/tasks/](docs/specs/tasks/)
4. Relevant ADR under [docs/decisions/](docs/decisions/)
5. Relevant BDD feature under [test/features/](test/features/)

For docs-only changes, still check [docs/release.md](docs/release.md) and [docs/compatibility/matrix.md](docs/compatibility/matrix.md) before writing readiness or parity claims.

## Development flow

For implementation tasks:

1. Confirm the task spec is `Ready`.
2. Run baseline green from `AGENTS.md`.
3. Add the RED test first.
4. Implement the minimum GREEN change.
5. Run the task's verification keys.
6. Backfill task completion notes.
7. Move task status to `Done` only after verification passes.

Maintainers may work directly on `master` because the current collaboration tier is `solo`. External contributors should use a branch and open a pull request.

## Verification

Common checks:

```bash
cargo check --workspace
cargo test --workspace
cargo build --workspace
```

Full S2V verification:

```bash
source docs/s2v/scripts/lib/preflight.sh
source docs/s2v/scripts/lib/verify.sh
s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"
```

Windows:

```powershell
& 'C:\Program Files\Git\bin\bash.exe' -lc 'source docs/s2v/scripts/lib/preflight.sh; source docs/s2v/scripts/lib/verify.sh; s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"'
```

## Compatibility changes

Any promptfoo compatibility change must update the compatibility evidence:

- P0 behavior needs runnable fixture or golden-diff evidence.
- P1 behavior needs snapshot or protocol evidence.
- P2 behavior needs a clear unsupported/later/bridge-backed reason.
- New unknowns must stay visible as blockers; do not delete them from the matrix.

## Documentation claims

Do not write:

- "bug-free"
- "no potential bugs"
- "complete replacement for latest promptfoo"
- "public stable release is available"

unless the release gate artifacts actually allow the claim.

Allowed wording today is:

> no known release-blocking defects under the declared gates

## Security

Never commit secrets, provider API keys, registry tokens, account credentials, or private service data. Report vulnerabilities privately; see [SECURITY.md](SECURITY.md).

## License

By contributing, you agree that your contribution is licensed under the MIT License used by this repository. No CLA is required.
