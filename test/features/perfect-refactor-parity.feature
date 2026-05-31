# language: en
# Maps to:
#   - docs/specs/tasks/task-11.1-current-upstream-target-policy.md
#   - docs/specs/tasks/task-11.2-item-level-capability-inventory.md
#   - docs/specs/tasks/task-11.3-compatibility-matrix-expansion.md
#   - docs/specs/tasks/task-12.1-p0-fixture-corpus.md
#   - docs/specs/tasks/task-12.2-executable-upstream-rs-runner.md
#   - docs/specs/tasks/task-12.3-golden-diff-ci-release-gate.md
#   - docs/specs/tasks/task-13.1-command-flag-parity.md
#   - docs/specs/tasks/task-13.2-eval-output-cache-parity.md
#   - docs/specs/tasks/task-14.1-provider-assertion-inventory-parity.md
#   - docs/specs/tasks/task-14.2-redteam-plugin-strategy-parity.md
#   - docs/specs/tasks/task-15.1-viewer-node-packaging-release.md
#   - docs/specs/tasks/task-15.2-performance-security-observability-gates.md
#   - docs/specs/tasks/task-16.1-cli-command-behavior-closure.md
#   - docs/specs/tasks/task-16.2-measured-release-gate-reports.md
#   - docs/specs/tasks/task-16.3-source-extracted-inventory-real-upstream-smoke.md

Feature: perfect refactor parity
  In order to make promptfoo-rs a complete audited refactor of promptfoo
  As a promptfoo-rs maintainer
  I want every audited parity gap to map to executable S2V tasks and verification evidence

  Scenario: SCEN-11.1.1 - compatibility target policy separates frozen baseline from moving upstream
    Given the promptfoo-rs audit package records both baseline and current upstream evidence
    When the compatibility target policy is generated
    Then TEST-11.1.1 proves stable releases reference exactly one compatibility target

  Scenario: SCEN-11.2.1 - upstream item-level inventory covers commands providers assertions redteam outputs configs and APIs
    Given upstream promptfoo source docs and examples are available
    When the inventory extractor runs
    Then TEST-11.2.1 proves every discovered item has a stable inventory id and source reference

  Scenario: SCEN-11.3.1 - expanded compatibility matrix has no silent omissions
    Given the item-level inventory exists
    When the matrix expansion check runs
    Then TEST-11.3.1 proves every inventory item has P0 P1 or P2 status and verification owner

  Scenario: SCEN-12.1.1 - P0 fixture corpus contains at least 50 tracked fixtures
    Given the expanded P0 matrix rows exist
    When the fixture corpus count check runs
    Then TEST-12.1.1 proves at least 50 P0 fixtures are tracked with metadata

  Scenario: SCEN-12.2.1 - executable runner creates upstream rs normalized and diff artifacts
    Given a tracked compatibility fixture
    When the executable golden diff runner runs
    Then TEST-12.2.1 proves upstream promptfoo and promptfoo-rs artifacts are persisted and normalized

  Scenario: SCEN-12.3.1 - CI release gate blocks stable on P0 bug or unclassified diff
    Given persisted golden diff artifacts
    When the release gate evaluates P0 findings
    Then TEST-12.3.1 proves stable release is blocked for bug or unclassified diffs

  Scenario: SCEN-13.1.1 - CLI commands and flags are either compatible or explicitly classified
    Given the upstream command and flag inventory
    When promptfoo-rs exposes CLI help and command behavior
    Then TEST-13.1.1 proves every upstream command path maps to compatible unsupported later or blocked evidence

  Scenario: SCEN-13.2.1 - eval output cache resume and retry behavior is golden diffed
    Given P0 eval fixtures with output and cache settings
    When promptfoo-rs runs eval compatibility fixtures
    Then TEST-13.2.1 proves stdout stderr exit code output files and cache state match or are classified

  Scenario: SCEN-14.1.1 - provider and assertion inventory parity has no unclassified P0 item
    Given provider and assertion inventory rows
    When provider and assertion parity checks run
    Then TEST-14.1.1 proves every P0 item has native bridge or blocking evidence

  Scenario: SCEN-14.2.1 - redteam plugin and strategy inventory parity has no missing reason
    Given redteam plugin and strategy inventory rows
    When redteam parity checks run
    Then TEST-14.2.1 proves P0 rows have fixtures and P2 rows have reasons

  Scenario: SCEN-15.1.1 - viewer and node wrapper packages build and smoke test
    Given viewer and npm package metadata exists
    When release packaging verification runs
    Then TEST-15.1.1 proves viewer browser smoke and node wrapper smoke pass

  Scenario: SCEN-15.2.1 - release candidate gates cover performance security and runtime smoke
    Given a release candidate build
    When full release verification runs
    Then TEST-15.2.1 proves lint integration e2e coverage runtime smoke performance and security gates are enforced

  Scenario: SCEN-16.1.1 - CLI later commands become executable local compatibility behavior
    Given local result and cache artifacts exist
    When view cache import and export commands run
    Then TEST-16.1.1 proves those commands return stable local JSON behavior instead of later placeholders

  Scenario: SCEN-16.2.1 - release reports are measured or derived from this runtime smoke
    Given runtime smoke executes release candidate checks
    When performance security and release candidate reports are written
    Then TEST-16.2.1 proves stable decisions come from measured gate evidence rather than fixed JSON literals

  Scenario: SCEN-16.3.1 - real upstream smoke and source inventory evidence back the matrix
    Given frozen promptfoo upstream artifacts are reachable
    When source extraction and real upstream smoke run
    Then TEST-16.3.1 proves matrix and golden artifacts are based on upstream promptfoo 0.121.13 evidence
