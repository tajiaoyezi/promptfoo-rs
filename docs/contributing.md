# Contributing

Contributions follow S2V. Read `AGENTS.md`, `docs/s2v-adapter.md`, and the relevant task spec before changing behavior.

## Development Flow

1. Find the task spec under `docs/specs/tasks/`.
2. Confirm the spec is Ready.
3. Run the baseline green check from `AGENTS.md`.
4. Write the RED test first.
5. Implement the minimum GREEN change.
6. Run the task verification keys.
7. Backfill completion notes and update status.

## Compatibility Matrix Updates

Any behavior that changes promptfoo compatibility must update `docs/compatibility/matrix.md`. P0 gaps block stable release unless fixed or classified; P1 needs snapshot evidence; P2 needs a visible known-gap reason.

## Security Defaults

Custom scripts are disabled unless explicitly authorized. Do not add default upload, share, cloud, or credential behavior without an ADR and a task spec.

## Release Contributions

Release changes must preserve the compatibility release gate. If the gate is blocked, stable release must remain disabled and the release may only be prerelease or nightly.
