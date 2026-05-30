# language: en
# Maps to:
#   - docs/specs/tasks/task-9.2-node-api-wrapper.md

Feature: node-api-wrapper
  In order to migrate existing promptfoo workflows to promptfoo-rs
  As a promptfoo-rs maintainer
  I want node-api-wrapper behavior to be specified, tested, and traceable to S2V task specs

  # Maps to: docs/specs/tasks/task-9.2-node-api-wrapper.md
  Scenario: SCEN-9.2.1 - Node API wrapper 不复写 eval 业务逻辑
    Given a promptfoo 0.121.13 compatibility fixture for node-api-wrapper
    When promptfoo-rs executes task 9.2 behavior
    Then TEST-9.2.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-9.2-node-api-wrapper.md
  Scenario: SCEN-9.2.2 - 参数、错误、结果 schema 有 contract snapshots
    Given a promptfoo 0.121.13 compatibility fixture for node-api-wrapper
    When promptfoo-rs executes task 9.2 behavior
    Then TEST-9.2.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-9.2-node-api-wrapper.md
  Scenario: SCEN-9.2.3 - wrapper/core drift test 进入 release gate
    Given a promptfoo 0.121.13 compatibility fixture for node-api-wrapper
    When promptfoo-rs executes task 9.2 behavior
    Then TEST-9.2.3 records the expected observable result
