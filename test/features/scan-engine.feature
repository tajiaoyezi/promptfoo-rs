# language: en
# Maps to:
#   - docs/specs/tasks/task-8.2-scan-audit-sarif.md

Feature: scan-engine
  In order to migrate existing promptfoo workflows to promptfoo-rs
  As a promptfoo-rs maintainer
  I want scan-engine behavior to be specified, tested, and traceable to S2V task specs

  # Maps to: docs/specs/tasks/task-8.2-scan-audit-sarif.md
  Scenario: SCEN-8.2.1 - scan 命令输出 finding schema snapshot
    Given a promptfoo 0.121.13 compatibility fixture for scan-engine
    When promptfoo-rs executes task 8.2 behavior
    Then TEST-8.2.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-8.2-scan-audit-sarif.md
  Scenario: SCEN-8.2.2 - SARIF writer 通过 schema fixture
    Given a promptfoo 0.121.13 compatibility fixture for scan-engine
    When promptfoo-rs executes task 8.2 behavior
    Then TEST-8.2.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-8.2-scan-audit-sarif.md
  Scenario: SCEN-8.2.3 - 误报率不作为 1.0 gate 但 known limitation 登记
    Given a promptfoo 0.121.13 compatibility fixture for scan-engine
    When promptfoo-rs executes task 8.2 behavior
    Then TEST-8.2.3 records the expected observable result
