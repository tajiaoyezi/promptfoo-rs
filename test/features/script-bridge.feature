# language: en
# Maps to:
#   - docs/specs/tasks/task-9.1-script-bridge-sandbox.md

Feature: script-bridge
  In order to migrate existing promptfoo workflows to promptfoo-rs
  As a promptfoo-rs maintainer
  I want script-bridge behavior to be specified, tested, and traceable to S2V task specs

  # Maps to: docs/specs/tasks/task-9.1-script-bridge-sandbox.md
  Scenario: SCEN-9.1.1 - 未启用 allow-scripts 时拒绝执行并返回稳定错误
    Given a promptfoo 0.121.13 compatibility fixture for script-bridge
    When promptfoo-rs executes task 9.1 behavior
    Then TEST-9.1.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-9.1-script-bridge-sandbox.md
  Scenario: SCEN-9.1.2 - 启用后子进程输入输出和超时有 fixture
    Given a promptfoo 0.121.13 compatibility fixture for script-bridge
    When promptfoo-rs executes task 9.1 behavior
    Then TEST-9.1.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-9.1-script-bridge-sandbox.md
  Scenario: SCEN-9.1.3 - env allowlist 与 secret redaction 有 tests
    Given a promptfoo 0.121.13 compatibility fixture for script-bridge
    When promptfoo-rs executes task 9.1 behavior
    Then TEST-9.1.3 records the expected observable result
