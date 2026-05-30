# language: en
# Maps to:
#   - docs/specs/tasks/task-4.2-assertion-engine.md
#   - docs/specs/tasks/task-4.3-custom-assertion-contracts.md

Feature: assertion-engine
  In order to migrate existing promptfoo workflows to promptfoo-rs
  As a promptfoo-rs maintainer
  I want assertion-engine behavior to be specified, tested, and traceable to S2V task specs

  # Maps to: docs/specs/tasks/task-4.2-assertion-engine.md
  Scenario: SCEN-4.2.1 - equals/contains/regex/json/schema 等 deterministic assertions 有 golden diff
    Given a promptfoo 0.121.13 compatibility fixture for assertion-engine
    When promptfoo-rs executes task 4.2 behavior
    Then TEST-4.2.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-4.2-assertion-engine.md
  Scenario: SCEN-4.2.2 - model-graded assertion 比较 prompt construction、threshold、score parsing 和 metadata schema
    Given a promptfoo 0.121.13 compatibility fixture for assertion-engine
    When promptfoo-rs executes task 4.2 behavior
    Then TEST-4.2.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-4.2-assertion-engine.md
  Scenario: SCEN-4.2.3 - assertion aggregation 输出稳定 pass/fail/error shape
    Given a promptfoo 0.121.13 compatibility fixture for assertion-engine
    When promptfoo-rs executes task 4.2 behavior
    Then TEST-4.2.3 records the expected observable result

  # Maps to: docs/specs/tasks/task-4.3-custom-assertion-contracts.md
  Scenario: SCEN-4.3.1 - JS/Python/Shell custom contract 在矩阵中标 P0/P1 与 bridge 状态
    Given a promptfoo 0.121.13 compatibility fixture for assertion-engine
    When promptfoo-rs executes task 4.3 behavior
    Then TEST-4.3.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-4.3-custom-assertion-contracts.md
  Scenario: SCEN-4.3.2 - 未启用 allow-scripts 时返回稳定拒绝错误
    Given a promptfoo 0.121.13 compatibility fixture for assertion-engine
    When promptfoo-rs executes task 4.3 behavior
    Then TEST-4.3.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-4.3-custom-assertion-contracts.md
  Scenario: SCEN-4.3.3 - custom assertion 输入输出 schema 有 snapshot
    Given a promptfoo 0.121.13 compatibility fixture for assertion-engine
    When promptfoo-rs executes task 4.3 behavior
    Then TEST-4.3.3 records the expected observable result
