# Compatibility Fixtures

**Status**: Ready
**Basis**: PRD §Compatibility Harness Design / §Success Metrics, ADR-006, ADR-007, task-12.1.

The P0 corpus lives under `compatibility/fixtures/**/fixture.yaml`. Each manifest links to one or more item-level matrix IDs, declares mock or recorded provider mode, lists normalization rules, and states whether it blocks stable release.

Task 12.1 establishes the corpus metadata and minimum count. Task 12.2 owns execution artifacts, and task 12.3 owns CI/release gate wiring.
