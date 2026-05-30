# language: en
# Maps to:
#   - docs/specs/tasks/task-2.2-config-loader.md

Feature: config-loader
  In order to migrate existing promptfoo workflows to promptfoo-rs
  As a promptfoo-rs maintainer
  I want config-loader behavior to be specified, tested, and traceable to S2V task specs

  # Maps to: docs/specs/tasks/task-2.2-config-loader.md
  Scenario: SCEN-2.2.1 - 配置 loader 能输出归一化 config model
    Given a promptfoo 0.121.13 compatibility fixture for config-loader
    When promptfoo-rs executes task 2.2 behavior
    Then TEST-2.2.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-2.2-config-loader.md
  Scenario: SCEN-2.2.2 - 路径、env、vars、prompts、tests 解析有 Windows/Linux fixture
    Given a promptfoo 0.121.13 compatibility fixture for config-loader
    When promptfoo-rs executes task 2.2 behavior
    Then TEST-2.2.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-2.2-config-loader.md
  Scenario: SCEN-2.2.3 - 解析差异能记录到 compatibility report
    Given a promptfoo 0.121.13 compatibility fixture for config-loader
    When promptfoo-rs executes task 2.2 behavior
    Then TEST-2.2.3 records the expected observable result
